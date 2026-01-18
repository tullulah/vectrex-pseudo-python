//! Function Code Generation
//!
//! Generates M6809 assembly for VPy functions

use vpy_parser::{Module, Function, Stmt, Expr};
use super::expressions;

/// Check if module uses PLAY_MUSIC or PLAY_SFX (needs AUDIO_UPDATE auto-injection)
/// Check if module uses PLAY_MUSIC or PLAY_SFX builtins
/// Used to determine if AUDIO_UPDATE helper should be auto-injected
pub fn has_audio_calls(module: &Module) -> bool {
    fn check_expr(expr: &Expr) -> bool {
        match expr {
            Expr::Call(call_info) => {
                call_info.name == "PLAY_MUSIC" || call_info.name == "PLAY_SFX"
            },
            _ => false,
        }
    }
    
    fn check_stmt(stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Expr(expr, _) => check_expr(expr),
            Stmt::If { cond, body, elifs, else_body, .. } => {
                check_expr(cond) ||
                body.iter().any(check_stmt) ||
                elifs.iter().any(|(e, b)| check_expr(e) || b.iter().any(check_stmt)) ||
                else_body.as_ref().map_or(false, |body| body.iter().any(check_stmt))
            },
            Stmt::While { cond, body, .. } => check_expr(cond) || body.iter().any(check_stmt),
            Stmt::For { body, .. } => body.iter().any(check_stmt),
            _ => false,
        }
    }
    
    module.items.iter().any(|item| {
        if let vpy_parser::Item::Function(func) = item {
            func.body.iter().any(check_stmt)
        } else {
            false
        }
    })
}

pub fn generate_functions(module: &Module) -> Result<String, String> {
    let mut asm = String::new();
    
    // Find main() and loop()
    let mut main_fn = None;
    let mut loop_fn = None;
    let mut other_fns = Vec::new();
    
    for item in &module.items {
        if let vpy_parser::Item::Function(func) = item {
            // IMPORTANT: After unifier, function names are uppercase
            match func.name.to_uppercase().as_str() {
                "MAIN" => main_fn = Some(func),
                "LOOP" => loop_fn = Some(func),
                _ => other_fns.push(func),
            }
        }
    }
    
    // Generate MAIN entry point
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; MAIN PROGRAM\n");
    asm.push_str(";***************************************************************************\n\n");
    
    asm.push_str("MAIN:\n");
    
    // Initialize global variables with their initial values
    asm.push_str("    ; Initialize global variables\n");
    for item in &module.items {
        if let vpy_parser::Item::GlobalLet { name, value, .. } = item {
            if let vpy_parser::Expr::List(_elements) = value {
                // Array initialization: load ROM address into RAM variable
                // The array data is in ROM with label ARRAY_{NAME}_DATA
                // VAR_{NAME} is a 2-byte RAM pointer to the array
                let array_label = format!("ARRAY_{}_DATA", name.to_uppercase());
                asm.push_str(&format!("    LDX #{}    ; Array literal\n", array_label));
                asm.push_str(&format!("    STX VAR_{}\n", name.to_uppercase()));
            } else {
                // Non-array initialization
                if let vpy_parser::Expr::Number(n) = value {
                    asm.push_str(&format!("    LDD #{}\n", n));
                    asm.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
            }
        }
    }
    
    // Call main() if exists
    if let Some(main) = main_fn {
        asm.push_str("    ; Call main() for initialization\n");
        generate_function_body(main, &mut asm)?;
    }
    
    // Infinite loop calling loop()
    asm.push_str("\n.MAIN_LOOP:\n");
    asm.push_str("    JSR LOOP_BODY\n");
    asm.push_str("    BRA .MAIN_LOOP\n\n");
    
    // Generate LOOP_BODY
    if let Some(loop_fn) = loop_fn {
        asm.push_str("LOOP_BODY:\n");
        // Inject WAIT_RECAL at the start of every loop
        asm.push_str("    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)\n");
        // Inject Reset0Ref to position beam at center (0,0) before drawing
        asm.push_str("    JSR Reset0Ref    ; Reset beam to center (0,0)\n");
        // CRITICAL (2026-01-19): Button reading with proper DP handling
        // This sequence MUST happen before any user code to ensure DP=$C8 for normal RAM access
        asm.push_str("    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access\n");
        asm.push_str("    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)\n");
        asm.push_str("    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access\n");
        generate_function_body(loop_fn, &mut asm)?;
        
        // Auto-inject AUDIO_UPDATE at END if module uses PLAY_MUSIC/PLAY_SFX
        if has_audio_calls(module) {
            asm.push_str("    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)\n");
        }
        
        asm.push_str("    RTS\n\n");
    } else {
        // Empty loop if not defined
        asm.push_str("LOOP_BODY:\n");
        asm.push_str("    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)\n");
        asm.push_str("    JSR Reset0Ref    ; Reset beam to center (0,0)\n");
        // CRITICAL: Button reading with proper DP handling (even if no user code)
        asm.push_str("    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access\n");
        asm.push_str("    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)\n");
        asm.push_str("    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access\n");
        asm.push_str("    RTS\n\n");
    }
    
    // Generate other user functions (excluding MAIN and LOOP which are handled above)
    for func in other_fns {
        // Double-check: skip MAIN and LOOP (should already be filtered but be safe)
        let name_upper = func.name.to_uppercase();
        if name_upper == "MAIN" || name_upper == "LOOP" {
            continue;  // Already handled above
        }
        
        // IMPORTANT: Function name already comes uppercase from unifier
        asm.push_str(&format!("; Function: {}\n", func.name));
        asm.push_str(&format!("{}:\n", func.name));
        generate_function_body(func, &mut asm)?;
        
        // Only add RTS if function doesn't end with explicit return
        let has_explicit_return = func.body.last()
            .map(|stmt| matches!(stmt, Stmt::Return(..)))
            .unwrap_or(false);
        if !has_explicit_return {
            asm.push_str("    RTS\n");
        }
        asm.push_str("\n");
    }
    
    Ok(asm)
}

fn generate_function_body(func: &Function, asm: &mut String) -> Result<(), String> {
    // Generate code for each statement
    for stmt in &func.body {
        generate_statement(stmt, asm)?;
    }
    Ok(())
}

fn generate_statement(stmt: &Stmt, asm: &mut String) -> Result<(), String> {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            match target {
                vpy_parser::AssignTarget::Ident { name, .. } => {
                    // Simple variable assignment: var = value
                    // 1. Evaluate expression
                    expressions::emit_simple_expr(value, asm);
                    
                    // 2. Store to variable (uppercase for consistency)
                    asm.push_str("    LDD RESULT\n");
                    asm.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
                
                vpy_parser::AssignTarget::Index { target: array_expr, index, .. } => {
                    // Array indexed assignment: arr[index] = value
                    // Only support simple variable arrays (not complex expressions)
                    let array_name = if let vpy_parser::Expr::Ident(id) = &**array_expr {
                        &id.name
                    } else {
                        return Err("Complex array expressions not yet supported in assignment".to_string());
                    };
                    
                    // 1. Evaluate index first
                    expressions::emit_simple_expr(index, asm);
                    asm.push_str("    LDD RESULT\n");
                    asm.push_str("    ASLB            ; Multiply index by 2 (16-bit elements)\n");
                    asm.push_str("    ROLA\n");
                    asm.push_str("    STD TMPPTR      ; Save offset temporarily\n");
                    
                    // 2. Load array base address (name already uppercase from unifier)
                    asm.push_str(&format!("    LDD #ARRAY_{}_DATA  ; Load array data address\n", array_name));
                    
                    // 3. Add offset to base pointer
                    asm.push_str("    TFR D,X         ; X = array base pointer\n");
                    asm.push_str("    LDD TMPPTR      ; D = offset\n");
                    asm.push_str("    LEAX D,X        ; X = base + offset\n");
                    asm.push_str("    STX TMPPTR2     ; Save computed address\n");
                    
                    // 4. Evaluate value to assign
                    expressions::emit_simple_expr(value, asm);
                    
                    // 5. Store value at computed address
                    asm.push_str("    LDX TMPPTR2     ; Load computed address\n");
                    asm.push_str("    LDD RESULT      ; Load value\n");
                    asm.push_str("    STD ,X          ; Store value\n");
                }
                
                _ => {
                    return Err(format!("Assignment target {:?} not yet supported", target));
                }
            }
        }
        
        Stmt::CompoundAssign { target, op, value, .. } => {
            // Load current value
            match target {
                vpy_parser::AssignTarget::Ident { name, .. } => {
                    // IMPORTANT: Name already comes uppercase from unifier
                    asm.push_str(&format!("    LDD VAR_{}\n", name));
                    asm.push_str("    PSHS D\n");
                    
                    // Evaluate right side
                    expressions::emit_simple_expr(value, asm);
                    asm.push_str("    LDD RESULT\n");
                    
                    // Perform operation
                    match op {
                        vpy_parser::BinOp::Add => asm.push_str("    ADDD ,S++\n"),
                        vpy_parser::BinOp::Sub => asm.push_str("    SUBD ,S++\n"),
                        _ => return Err(format!("Aug-assign {:?} not yet supported", op)),
                    }
                    
                    // Store back (uppercase for consistency)
                    asm.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
                _ => return Err("Complex assignment targets not yet supported".to_string()),
            }
        }
        
        Stmt::Expr(expr, ..) => {
            expressions::emit_simple_expr(expr, asm);
        }
        
        Stmt::If { cond, body, else_body, .. } => {
            // Evaluate condition
            expressions::emit_simple_expr(cond, asm);
            
            // Branch if zero (false)
            asm.push_str("    LDD RESULT\n");
            asm.push_str("    LBEQ .IF_ELSE\n");
            
            // Then body
            for stmt in body {
                generate_statement(stmt, asm)?;
            }
            asm.push_str("    LBRA .IF_END\n");
            
            // Else body (ignoring elifs for now)
            asm.push_str(".IF_ELSE:\n");
            if let Some(else_stmts) = else_body {
                for stmt in else_stmts {
                    generate_statement(stmt, asm)?;
                }
            }
            
            asm.push_str(".IF_END:\n");
        }
        
        Stmt::While { cond, body, .. } => {
            asm.push_str(".WHILE_START:\n");
            
            // Evaluate condition
            expressions::emit_simple_expr(cond, asm);
            asm.push_str("    LDD RESULT\n");
            asm.push_str("    LBEQ .WHILE_END\n");
            
            // Body
            for stmt in body {
                generate_statement(stmt, asm)?;
            }
            
            asm.push_str("    LBRA .WHILE_START\n");
            asm.push_str(".WHILE_END:\n");
        }
        
        Stmt::Return(expr, ..) => {
            if let Some(e) = expr {
                expressions::emit_simple_expr(e, asm);
            }
            asm.push_str("    RTS\n");
        }
        
        _ => {
            asm.push_str(&format!("    ; TODO: Statement {:?}\n", stmt));
        }
    }
    
    Ok(())
}
