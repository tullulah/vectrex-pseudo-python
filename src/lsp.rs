use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::*;

use crate::lexer::{lex, TokenKind};
use crate::parser::parse_with_filename;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn run_stdio_server() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::build(|client| Backend { client, docs: Arc::new(Mutex::new(HashMap::new())) }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    client: Client,
    docs: Arc<Mutex<HashMap<Url, String>>>,
}


fn compute_diagnostics(uri: &Url, text: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    match lex(text) {
        Ok(tokens) => {
            if let Err(e) = parse_with_filename(&tokens, uri.path()) {
                // Parse error format: filename:line:col: error: message
                let msg = e.to_string();
                // Try to extract line/col
                let (line, col, detail) = if let Some((_, rest)) = msg.split_once(":") {
                    // filename already consumed
                    let mut parts = rest.split(':');
                    let line_s = parts.next().unwrap_or("0");
                    let col_s = parts.next().unwrap_or("0");
                    let _err = parts.next(); // the word ' error' maybe
                    let remaining = parts.collect::<Vec<_>>().join(":");
                    (line_s.parse::<u32>().unwrap_or(0).saturating_sub(1), col_s.parse::<u32>().unwrap_or(0).saturating_sub(1), remaining.trim().to_string())
                } else { (0,0,msg) };
                diags.push(Diagnostic {
                    range: Range { start: Position { line, character: col }, end: Position { line, character: col + 1 } },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("vpy".into()),
                    message: detail,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            } else {
                // Example heuristic: warn on POLYGON 2
                for (i, line) in text.lines().enumerate() {
                    if line.contains("POLYGON") && line.contains(" 2") {
                        diags.push(Diagnostic {
                            range: Range { start: Position { line: i as u32, character: 0 }, end: Position { line: i as u32, character: line.len() as u32 } },
                            severity: Some(DiagnosticSeverity::WARNING),
                            code: None,
                            code_description: None,
                            source: Some("vpy".into()),
                            message: "POLYGON count 2 genera lista degenerada (usa >=3 o un RECT delgado).".into(),
                            related_information: None,
                            tags: None,
                            data: None,
                        });
                    }
                }
            }
        }
        Err(err) => {
            // Lexer error
            let msg = err.to_string();
            diags.push(Diagnostic {
                range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 1 } },
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("vpy".into()),
                message: msg,
                related_information: None,
                tags: None,
                data: None,
            });
        }
    }
    diags
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        let legend = SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::KEYWORD,        // 0
                SemanticTokenType::FUNCTION,       // 1
                SemanticTokenType::VARIABLE,       // 2
                SemanticTokenType::PARAMETER,      // 3 (future)
                SemanticTokenType::NUMBER,         // 4
                SemanticTokenType::STRING,         // 5
                SemanticTokenType::OPERATOR,       // 6
            ],
            token_modifiers: vec![],
        };
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions { resolve_provider: None, trigger_characters: None, work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: Default::default(),
                    legend,
                    range: None,
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                })),
                // semantic tokens announced later
                ..Default::default()
            },
            server_info: Some(ServerInfo { name: "vpy-lsp".into(), version: Some("0.1.0".into()) }),
        })
    }

    async fn initialized(&self, _: InitializedParams) { let _ = self.client.log_message(MessageType::INFO, "VPy LSP inicializado").await; }

    async fn shutdown(&self) -> LspResult<()> { Ok(()) }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.docs.lock().unwrap().insert(uri.clone(), text.clone());
        let diags = compute_diagnostics(&uri, &text);
        let _ = self.client.publish_diagnostics(uri, diags, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.docs.lock().unwrap().insert(uri.clone(), change.text.clone());
            let diags = compute_diagnostics(&uri, &change.text);
            let _ = self.client.publish_diagnostics(uri, diags, None).await;
        }
    }

    async fn completion(&self, _: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let items = [
            "VECTORLIST","INTENSITY","ORIGIN","MOVE","RECT","POLYGON","CIRCLE","ARC","SPIRAL","CONST","VAR","META","TITLE","MUSIC","COPYRIGHT","def","for","while","if","switch"
        ].iter().map(|s| CompletionItem { label: s.to_string(), kind: Some(CompletionItemKind::KEYWORD), ..Default::default() }).collect();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let docs = self.docs.lock().unwrap();
        let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) };
        drop(docs);
        let lines: Vec<&str> = text.lines().collect();
        let mut data: Vec<SemanticToken> = Vec::new();
        if let Ok(tokens) = lex(&text) {
            // collect raw tokens with absolute positions
            let mut raw: Vec<(u32,u32,u32,u32)> = Vec::new(); // line, col, length, type
            for tk in tokens.iter() {
                let line1 = tk.line; if line1 == 0 { continue; }
                let line0 = (line1 - 1) as u32;
                let line_str = lines.get(line0 as usize).copied().unwrap_or("");
                let base_col = tk.col as u32; // note: indentation lost in lexer; best-effort
                let (ttype, length) = match &tk.kind {
                    TokenKind::Def | TokenKind::If | TokenKind::Elif | TokenKind::Else | TokenKind::For | TokenKind::In | TokenKind::Range | TokenKind::Return | TokenKind::While | TokenKind::Break | TokenKind::Continue | TokenKind::Let | TokenKind::Const | TokenKind::Var | TokenKind::VectorList | TokenKind::Switch | TokenKind::Case | TokenKind::Default | TokenKind::Meta | TokenKind::And | TokenKind::Or | TokenKind::Not | TokenKind::True | TokenKind::False => {
                        // derive keyword length from line text starting at col
                        let rest = &line_str.get(base_col as usize..).unwrap_or("");
                        let kw_len = rest.split(|c: char| !c.is_alphanumeric()).next().unwrap_or("").len() as u32;
                        (0, kw_len.max(2))
                    }
                    TokenKind::Identifier(s) => (2, s.len() as u32),
                    TokenKind::Number(n) => (4, n.to_string().len() as u32),
                    TokenKind::StringLit(s) => (5, (s.len() + 2) as u32),
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Amp | TokenKind::Pipe | TokenKind::Caret | TokenKind::Tilde | TokenKind::ShiftLeft | TokenKind::ShiftRight | TokenKind::Equal | TokenKind::EqEq | TokenKind::NotEq | TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge | TokenKind::Dot | TokenKind::Colon | TokenKind::Comma => (6, 1),
                    _ => continue,
                };
                raw.push((line0, base_col, length, ttype));
            }
            raw.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            // delta encode into SemanticToken structs
            let mut last_line = 0u32; let mut last_col = 0u32; let mut first = true;
            for (line, col, length, ttype) in raw {
                let delta_line = if first { line } else { line - last_line };
                let delta_start = if first { col } else if delta_line == 0 { col - last_col } else { col };
                data.push(SemanticToken { delta_line, delta_start, length, token_type: ttype, token_modifiers_bitset: 0 });
                last_line = line; last_col = col; first = false;
            }
        }
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data })))
    }
}

// Binary entry (used by cargo run --bin vpy_lsp)
pub async fn run() -> anyhow::Result<()> { run_stdio_server().await; Ok(()) }
