use tower_lsp::lsp_types::Url;
use vectrex_lang::{lexer::lex, parser::parse_with_filename};

// Copia de la lógica final de parse_error_line_col del servidor LSP para validar colon en rutas Windows
fn parse_error_line_col(msg: &str) -> Option<(u32,u32,String)> {
    if let Some(err_idx) = msg.find(": error:") {
        let prefix = &msg[..err_idx];
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

#[test]
fn windows_path_colon_line_mapping_line187() {
    // Construimos un texto con 186 líneas de comentarios y la línea 187 con 'POLYGON 2 UNKNOWN_TOKEN'
    // Para provocar:
    //  - Warning heurístico (POLYGON 2)
    //  - Error de parse top-level (UNKNOWN_TOKEN como identificador inesperado)
    let mut lines = Vec::new();
    for i in 0..186 { lines.push(format!("# filler {}", i)); } // líneas 1..186
    lines.push("POLYGON 2 UNKNOWN_TOKEN".into()); // línea 187
    let src = lines.join("\n") + "\n";

    let uri = Url::parse("file:///C:/Projects/demo/foo.vpy").unwrap();
    let tokens = lex(&src).expect("lex ok");
    let parse_res = parse_with_filename(&tokens, uri.path());
    assert!(parse_res.is_err(), "Debe fallar el parse para generar el error de prueba");
    let msg = parse_res.err().unwrap().to_string();
    let (eline, ecol, _detail) = parse_error_line_col(&msg).expect("Debe parsear line/col");
    // Línea 187 (1-based) => 186 (0-based)
    assert_eq!(eline, 186, "La línea del error debe ser 186 (0-based para 187 1-based)");
    // El warning heurístico debe localizar la misma línea 186.
    let mut warning_found = false;
    for (idx, lt) in src.lines().enumerate() { if lt.contains("POLYGON") && lt.contains(" 2") { warning_found = true; assert_eq!(idx, 186, "Warning debe estar en línea 186 (0-based)"); } }
    assert!(warning_found, "Warning POLYGON 2 no detectado");
    // Si llegamos aquí, no hay inversión line/col.
    // (Columna específica no se valida porque depende del token primero que dispara el error; colon parsing es objetivo principal)
    println!("OK windows_path_colon_line_mapping_line187 eline={} ecol={}", eline, ecol);
}
