// Collectors - Symbol and variable collection functions for M6809 backend
use crate::ast::{Expr, Item, Module, Stmt};
use super::{collect_expr_syms, collect_stmt_syms, collect_locals};
use std::collections::BTreeSet;

pub fn collect_all_vars(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut all_vars = BTreeSet::new();
    for item in &module.items {
        if let Item::Function(f) = item {
            for stmt in &f.body { collect_stmt_syms(stmt, &mut all_vars); }
        } else if let Item::GlobalLet { name, .. } = item { 
            all_vars.insert(name.clone()); 
        } else if let Item::ExprStatement(expr) = item {
            collect_expr_syms(expr, &mut all_vars);
        }
    }
    // Don't remove locals - we need ALL variables for assembly generation
    all_vars.into_iter().collect()
}

// collect_symbols: gather variable identifiers.
#[allow(dead_code)]
pub fn collect_symbols(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut globals = BTreeSet::new();
    let mut locals = BTreeSet::new();
    
    // First pass: collect global names
    let global_names: Vec<String> = module.items.iter()
        .filter_map(|item| {
            if let Item::GlobalLet { name, .. } = item {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();
    
    for item in &module.items {
        if let Item::Function(f) = item {
            for stmt in &f.body { collect_stmt_syms(stmt, &mut globals); }
            for l in collect_locals(&f.body, &global_names) { locals.insert(l); }
        } else if let Item::GlobalLet { name, .. } = item { 
            globals.insert(name.clone()); 
        } else if let Item::ExprStatement(expr) = item {
            collect_expr_syms(expr, &mut globals);
        }
    }
    for l in &locals { globals.remove(l); }
    globals.into_iter().collect()
}

// NEW: Collect global variables with their initial values
pub fn collect_global_vars(module: &Module) -> Vec<(String, Expr)> {
    let mut vars = Vec::new();
    for item in &module.items {
        if let Item::GlobalLet { name, value, .. } = item {
            vars.push((name.clone(), value.clone()));
        }
    }
    vars
}

/// Collect global variables WITH source line numbers
pub fn collect_global_vars_with_line(module: &Module) -> Vec<(String, Expr, usize)> {
    let mut vars = Vec::new();
    for item in &module.items {
        if let Item::GlobalLet { name, value, source_line } = item {
            vars.push((name.clone(), value.clone(), *source_line));
        }
    }
    vars
}

/// Collect constant declarations (const name = value)
/// These are stored in ROM only, not allocated as RAM variables
pub fn collect_const_vars(module: &Module) -> Vec<(String, Expr)> {
    let mut consts = Vec::new();
    for item in &module.items {
        if let Item::Const { name, value, .. } = item {
            consts.push((name.clone(), value.clone()));
        }
    }
    consts
}

/// Collect constant variables WITH source line numbers
pub fn collect_const_vars_with_line(module: &Module) -> Vec<(String, Expr, usize)> {
    let mut consts = Vec::new();
    for item in &module.items {
        if let Item::Const { name, value, source_line } = item {
            consts.push((name.clone(), value.clone(), *source_line));
        }
    }
    consts
}

/// Collect ALL inline array literals from function bodies
/// Returns Vec<(label, elements)> where label is unique for each array
pub fn collect_inline_array_literals(module: &Module) -> Vec<(String, Vec<Expr>)> {
    use crate::ast::{Stmt, Item};
    
    let mut arrays = Vec::new();
    let mut counter = 0;
    
    for item in &module.items {
        if let Item::Function(func) = item {
            collect_inline_arrays_from_stmts(&func.body, &func.name, &mut arrays, &mut counter);
        }
    }
    
    arrays
}

fn collect_inline_arrays_from_stmts(
    stmts: &[Stmt],
    func_name: &str,
    arrays: &mut Vec<(String, Vec<Expr>)>,
    counter: &mut usize
) {
    for stmt in stmts {
        collect_inline_arrays_from_stmt(stmt, func_name, arrays, counter);
    }
}

fn collect_inline_arrays_from_stmt(
    stmt: &Stmt,
    func_name: &str,
    arrays: &mut Vec<(String, Vec<Expr>)>,
    counter: &mut usize
) {
    match stmt {
        Stmt::Let { value, .. } => {
            collect_inline_arrays_from_expr(value, func_name, arrays, counter);
        }
        Stmt::Assign { value, .. } => {
            collect_inline_arrays_from_expr(value, func_name, arrays, counter);
        }
        Stmt::Expr(e, _) => {
            collect_inline_arrays_from_expr(e, func_name, arrays, counter);
        }
        Stmt::Return(Some(e), _) => {
            collect_inline_arrays_from_expr(e, func_name, arrays, counter);
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            collect_inline_arrays_from_expr(cond, func_name, arrays, counter);
            collect_inline_arrays_from_stmts(body, func_name, arrays, counter);
            for (elif_cond, elif_body) in elifs {
                collect_inline_arrays_from_expr(elif_cond, func_name, arrays, counter);
                collect_inline_arrays_from_stmts(elif_body, func_name, arrays, counter);
            }
            if let Some(else_stmts) = else_body {
                collect_inline_arrays_from_stmts(else_stmts, func_name, arrays, counter);
            }
        }
        Stmt::While { cond, body, .. } => {
            collect_inline_arrays_from_expr(cond, func_name, arrays, counter);
            collect_inline_arrays_from_stmts(body, func_name, arrays, counter);
        }
        Stmt::For { body, .. } | Stmt::ForIn { body, .. } => {
            collect_inline_arrays_from_stmts(body, func_name, arrays, counter);
        }
        Stmt::Switch { expr, cases, default, .. } => {
            collect_inline_arrays_from_expr(expr, func_name, arrays, counter);
            for (_, case_body) in cases {
                collect_inline_arrays_from_stmts(case_body, func_name, arrays, counter);
            }
            if let Some(default_body) = default {
                collect_inline_arrays_from_stmts(default_body, func_name, arrays, counter);
            }
        }
        _ => {}
    }
}

fn collect_inline_arrays_from_expr(
    expr: &Expr,
    func_name: &str,
    arrays: &mut Vec<(String, Vec<Expr>)>,
    counter: &mut usize
) {
    match expr {
        Expr::List(elements) => {
            // Found an inline array literal - register it
            let label = format!("ARRAY_{}_{}", func_name.to_uppercase(), counter);
            *counter += 1;
            arrays.push((label, elements.clone()));
        }
        Expr::Binary { left, right, .. } => {
            collect_inline_arrays_from_expr(left, func_name, arrays, counter);
            collect_inline_arrays_from_expr(right, func_name, arrays, counter);
        }
        Expr::Call(call_info) => {
            for arg in &call_info.args {
                collect_inline_arrays_from_expr(arg, func_name, arrays, counter);
            }
        }
        Expr::Index { target, index } => {
            collect_inline_arrays_from_expr(target, func_name, arrays, counter);
            collect_inline_arrays_from_expr(index, func_name, arrays, counter);
        }
        _ => {}
    }
}
