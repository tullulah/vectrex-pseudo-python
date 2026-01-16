//! Expression evaluation for assembler

use std::collections::HashMap;

/// Evaluate arithmetic expression with symbols
/// Supports: +, -, *, /, (), symbols, hex ($HHHHH), decimal
pub fn evaluate_expression(expr: &str, equates: &HashMap<String, u16>) -> Result<u16, String> {
    let expr = expr.trim();
    
    // Handle parentheses first (recursive)
    if let Some(open) = expr.find('(') {
        if let Some(close) = expr.rfind(')') {
            let before = &expr[..open];
            let inside = &expr[open + 1..close];
            let after = &expr[close + 1..];
            
            let inside_val = evaluate_expression(inside, equates)?;
            let new_expr = format!("{}{}{}", before, inside_val, after);
            return evaluate_expression(&new_expr, equates);
        }
    }
    
    // Handle addition
    if let Some(pos) = expr.rfind('+') {
        let left = &expr[..pos];
        let right = &expr[pos + 1..];
        let left_val = evaluate_expression(left, equates)?;
        let right_val = evaluate_expression(right, equates)?;
        return Ok(left_val.wrapping_add(right_val));
    }
    
    // Handle subtraction (but not negative literals like -10)
    if let Some(pos) = expr.rfind('-') {
        if pos > 0 {
            let left = &expr[..pos];
            let right = &expr[pos + 1..];
            let left_val = evaluate_expression(left, equates)?;
            let right_val = evaluate_expression(right, equates)?;
            return Ok(left_val.wrapping_sub(right_val));
        }
    }
    
    // Handle multiplication
    if let Some(pos) = expr.rfind('*') {
        let left = &expr[..pos];
        let right = &expr[pos + 1..];
        let left_val = evaluate_expression(left, equates)?;
        let right_val = evaluate_expression(right, equates)?;
        return Ok(left_val.wrapping_mul(right_val));
    }
    
    // Handle division
    if let Some(pos) = expr.rfind('/') {
        let left = &expr[..pos];
        let right = &expr[pos + 1..];
        let left_val = evaluate_expression(left, equates)?;
        let right_val = evaluate_expression(right, equates)?;
        if right_val == 0 {
            return Err("Division by zero".to_string());
        }
        return Ok(left_val / right_val);
    }
    
    // Base case: single value (number or symbol)
    resolve_symbol_value(expr, equates)
}

/// Resolve symbol or literal value
pub fn resolve_symbol_value(symbol: &str, equates: &HashMap<String, u16>) -> Result<u16, String> {
    let symbol = symbol.trim();
    
    // Try as number first
    if let Ok(value) = parse_number(symbol) {
        return Ok(value);
    }
    
    // Try as symbol
    if let Some(&value) = equates.get(symbol) {
        return Ok(value);
    }
    
    // Handle high/low byte operators
    if symbol.starts_with('>') {
        let inner = &symbol[1..];
        let value = resolve_symbol_value(inner, equates)?;
        return Ok((value >> 8) & 0xFF);
    }
    
    if symbol.starts_with('<') {
        let inner = &symbol[1..];
        let value = resolve_symbol_value(inner, equates)?;
        return Ok(value & 0xFF);
    }
    
    Err(format!("Unknown symbol: {}", symbol))
}

/// Parse number (hex $HHHH or decimal)
pub fn parse_number(s: &str) -> Result<u16, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('$') {
        u16::from_str_radix(hex, 16)
            .map_err(|_| format!("Invalid hex: {}", s))
    } else {
        s.parse::<u16>()
            .map_err(|_| format!("Invalid number: {}", s))
    }
}

/// Parse symbol and optional addend (e.g., "SYMBOL+10" or "SYMBOL-5")
pub fn parse_symbol_and_addend(sym_expr: &str) -> Result<(String, i16), String> {
    let sym_expr = sym_expr.trim();
    
    // Find last + or - (but not at start for negative numbers)
    let mut op_pos = None;
    let mut op_char = ' ';
    
    for (i, ch) in sym_expr.char_indices().rev() {
        if (ch == '+' || ch == '-') && i > 0 {
            op_pos = Some(i);
            op_char = ch;
            break;
        }
    }
    
    if let Some(pos) = op_pos {
        let symbol = sym_expr[..pos].trim().to_string();
        let addend_str = sym_expr[pos + 1..].trim();
        let addend_val = parse_number(addend_str)
            .map_err(|_| format!("Invalid addend: {}", addend_str))?;
        
        let addend = if op_char == '-' {
            -(addend_val as i16)
        } else {
            addend_val as i16
        };
        
        Ok((symbol, addend))
    } else {
        Ok((sym_expr.to_string(), 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("$FF"), Ok(255));
        assert_eq!(parse_number("100"), Ok(100));
        assert!(parse_number("invalid").is_err());
    }

    #[test]
    fn test_evaluate_simple() {
        let mut equates = HashMap::new();
        equates.insert("BASE".to_string(), 0x1000);
        
        assert_eq!(evaluate_expression("$FF", &equates), Ok(255));
        assert_eq!(evaluate_expression("BASE", &equates), Ok(0x1000));
    }

    #[test]
    fn test_evaluate_arithmetic() {
        let equates = HashMap::new();
        
        assert_eq!(evaluate_expression("10+20", &equates), Ok(30));
        assert_eq!(evaluate_expression("100-50", &equates), Ok(50));
        assert_eq!(evaluate_expression("5*4", &equates), Ok(20));
        assert_eq!(evaluate_expression("100/5", &equates), Ok(20));
    }

    #[test]
    fn test_parse_symbol_and_addend() {
        assert_eq!(parse_symbol_and_addend("SYMBOL"), Ok(("SYMBOL".to_string(), 0)));
        assert_eq!(parse_symbol_and_addend("SYMBOL+10"), Ok(("SYMBOL".to_string(), 10)));
        assert_eq!(parse_symbol_and_addend("SYMBOL-5"), Ok(("SYMBOL".to_string(), -5)));
    }
}
