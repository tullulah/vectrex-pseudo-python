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
pub enum AritySpec {
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

pub fn get_builtin_arity(func_name: &str) -> Option<AritySpec> {
    let upper = func_name.to_ascii_uppercase();
    match upper.as_str() {
        // Funciones unificadas (funcionan tanto global como en vectorlist)
        "MOVE" => Some(AritySpec::Exact(2)),                    // x, y
        "SET_INTENSITY" => Some(AritySpec::Exact(1)),           // val
        "DRAW_TO" => Some(AritySpec::Exact(2)),                 // x, y
        "DRAW_LINE" => Some(AritySpec::Exact(5)),               // x1, y1, x2, y2, intensity
        "SET_ORIGIN" => Some(AritySpec::Exact(0)),              // no arguments
        
        // Funciones específicas de dibujo directo (solo globales)
        "RECT" => Some(AritySpec::Exact(4)),                    // x, y, w, h
        "CIRCLE" => Some(AritySpec::Exact(1)),                  // r
        "ARC" => Some(AritySpec::Exact(3)),                     // r, startAngle, endAngle
        "SPIRAL" => Some(AritySpec::Exact(2)),                  // r, turns
        "POLYGON" => Some(AritySpec::Variable(3)),              // n, x1, y1, ... (minimum 3: count + at least one point)
        "PRINT_TEXT" => Some(AritySpec::Exact(3)),              // x, y, text
        "DEBUG_PRINT" => Some(AritySpec::Exact(1)),             // value - debug output to console
        "DEBUG_PRINT_LABELED" => Some(AritySpec::Exact(2)),     // label, value - debug output with label
        
        // Funciones de dibujo con intensidad explícita
        "DRAW_POLYGON" => Some(AritySpec::Variable(4)),         // n, intensity, x1, y1, ... (minimum 4: count + intensity + at least one point)
        "DRAW_CIRCLE" => Some(AritySpec::Exact(4)),             // x, y, r, intensity
        "DRAW_CIRCLE_SEG" => Some(AritySpec::Exact(5)),         // segments, x, y, r, intensity
        "DRAW_VECTORLIST" | "VECTREX_DRAW_VECTORLIST" => Some(AritySpec::Exact(2)), // addr, len
        
        // Funciones específicas de vectorlist
        "FRAME_BEGIN" | "VECTREX_FRAME_BEGIN" => Some(AritySpec::Exact(1)), // intensity
        "VECTOR_PHASE_BEGIN" | "VECTREX_VECTOR_PHASE_BEGIN" => Some(AritySpec::Exact(0)),
        "WAIT_RECAL" | "VECTREX_WAIT_RECAL" => Some(AritySpec::Exact(0)),
        "PLAY_MUSIC1" | "VECTREX_PLAY_MUSIC1" => Some(AritySpec::Exact(0)),
        "DBG_STATIC_VL" | "VECTREX_DBG_STATIC_VL" => Some(AritySpec::Exact(0)),
        "DRAW_VL" | "VECTREX_DRAW_VL" => Some(AritySpec::Exact(2)),         // addr, len
        
        // Compatibilidad hacia atrás con nombres antiguos (deprecated)
        "INTENSITY" => Some(AritySpec::Exact(1)),               // deprecated: use SET_INTENSITY
        "ORIGIN" => Some(AritySpec::Exact(0)),                  // deprecated: use SET_ORIGIN
        "MOVE_TO" | "VECTREX_MOVE_TO" => Some(AritySpec::Exact(2)),         // deprecated: use MOVE
        "VECTREX_DRAW_TO" => Some(AritySpec::Exact(2)),         // deprecated: use DRAW_TO
        "VECTREX_DRAW_LINE" => Some(AritySpec::Exact(5)),       // deprecated: use DRAW_LINE
        "VECTREX_SET_ORIGIN" => Some(AritySpec::Exact(0)),      // deprecated: use SET_ORIGIN
        "VECTREX_SET_INTENSITY" => Some(AritySpec::Exact(1)),   // deprecated: use SET_INTENSITY
        
        // Funciones trigonométricas (tablas precalculadas)
        "SIN" | "MATH_SIN" | "MATH.SIN" => Some(AritySpec::Exact(1)),       // angle (0-127 represents 0-2π)
        "COS" | "MATH_COS" | "MATH.COS" => Some(AritySpec::Exact(1)),       // angle (0-127 represents 0-2π)
        "TAN" | "MATH_TAN" | "MATH.TAN" => Some(AritySpec::Exact(1)),       // angle (0-127 represents 0-2π)
        
        _ => None,
    }
}

// Helper para detectar si un nombre es una función builtin (unificada para global y vectorlist)
pub fn is_builtin_function(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    
    // Funciones unificadas (global + vectorlist)
    if matches!(upper.as_str(),
        "MOVE"|"SET_INTENSITY"|"DRAW_TO"|"DRAW_LINE"|"SET_ORIGIN"|"PRINT_TEXT"
    ) {
        return true;
    }
    
    // Funciones específicas de dibujo directo (solo globales)
    if upper.starts_with("DRAW_") || matches!(upper.as_str(), 
        "RECT"|"POLYGON"|"CIRCLE"|"ARC"|"SPIRAL"|"DRAW_VECTORLIST"
    ) {
        return true;
    }
    
    // Funciones específicas de vectorlist
    if matches!(upper.as_str(),
        "FRAME_BEGIN"|"VECTOR_PHASE_BEGIN"|"WAIT_RECAL"|"PLAY_MUSIC1"|
        "DBG_STATIC_VL"|"DRAW_VL"
    ) {
        return true;
    }
    
    // Compatibilidad hacia atrás (deprecated)
    if matches!(upper.as_str(),
        "INTENSITY"|"ORIGIN"|"MOVE_TO"|
        "VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"|
        "VECTREX_FRAME_BEGIN"|"VECTREX_VECTOR_PHASE_BEGIN"|"VECTREX_WAIT_RECAL"|"VECTREX_PLAY_MUSIC1"|
        "VECTREX_DBG_STATIC_VL"|"VECTREX_DRAW_VL"|"VECTREX_DRAW_VECTORLIST"
    ) {
        return true;
    }
    
    // Funciones trigonométricas (tablas precalculadas)
    if matches!(upper.as_str(),
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|"MATH.SIN"|"MATH.COS"|"MATH.TAN"
    ) {
        return true;
    }
    
    false
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
        // Documentación para funciones unificadas (global + vectorlist)
        ("en", "doc.MOVE") => "MOVE(x, y)  - moves the beam to position (x,y) without drawing.",
        ("es", "doc.MOVE") => "MOVE(x, y)  - mueve el haz a la posición (x,y) sin dibujar.",
        ("en", "doc.SET_INTENSITY") => "SET_INTENSITY(val)  - sets beam intensity.",
        ("es", "doc.SET_INTENSITY") => "SET_INTENSITY(val)  - fija la intensidad del haz.",
        ("en", "doc.DRAW_TO") => "DRAW_TO(x, y)  - draws line to position.",
        ("es", "doc.DRAW_TO") => "DRAW_TO(x, y)  - dibuja línea a posición.",
        ("en", "doc.DRAW_LINE") => "DRAW_LINE(x1, y1, x2, y2, intensity)  - draws line segment.",
        ("es", "doc.DRAW_LINE") => "DRAW_LINE(x1, y1, x2, y2, intensidad)  - dibuja segmento.",
        ("en", "doc.SET_ORIGIN") => "SET_ORIGIN()  - resets origin (0,0).",
        ("es", "doc.SET_ORIGIN") => "SET_ORIGIN()  - restablece origen (0,0).",
        ("en", "doc.PRINT_TEXT") => "PRINT_TEXT(x, y, \"text\")  - shows vector text.",
        ("es", "doc.PRINT_TEXT") => "PRINT_TEXT(x, y, \"texto\")  - muestra texto vectorial.",
        
        // Funciones específicas de dibujo directo
        ("en", "doc.RECT") => "RECT(x1, y1, x2, y2)  - draws a rectangle.",
        ("es", "doc.RECT") => "RECT(x1, y1, x2, y2)  - dibuja un rectángulo.",
        ("en", "doc.POLYGON") => "POLYGON(n, x1, y1, ...)  - draws a polygon with n vertices.",
        ("es", "doc.POLYGON") => "POLYGON(n, x1, y1, ...)  - dibuja un polígono de n vértices.",
        ("en", "doc.CIRCLE") => "CIRCLE(cx, cy, r) or CIRCLE(cx, cy, r, segs)  - draws a circle.",
        ("es", "doc.CIRCLE") => "CIRCLE(cx, cy, r) o CIRCLE(cx, cy, r, segs)  - dibuja un círculo.",
        ("en", "doc.ARC") => "ARC(cx, cy, r, startDeg, sweepDeg) or ARC(..., segs)  - draws an arc.",
        ("es", "doc.ARC") => "ARC(cx, cy, r, angIni, angFin) o ARC(..., segs)  - dibuja un arco.",
        ("en", "doc.SPIRAL") => "SPIRAL(cx, cy, r_start, r_end, turns) or SPIRAL(..., segs)  - draws a spiral.",
        ("es", "doc.SPIRAL") => "SPIRAL(cx, cy, r_ini, r_fin, vueltas) o SPIRAL(..., segs)  - dibuja una espiral.",
        ("en", "doc.DRAW_VECTORLIST") => "DRAW_VECTORLIST addr,len  - draws a raw vector list (advanced).",
        ("es", "doc.DRAW_VECTORLIST") => "DRAW_VECTORLIST dir,long  - dibuja una vector list cruda (avanzado).",
        
        // Funciones específicas de vectorlist
        ("en", "doc.FRAME_BEGIN") => "FRAME_BEGIN intensity  - begins new frame.",
        ("es", "doc.FRAME_BEGIN") => "FRAME_BEGIN intensidad  - iniciar nuevo frame.",
        ("en", "doc.VECTOR_PHASE_BEGIN") => "VECTOR_PHASE_BEGIN  - begins vector phase.",
        ("es", "doc.VECTOR_PHASE_BEGIN") => "VECTOR_PHASE_BEGIN  - iniciar fase vectorial.",
        ("en", "doc.WAIT_RECAL") => "WAIT_RECAL  - waits for recalibration.",
        ("es", "doc.WAIT_RECAL") => "WAIT_RECAL  - esperar recalibración.",
        ("en", "doc.PLAY_MUSIC1") => "PLAY_MUSIC1  - plays music track 1.",
        ("es", "doc.PLAY_MUSIC1") => "PLAY_MUSIC1  - reproducir música pista 1.",
        ("en", "doc.DBG_STATIC_VL") => "DBG_STATIC_VL  - debug static vectorlist.",
        ("es", "doc.DBG_STATIC_VL") => "DBG_STATIC_VL  - debug vectorlist estática.",
        ("en", "doc.DRAW_VL") => "DRAW_VL addr,len  - draws raw vectorlist.",
        ("es", "doc.DRAW_VL") => "DRAW_VL dir,long  - dibuja vectorlist cruda.",
        
        // Compatibilidad hacia atrás (deprecated)
        ("en", "doc.INTENSITY") => "INTENSITY val  - DEPRECATED: use SET_INTENSITY instead.",
        ("es", "doc.INTENSITY") => "INTENSITY val  - OBSOLETO: usar SET_INTENSITY en su lugar.",
        ("en", "doc.ORIGIN") => "ORIGIN  - DEPRECATED: use SET_ORIGIN instead.",
        ("es", "doc.ORIGIN") => "ORIGIN  - OBSOLETO: usar SET_ORIGIN en su lugar.",
        ("en", "doc.MOVE_TO") => "MOVE_TO x,y  - DEPRECATED: use MOVE instead.",
        ("es", "doc.MOVE_TO") => "MOVE_TO x,y  - OBSOLETO: usar MOVE en su lugar.",
        _ => key,
    };
    val.to_string()
}

fn builtin_doc(locale: &str, upper: &str) -> Option<String> {
    let key = match upper {
        // Funciones unificadas (global + vectorlist)
        "MOVE" => "doc.MOVE",
        "SET_INTENSITY" => "doc.SET_INTENSITY",
        "DRAW_TO" => "doc.DRAW_TO",
        "DRAW_LINE" => "doc.DRAW_LINE",
        "SET_ORIGIN" => "doc.SET_ORIGIN",
        "PRINT_TEXT" => "doc.PRINT_TEXT",
        
        // Funciones específicas de dibujo directo
        "RECT" => "doc.RECT",
        "POLYGON" => "doc.POLYGON",
        "CIRCLE" => "doc.CIRCLE",
        "ARC" => "doc.ARC",
        "SPIRAL" => "doc.SPIRAL",
        "DRAW_VECTORLIST" | "VECTREX_DRAW_VECTORLIST" => "doc.DRAW_VECTORLIST",
        
        // Funciones específicas de vectorlist
        "FRAME_BEGIN" | "VECTREX_FRAME_BEGIN" => "doc.FRAME_BEGIN",
        "VECTOR_PHASE_BEGIN" | "VECTREX_VECTOR_PHASE_BEGIN" => "doc.VECTOR_PHASE_BEGIN",
        "WAIT_RECAL" | "VECTREX_WAIT_RECAL" => "doc.WAIT_RECAL",
        "PLAY_MUSIC1" | "VECTREX_PLAY_MUSIC1" => "doc.PLAY_MUSIC1",
        "DBG_STATIC_VL" | "VECTREX_DBG_STATIC_VL" => "doc.DBG_STATIC_VL",
        "DRAW_VL" | "VECTREX_DRAW_VL" => "doc.DRAW_VL",
        
        // Compatibilidad hacia atrás (deprecated)
        "INTENSITY" => "doc.INTENSITY",
        "ORIGIN" => "doc.ORIGIN",
        "MOVE_TO" | "VECTREX_MOVE_TO" => "doc.MOVE_TO",
        "VECTREX_DRAW_TO" => "doc.DRAW_TO",
        "VECTREX_DRAW_LINE" => "doc.DRAW_LINE",
        "VECTREX_SET_ORIGIN" => "doc.SET_ORIGIN",
        "VECTREX_SET_INTENSITY" => "doc.SET_INTENSITY",
        
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

/// Validate import statements and report errors for unresolved modules
fn validate_import_statement(uri: &Url, line: &str, line_num: u32, _locale: &str, diags: &mut Vec<Diagnostic>) {
    let trimmed = line.trim();
    
    // Check "from X import Y" syntax
    if trimmed.starts_with("from ") {
        if let Some(after_from) = trimmed.strip_prefix("from ") {
            let parts: Vec<&str> = after_from.splitn(2, " import ").collect();
            if parts.len() == 2 {
                let module_path = parts[0].trim();
                
                // Try to resolve the module
                if let Ok(current_path) = uri.to_file_path() {
                    if let Some(current_dir) = current_path.parent() {
                        let resolved = resolve_module_path_for_diagnostic(module_path, current_dir);
                        if !resolved {
                            diags.push(Diagnostic {
                                range: Range {
                                    start: Position { line: line_num, character: 5 },
                                    end: Position { line: line_num, character: 5 + module_path.len() as u32 },
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: Some("vpy".into()),
                                message: format!("Cannot resolve module '{}'. Check that the file exists.", module_path),
                                related_information: None,
                                tags: None,
                                data: None,
                            });
                        }
                    }
                }
            } else if !after_from.contains(" import ") {
                // Missing "import" keyword
                diags.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num, character: 0 },
                        end: Position { line: line_num, character: line.len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("vpy".into()),
                    message: "Invalid import syntax. Use: from module import symbol".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }
    
    // Check "import X" syntax
    if trimmed.starts_with("import ") && !trimmed.contains(" as ") {
        // Simple import validation
        if let Some(module_name) = trimmed.strip_prefix("import ") {
            let module_name = module_name.trim();
            if module_name.is_empty() {
                diags.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num, character: 0 },
                        end: Position { line: line_num, character: line.len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("vpy".into()),
                    message: "Missing module name after 'import'".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }
}

/// Helper to resolve module path for diagnostic purposes
fn resolve_module_path_for_diagnostic(module_path: &str, current_dir: &std::path::Path) -> bool {
    // Handle relative imports (starting with .)
    let (is_relative, clean_path) = if module_path.starts_with('.') {
        let level = module_path.chars().take_while(|c| *c == '.').count();
        let rest = module_path.trim_start_matches('.');
        (true, (level, rest))
    } else {
        (false, (0, module_path))
    };
    
    let (level, path_str) = clean_path;
    let mut target_dir = current_dir.to_path_buf();
    
    if is_relative {
        // Go up directories based on level
        for _ in 1..level {
            if let Some(parent) = target_dir.parent() {
                target_dir = parent.to_path_buf();
            } else {
                return false;
            }
        }
    } else {
        // Absolute import - try from src/ directory
        let mut search_dir = current_dir.to_path_buf();
        while !search_dir.ends_with("src") && search_dir.parent().is_some() {
            search_dir = search_dir.parent().unwrap().to_path_buf();
        }
        if search_dir.ends_with("src") {
            target_dir = search_dir;
        }
    }
    
    // Add module path components
    for part in path_str.split('.') {
        if !part.is_empty() {
            target_dir = target_dir.join(part);
        }
    }
    
    // Try with .vpy extension
    let with_ext = target_dir.with_extension("vpy");
    if with_ext.exists() {
        return true;
    }
    
    // Try as directory with __init__.vpy
    let init_file = target_dir.join("__init__.vpy");
    if init_file.exists() {
        return true;
    }
    
    false
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
                // Validate import statements
                validate_import_statement(uri, line_txt, i as u32, locale, &mut diags);
                
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

impl Backend {
    /// Resolve an import target - find the file and symbol location for an imported name
    fn resolve_import_target(&self, text: &str, symbol_name: &str, current_uri: &Url) -> Option<Location> {
        // Parse import statements from the current file
        // Look for: from X import Y or from X import Y as Z
        for line in text.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("from ") { continue; }
            
            // Parse: from module import symbol1, symbol2 as alias
            let after_from = trimmed.strip_prefix("from ")?;
            let parts: Vec<&str> = after_from.splitn(2, " import ").collect();
            if parts.len() != 2 { continue; }
            
            let module_path = parts[0].trim();
            let imports_str = parts[1].trim();
            
            // Check if our symbol is in the import list
            for import_item in imports_str.split(',') {
                let import_item = import_item.trim();
                // Handle "X as Y" syntax
                let (original_name, alias) = if import_item.contains(" as ") {
                    let as_parts: Vec<&str> = import_item.splitn(2, " as ").collect();
                    (as_parts[0].trim(), Some(as_parts[1].trim()))
                } else {
                    (import_item, None)
                };
                
                // Check if this matches our symbol (either by alias or original name)
                let matches = alias.map(|a| a == symbol_name).unwrap_or(original_name == symbol_name);
                if !matches { continue; }
                
                // Resolve module path to file
                if let Some(target_uri) = self.resolve_module_to_uri(module_path, current_uri) {
                    // Look for the symbol definition in the target file
                    let symbols = SYMBOLS.lock().unwrap();
                    if let Some(def) = symbols.iter().find(|d| d.name == original_name && d.uri == target_uri) {
                        return Some(Location { uri: def.uri.clone(), range: def.range.clone() });
                    }
                    // If symbol not found in SYMBOLS, return start of file
                    return Some(Location {
                        uri: target_uri,
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: 0, character: 0 },
                        },
                    });
                }
            }
        }
        None
    }
    
    /// Resolve a module path (like "utils" or "utils.math") to a file URI
    fn resolve_module_to_uri(&self, module_path: &str, current_uri: &Url) -> Option<Url> {
        // Get the directory of the current file
        let current_path = current_uri.to_file_path().ok()?;
        let current_dir = current_path.parent()?;
        
        // Handle relative imports (starting with .)
        let (is_relative, clean_path) = if module_path.starts_with('.') {
            let level = module_path.chars().take_while(|c| *c == '.').count();
            let rest = module_path.trim_start_matches('.');
            (true, (level, rest))
        } else {
            (false, (0, module_path))
        };
        
        let (level, path_str) = clean_path;
        
        // Build the target path
        let mut target_dir = current_dir.to_path_buf();
        
        if is_relative {
            // Go up directories based on level
            for _ in 1..level {
                target_dir = target_dir.parent()?.to_path_buf();
            }
        } else {
            // Absolute import - try from src/ directory
            // Go up until we find src/ or use current dir
            let mut search_dir = current_dir.to_path_buf();
            while !search_dir.ends_with("src") && search_dir.parent().is_some() {
                search_dir = search_dir.parent().unwrap().to_path_buf();
            }
            if search_dir.ends_with("src") {
                target_dir = search_dir;
            }
        }
        
        // Add module path components
        for part in path_str.split('.') {
            if !part.is_empty() {
                target_dir = target_dir.join(part);
            }
        }
        
        // Try with .vpy extension
        let with_ext = target_dir.with_extension("vpy");
        if with_ext.exists() {
            return Url::from_file_path(&with_ext).ok();
        }
        
        // Try as directory with __init__.vpy
        let init_file = target_dir.join("__init__.vpy");
        if init_file.exists() {
            return Url::from_file_path(&init_file).ok();
        }
        
        None
    }
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
    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> { 
        let uri = params.text_document_position.text_document.uri;
        let docs = self.docs.lock().unwrap();
        let text = docs.get(&uri).cloned().unwrap_or_default();
        drop(docs);
        
        // Funciones unificadas (global + vectorlist) y palabras clave VPy
        let unified_items = [ 
            // Funciones unificadas (funcionan en ambos contextos)
            "MOVE","SET_INTENSITY","DRAW_TO","DRAW_LINE","SET_ORIGIN","PRINT_TEXT",
            // Funciones específicas de dibujo directo
            "DRAW_POLYGON","DRAW_CIRCLE","DRAW_CIRCLE_SEG","DRAW_ARC","DRAW_SPIRAL",
            "RECT","POLYGON","CIRCLE","ARC","SPIRAL","DRAW_VECTORLIST",
            // Declaraciones y estructuras VPy
            "VECTORLIST","CONST","VAR","META","TITLE","MUSIC","COPYRIGHT",
            // Palabras clave de control
            "def","for","while","if","switch",
            // Import keywords
            "from","import","export","as"
        ];
        
        // Funciones específicas de vectorlist
        let vectorlist_only = [
            "FRAME_BEGIN","VECTOR_PHASE_BEGIN","WAIT_RECAL","PLAY_MUSIC1",
            "DBG_STATIC_VL","DRAW_VL"
        ];
        
        let mut items = Vec::new();
        
        // Añadir funciones unificadas como Keywords
        for &name in &unified_items {
            items.push(CompletionItem { 
                label: name.to_string(), 
                kind: Some(CompletionItemKind::KEYWORD), 
                insert_text: None, 
                detail: Some("Unified function (global/vectorlist)".to_string()),
                ..Default::default() 
            });
        }
        
        // Añadir funciones específicas de vectorlist como Methods
        for &name in &vectorlist_only {
            items.push(CompletionItem { 
                label: name.to_string(), 
                kind: Some(CompletionItemKind::METHOD), 
                insert_text: None, 
                detail: Some("Vectorlist-only function".to_string()),
                ..Default::default() 
            });
        }
        
        // Add user-defined functions from current file and all open files
        let symbols = SYMBOLS.lock().unwrap();
        for def in symbols.iter() {
            // Add all known user functions
            items.push(CompletionItem {
                label: def.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                insert_text: Some(format!("{}($0)", def.name)),
                insert_text_format: Some(tower_lsp::lsp_types::InsertTextFormat::SNIPPET),
                detail: if def.uri == uri {
                    Some("Function (current file)".to_string())
                } else {
                    Some(format!("Function ({})", def.uri.path().rsplit('/').next().unwrap_or("")))
                },
                ..Default::default()
            });
        }
        drop(symbols);
        
        // Add symbols from imports
        for line in text.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("from ") { continue; }
            
            // Parse: from module import symbol1, symbol2 as alias
            if let Some(after_from) = trimmed.strip_prefix("from ") {
                let parts: Vec<&str> = after_from.splitn(2, " import ").collect();
                if parts.len() != 2 { continue; }
                
                let module_name = parts[0].trim();
                let imports_str = parts[1].trim();
                
                // Add imported symbols to completions
                for import_item in imports_str.split(',') {
                    let import_item = import_item.trim();
                    // Handle "X as Y" syntax - use alias as label
                    let label = if import_item.contains(" as ") {
                        import_item.split(" as ").last().unwrap_or(import_item).trim()
                    } else {
                        import_item
                    };
                    
                    // Only add if not already in items
                    if !items.iter().any(|i| i.label == label) {
                        items.push(CompletionItem {
                            label: label.to_string(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            insert_text: Some(format!("{}($0)", label)),
                            insert_text_format: Some(tower_lsp::lsp_types::InsertTextFormat::SNIPPET),
                            detail: Some(format!("Imported from {}", module_name)),
                            ..Default::default()
                        });
                    }
                }
            }
        }
        
        Ok(Some(CompletionResponse::Array(items))) 
    }
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t) => t.clone(), None => return Ok(None) }; drop(docs); let lines: Vec<&str> = text.lines().collect(); let mut data: Vec<SemanticToken> = Vec::new(); let mut defs: Vec<SymbolDef> = Vec::new(); if let Ok(tokens) = lex(&text) { const KEYWORD: u32 = 0; const FUNCTION: u32 = 1; const VARIABLE: u32 = 2; const NUMBER: u32 = 4; const STRING: u32 = 5; const OPERATOR: u32 = 6; const ENUM_MEMBER: u32 = 7; const MOD_READONLY: u32 = 1 << 0; const MOD_DECL: u32 = 1 << 1; const MOD_DEFAULT_LIB: u32 = 1 << 2; fn keyword_len(kind: &TokenKind) -> Option<usize> { Some(match kind { TokenKind::Def => 3, TokenKind::If => 2, TokenKind::Elif => 4, TokenKind::Else => 4, TokenKind::For => 3, TokenKind::In => 2, TokenKind::Range => 5, TokenKind::Return => 6, TokenKind::While => 5, TokenKind::Break => 5, TokenKind::Continue => 8, TokenKind::Let => 3, TokenKind::Const => 5, TokenKind::Var => 3, TokenKind::VectorList => 10, TokenKind::Switch => 6, TokenKind::Case => 4, TokenKind::Default => 7, TokenKind::Meta => 4, TokenKind::And => 3, TokenKind::Or => 2, TokenKind::Not => 3, TokenKind::True => 4, TokenKind::False => 5, _ => return None }) } let mut raw: Vec<(u32,u32,u32,u32,u32)> = Vec::new(); for (idx, tk) in tokens.iter().enumerate() { let line1 = tk.line; if line1 == 0 { continue; } let line0 = (line1 - 1) as u32; let line_str = lines.get(line0 as usize).copied().unwrap_or(""); let indent = line_str.chars().take_while(|c| *c==' ').count() as u32; let base_col = indent + tk.col as u32; match &tk.kind { k if keyword_len(k).is_some() => { let length = keyword_len(k).unwrap() as u32; raw.push((line0, base_col, length, KEYWORD, 0)); } TokenKind::Identifier(name) => { let mut is_after_def = false; if idx > 0 { let mut j = idx as isize - 1; while j >= 0 { match tokens[j as usize].kind { TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent => { j -= 1; continue; } TokenKind::Def => { is_after_def = true; } _ => {} } break; } } let upper = name.to_ascii_uppercase(); let is_builtin = is_builtin_function(name); let is_constant = upper.starts_with("I_"); if is_after_def { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DECL)); defs.push(SymbolDef { name: name.clone(), uri: uri.clone(), range: Range { start: Position { line: line0, character: base_col }, end: Position { line: line0, character: base_col + name.len() as u32 } } }); } else if is_builtin { raw.push((line0, base_col, name.len() as u32, FUNCTION, MOD_DEFAULT_LIB)); } else if is_constant { raw.push((line0, base_col, name.len() as u32, ENUM_MEMBER, MOD_READONLY)); } else { raw.push((line0, base_col, name.len() as u32, VARIABLE, 0)); } } TokenKind::Number(_)=> { let slice = &line_str[(base_col as usize)..]; let mut len = 0; for ch in slice.chars() { if ch.is_ascii_hexdigit() || ch=='x'||ch=='X'||ch=='b'||ch=='B' { len+=1; } else { break; } } if len==0 { len=1; } raw.push((line0, base_col, len as u32, NUMBER, 0)); } TokenKind::StringLit(s) => { let length = (s.len()+2) as u32; raw.push((line0, base_col, length, STRING, 0)); } TokenKind::Plus|TokenKind::Minus|TokenKind::Star|TokenKind::Slash|TokenKind::Percent|TokenKind::Amp|TokenKind::Pipe|TokenKind::Caret|TokenKind::Tilde|TokenKind::Dot|TokenKind::Colon|TokenKind::Comma|TokenKind::Equal|TokenKind::Lt|TokenKind::Gt => { raw.push((line0, base_col, 1, OPERATOR, 0)); } TokenKind::ShiftLeft|TokenKind::ShiftRight|TokenKind::EqEq|TokenKind::NotEq|TokenKind::Le|TokenKind::Ge => { raw.push((line0, base_col, 2, OPERATOR, 0)); } _ => {} } } raw.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))); let mut last_line=0; let mut last_col=0; let mut first=true; for (line,col,length,ttype,mods) in raw { let delta_line = if first { line } else { line - last_line }; let delta_start = if first { col } else if delta_line==0 { col - last_col } else { col }; data.push(SemanticToken { delta_line, delta_start, length, token_type: ttype, token_modifiers_bitset: mods }); last_line=line; last_col=col; first=false; } } if !defs.is_empty() { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); SYMBOLS.lock().unwrap().extend(defs); } else { SYMBOLS.lock().unwrap().retain(|d| d.uri != uri); } Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))) }
    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        eprintln!("[vpy_lsp][hover] request pos= {:?} uri= {}", params.text_document_position_params.position, params.text_document_position_params.text_document.uri);
        let pos = params.text_document_position_params.position; let uri = params.text_document_position_params.text_document.uri; let docs = self.docs.lock().unwrap(); let text = match docs.get(&uri) { Some(t)=>t.clone(), None=>return Ok(None) }; drop(docs); let line = text.lines().nth(pos.line as usize).unwrap_or(""); let chars: Vec<char> = line.chars().collect(); if (pos.character as usize) > chars.len() { return Ok(None); } let mut start = pos.character as isize; let mut end = pos.character as usize; while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; } while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; } if start as usize >= end { return Ok(None); } let word = &line[start as usize .. end]; let upper = word.to_ascii_uppercase(); let loc = self.locale.lock().unwrap().clone(); if let Some(doc) = builtin_doc(&loc, &upper) { return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: doc }), range: None })); } if let Some(def) = SYMBOLS.lock().unwrap().iter().find(|d| d.name == word && d.uri == uri) { let template = tr(&loc, "hover.user_function.line"); let value = template.replacen("{}", &def.name, 1).replacen("{}", &(def.range.start.line + 1).to_string(), 1); return Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }), range: Some(def.range.clone()) })); } Ok(None) }
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
        let symbols = SYMBOLS.lock().unwrap();
        
        // First, try to find in current file
        if let Some(def) = symbols.iter().find(|d| d.name == word && d.uri == uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location { uri: def.uri.clone(), range: def.range.clone() })));
        }
        
        // Then, try to find in any other file (cross-file navigation)
        if let Some(def) = symbols.iter().find(|d| d.name == word) {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location { uri: def.uri.clone(), range: def.range.clone() })));
        }
        
        // Check if this is an imported symbol - try to resolve from import statement
        if let Some(import_target) = self.resolve_import_target(&text, word, &uri) {
            return Ok(Some(GotoDefinitionResponse::Scalar(import_target)));
        }
        
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> LspResult<Option<WorkspaceEdit>> {
        let pos = params.text_document_position.position;
        let uri = params.text_document_position.text_document.uri;
        let new_name = params.new_name;
        let docs_guard = self.docs.lock().unwrap();
        let all_docs: Vec<(Url, String)> = docs_guard.iter().map(|(u, t)| (u.clone(), t.clone())).collect();
        let text = match docs_guard.get(&uri) { Some(t) => t.clone(), None => return Ok(None) };
        drop(docs_guard);
        
        let line = text.lines().nth(pos.line as usize).unwrap_or("");
        let chars: Vec<char> = line.chars().collect();
        if (pos.character as usize) > chars.len() { return Ok(None); }
        
        let mut start = pos.character as isize;
        let mut end = pos.character as usize;
        while start > 0 && (chars[(start-1) as usize].is_alphanumeric() || chars[(start-1) as usize]=='_') { start -= 1; }
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end]=='_') { end += 1; }
        if start as usize >= end { return Ok(None); }
        
        let original = &line[start as usize .. end];
        if original.is_empty() || new_name.is_empty() { return Ok(None); }
        
        // Check if it's a recorded function symbol (in any file)
        let symbols = SYMBOLS.lock().unwrap();
        let maybe_def = symbols.iter().find(|d| d.name == original);
        if maybe_def.is_none() { return Ok(None); }
        drop(symbols);
        
        // Collect edits for ALL open files that reference this symbol
        let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
        
        for (doc_uri, doc_text) in &all_docs {
            let mut edits: Vec<TextEdit> = Vec::new();
            
            for (line_idx, line_txt) in doc_text.lines().enumerate() {
                let mut search_idx: usize = 0;
                let bytes = line_txt.as_bytes();
                
                while search_idx < bytes.len() {
                    if line_txt[search_idx..].starts_with(original) {
                        // Boundary check
                        let before_ok = if search_idx == 0 { true } 
                            else { !line_txt.as_bytes()[search_idx-1].is_ascii_alphanumeric() && line_txt.as_bytes()[search_idx-1] != b'_' };
                        let after_pos = search_idx + original.len();
                        let after_ok = if after_pos >= bytes.len() { true } 
                            else { !line_txt.as_bytes()[after_pos].is_ascii_alphanumeric() && line_txt.as_bytes()[after_pos] != b'_' };
                        
                        if before_ok && after_ok {
                            edits.push(TextEdit {
                                range: Range {
                                    start: Position { line: line_idx as u32, character: search_idx as u32 },
                                    end: Position { line: line_idx as u32, character: (search_idx + original.len()) as u32 }
                                },
                                new_text: new_name.clone()
                            });
                        }
                        search_idx += original.len();
                    } else {
                        search_idx += 1;
                    }
                }
            }
            
            if !edits.is_empty() {
                changes.insert(doc_uri.clone(), edits);
            }
        }
        
        if changes.is_empty() { return Ok(None); }
        
        Ok(Some(WorkspaceEdit { changes: Some(changes), document_changes: None, change_annotations: None }))
    }

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
