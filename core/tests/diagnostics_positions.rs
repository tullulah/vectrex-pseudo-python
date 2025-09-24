use vectrex_lang::{lexer::lex, parser::parse_with_filename};
use tower_lsp::lsp_types::Url;

// Mini réplica de compute_diagnostics para aislar el orden line/col en tests.
fn fake_compute(text: &str) -> Vec<(u32, u32, String)> {
    let uri = Url::parse("file:///test.vpy").unwrap();
    let mut out = Vec::new();
    match lex(text) {
        Ok(tokens) => {
            let parse_res = parse_with_filename(&tokens, uri.path());
            if let Err(e) = parse_res {
                let msg = e.to_string();
                let (line, col, detail) = if let Some((_, rest)) = msg.split_once(":") {
                    let mut parts = rest.split(':');
                    let line_s = parts.next().unwrap_or("0");
                    let col_s = parts.next().unwrap_or("0");
                    let remaining = parts.collect::<Vec<_>>().join(":");
                    (
                        line_s.trim().parse::<u32>().unwrap_or(0).saturating_sub(1),
                        col_s.trim().parse::<u32>().unwrap_or(0).saturating_sub(1),
                        remaining.trim().to_string(),
                    )
                } else {
                    (0, 0, msg)
                };
                out.push((line, col, detail));
            }
            // Escaneo manual para warnings sintéticos de POLYGON 2
            for (i, line_txt) in text.lines().enumerate() {
                if line_txt.contains("POLYGON") && line_txt.contains(" 2") {
                    out.push((i as u32, 0, "POLYGON degenerate".into()));
                }
            }
        }
        Err(e) => out.push((0, 0, e.to_string())),
    }
    out
}

// Construye una fuente más extensa basada en comentarios y un VECTORLIST válido.
// Objetivos:
//  - WARNING en línea 20 (1-based) -> índice 19 (0-based) con 'POLYGON 2 ...'.
//  - ERROR en línea 30 (1-based) -> índice 29 (0-based) mediante token inesperado al top-level.
#[test]
fn extended_source_warning_line20_error_line30() {
    let mut lines: Vec<String> = Vec::new();
    // Líneas 1..18 (indices 0..17): relleno válido (comentarios)
    for i in 0..18 { lines.push(format!("# filler {}", i)); }
    // Línea 19 (idx18): inicio de vectorlist válido
    lines.push("VECTORLIST shapes:".into());
    // Línea 20 (idx19): comando POLYGON con count=2 => warning sintetizado, pero sintaxis válida dentro del bloque
    lines.push("    POLYGON 2 0 0 1 1".into());
    // Línea 21 (idx20): otro comando válido
    lines.push("    INTENSITY 50".into());
    // Línea 22 (idx21): cerrar bloque (dedent) con comentario
    lines.push("# after vectorlist".into());
    // Función válida para seguir agregando líneas
    // Línea 23 (idx22)
    lines.push("def main():".into());
    // Cuerpo líneas 24..27 (idx23..26)
    lines.push("    # body line 1".into());
    lines.push("    PRINT_TEXT(0,0,\"OK\")".into());
    lines.push("    # body line 3".into());
    lines.push("    PRINT_TEXT(0,0,\"OK2\")".into());
    // Línea 28 (idx27)
    lines.push("# top-level after function".into());
    // Línea 29 (idx28)
    lines.push("# padding line".into());
    // Línea 30 (idx29): token inesperado que forzará parse error al top-level
    lines.push("UNKNOWN_TOKEN".into());

    let src = lines.join("\n") + "\n"; // asegurar newline final
    // Verificación manual de posiciones esperadas
    assert_eq!(src.lines().nth(19).unwrap().trim_start(), "POLYGON 2 0 0 1 1", "La línea 20 (1-based) debe ser POLYGON");
    assert_eq!(src.lines().nth(29).unwrap().trim_start(), "UNKNOWN_TOKEN", "La línea 30 (1-based) debe ser el error");

    let diags = fake_compute(&src);
    eprintln!("DIAGS: {:?}", diags);
    assert!(diags.len() >= 2, "Esperábamos al menos 2 diagnósticos (error + warning), obtuvimos {}", diags.len());

    let warning = diags.iter().find(|d| d.2.contains("POLYGON")).expect("Debe existir warning POLYGON");
    assert_eq!(warning.0, 19, "Warning debe estar en línea 20 (1-based) => idx 19");
    assert_eq!(warning.1, 0, "Columna del warning debe ser 0");

    // Buscamos error (Unexpected token ...) y validamos línea 30.
    let maybe_err = diags.iter().find(|d| d.2.contains("Unexpected") || d.2.contains("error"));
    if let Some(err) = maybe_err {
        // Dependiendo del orden push, error puede estar primero pero su línea debe ser >= warning.
        assert_eq!(err.0, 29, "Error debe estar en línea 30 (1-based) => idx 29, got {}", err.0);
    } else {
        panic!("No se encontró diagnóstico de error parse (Unexpected token)");
    }
}
