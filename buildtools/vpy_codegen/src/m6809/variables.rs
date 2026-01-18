use vpy_parser::{Module, Item, Expr, Stmt, AssignTarget};
use std::collections::HashSet;
use super::ram_layout::RamLayout;

/// Generate user variables using a RamLayout that already has system variables allocated
/// Returns ASM string for array data sections (not EQU definitions - those come from RamLayout)
pub fn generate_user_variables(module: &Module, ram: &mut RamLayout) -> Result<String, String> {
    let asm = String::new();
    let mut vars = HashSet::new();
    let mut arrays = Vec::new();  // Track arrays for data generation
    
    // Collect all variable names from module (GlobalLet items)
    for item in &module.items {
        if let Item::GlobalLet { name, value, .. } = item {
            vars.insert(name.clone());
            
            // Check if this is an array initialization
            if matches!(value, Expr::List(_)) {
                arrays.push((name.clone(), value.clone()));
            }
        }
    }
    
    // CRITICAL FIX: Also collect all identifiers used in functions
    // This includes function parameters and local variables
    // Treat them all as globals for now (simple solution)
    for item in &module.items {
        if let Item::Function(func) = item {
            // Collect parameters - they are Vec<String> not Vec<Param>
            for param in &func.params {
                vars.insert(param.clone());
            }
            
            // Collect local variables from function body
            collect_identifiers_from_stmts(&func.body, &mut vars);
        }
    }
    
    // Allocate all user variables using RamLayout
    // This ensures no collisions with system variables
    for var in vars.iter() {
        // Variables use uppercase labels for consistency with array/const naming
        ram.allocate(&format!("VAR_{}", var.to_uppercase()), 2, &format!("User variable: {}", var));
    }
    
    // NOTE: Array data moved to emit_array_data() function
    // Arrays are emitted BEFORE code (after EQU definitions) to ensure labels are defined before use
    
    // NOTE: VAR_ARG definitions are now in helpers.rs using ram.allocate_fixed()
    // They are emitted alongside system variables because they need fixed addresses
    
    // NOTE: Internal variables (DRAW_VEC_X, MIRROR_Y, etc.) are now allocated via RamLayout in helpers.rs
    // This prevents collisions with scratchpad variables like TEMP_YX
    
    Ok(asm)
}

/// Emit array data sections (must be called AFTER EQU definitions, BEFORE code)
/// Arrays stored in ROM with ARRAY_{name}_DATA labels
/// At runtime, main() initializes VAR_{name} (RAM pointer) to point to this ROM data
pub fn emit_array_data(module: &Module) -> String {
    let mut asm = String::new();
    let mut arrays = Vec::new();
    
    // Collect arrays from module (both GlobalLet and Const)
    for item in &module.items {
        match item {
            Item::GlobalLet { name, value, .. } => {
                if matches!(value, Expr::List(_)) {
                    arrays.push((name.clone(), value.clone()));
                }
            }
            Item::Const { name, value, .. } => {
                if matches!(value, Expr::List(_)) {
                    arrays.push((name.clone(), value.clone()));
                }
            }
            _ => {}
        }
    }
    
    if arrays.is_empty() {
        return asm;
    }
    
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; ARRAY DATA (ROM literals)\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; Arrays are stored in ROM and accessed via pointers\n");
    asm.push_str("; At startup, main() initializes VAR_{name} to point to ARRAY_{name}_DATA\n\n");
    
    // Emit array data in ROM (no ORG - flows naturally after EQU definitions)
    for (name, value) in arrays {
        if let Expr::List(elements) = value {
            let array_label = format!("ARRAY_{}_DATA", name.to_uppercase());
            
            // Check if this is a string array (all elements are StringLit)
            let is_string_array = elements.iter().all(|e| matches!(e, Expr::StringLit(_)));
            
            if is_string_array {
                // String array: emit individual strings with labels + pointer table
                asm.push_str(&format!("; String array literal for variable '{}' ({} elements)\n", name, elements.len()));
                
                let mut string_labels = Vec::new();
                
                // Emit individual strings
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::StringLit(s) = elem {
                        let str_label = format!("{}_STR_{}", array_label, i);
                        string_labels.push(str_label.clone());
                        
                        asm.push_str(&format!("{}:\n", str_label));
                        asm.push_str(&format!("    FCC \"{}\"\n", s.to_ascii_uppercase()));
                        asm.push_str("    FCB $80   ; String terminator (high bit)\n");
                    }
                }
                
                // Emit pointer table
                asm.push_str(&format!("\n{}:  ; Pointer table for {}\n", array_label, name));
                for str_label in string_labels {
                    asm.push_str(&format!("    FDB {}  ; Pointer to string\n", str_label));
                }
                asm.push_str("\n");
            } else {
                // Number array: emit FDB values
                asm.push_str(&format!("; Array literal for variable '{}' ({} elements)\n", name, elements.len()));
                asm.push_str(&format!("{}:\n", array_label));
                
                // Emit array elements
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::Number(n) = elem {
                        asm.push_str(&format!("    FDB {}   ; Element {}\n", n, i));
                    } else {
                        asm.push_str(&format!("    FDB 0    ; Element {} (TODO: complex init)\n", i));
                    }
                }
                asm.push_str("\n");
            }
        }
    }
    
    asm
}

/// OLD FUNCTION - kept for backward compatibility but not used anymore
/// Use generate_user_variables() instead with RamLayout parameter
pub fn generate_variables(module: &Module) -> Result<String, String> {
    let mut asm = String::new();
    let mut vars = HashSet::new();
    let mut arrays = Vec::new();  // Track arrays for data generation
    
    // Collect all variable names from module (GlobalLet items)
    for item in &module.items {
        if let Item::GlobalLet { name, value, .. } = item {
            vars.insert(name.clone());
            
            // Check if this is an array initialization
            if matches!(value, Expr::List(_)) {
                arrays.push((name.clone(), value.clone()));
            }
        }
    }
    
    // CRITICAL FIX: Also collect all identifiers used in functions
    // This includes function parameters and local variables
    // Treat them all as globals for now (simple solution)
    for item in &module.items {
        if let Item::Function(func) = item {
            // Collect parameters - they are Vec<String> not Vec<Param>
            for param in &func.params {
                vars.insert(param.clone());
            }
            
            // Collect local variables from function body
            collect_identifiers_from_stmts(&func.body, &mut vars);
        }
    }
    
    // Generate RAM variable definitions
    if !vars.is_empty() {
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; USER VARIABLES\n");
        asm.push_str(";***************************************************************************\n");
        
        let mut offset = 0;
        for var in vars.iter() {
            // Variables use uppercase labels for consistency with array/const naming
            asm.push_str(&format!("VAR_{} EQU $CF10+{}\n", var.to_uppercase(), offset));
            offset += 2;  // 16-bit variables
        }
        
        asm.push_str("\n");
    }
    
    // Generate array data sections
    // Arrays are stored in ROM as FDB data with ARRAY_{name}_DATA labels
    // At runtime, main() initializes VAR_{name} (RAM pointer) to point to this ROM data
    if !arrays.is_empty() {
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; ARRAY DATA (ROM literals)\n");
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; Arrays are stored in ROM and accessed via pointers\n");
        asm.push_str("; At startup, main() initializes VAR_{name} to point to ARRAY_{name}_DATA\n\n");
        
        // Emit array data in ROM (no ORG - flows naturally after code)
        for (name, value) in arrays {
            if let Expr::List(elements) = value {
                let array_label = format!("ARRAY_{}_DATA", name.to_uppercase());
                asm.push_str(&format!("; Array literal for variable '{}' ({} elements)\n", name, elements.len()));
                asm.push_str(&format!("{}:\n", array_label));
                
                // Emit array elements
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::Number(n) = elem {
                        asm.push_str(&format!("    FDB {}   ; Element {}\n", n, i));
                    } else {
                        asm.push_str(&format!("    FDB 0    ; Element {} (TODO: complex init)\n", i));
                    }
                }
                asm.push_str("\n");
            }
        }
    }
    
    // NOTE: VAR_ARG definitions moved to Bank #31 (helpers bank) in mod.rs
    // They are emitted alongside helpers because Bank #31 is always visible at $4000-$7FFF
    // This allows all banks to access VAR_ARG without duplication
    // See mod.rs line ~325 for the actual emission
    
    // NOTE: Internal variables (DRAW_VEC_X, MIRROR_Y, etc.) are now allocated via RamLayout in helpers.rs
    // This prevents collisions with scratchpad variables like TEMP_YX
    
    Ok(asm)
}

/// Recursively collect all identifiers from statements
/// This captures local variables and any identifiers used in expressions
fn collect_identifiers_from_stmts(stmts: &[Stmt], vars: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Assign { target, value, .. } => {
                // Collect from assignment target
                match target {
                    AssignTarget::Ident { name, .. } => {
                        vars.insert(name.clone());
                    }
                    AssignTarget::Index { target, .. } => {
                        collect_identifiers_from_expr(target, vars);
                    }
                    AssignTarget::FieldAccess { target, .. } => {
                        collect_identifiers_from_expr(target, vars);
                    }
                }
                
                // Collect from value expression
                collect_identifiers_from_expr(value, vars);
            }
            Stmt::Let { name, value, .. } => {
                vars.insert(name.clone());
                collect_identifiers_from_expr(value, vars);
            }
            Stmt::If { cond, body, elifs, else_body, .. } => {
                collect_identifiers_from_expr(cond, vars);
                collect_identifiers_from_stmts(body, vars);
                for (elif_cond, elif_body) in elifs {
                    collect_identifiers_from_expr(elif_cond, vars);
                    collect_identifiers_from_stmts(elif_body, vars);
                }
                if let Some(else_stmts) = else_body {
                    collect_identifiers_from_stmts(else_stmts, vars);
                }
            }
            Stmt::While { cond, body, .. } => {
                collect_identifiers_from_expr(cond, vars);
                collect_identifiers_from_stmts(body, vars);
            }
            Stmt::For { var, start, end, step, body, .. } => {
                vars.insert(var.clone());
                collect_identifiers_from_expr(start, vars);
                collect_identifiers_from_expr(end, vars);
                if let Some(step_expr) = step {
                    collect_identifiers_from_expr(step_expr, vars);
                }
                collect_identifiers_from_stmts(body, vars);
            }
            Stmt::ForIn { var, iterable, body, .. } => {
                vars.insert(var.clone());
                collect_identifiers_from_expr(iterable, vars);
                collect_identifiers_from_stmts(body, vars);
            }
            Stmt::Return(value, _) => {
                if let Some(expr) = value {
                    collect_identifiers_from_expr(expr, vars);
                }
            }
            Stmt::Expr(expr, _) => {
                collect_identifiers_from_expr(expr, vars);
            }
            Stmt::CompoundAssign { target, value, .. } => {
                match target {
                    AssignTarget::Ident { name, .. } => {
                        vars.insert(name.clone());
                    }
                    AssignTarget::Index { target, .. } => {
                        collect_identifiers_from_expr(target, vars);
                    }
                    AssignTarget::FieldAccess { target, .. } => {
                        collect_identifiers_from_expr(target, vars);
                    }
                }
                collect_identifiers_from_expr(value, vars);
            }
            _ => {}
        }
    }
}

/// Recursively collect identifiers from an expression
fn collect_identifiers_from_expr(expr: &Expr, vars: &mut HashSet<String>) {
    match expr {
        Expr::Ident(id) => {
            vars.insert(id.name.clone());
        }
        Expr::Binary { left, right, .. } => {
            collect_identifiers_from_expr(left, vars);
            collect_identifiers_from_expr(right, vars);
        }
        Expr::Compare { left, right, .. } => {
            collect_identifiers_from_expr(left, vars);
            collect_identifiers_from_expr(right, vars);
        }
        Expr::Logic { left, right, .. } => {
            collect_identifiers_from_expr(left, vars);
            collect_identifiers_from_expr(right, vars);
        }
        Expr::Not(operand) => {
            collect_identifiers_from_expr(operand, vars);
        }
        Expr::BitNot(operand) => {
            collect_identifiers_from_expr(operand, vars);
        }
        Expr::Call(call_info) => {
            // CallInfo has name field, not func
            for arg in &call_info.args {
                collect_identifiers_from_expr(arg, vars);
            }
        }
        Expr::MethodCall(method_info) => {
            collect_identifiers_from_expr(&method_info.target, vars);
            for arg in &method_info.args {
                collect_identifiers_from_expr(arg, vars);
            }
        }
        Expr::Index { target, index, .. } => {
            collect_identifiers_from_expr(target, vars);
            collect_identifiers_from_expr(index, vars);
        }
        Expr::FieldAccess { target, .. } => {
            collect_identifiers_from_expr(target, vars);
        }
        Expr::List(elements) => {
            for elem in elements {
                collect_identifiers_from_expr(elem, vars);
            }
        }
        _ => {}
    }
}
