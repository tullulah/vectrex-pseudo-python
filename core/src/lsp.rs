//! VPy LSP server implementation (diagnostics, completion, semantic tokens, hover, goto definition).
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::*;
use crate::lexer::{lex, TokenKind};
use crate::parser::parse_with_filename;

pub async fn run_stdio_server() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::build(|client| Backend {
        client,
        docs: Arc::new(Mutex::new(HashMap::new())),
        locale: Arc::new(Mutex::new("en".to_string())),
    }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    client: Client,
    docs: Arc<Mutex<HashMap<Url, String>>>,
    locale: Arc<Mutex<String>>,
}

#[derive(Clone)]
struct SymbolDef { name: String, uri: Url, range: Range }

lazy_static::lazy_static! {
    static ref SYMBOLS: Mutex<Vec<SymbolDef>> = Mutex::new(Vec::new());
}

fn tr(locale: &str, key: &str) -> String {
    let l = if locale.starts_with("es") { "es" } else { "en" };
    let val = match (l, key) {
        ("en", "init.ready") => "VPy LSP initialized",
        ("es", "init.ready") => "VPy LSP inicializado",
        ("en", "diagnostic.polygon.degenerate") => "POLYGON count 2 produces a degenerate list (use >=3 or a thin RECT).",
        ("es", "diagnostic.polygon.degenerate") => "POLYGON count 2 genera lista degenerada (usa >=3 o un RECT delgado).",
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
        "MOVE" => "doc.MOVE",
        "RECT" => "doc.RECT",
        "POLYGON" => "doc.POLYGON",
        "CIRCLE" => "doc.CIRCLE",
        "ARC" => "doc.ARC",
        "SPIRAL" => "doc.SPIRAL",
        "ORIGIN" => "doc.ORIGIN",
        "INTENSITY" => "doc.INTENSITY",
        "PRINT_TEXT" => "doc.PRINT_TEXT",
        _ => return None,
    };
    Some(tr(locale, key))
}

fn compute_diagnostics(uri: &Url, text: &str, locale: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    match lex(text) {
        Ok(tokens) => {
            if let Err(e) = parse_with_filename(&tokens, uri.path()) {
                let msg = e.to_string();
                let (line, col, detail) = if let Some((_, rest)) = msg.split_once(":") {
                    let mut parts = rest.split(':');
                    let line_s = parts.next().unwrap_or("0");
                    let col_s = parts.next().unwrap_or("0");
                    let _err = parts.next();
                    let remaining = parts.collect::<Vec<_>>().join(":");
                    (
                        line_s.parse::<u32>().unwrap_or(0).saturating_sub(1),
                        col_s.parse::<u32>().unwrap_or(0).saturating_sub(1),
                        remaining.trim().to_string(),
                    )
                } else { (0,0,msg) };
                diags.push(Diagnostic { range: Range { start: Position { line, character: col }, end: Position { line, character: col + 1 } }, severity: Some(DiagnosticSeverity::ERROR), code: None, code_description: None, source: Some("vpy".into()), message: detail, related_information: None, tags: None, data: None });
            }
            // Heuristic warning independent of parse success
            for (i, line_txt) in text.lines().enumerate() {
                if line_txt.contains("POLYGON") && line_txt.contains(" 2") {
                    diags.push(Diagnostic { range: Range { start: Position { line: i as u32, character: 0 }, end: Position { line: i as u32, character: line_txt.len() as u32 } }, severity: Some(DiagnosticSeverity::WARNING), code: None, code_description: None, source: Some("vpy".into()), message: tr(locale, "diagnostic.polygon.degenerate"), related_information: None, tags: None, data: None });
                }
            }
        }
        Err(err) => {
            let msg = err.to_string();
            diags.push(Diagnostic { range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 1 } }, severity: Some(DiagnosticSeverity::ERROR), code: None, code_description: None, source: Some("vpy".into()), message: msg, related_information: None, tags: None, data: None });
        }
    }
    diags
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        if let Some(loc) = params.locale.clone() { *self.locale.lock().unwrap() = loc; }
        let legend = SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::KEYWORD,
                SemanticTokenType::FUNCTION,
                SemanticTokenType::VARIABLE,
                SemanticTokenType::PARAMETER,
                SemanticTokenType::NUMBER,
                SemanticTokenType::STRING,
                SemanticTokenType::OPERATOR,
                SemanticTokenType::ENUM_MEMBER,
            ],
            token_modifiers: vec![
                SemanticTokenModifier::READONLY,
                SemanticTokenModifier::DECLARATION,
                SemanticTokenModifier::DEFAULT_LIBRARY,
            ],
        };
        Ok(InitializeResult { capabilities: ServerCapabilities { text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)), completion_provider: Some(CompletionOptions { resolve_provider: None, trigger_characters: None, work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }), semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions { work_done_progress_options: Default::default(), legend, range: None, full: Some(SemanticTokensFullOptions::Bool(true)), })), hover_provider: Some(HoverProviderCapability::Simple(true)), definition_provider: Some(OneOf::Left(true)), ..Default::default() }, server_info: Some(ServerInfo { name: "vpy-lsp".into(), version: Some("0.1.0".into()) }) })
    }
    async fn initialized(&self, _: InitializedParams) { let loc = self.locale.lock().unwrap().clone(); let _ = self.client.log_message(MessageType::INFO, tr(&loc, "init.ready")).await; }
    async fn shutdown(&self) -> LspResult<()> { Ok(()) }
    async fn did_open(&self, params: DidOpenTextDocumentParams) { let uri = params.text_document.uri; let text = params.text_document.text; self.docs.lock().unwrap().insert(uri.clone(), text.clone()); let loc = self.locale.lock().unwrap().clone(); let diags = compute_diagnostics(&uri, &text, &loc); let _ = self.client.publish_diagnostics(uri, diags, None).await; }
    async fn did_change(&self, params: DidChangeTextDocumentParams) { let uri = params.text_document.uri; if let Some(change) = params.content_changes.into_iter().last() { self.docs.lock().unwrap().insert(uri.clone(), change.text.clone()); let loc = self.locale.lock().unwrap().clone(); let diags = compute_diagnostics(&uri, &change.text, &loc); let _ = self.client.publish_diagnostics(uri, diags, None).await; } }
    async fn completion(&self, _: CompletionParams) -> LspResult<Option<CompletionResponse>> { let items = [ "DRAW_POLYGON","DRAW_CIRCLE","DRAW_CIRCLE_SEG","DRAW_ARC","DRAW_SPIRAL","PRINT_TEXT","VECTORLIST","INTENSITY","ORIGIN","MOVE","RECT","POLYGON","CIRCLE","ARC","SPIRAL","CONST","VAR","META","TITLE","MUSIC","COPYRIGHT","def","for","while","if","switch" ].iter().map(|s| CompletionItem { label: s.to_string(), kind: Some(CompletionItemKind::KEYWORD), insert_text: None, ..Default::default() }).collect(); Ok(Some(CompletionResponse::Array(items))) }
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) }; drop(docs); let lines: Vec<&str> = text.lines().collect(); let mut data: Vec<SemanticToken> = Vec::new(); let mut defs: Vec<SymbolDef> = Vec::new(); if let Ok(tokens) = lex(&text) { const KEYWORD: u32 = 0; const FUNCTION: u32 = 1; const VARIABLE: u32 = 2; const NUMBER: u32 = 4; const STRING: u32 = 5; const OPERATOR: u32 = 6; const ENUM_MEMBER: u32 = 7; const MOD_READONLY: u32 = 1 << 0; const MOD_DECL: u32 = 1 << 1; const MOD_DEFAULT_LIB: u32 = 1 << 2; fn keyword_len(kind: &TokenKind) -> Option<usize> { Some(match kind { TokenKind::Def => 3, TokenKind::If => 2, TokenKind::Elif => 4, TokenKind::Else => 4, TokenKind::For => 3, TokenKind::In => 2, TokenKind::Range => 5, TokenKind::Return => 6, TokenKind::While => 5, TokenKind::Break => 5, TokenKind::Continue => 8, TokenKind::Let => 3, TokenKind::Const => 5, TokenKind::Var => 3, TokenKind::VectorList => 10, TokenKind::Switch => 6, TokenKind::Case => 4, TokenKind::Default => 7, TokenKind::Meta => 4, TokenKind::And => 3, TokenKind::Or => 2, TokenKind::Not => 3, TokenKind::True => 4, TokenKind::False => 5, _ => return None }) } let mut raw: Vec<(u32,u32,u32,u32,u32)> = Vec::new(); for (idx, tk) in tokens.iter().enumerate() { let line1 = tk.line; if line1 == 0 { continue; } let line0 = (line1 - 1) as u32; let line_str = lines.get(line0 as usize).copied().unwrap_or(""); let indent = line_str.chars().take_while(|c| *c==' ').count() as u32; let base_col = indent + tk.col as u32; match &tk.kind { k if keyword_len(k).is_some() => { let length = keyword_len(k).unwrap() as u32; raw.push((line0, base_col, length, KEYWORD, 0)); } TokenKind::Identifier(name) => { let mut is_after_def = false; if idx > 0 { let mut j = idx as isize - 1; while j >= 0 { match tokens[j as usize].kind { TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent => { j -= 1; continue; } TokenKind::Def => { is_after_def = true; } _ => {} } break; } } let upper = name.to_ascii_uppercase(); let is_builtin = upper.starts_with("DRAW_") || matches!(upper.as_str(), "MOVE"|"RECT"|"POLYGON"|"CIRCLE"|"ARC"|"SPIRAL"|"ORIGIN"|"INTENSITY"|"PRINT_TEXT"); let is_constant = upper.starts_with("I_"); if is_after_def { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DECL)); defs.push(SymbolDef { name: name.clone(), uri: uri.clone(), range: Range { start: Position { line: line0, character: base_col }, end: Position { line: line0, character: base_col + name.len() as u32 } } }); } else if is_builtin { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DEFAULT_LIB)); } else if is_constant { raw.push((line0, base_col, name.len() as u32, ENUM_MEMBER, MOD_READONLY)); } else { raw.push((line0, base_col, name.len() as u32, VARIABLE, 0)); } } TokenKind::Number(_)=> { let slice = &line_str[(base_col as usize)..]; let mut len = 0; for ch in slice.chars() { if ch.is_ascii_hexdigit() || ch=='x'||ch=='X'||ch=='b'||ch=='B' { len+=1; } else { break; } } if len==0 { len=1; } raw.push((line0, base_col, len as u32, NUMBER, 0)); } TokenKind::StringLit(s) => { let length = (s.len()+2) as u32; raw.push((line0, base_col, length, STRING, 0)); } TokenKind::Plus|TokenKind::Minus|TokenKind::Star|TokenKind::Slash|TokenKind::Percent|TokenKind::Amp|TokenKind::Pipe|TokenKind::Caret|TokenKind::Tilde|TokenKind::Dot|TokenKind::Colon|TokenKind::Comma|TokenKind::Equal|TokenKind::Lt|TokenKind::Gt => { raw.push((line0, base_col, 1, OPERATOR, 0)); } TokenKind::ShiftLeft|TokenKind::ShiftRight|TokenKind::EqEq|TokenKind::NotEq|TokenKind::Le|TokenKind::Ge => { raw.push((line0, base_col, 2, OPERATOR, 0)); } _ => {} } } raw.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))); let mut last_line=0; let mut last_col=0; let mut first=true; for (line,col,length,ttype,mods) in raw { let delta_line = if first { line } else { line - last_line }; let delta_start = if first { col } else if delta_line==0 { col - last_col } else { col }; data.push(SemanticToken { delta_line, delta_start, length, token_type: ttype, token_modifiers_bitset: mods }); last_line=line; last_col=col; first=false; } } if !defs.is_empty() { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); SYMBOLS.lock().unwrap().extend(defs); } else { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); } Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))) }
    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> { let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let word = &line[start as usize .. end]; let upper = word.to_ascii_uppercase(); let loc = self.locale.lock().unwrap().clone(); if let Some(doc) = builtin_doc(&loc, &upper) { return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: doc }), range: None })); } if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) { let template = tr(&loc, "hover.user_function.line"); let value = template.replacen("{}", &def.name, 1).replacen("{}", &(def.range.start.line + 1).to_string(), 1); return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }), range: Some(def.range.clone()) })); } Ok(None) }
    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> { let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let word = &line[start as usize .. end]; if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) { return Ok(Some(GotoDefinitionResponse::Scalar(Location { uri: def.uri.clone(), range: def.range.clone() }))); } Ok(None) }
}

pub async fn run() -> anyhow::Result<()> { run_stdio_server().await; Ok(()) }
