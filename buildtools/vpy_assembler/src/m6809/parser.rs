//! ASM directive parsing utilities

/// Parse VPy line marker from comment
/// Format: `; VPy line 42`
pub fn parse_vpy_line_marker(line: &str) -> Option<usize> {
    if let Some(marker) = line.strip_prefix("; VPy line ") {
        marker.trim().parse().ok()
    } else {
        None
    }
}

/// Parse EQU directive (raw, returns expression as string)
/// Format: `LABEL EQU expression`
pub fn parse_equ_directive_raw(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 && parts[1].eq_ignore_ascii_case("EQU") {
        let name = parts[0].to_string();
        let expr = parts[2..].join(" ");
        Some((name, expr))
    } else {
        None
    }
}

/// Parse label (ends with :)
/// Returns label name without the colon
pub fn parse_label(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    
    // Check if line ends with colon
    if !trimmed.ends_with(':') {
        return None;
    }
    
    // Extract label name (everything before colon)
    let label = &trimmed[..trimmed.len() - 1];
    
    // Valid label: starts with letter or underscore, no spaces
    if !label.is_empty() && !label.contains(' ') {
        Some(label)
    } else {
        None
    }
}

/// Parse INCLUDE directive
/// Format: `INCLUDE "path/to/file.i"`
pub fn parse_include_directive(line: &str) -> Option<String> {
    let upper = line.to_uppercase();
    if !upper.trim().starts_with("INCLUDE") {
        return None;
    }
    
    // Extract path between quotes
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            let path = &line[start + 1..start + 1 + end];
            return Some(path.to_string());
        }
    }
    
    None
}

/// Expand local label with last global label prefix
/// Local labels start with '.'
pub fn expand_local_label(operand: &str, last_global: &str) -> String {
    if operand.starts_with('.') {
        format!("{}{}", last_global, operand)
    } else {
        operand.to_string()
    }
}

/// Check if operand looks like a label (not a number or address mode)
pub fn is_label(operand: &str) -> bool {
    !operand.starts_with('#') 
        && !operand.starts_with('$')
        && !operand.starts_with('<')
        && !operand.starts_with('>')
        && !operand.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vpy_line_marker() {
        assert_eq!(parse_vpy_line_marker("; VPy line 42"), Some(42));
        assert_eq!(parse_vpy_line_marker("; Other comment"), None);
    }

    #[test]
    fn test_parse_label() {
        assert_eq!(parse_label("MAIN:"), Some("MAIN"));
        assert_eq!(parse_label(".local:"), Some(".local"));
        assert_eq!(parse_label("    LDA #10"), None);
    }

    #[test]
    fn test_expand_local_label() {
        assert_eq!(expand_local_label(".local", "MAIN"), "MAIN.local");
        assert_eq!(expand_local_label("GLOBAL", "MAIN"), "GLOBAL");
    }

    #[test]
    fn test_is_label() {
        assert!(is_label("SYMBOL"));
        assert!(!is_label("#10"));
        assert!(!is_label("$D000"));
        assert!(!is_label("123"));
    }
}
