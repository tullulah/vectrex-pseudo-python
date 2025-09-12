use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::*;

use crate::lexer::lex;
use crate::parser::parse_with_filename;

pub async fn run_stdio_server() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::build(|client| Backend { client }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    client: Client,
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
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions { resolve_provider: None, trigger_characters: None, work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }),
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
        let diags = compute_diagnostics(&uri, &text);
        let _ = self.client.publish_diagnostics(uri, diags, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
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
}

// Binary entry (used by cargo run --bin vpy_lsp)
pub async fn run() -> anyhow::Result<()> { run_stdio_server().await; Ok(()) }
