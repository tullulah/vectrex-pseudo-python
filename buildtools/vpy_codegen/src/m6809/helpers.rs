//! Runtime Helper Functions
//!
//! Mathematical and utility functions

use vpy_parser::{Module, Item, Stmt, Expr};
use std::collections::HashSet;

/// Analyze module to detect which runtime helpers are needed
/// Returns set of helper names that should be emitted
fn analyze_needed_helpers(module: &Module) -> HashSet<String> {
    let mut needed = HashSet::new();
    
    // Scan all functions in module
    for item in &module.items {
        if let Item::Function { body, .. } = item {
            for stmt in body {
                analyze_stmt_for_helpers(stmt, &mut needed);
            }
        }
    }
    
    needed
}

/// Recursively analyze statement for helper usage
fn analyze_stmt_for_helpers(stmt: &Stmt, needed: &mut HashSet<String>) {
    match stmt {
        Stmt::Expr(expr) => analyze_expr_for_helpers(expr, needed),
        Stmt::Assign { value, .. } => analyze_expr_for_helpers(value, needed),
        Stmt::If { condition, then_block, else_block, .. } => {
            analyze_expr_for_helpers(condition, needed);
            for s in then_block {
                analyze_stmt_for_helpers(s, needed);
            }
            if let Some(else_stmts) = else_block {
                for s in else_stmts {
                    analyze_stmt_for_helpers(s, needed);
                }
            }
        }
        Stmt::While { condition, body, .. } => {
            analyze_expr_for_helpers(condition, needed);
            for s in body {
                analyze_stmt_for_helpers(s, needed);
            }
        }
        Stmt::Return(Some(expr)) => analyze_expr_for_helpers(expr, needed),
        _ => {}
    }
}

/// Recursively analyze expression for helper usage
fn analyze_expr_for_helpers(expr: &Expr, needed: &mut HashSet<String>) {
    match expr {
        // Builtin calls that may need runtime helpers
        Expr::Call { name, args } => {
            let name_upper = name.to_uppercase();
            
            // Drawing helpers: Need runtime if args contain non-constants
            if name_upper == "DRAW_CIRCLE" && has_variable_args(args) {
                needed.insert("DRAW_CIRCLE_RUNTIME".to_string());
            }
            if name_upper == "DRAW_RECT" && has_variable_args(args) {
                needed.insert("DRAW_RECT_RUNTIME".to_string());
            }
            
            // Joystick helpers: Always needed when called
            if name_upper == "J1_X" {
                needed.insert("J1X_BUILTIN".to_string());
            }
            if name_upper == "J1_Y" {
                needed.insert("J1Y_BUILTIN".to_string());
            }
            if name_upper == "J2_X" {
                needed.insert("J2X_BUILTIN".to_string());
            }
            if name_upper == "J2_Y" {
                needed.insert("J2Y_BUILTIN".to_string());
            }
            
            // Level system helpers
            if name_upper == "SHOW_LEVEL" {
                needed.insert("SHOW_LEVEL_RUNTIME".to_string());
            }
            
            // Utility helpers
            if name_upper == "FADE_IN" {
                needed.insert("FADE_IN_RUNTIME".to_string());
            }
            if name_upper == "FADE_OUT" {
                needed.insert("FADE_OUT_RUNTIME".to_string());
            }
            
            // Math helpers: Need runtime if operands contain variables
            if name_upper == "SQRT" && has_variable_args(args) {
                needed.insert("SQRT_HELPER".to_string());
                needed.insert("DIV16".to_string()); // SQRT uses DIV16
            }
            if name_upper == "POW" && has_variable_args(args) {
                needed.insert("POW_HELPER".to_string());
            }
            if name_upper == "ATAN2" && has_variable_args(args) {
                needed.insert("ATAN2_HELPER".to_string());
            }
            if name_upper == "RAND" {
                needed.insert("RAND_HELPER".to_string());
            }
            if name_upper == "RAND_RANGE" {
                needed.insert("RAND_RANGE_HELPER".to_string());
                needed.insert("RAND_HELPER".to_string()); // RAND_RANGE uses RAND
            }
            
            // Recursively analyze arguments
            for arg in args {
                analyze_expr_for_helpers(arg, needed);
            }
        }
        
        // Binary operations that may need math helpers
        Expr::BinaryOp { left, op, right } => {
            // Check if operands are variables (not constants)
            let left_is_const = matches!(**left, Expr::Number(_));
            let right_is_const = matches!(**right, Expr::Number(_));
            
            if !left_is_const || !right_is_const {
                match op.as_str() {
                    "*" => { needed.insert("MUL16".to_string()); }
                    "/" => { needed.insert("DIV16".to_string()); }
                    "%" => { needed.insert("MOD16".to_string()); }
                    _ => {}
                }
            }
            
            analyze_expr_for_helpers(left, needed);
            analyze_expr_for_helpers(right, needed);
        }
        
        // Other expression types
        Expr::UnaryOp { operand, .. } => analyze_expr_for_helpers(operand, needed),
        Expr::Index { target, index } => {
            analyze_expr_for_helpers(target, needed);
            analyze_expr_for_helpers(index, needed);
        }
        Expr::List(items) => {
            for item in items {
                analyze_expr_for_helpers(item, needed);
            }
        }
        _ => {}
    }
}

/// Check if any argument is not a constant (i.e., contains variables)
fn has_variable_args(args: &[Expr]) -> bool {
    args.iter().any(|arg| !matches!(arg, Expr::Number(_) | Expr::StringLit(_)))
}

/// Get BIOS function address from VECTREX.I
/// Returns the address as a hex string (e.g., "$F1AA")
/// Falls back to hardcoded value if VECTREX.I cannot be read
fn get_bios_address(symbol_name: &str, fallback_address: &str) -> String {
    // Try to get from VECTREX.I
    let possible_paths = vec![
        "ide/frontend/public/include/VECTREX.I",
        "../ide/frontend/public/include/VECTREX.I",
        "../../ide/frontend/public/include/VECTREX.I",
        "./ide/frontend/public/include/VECTREX.I",
    ];
    
    for path in &possible_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            // Parse VECTREX.I to find the symbol
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with(';') {
                    continue;
                }
                
                // Parse lines like: "Wait_Recal  EQU     $F192"
                if let Some(equ_pos) = line.find("EQU") {
                    let name_part = line[..equ_pos].trim();
                    let value_part = line[equ_pos + 3..].trim();
                    
                    if name_part.eq_ignore_ascii_case(symbol_name) {
                        // Extract just the address (e.g., "$F1AA" or "$F1AA   ; comment")
                        if let Some(addr) = value_part.split_whitespace().next() {
                            if addr.starts_with('$') || addr.starts_with("0x") {
                                return addr.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to hardcoded value
    fallback_address.to_string()
}

pub fn generate_helpers(module: &Module) -> Result<String, String> {
    eprintln!("[DEBUG HELPERS] Generating runtime helpers...");
    let mut asm = String::new();
    
    // Analyze module to detect which helpers are needed
    let needed = analyze_needed_helpers(module);
    eprintln!("[DEBUG HELPERS] Detected {} needed helpers: {:?}", needed.len(), needed);
    
    // Get BIOS function addresses from VECTREX.I
    let dp_to_d0 = get_bios_address("DP_to_D0", "$F1AA");
    let dp_to_c8 = get_bios_address("DP_to_C8", "$F1AF");
    
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; RUNTIME HELPERS\n");
    asm.push_str(";***************************************************************************\n\n");
    
    // VECTREX_PRINT_TEXT: Call Print_Str_d with proper setup
    // Entry: VAR_ARG0=x, VAR_ARG1=y, VAR_ARG2=string pointer
    asm.push_str("VECTREX_PRINT_TEXT:\n");
    asm.push_str("    ; VPy signature: PRINT_TEXT(x, y, string)\n");
    asm.push_str("    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_D0 - set Direct Page for BIOS/VIA access\n", dp_to_d0));
    asm.push_str("    LDU VAR_ARG2   ; string pointer (third parameter)\n");
    asm.push_str("    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)\n");
    asm.push_str("    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)\n");
    asm.push_str("    JSR Print_Str_d ; Print string from U register\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_C8 - restore DP before return\n", dp_to_c8));
    asm.push_str("    RTS\n\n");
    
    // VECTREX_PRINT_NUMBER: Print number at position
    // Entry: VAR_ARG0=x, VAR_ARG1=y, VAR_ARG2=number
    asm.push_str("VECTREX_PRINT_NUMBER:\n");
    asm.push_str("    ; VPy signature: PRINT_NUMBER(x, y, num)\n");
    asm.push_str("    ; Convert number to hex string and print\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_D0 - set Direct Page for BIOS/VIA access\n", dp_to_d0));
    asm.push_str("    LDA VAR_ARG1+1   ; Y position\n");
    asm.push_str("    LDB VAR_ARG0+1   ; X position\n");
    asm.push_str("    JSR Moveto_d     ; Move to position\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert number to string (show low byte as hex)\n");
    asm.push_str("    LDA VAR_ARG2+1   ; Load number value\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert high nibble to ASCII\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    ANDA #$0F\n");
    asm.push_str("    CMPA #10\n");
    asm.push_str("    BLO PN_DIGIT1\n");
    asm.push_str("    ADDA #7          ; A-F\n");
    asm.push_str("PN_DIGIT1:\n");
    asm.push_str("    ADDA #'0'\n");
    asm.push_str("    STA NUM_STR      ; Store first digit\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert low nibble to ASCII  \n");
    asm.push_str("    LDA VAR_ARG2+1\n");
    asm.push_str("    ANDA #$0F\n");
    asm.push_str("    CMPA #10\n");
    asm.push_str("    BLO PN_DIGIT2\n");
    asm.push_str("    ADDA #7          ; A-F\n");
    asm.push_str("PN_DIGIT2:\n");
    asm.push_str("    ADDA #'0'\n");
    asm.push_str("    ORA #$80         ; Set high bit for string termination\n");
    asm.push_str("    STA NUM_STR+1    ; Store second digit with high bit\n");
    asm.push_str("    \n");
    asm.push_str("    ; Print the string\n");
    asm.push_str("    LDU #NUM_STR     ; Point to our number string\n");
    asm.push_str("    JSR Print_Str_d  ; Print using BIOS\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_C8 - restore DP before return\n", dp_to_c8));
    asm.push_str("    RTS\n\n");
    
    // Call module-specific runtime helpers with analyzed needed set
    super::math::emit_runtime_helpers(&mut asm, &needed);
    super::joystick::emit_runtime_helpers(&mut asm, &needed);
    super::drawing::emit_runtime_helpers(&mut asm, &needed);
    super::level::emit_runtime_helpers(&mut asm, &needed);
    super::utilities::emit_runtime_helpers(&mut asm, &needed);
    
    eprintln!("[DEBUG HELPERS] ASM length after all helpers: {}", asm.len());
    
    Ok(asm)
}
