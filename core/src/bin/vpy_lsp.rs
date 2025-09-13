use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use vectrex_lang::{lexer::lex, parser::parse_with_filename};

struct Backend { client: Client, docs: Arc<Mutex<HashMap<Url, String>>> }

fn compute_diagnostics(uri: &Url, text: &str) -> Vec<Diagnostic> {
    // Siempre calculamos warnings ligeros basados en el texto completo (independiente de parse/lex)
    let mut warnings: Vec<Diagnostic> = text
        .lines()
        .enumerate()
        .filter(|(_, line_txt)| line_txt.contains("POLYGON") && line_txt.contains(" 2"))
        .map(|(i, line_txt)| Diagnostic {
            range: Range { start: Position { line: i as u32, character: 0 }, end: Position { line: i as u32, character: line_txt.len() as u32 } },
            severity: Some(DiagnosticSeverity::WARNING),
            code: None,
            code_description: None,
            source: Some("vpy".into()),
            message: "POLYGON count 2 produces a degenerate list (use >=3 or a thin RECT).".into(),
            related_information: None,
            tags: None,
            data: None,
        })
        .collect();

    // Intentamos lex y parse para detectar error sintáctico
    let parse_error = match lex(text) {
        Ok(tokens) => match parse_with_filename(&tokens, uri.path()) {
            Ok(_) => None,
            Err(e) => Some(e.to_string()),
        },
        Err(e) => Some(e.to_string()),
    };

    let mut diags: Vec<Diagnostic> = Vec::new();
    if let Some(msg) = parse_error {
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
    // Añadir warnings después para que el error principal aparezca primero
    diags.append(&mut warnings);
    diags
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult { capabilities: ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            completion_provider: Some(CompletionOptions { resolve_provider: None, trigger_characters: None, work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }),
            ..Default::default()
        }, server_info: Some(ServerInfo { name: "vpy-lsp".into(), version: Some("0.1.0".into()) }) })
    }
    async fn initialized(&self, _: InitializedParams) { let _ = self.client.log_message(MessageType::INFO, "VPy LSP initialized").await; }
    async fn shutdown(&self) -> LspResult<()> { Ok(()) }
    async fn did_open(&self, params: DidOpenTextDocumentParams) { let uri=params.text_document.uri; let text=params.text_document.text; self.docs.lock().unwrap().insert(uri.clone(), text.clone()); let diags=compute_diagnostics(&uri,&text); let _=self.client.publish_diagnostics(uri, diags, None).await; }
    async fn did_change(&self, params: DidChangeTextDocumentParams) { let uri=params.text_document.uri; if let Some(change)=params.content_changes.into_iter().last(){ self.docs.lock().unwrap().insert(uri.clone(), change.text.clone()); let diags=compute_diagnostics(&uri,&change.text); let _=self.client.publish_diagnostics(uri, diags, None).await; } }
    async fn completion(&self, _: CompletionParams) -> LspResult<Option<CompletionResponse>> { const ITEMS:&[&str]=&["DRAW_POLYGON","DRAW_CIRCLE","PRINT_TEXT","VECTORLIST","INTENSITY","ORIGIN","MOVE","RECT","POLYGON","CIRCLE","ARC","SPIRAL","CONST","VAR","META","def","for","while","if","switch"]; let items=ITEMS.iter().map(|s| CompletionItem{ label:s.to_string(), kind:Some(CompletionItemKind::KEYWORD), insert_text:None, ..Default::default() }).collect(); Ok(Some(CompletionResponse::Array(items))) }
}

#[tokio::main]
async fn main() {
    eprintln!("[vpy_lsp] starting minimal LSP server...");
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::build(|client| Backend { client, docs: Arc::new(Mutex::new(HashMap::new())) }).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
