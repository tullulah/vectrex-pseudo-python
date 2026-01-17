use vpy_parser::{Module, Item, Expr, Stmt, AssignTarget};
use std::collections::HashSet;

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
            // IMPORTANT: Variable name already comes uppercase from unifier
            asm.push_str(&format!("VAR_{} EQU $CF10+{}\n", var, offset));
            offset += 2;  // 16-bit variables
        }
        
        asm.push_str("\n");
    }
    
    // Generate array data sections
    if !arrays.is_empty() {
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; ARRAY DATA\n");
        asm.push_str(";***************************************************************************\n");
        
        // Calculate starting address for array data (after all variable pointers)
        // Variables start at $CF10, each takes 2 bytes
        let array_data_start = 0xCF10 + (vars.len() * 2);
        let mut data_offset = 0;
        
        // First pass: Generate EQU definitions for array data addresses
        for (name, value) in arrays.iter() {
            if let Expr::List(elements) = value {
                // Define array data address as EQU (so assembler can resolve in first pass)
                asm.push_str(&format!("VAR_{}_DATA EQU ${:04X}\n", name, array_data_start + data_offset));
                data_offset += elements.len() * 2;  // 16-bit elements
            }
        }
        asm.push_str("\n");
        
        // Second pass: Emit actual array data
        asm.push_str("; Array data storage\n");
        asm.push_str(&format!("    ORG ${:04X}  ; Start of array data section\n", array_data_start));
        for (name, value) in arrays {
            if let Expr::List(elements) = value {
                asm.push_str(&format!("; Array: VAR_{}_DATA\n", name));
                
                // Emit array elements
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::Number(n) = elem {
                        asm.push_str(&format!("    FDB {}    ; Element {}\n", n, i));
                    } else {
                        asm.push_str(&format!("    FDB 0     ; Element {} (TODO: complex init)\n", i));
                    }
                }
            }
        }
        asm.push_str("\n");
    }
    
    // Always define ARG variables for function calls
    asm.push_str("; Function argument slots\n");
    for i in 0..5 {
        asm.push_str(&format!("VAR_ARG{} EQU $CFE0+{}\n", i, i * 2));
    }
    asm.push_str("\n");
    
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
