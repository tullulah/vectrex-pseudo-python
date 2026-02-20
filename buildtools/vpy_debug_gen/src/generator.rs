//! Debug symbol generation - extracts info from ASM source and linker output

/// Extract label names from raw binary treated as text (best-effort).
///
/// Binary data doesn't carry symbol names, so this only works when the
/// caller passes ASM source bytes.  For a proper symbol table, use
/// `generate_pdb()` in `lib.rs` which accepts the ASM source string
/// together with the linker symbol map.
pub fn extract_symbols(binary_data: &[u8]) -> crate::DebugResult<Vec<(String, u32)>> {
    // Try to interpret the bytes as UTF-8 text (e.g. when ASM source is passed)
    let text = match std::str::from_utf8(binary_data) {
        Ok(t) => t,
        // Binary ROM data: no symbol names can be extracted
        Err(_) => return Ok(vec![]),
    };

    let mut symbols = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_end();
        // Match lines like "LABEL:" or "LABEL:   ; comment"
        if let Some(label) = parse_label_definition(trimmed) {
            symbols.push((label.to_string(), 0u32));
        }
    }
    Ok(symbols)
}

/// Return the label name if `line` is a label definition (`NAME:`).
///
/// Skips local labels (starting with `.`), purely numeric strings,
/// and lines that don't start with an identifier character.
pub(crate) fn parse_label_definition(line: &str) -> Option<&str> {
    // Must not start with whitespace (labels are at column 0)
    if line.starts_with(' ') || line.starts_with('\t') {
        return None;
    }
    // Strip trailing comment
    let code = match line.find(';') {
        Some(pos) => line[..pos].trim_end(),
        None => line.trim_end(),
    };
    let label = code.strip_suffix(':')?;
    let label = label.trim_end();
    // Skip local labels and empty strings
    if label.is_empty() || label.starts_with('.') {
        return None;
    }
    // Must be a valid identifier: [A-Za-z_][A-Za-z0-9_]*
    let mut chars = label.chars();
    let first = chars.next()?;
    if !first.is_alphabetic() && first != '_' {
        return None;
    }
    if chars.any(|c| !c.is_alphanumeric() && c != '_') {
        return None;
    }
    Some(label)
}

/// Parse a `; VPy_LINE:N` annotation from an ASM line.
/// Returns the VPy source line number if present.
pub(crate) fn parse_vpy_line_annotation(line: &str) -> Option<usize> {
    // Accepts "; VPy_LINE:42" or "; VPy_LINE: 42" (case-insensitive prefix)
    let comment_start = line.find(';')?;
    let comment = line[comment_start + 1..].trim();
    let rest = comment.strip_prefix("VPy_LINE:")?;
    rest.trim().parse::<usize>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_extraction_empty() {
        let symbols = extract_symbols(&[]).unwrap();
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_symbol_extraction_from_asm_text() {
        let asm = b"START:\n    NOP\nLOOP:\n    BRA LOOP\n";
        let symbols = extract_symbols(asm).unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].0, "START");
        assert_eq!(symbols[1].0, "LOOP");
    }

    #[test]
    fn test_parse_label_definition() {
        assert_eq!(parse_label_definition("START:"), Some("START"));
        assert_eq!(parse_label_definition("LOOP:   ; comment"), Some("LOOP"));
        assert_eq!(parse_label_definition("    NOP"), None); // indented
        assert_eq!(parse_label_definition(".local:"), None); // local label
        assert_eq!(parse_label_definition(""), None);
    }

    #[test]
    fn test_parse_vpy_line_annotation() {
        assert_eq!(parse_vpy_line_annotation("    NOP   ; VPy_LINE:42"), Some(42));
        assert_eq!(parse_vpy_line_annotation("    NOP   ; VPy_LINE: 7"), Some(7));
        assert_eq!(parse_vpy_line_annotation("    NOP"), None);
        assert_eq!(parse_vpy_line_annotation("    NOP   ; comment"), None);
    }
}
