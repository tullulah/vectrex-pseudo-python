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

#[derive(Debug, Clone)]
enum AritySpec {
    Exact(usize),      // Exact number of arguments required
    Variable(usize),   // At least N arguments required (for POLYGON, etc.)
}

fn is_python_keyword_or_builtin(word: &str) -> bool {
    let lower = word.to_ascii_lowercase();
    matches!(lower.as_str(), 
        // Python keywords
        "and" | "as" | "assert" | "break" | "class" | "continue" | "def" | "del" | "elif" | "else" | "except" | "finally" | 
        "for" | "from" | "global" | "if" | "import" | "in" | "is" | "lambda" | "nonlocal" | "not" | "or" | "pass" | 
        "raise" | "return" | "try" | "while" | "with" | "yield" | "true" | "false" | "none" |
        // VPy variable declarations and control structures
        "var" | "let" | "const" |
        // Assignment patterns (contains =)
        _ if word.contains('=')
    )
}

fn get_builtin_arity(func_name: &str) -> Option<AritySpec> {
    let upper = func_name.to_ascii_uppercase();
    match upper.as_str() {
        "MOVE" => Some(AritySpec::Exact(2)),                    // x, y
        "RECT" => Some(AritySpec::Exact(4)),                    // x, y, w, h
        "CIRCLE" => Some(AritySpec::Exact(1)),                  // r
        "ARC" => Some(AritySpec::Exact(3)),                     // r, startAngle, endAngle
        "SPIRAL" => Some(AritySpec::Exact(2)),                  // r, turns
        "ORIGIN" => Some(AritySpec::Exact(0)),                  // no arguments
        "INTENSITY" => Some(AritySpec::Exact(1)),               // val
        "PRINT_TEXT" => Some(AritySpec::Exact(3)),              // x, y, text
        "POLYGON" => Some(AritySpec::Variable(3)),              // n, x1, y1, ... (minimum 3: count + at least one point)
        "DRAW_POLYGON" => Some(AritySpec::Variable(4)),         // n, intensity, x1, y1, ... (minimum 4: count + intensity + at least one point)
        "DRAW_CIRCLE" => Some(AritySpec::Exact(4)),             // x, y, r, intensity
        "DRAW_CIRCLE_SEG" => Some(AritySpec::Exact(5)),         // segments, x, y, r, intensity
        "DRAW_VECTORLIST" | "VECTREX_DRAW_VECTORLIST" => Some(AritySpec::Exact(2)), // addr, len
        _ => None,
    }
}

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
        ("en", "diagnostic.arity.too_few") => "Function `{}` expects {} arguments, but {} were provided.",
        ("es", "diagnostic.arity.too_few") => "La función `{}` espera {} argumentos, pero se proporcionaron {}.",
        ("en", "diagnostic.arity.too_many") => "Function `{}` expects {} arguments, but {} were provided.",
        ("es", "diagnostic.arity.too_many") => "La función `{}` espera {} argumentos, pero se proporcionaron {}.",
        ("en", "diagnostic.arity.variable") => "Function `{}` expects at least {} arguments, but {} were provided.",
        ("es", "diagnostic.arity.variable") => "La función `{}` espera al menos {} argumentos, pero se proporcionaron {}.",
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
        ("en", "doc.DRAW_VECTORLIST") => "DRAW_VECTORLIST addr,len  - draws a raw vector list (advanced).",
        ("es", "doc.DRAW_VECTORLIST") => "DRAW_VECTORLIST dir,long  - dibuja una vector list cruda (avanzado).",
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
        "DRAW_VECTORLIST" | "VECTREX_DRAW_VECTORLIST" => "doc.DRAW_VECTORLIST",
        _ => return None,
    };
    Some(tr(locale, key))
}

// Robustly parse a parser error message of form
//   <path possibly containing colons>:<line>:<col>: error: <detail>
// We cannot split on the first ':' because Windows drive letters (C:) or absolute forms (/C:/...) introduce early colons.
// Strategy: locate the sentinel substring ": error:" then walk backwards extracting the two numeric segments.
fn parse_error_line_col(msg: &str) -> Option<(u32,u32,String)> {
    if let Some(err_idx) = msg.find(": error:") {
        let prefix = &msg[..err_idx]; // ends with :<col>
        // First, find last ':' => separates col
        if let Some(colon2) = prefix.rfind(':') {
            let col_s = &prefix[colon2+1..];
            let prefix2 = &prefix[..colon2];
            if let Some(colon1) = prefix2.rfind(':') {
                let line_s = &prefix2[colon1+1..];
                let line = line_s.trim().parse::<u32>().ok()?.saturating_sub(1);
                let col = col_s.trim().parse::<u32>().ok()?.saturating_sub(1);
                let detail = msg[err_idx + ": error:".len() ..].trim().to_string();
                return Some((line,col,detail));
            }
        }
    }
    None
}

// Parse lexer errors that have the format: "message (line N)"
fn parse_lexer_error(msg: &str) -> Option<(u32, u32, String)> {
    if let Some(line_start) = msg.rfind("(line ") {
        let line_part = &msg[line_start + 6..]; // Skip "(line "
        if let Some(close_paren) = line_part.find(')') {
            let line_str = &line_part[..close_paren];
            if let Ok(line_num) = line_str.parse::<u32>() {
                let message_part = msg[..line_start].trim();
                // Convert to 0-based indexing for LSP
                return Some((line_num.saturating_sub(1), 0, message_part.to_string()));
            }
        }
    }
    None
}

// Function call parser for arity validation - VPy uses Python-style syntax with parentheses
fn validate_function_arity(original_line: &str, line_num: u32, locale: &str, diags: &mut Vec<Diagnostic>) {
    // VPy should use Python-style function calls: FUNCTION(arg1, arg2, ...)
    // Calls without parentheses like "FUNCTION arg1, arg2" are syntax errors
    
    let trimmed = original_line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
        return; // Skip comments and empty lines
    }
    
    // Skip Python control structures that end with ':'
    if trimmed.ends_with(':') {
        return;
    }
    
    // Skip Python keywords and declarations early
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if !parts.is_empty() {
        let first_word = parts[0];
        if is_python_keyword_or_builtin(first_word) {
            return; // Skip Python keywords, var declarations, assignments, etc.
        }
    }
    
    // Look for Python-style function calls: FUNCTION(args)
    // But ignore parentheses that appear in comments
    let comment_pos = trimmed.find('#').unwrap_or(trimmed.len());
    let code_part = &trimmed[..comment_pos].trim();
    
    if let Some(paren_pos) = code_part.find('(') {
        let func_name = code_part[..paren_pos].trim();
        let remaining = &code_part[paren_pos+1..];
        
        // Skip if this looks like a Python keyword or declaration
        if is_python_keyword_or_builtin(func_name) {
            return;
        }
        
        if let Some(close_paren) = remaining.find(')') {
            let args_part = &remaining[..close_paren];
            
            // Check if this is a known builtin function
            if let Some(arity_spec) = get_builtin_arity(func_name) {
                // Count arguments
                let arg_count = if args_part.trim().is_empty() {
                    0
                } else {
                    // Count non-empty arguments separated by commas
                    args_part.split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .count()
                };
                
                let (is_valid, message_key) = match arity_spec {
                    AritySpec::Exact(expected) => {
                        if arg_count == expected {
                            (true, "")
                        } else if arg_count < expected {
                            (false, "diagnostic.arity.too_few")
                        } else {
                            (false, "diagnostic.arity.too_many")
                        }
                    }
                    AritySpec::Variable(min_args) => {
                        if arg_count >= min_args {
                            (true, "")
                        } else {
                            (false, "diagnostic.arity.variable")
                        }
                    }
                };
                
                if !is_valid {
                    let expected_str = match arity_spec {
                        AritySpec::Exact(n) => n.to_string(),
                        AritySpec::Variable(n) => format!("at least {}", n),
                    };
                    
                    let message = match message_key {
                        "diagnostic.arity.variable" => {
                            let template = tr(locale, message_key);
                            template.replacen("{}", func_name, 1)
                                   .replacen("{}", &expected_str, 1)
                                   .replacen("{}", &arg_count.to_string(), 1)
                        }
                        _ => {
                            let template = tr(locale, message_key);
                            template.replacen("{}", func_name, 1)
                                   .replacen("{}", &expected_str, 1)
                                   .replacen("{}", &arg_count.to_string(), 1)
                        }
                    };
                    
                    diags.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num, character: 0 },
                            end: Position { line: line_num, character: original_line.len() as u32 }
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("vpy".into()),
                        message,
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            } else if !func_name.is_empty() {
                // Unknown function - this is an error
                let message = format!("Unknown function '{}'", func_name);
                
                diags.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num, character: 0 },
                        end: Position { line: line_num, character: original_line.len() as u32 }
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("vpy".into()),
                    message,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    } else {
        // Check for old-style function calls without parentheses (these should be syntax errors)
        // Only for known VPy functions, and skip Python keywords
        if !parts.is_empty() {
            let func_name = parts[0];
            
            if get_builtin_arity(func_name).is_some() {
                // This is a known function called without parentheses - syntax error
                let message = format!("Function '{}' must be called with parentheses: {}(...)", func_name, func_name);
                
                diags.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num, character: 0 },
                        end: Position { line: line_num, character: original_line.len() as u32 }
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("vpy".into()),
                    message,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }
}

fn compute_diagnostics(uri: &Url, text: &str, locale: &str) -> Vec<Diagnostic> {
    eprintln!("[LSP] compute_diagnostics called for URI: {}", uri);
    eprintln!("[LSP] Text length: {} lines", text.lines().count());
    let mut diags = Vec::new();
    
    match lex(text) {
        Ok(tokens) => {
            // If lexing succeeded, try parsing
            if let Err(e) = parse_with_filename(&tokens, uri.path()) {
                let msg = e.to_string();
                let (line, col, detail) = if let Some(parsed) = parse_error_line_col(&msg) {
                    parsed
                } else {
                    // Fallback to previous heuristic (may invert on Windows paths but keeps legacy behavior)
                    if let Some((_, rest)) = msg.split_once(":") {
                        let mut parts = rest.split(':');
                        let line_s = parts.next().unwrap_or("0");
                        let col_s = parts.next().unwrap_or("0");
                        let remaining = parts.collect::<Vec<_>>().join(":");
                        (
                            line_s.trim().parse::<u32>().unwrap_or(0).saturating_sub(1),
                            col_s.trim().parse::<u32>().unwrap_or(0).saturating_sub(1),
                            remaining.trim().to_string()
                        )
                    } else { (0,0,msg.clone()) }
                };
                diags.push(Diagnostic { 
                    range: Range { 
                        start: Position { line, character: col }, 
                        end: Position { line, character: col + 1 } 
                    }, 
                    severity: Some(DiagnosticSeverity::ERROR), 
                    code: None, 
                    code_description: None, 
                    source: Some("vpy".into()), 
                    message: detail, 
                    related_information: None, 
                    tags: None, 
                    data: None 
                });
            }
            
            // Additional validation only if lexing succeeded
            for (i, line_txt) in text.lines().enumerate() {
                // Specific POLYGON count 2 warning
                if line_txt.contains("POLYGON") && line_txt.contains(" 2") {
                    diags.push(Diagnostic { 
                        range: Range { 
                            start: Position { line: i as u32, character: 0 }, 
                            end: Position { line: i as u32, character: line_txt.len() as u32 } 
                        }, 
                        severity: Some(DiagnosticSeverity::WARNING), 
                        code: None, 
                        code_description: None, 
                        source: Some("vpy".into()), 
                        message: tr(locale, "diagnostic.polygon.degenerate"), 
                        related_information: None, 
                        tags: None, 
                        data: None 
                    });
                }
                
                // General arity validation for function calls
                validate_function_arity(line_txt, i as u32, locale, &mut diags);
            }
        }
        Err(e) => {
            // Lexer error (e.g., indentation errors)
            let msg = e.to_string();
            let (line, col, detail) = if let Some(lexer_parsed) = parse_lexer_error(&msg) {
                lexer_parsed
            } else {
                // Fallback for lexer errors that don't match expected pattern
                (0, 0, msg.clone())
            };
            diags.push(Diagnostic { 
                range: Range { 
                    start: Position { line, character: col }, 
                    end: Position { line, character: col + 1 } 
                }, 
                severity: Some(DiagnosticSeverity::ERROR), 
                code: None, 
                code_description: None, 
                source: Some("vpy".into()), 
                message: detail, 
                related_information: None, 
                tags: None, 
                data: None 
            });
        }
    }
    
    eprintln!("[LSP] compute_diagnostics returning {} diagnostics", diags.len());
    for (i, diag) in diags.iter().enumerate() {
        eprintln!("[LSP] Diagnostic {}: line={}, message={}", i, diag.range.start.line, diag.message);
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
    Ok(InitializeResult { capabilities: ServerCapabilities { text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)), completion_provider: Some(CompletionOptions { resolve_provider: None, trigger_characters: None, work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }), semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions { work_done_progress_options: Default::default(), legend, range: None, full: Some(SemanticTokensFullOptions::Bool(true)), })), hover_provider: Some(HoverProviderCapability::Simple(true)), definition_provider: Some(OneOf::Left(true)), rename_provider: Some(OneOf::Left(true)), signature_help_provider: Some(SignatureHelpOptions { trigger_characters: Some(vec!["(".to_string(), ",".to_string()]), retrigger_characters: None, work_done_progress_options: Default::default() }), ..Default::default() }, server_info: Some(ServerInfo { name: "vpy-lsp".into(), version: Some("0.1.0".into()) }) })
    }
    async fn initialized(&self, _: InitializedParams) { let loc = self.locale.lock().unwrap().clone(); let _ = self.client.log_message(MessageType::INFO, tr(&loc, "init.ready")).await; }
    async fn shutdown(&self) -> LspResult<()> { Ok(()) }
    async fn did_open(&self, params: DidOpenTextDocumentParams) { let uri = params.text_document.uri; let text = params.text_document.text; self.docs.lock().unwrap().insert(uri.clone(), text.clone()); let loc = self.locale.lock().unwrap().clone(); let diags = compute_diagnostics(&uri, &text, &loc); let _ = self.client.publish_diagnostics(uri, diags, None).await; }
    async fn did_change(&self, params: DidChangeTextDocumentParams) { let uri = params.text_document.uri; if let Some(change) = params.content_changes.into_iter().last() { self.docs.lock().unwrap().insert(uri.clone(), change.text.clone()); let loc = self.locale.lock().unwrap().clone(); let diags = compute_diagnostics(&uri, &change.text, &loc); let _ = self.client.publish_diagnostics(uri, diags, None).await; } }
    async fn completion(&self, _: CompletionParams) -> LspResult<Option<CompletionResponse>> { let items = [ "DRAW_POLYGON","DRAW_CIRCLE","DRAW_CIRCLE_SEG","DRAW_ARC","DRAW_SPIRAL","PRINT_TEXT","VECTORLIST","INTENSITY","ORIGIN","MOVE","RECT","POLYGON","CIRCLE","ARC","SPIRAL","CONST","VAR","META","TITLE","MUSIC","COPYRIGHT","def","for","while","if","switch" ].iter().map(|s| CompletionItem { label: s.to_string(), kind: Some(CompletionItemKind::KEYWORD), insert_text: None, ..Default::default() }).collect(); Ok(Some(CompletionResponse::Array(items))) }
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) }; drop(docs); let lines: Vec<&str> = text.lines().collect(); let mut data: Vec<SemanticToken> = Vec::new(); let mut defs: Vec<SymbolDef> = Vec::new(); if let Ok(tokens) = lex(&text) { const KEYWORD: u32 = 0; const FUNCTION: u32 = 1; const VARIABLE: u32 = 2; const NUMBER: u32 = 4; const STRING: u32 = 5; const OPERATOR: u32 = 6; const ENUM_MEMBER: u32 = 7; const MOD_READONLY: u32 = 1 << 0; const MOD_DECL: u32 = 1 << 1; const MOD_DEFAULT_LIB: u32 = 1 << 2; fn keyword_len(kind: &TokenKind) -> Option<usize> { Some(match kind { TokenKind::Def => 3, TokenKind::If => 2, TokenKind::Elif => 4, TokenKind::Else => 4, TokenKind::For => 3, TokenKind::In => 2, TokenKind::Range => 5, TokenKind::Return => 6, TokenKind::While => 5, TokenKind::Break => 5, TokenKind::Continue => 8, TokenKind::Let => 3, TokenKind::Const => 5, TokenKind::Var => 3, TokenKind::VectorList => 10, TokenKind::Switch => 6, TokenKind::Case => 4, TokenKind::Default => 7, TokenKind::Meta => 4, TokenKind::And => 3, TokenKind::Or => 2, TokenKind::Not => 3, TokenKind::True => 4, TokenKind::False => 5, _ => return None }) } let mut raw: Vec<(u32,u32,u32,u32,u32)> = Vec::new(); for (idx, tk) in tokens.iter().enumerate() { let line1 = tk.line; if line1 == 0 { continue; } let line0 = (line1 - 1) as u32; let line_str = lines.get(line0 as usize).copied().unwrap_or(""); let indent = line_str.chars().take_while(|c| *c==' ').count() as u32; let base_col = indent + tk.col as u32; match &tk.kind { k if keyword_len(k).is_some() => { let length = keyword_len(k).unwrap() as u32; raw.push((line0, base_col, length, KEYWORD, 0)); } TokenKind::Identifier(name) => { let mut is_after_def = false; if idx > 0 { let mut j = idx as isize - 1; while j >= 0 { match tokens[j as usize].kind { TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent => { j -= 1; continue; } TokenKind::Def => { is_after_def = true; } _ => {} } break; } } let upper = name.to_ascii_uppercase(); let is_builtin = upper.starts_with("DRAW_") || matches!(upper.as_str(), "MOVE"|"RECT"|"POLYGON"|"CIRCLE"|"ARC"|"SPIRAL"|"ORIGIN"|"INTENSITY"|"PRINT_TEXT"); let is_constant = upper.starts_with("I_"); if is_after_def { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DECL)); defs.push(SymbolDef { name: name.clone(), uri: uri.clone(), range: Range { start: Position { line: line0, character: base_col }, end: Position { line: line0, character: base_col + name.len() as u32 } } }); } else if is_builtin { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DEFAULT_LIB)); } else if is_constant { raw.push((line0, base_col, name.len() as u32, ENUM_MEMBER, MOD_READONLY)); } else { raw.push((line0, base_col, name.len() as u32, VARIABLE, 0)); } } TokenKind::Number(_)=> { let slice = &line_str[(base_col as usize)..]; let mut len = 0; for ch in slice.chars() { if ch.is_ascii_hexdigit() || ch=='x'||ch=='X'||ch=='b'||ch=='B' { len+=1; } else { break; } } if len==0 { len=1; } raw.push((line0, base_col, len as u32, NUMBER, 0)); } TokenKind::StringLit(s) => { let length = (s.len()+2) as u32; raw.push((line0, base_col, length, STRING, 0)); } TokenKind::Plus|TokenKind::Minus|TokenKind::Star|TokenKind::Slash|TokenKind::Percent|TokenKind::Amp|TokenKind::Pipe|TokenKind::Caret|TokenKind::Tilde|TokenKind::Dot|TokenKind::Colon|TokenKind::Comma|TokenKind::Equal|TokenKind::Lt|TokenKind::Gt => { raw.push((line0, base_col, 1, OPERATOR, 0)); } TokenKind::ShiftLeft|TokenKind::ShiftRight|TokenKind::EqEq|TokenKind::NotEq|TokenKind::Le|TokenKind::Ge => { raw.push((line0, base_col, 2, OPERATOR, 0)); } _ => {} } } raw.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))); let mut last_line=0; let mut last_col=0; let mut first=true; for (line,col,length,ttype,mods) in raw { let delta_line = if first { line } else { line - last_line }; let delta_start = if first { col } else if delta_line==0 { col - last_col } else { col }; data.push(SemanticToken { delta_line, delta_start, length, token_type: ttype, token_modifiers_bitset: mods }); last_line=line; last_col=col; first=false; } } if !defs.is_empty() { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); SYMBOLS.lock().unwrap().extend(defs); } else { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); } Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))) }
    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        eprintln!("[vpy_lsp][hover] request pos= {:?} uri= {}", params.text_document_position_params.position, params.text_document_position_params.text_document.uri);
        let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let word = &line[start as usize .. end]; let upper = word.to_ascii_uppercase(); let loc = self.locale.lock().unwrap().clone(); if let Some(doc) = builtin_doc(&loc, &upper) { return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: doc }), range: None })); } if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) { let template = tr(&loc, "hover.user_function.line"); let value = template.replacen("{}", &def.name, 1).replacen("{}", &(def.range.start.line + 1).to_string(), 1); return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }), range: Some(def.range.clone()) })); } Ok(None) }
    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> { let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let word = &line[start as usize .. end]; if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) { return Ok(Some(GotoDefinitionResponse::Scalar(Location { uri: def.uri.clone(), range: def.range.clone() }))); } Ok(None) }

    async fn rename(&self, params: RenameParams) -> LspResult<Option<WorkspaceEdit>> { let pos = params.text_document_position.position; let uri = params.text_document_position.text_document.uri; let new_name = params.new_name; let docs_guard = self.docs.lock().unwrap(); let text = match docs_guard.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs_guard); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let original = &line[start as usize .. end]; if original.is_empty() || new_name.is_empty() { return Ok(None); } // restrict: only rename if it's a recorded function symbol
        let symbols = SYMBOLS.lock().unwrap(); let maybe_def = symbols.iter().find(|d| d.name == original && d.uri == uri); if maybe_def.is_none() { return Ok(None); } drop(symbols); // compute all ranges in doc exactly matching original
        let mut edits: Vec<TextEdit> = Vec::new(); for (line_idx, line_txt) in text.lines().enumerate() { let mut search_idx: usize = 0; let bytes = line_txt.as_bytes(); while search_idx < bytes.len() { if line_txt[search_idx..].starts_with(original) { // boundary check
                    let before_ok = if search_idx==0 { true } else { !line_txt.as_bytes()[search_idx-1].is_ascii_alphanumeric() && line_txt.as_bytes()[search_idx-1] != b'_' };
                    let after_pos = search_idx + original.len(); let after_ok = if after_pos>=bytes.len() { true } else { !line_txt.as_bytes()[after_pos].is_ascii_alphanumeric() && line_txt.as_bytes()[after_pos] != b'_' };
                    if before_ok && after_ok { edits.push(TextEdit { range: Range { start: Position { line: line_idx as u32, character: search_idx as u32 }, end: Position { line: line_idx as u32, character: (search_idx + original.len()) as u32 } }, new_text: new_name.clone() }); }
                    search_idx += original.len();
                } else { search_idx += 1; }
            }
        }
        if edits.is_empty() { return Ok(None); }
        let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new(); changes.insert(uri.clone(), edits); Ok(Some(WorkspaceEdit { changes: Some(changes), document_changes: None, change_annotations: None })) }

    async fn signature_help(&self, params: SignatureHelpParams) -> LspResult<Option<SignatureHelp>> { let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); // Find the current line up to cursor and count commas after last '('
        let line = text.lines().nth(pos.line as usize).unwrap_or(""); let prefix = &line[..std::cmp::min(pos.character as usize, line.len())]; // Find last '('
        if let Some(idx) = prefix.rfind('(') { let call_prefix = &prefix[..idx]; // extract function identifier backwards
            let ident: String = call_prefix.chars().rev().take_while(|c| c.is_alphanumeric() || *c=='_').collect::<String>().chars().rev().collect(); if ident.is_empty() { return Ok(None); }
            let arg_str = &prefix[idx+1..]; let param_index = if arg_str.trim().is_empty() { 0 } else { arg_str.chars().filter(|c| *c==',').count() }; // Provide simple builtin signatures
            let sigs = vec![
                ("MOVE", vec!["x","y"], "Move beam to (x,y)"),
                ("RECT", vec!["x","y","w","h"], "Draw rectangle"),
                ("POLYGON", vec!["n","x1y1..."], "Draw polygon"),
                ("CIRCLE", vec!["r"], "Draw circle"),
                ("ARC", vec!["r","startAngle","endAngle"], "Draw arc"),
                ("SPIRAL", vec!["r","turns"], "Draw spiral"),
                ("INTENSITY", vec!["val"], "Set beam intensity"),
                ("PRINT_TEXT", vec!["x","y","text"], "Print text")
            ];
            let upper = ident.to_ascii_uppercase();
            if let Some((name, params_list, doc)) = sigs.into_iter().find(|(n,_,_)| *n==upper) { let label = format!("{}({})", name, params_list.join(", ")); let parameters: Vec<ParameterInformation> = params_list.iter().map(|p| ParameterInformation { label: ParameterLabel::Simple(p.to_string()), documentation: None }).collect(); let sig = SignatureInformation { label, documentation: Some(Documentation::String(doc.to_string())), parameters: Some(parameters), active_parameter: None, }; let max_index = params_list.len().saturating_sub(1); let active_param_u32 = std::cmp::min(param_index as usize, max_index) as u32; return Ok(Some(SignatureHelp { signatures: vec![sig], active_signature: Some(0), active_parameter: Some(active_param_u32), })) }
        }
        Ok(None) }
}

pub async fn run() -> anyhow::Result<()> { run_stdio_server().await; Ok(()) }
