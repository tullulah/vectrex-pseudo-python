use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::*;

use crate::lexer::{lex, TokenKind};
use crate::parser::parse_with_filename;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn run_stdio_server() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::build(|client| Backend { client, docs: Arc::new(Mutex::new(HashMap::new())), locale: Arc::new(Mutex::new("en".to_string())) }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    client: Client,
    docs: Arc<Mutex<HashMap<Url, String>>>,
    locale: Arc<Mutex<String>>, // current locale ("en" default)
}

#[derive(Clone, Debug)]
struct SymbolDef {
    name: String,
    uri: Url,
    range: Range,
    kind: SymbolKind,
    detail: Option<String>,
}

// Very lightweight symbol index (only function definitions for now)
// Rebuilt on each semantic token pass / text change for simplicity (FULL sync).
// For performance later we could incrementalize.
type SymbolIndex = Arc<Mutex<Vec<SymbolDef>>>;

// Translation helper (static table). Extend later with external resource loading.
fn tr(locale: &str, key: &str) -> String {
    let l = if locale.starts_with("es") { "es" } else { "en" };
    let val = match (l, key) {
        ("en", "init.ready") => "VPy LSP initialized",
        ("es", "init.ready") => "VPy LSP inicializado",
        ("en", "diagnostic.polygon.degenerate") => "POLYGON count 2 produces a degenerate list (use >=3 or a thin RECT).",
        ("es", "diagnostic.polygon.degenerate") => "POLYGON count 2 genera lista degenerada (usa >=3 o un RECT delgado).",
        ("en", "symbol.user_function.detail") => "user-defined function",
        ("es", "symbol.user_function.detail") => "función definida por el usuario",
        ("en", "hover.user_function.line") => "Function `{}` defined at line {}",
        ("es", "hover.user_function.line") => "Función `{}` definida en línea {}",
        ("en", "doc.MOVE") => "MOVE x,y  - moves the beam to position (x,y) without drawing.",
        ("es", "doc.MOVE") => "MOVE x,y  - mueve el haz a la posición (x,y) sin dibujar.",
        ("en", "doc.RECT") => "RECT x,y,w,h  - draws a rectangle.",
        ("es", "doc.RECT") => "RECT x,y,w,h  - dibuja un rectángulo.",
        ("en", "doc.POLYGON") => "POLYGON n x1,y1 ...  - draws a polygon with n vertices.",
        ("es", "doc.POLYGON") => "POLYGON n x1,y1 ...  - dibuja un polígono de n vértices.",
        ("en", "doc.CIRCLE") => "CIRCLE r  - draws a circle of radius r.",
        ("es", "doc.CIRCLE") => "CIRCLE r  - dibuja un círculo de radio r.",
        ("en", "doc.ARC") => "ARC r startAngle endAngle  - draws an arc.",
        ("es", "doc.ARC") => "ARC r angIni angFin  - dibuja un arco.",
        ("en", "doc.SPIRAL") => "SPIRAL r turns  - draws a spiral.",
        ("es", "doc.SPIRAL") => "SPIRAL r vueltas  - dibuja una espiral.",
        ("en", "doc.ORIGIN") => "ORIGIN  - resets the origin (0,0).",
        ("es", "doc.ORIGIN") => "ORIGIN  - restablece el origen (0,0).",
        ("en", "doc.INTENSITY") => "INTENSITY val  - sets beam intensity.",
        ("es", "doc.INTENSITY") => "INTENSITY val  - fija la intensidad del haz.",
        ("en", "doc.PRINT_TEXT") => "PRINT_TEXT x,y,\"text\"  - shows vector text.",
        ("es", "doc.PRINT_TEXT") => "PRINT_TEXT x,y,\"texto\"  - muestra texto vectorial.",
        _ => key,
    };
    val.to_string()
}

fn builtin_doc(locale: &str, upper: &str) -> Option<String> {
    let key = match upper {
        "MOVE" => Some("doc.MOVE"),
        "RECT" => Some("doc.RECT"),
        "POLYGON" => Some("doc.POLYGON"),
        "CIRCLE" => Some("doc.CIRCLE"),
        "ARC" => Some("doc.ARC"),
        "SPIRAL" => Some("doc.SPIRAL"),
        "ORIGIN" => Some("doc.ORIGIN"),
        "INTENSITY" => Some("doc.INTENSITY"),
        "PRINT_TEXT" => Some("doc.PRINT_TEXT"),
        _ => None,
    }?;
    Some(tr(locale, key))
}


fn compute_diagnostics(uri: &Url, text: &str, locale: &str) -> Vec<Diagnostic> {
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
                            message: tr(locale, "diagnostic.polygon.degenerate"),
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
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        if let Some(loc) = params.locale.clone() { *self.locale.lock().unwrap() = loc; }
        // Order here defines the numeric indexes we later emit in semantic_tokens_full
        // Keep synchronized with the classifier constants below.
        let legend = SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::KEYWORD,        // 0
                SemanticTokenType::FUNCTION,       // 1
                SemanticTokenType::VARIABLE,       // 2
                SemanticTokenType::PARAMETER,      // 3 (reserved for future use: parameters)
                SemanticTokenType::NUMBER,         // 4
                SemanticTokenType::STRING,         // 5
                SemanticTokenType::OPERATOR,       // 6
                SemanticTokenType::ENUM_MEMBER,    // 7 (used for immutable CONSTANT like I_*)
            ],
            token_modifiers: vec![
                SemanticTokenModifier::READONLY,        // bit 0
                SemanticTokenModifier::DECLARATION,     // bit 1
                SemanticTokenModifier::DEFAULT_LIBRARY, // bit 2
            ],
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
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                // semantic tokens announced later
                ..Default::default()
            },
            server_info: Some(ServerInfo { name: "vpy-lsp".into(), version: Some("0.1.0".into()) }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
    let loc = self.locale.lock().unwrap().clone();
    let _ = self.client.log_message(MessageType::INFO, tr(&loc, "init.ready")).await;
    }

    async fn shutdown(&self) -> LspResult<()> { Ok(()) }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.docs.lock().unwrap().insert(uri.clone(), text.clone());
        let loc = self.locale.lock().unwrap().clone();
    let diags = compute_diagnostics(&uri, &text, &loc);
        let _ = self.client.publish_diagnostics(uri, diags, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.docs.lock().unwrap().insert(uri.clone(), change.text.clone());
            let loc = self.locale.lock().unwrap().clone();
            let diags = compute_diagnostics(&uri, &change.text, &loc);
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
        // We'll collect function definitions ("def name") while scanning tokens for semantics to power hover/definition.
        let mut defs: Vec<SymbolDef> = Vec::new();
        if let Ok(tokens) = lex(&text) {
            // Constants for legend indices (keep synchronized with legend order above)
            const KEYWORD: u32 = 0;
            const FUNCTION: u32 = 1;
            const VARIABLE: u32 = 2;
            const _PARAMETER: u32 = 3; // reserved
            const NUMBER: u32 = 4;
            const STRING: u32 = 5;
            const OPERATOR: u32 = 6;
            const ENUM_MEMBER: u32 = 7; // used for constants (I_*)
            // modifier bit positions must match legend order: READONLY=0, DECLARATION=1, DEFAULT_LIBRARY=2
            const MOD_READONLY: u32 = 1 << 0;
            const MOD_DECL: u32 = 1 << 1;
            const MOD_DEFAULT_LIB: u32 = 1 << 2;

            // Helper: keyword lexeme length mapping
            fn keyword_len(kind: &TokenKind) -> Option<usize> {
                Some(match kind {
                    TokenKind::Def => "def".len(),
                    TokenKind::If => "if".len(),
                    TokenKind::Elif => "elif".len(),
                    TokenKind::Else => "else".len(),
                    TokenKind::For => "for".len(),
                    TokenKind::In => "in".len(),
                    TokenKind::Range => "range".len(),
                    TokenKind::Return => "return".len(),
                    TokenKind::While => "while".len(),
                    TokenKind::Break => "break".len(),
                    TokenKind::Continue => "continue".len(),
                    TokenKind::Let => "let".len(),
                    TokenKind::Const => "const".len(),
                    TokenKind::Var => "var".len(),
                    TokenKind::VectorList => "vectorlist".len(),
                    TokenKind::Switch => "switch".len(),
                    TokenKind::Case => "case".len(),
                    TokenKind::Default => "default".len(),
                    TokenKind::Meta => "meta".len(),
                    TokenKind::And => "and".len(),
                    TokenKind::Or => "or".len(),
                    TokenKind::Not => "not".len(),
                    TokenKind::True => "True".len(),
                    TokenKind::False => "False".len(),
                    _ => return None,
                })
            }

            // collect raw tokens with absolute positions
            let mut raw: Vec<(u32,u32,u32,u32,u32)> = Vec::new(); // line, col, length, type, modifiers
            for (idx, tk) in tokens.iter().enumerate() {
                let line1 = tk.line; if line1 == 0 { continue; }
                let line0 = (line1 - 1) as u32;
                let line_str = lines.get(line0 as usize).copied().unwrap_or("");
                // compute indentation (spaces only supported) then real column = indent + tk.col
                let indent = line_str.chars().take_while(|c| *c==' ').count() as u32;
                let base_col = indent + tk.col as u32;

                // classify
                match &tk.kind {
                    k if keyword_len(k).is_some() => {
                        let length = keyword_len(k).unwrap() as u32;
                        raw.push((line0, base_col, length as u32, KEYWORD, 0));
                    }
                    TokenKind::Identifier(name) => {
                        // function def name? (previous *significant* token is Def)
                        let mut is_after_def = false;
                        if idx > 0 {
                            // look backwards skipping Newline / Indent / Dedent
                            let mut j = idx as isize - 1;
                            while j >= 0 {
                                match tokens[j as usize].kind {
                                    TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent => { j -= 1; continue; }
                                    TokenKind::Def => { is_after_def = true; }
                                    _ => {}
                                }
                                break;
                            }
                        }
                        let upper = name.to_ascii_uppercase();
                        let is_builtin_draw_fn = upper.starts_with("DRAW_") || matches!(upper.as_str(),
                            "MOVE"|"RECT"|"POLYGON"|"CIRCLE"|"ARC"|"SPIRAL"|"ORIGIN"|"INTENSITY"|"PRINT_TEXT");
                        let is_constant = upper.starts_with("I_");
                        if is_after_def {
                            raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DECL));
                            // record definition range (line0, base_col .. base_col+len)
                            defs.push(SymbolDef {
                                name: name.clone(),
                                uri: uri.clone(),
                                range: Range { start: Position { line: line0, character: base_col }, end: Position { line: line0, character: base_col + name.len() as u32 } },
                                kind: SymbolKind::FUNCTION,
                                // Store detail in default English; hover will localize dynamic message.
                                detail: Some(tr("en", "symbol.user_function.detail")),
                            });
                        } else if is_builtin_draw_fn {
                            raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DEFAULT_LIB));
                        } else if is_constant {
                            raw.push((line0, base_col, name.len() as u32, ENUM_MEMBER, MOD_READONLY));
                        } else {
                            raw.push((line0, base_col, name.len() as u32, VARIABLE, 0));
                        }
                    }
                    TokenKind::Number(_n) => {
                        // Attempt to measure full numeric literal length from source line starting at base_col
                        let slice = &line_str[(base_col as usize)..];
                        let mut len = 0usize;
                        for ch in slice.chars() {
                            if ch.is_ascii_hexdigit() || ch=='x' || ch=='X' || ch=='b' || ch=='B' { len += 1; } else { break; }
                        }
                        if len==0 { len = 1; }
                        raw.push((line0, base_col, len as u32, NUMBER, 0));
                    }
                    TokenKind::StringLit(s) => {
                        // include the surrounding quotes (we know tokens were produced from valid lexeme)
                        let length = (s.len() + 2) as u32;
                        raw.push((line0, base_col, length, STRING, 0));
                    }
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Amp | TokenKind::Pipe | TokenKind::Caret | TokenKind::Tilde | TokenKind::Dot | TokenKind::Colon | TokenKind::Comma | TokenKind::Equal | TokenKind::Lt | TokenKind::Gt => {
                        raw.push((line0, base_col, 1, OPERATOR, 0));
                    }
                    TokenKind::ShiftLeft | TokenKind::ShiftRight | TokenKind::EqEq | TokenKind::NotEq | TokenKind::Le | TokenKind::Ge => {
                        raw.push((line0, base_col, 2, OPERATOR, 0));
                    }
                    _ => { /* ignore others */ }
                }
            }
            raw.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            // delta encode into SemanticToken structs
            let mut last_line = 0u32; let mut last_col = 0u32; let mut first = true;
            for (line, col, length, ttype, mods) in raw {
                let delta_line = if first { line } else { line - last_line };
                let delta_start = if first { col } else if delta_line == 0 { col - last_col } else { col };
                data.push(SemanticToken { delta_line, delta_start, length, token_type: ttype, token_modifiers_bitset: mods });
                last_line = line; last_col = col; first = false;
            }
        }
        // Publish (store) symbol index for this document (replace previous defs for same URI)
        if !defs.is_empty() {
            SYMBOLS.lock().unwrap().retain(|d| d.uri != uri);
            SYMBOLS.lock().unwrap().extend(defs);
        } else {
            // If no defs now, remove any stale ones for the file
            SYMBOLS.lock().unwrap().retain(|d| d.uri != uri);
        }
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data })))
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let pos = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;
        let docs = self.docs.lock().unwrap();
        let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) };
        drop(docs);
        let line = text.lines().nth(pos.line as usize).unwrap_or("");
        // Extract word under cursor
        let chars: Vec<char> = line.chars().collect();
        if (pos.character as usize) > chars.len() { return Ok(None); }
        let mut start = pos.character as isize;
        let mut end = pos.character as usize;
        while start > 0 && chars[(start-1) as usize].is_alphanumeric() || (start>0 && chars[(start-1) as usize]=='_') { start -= 1; }
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; }
        if start as usize >= end { return Ok(None); }
        let word = &line[start as usize .. end];
        let upper = word.to_ascii_uppercase();
        // Builtin docs
        let loc = self.locale.lock().unwrap().clone();
    if let Some(doc) = builtin_doc(&loc, &upper) { return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: doc }), range: None })); }
        // Look for user function definition
        if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) {
            let template = tr(&loc, "hover.user_function.line");
            // simple sequential replacement for two placeholders
            let value = template
                .replacen("{}", &def.name, 1)
                .replacen("{}", &(def.range.start.line + 1).to_string(), 1);
            return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }), range: Some(def.range.clone()) }));
        }
        Ok(None)
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
        let pos = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;
        let docs = self.docs.lock().unwrap();
        let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) };
        drop(docs);
        let line = text.lines().nth(pos.line as usize).unwrap_or("");
        let chars: Vec<char> = line.chars().collect();
        if (pos.character as usize) > chars.len() { return Ok(None); }
        let mut start = pos.character as isize;
        let mut end = pos.character as usize;
        while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; }
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; }
        if start as usize >= end { return Ok(None); }
        let word = &line[start as usize .. end];
        if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location { uri: def.uri.clone(), range: def.range.clone() })));
        }
        Ok(None)
    }
}

// Binary entry (used by cargo run --bin vpy_lsp)
pub async fn run() -> anyhow::Result<()> { run_stdio_server().await; Ok(()) }

// Global symbol store (simple). In production you'd scope per workspace and handle concurrency.
lazy_static::lazy_static! {
    static ref SYMBOLS: Mutex<Vec<SymbolDef>> = Mutex::new(Vec::new());
}
