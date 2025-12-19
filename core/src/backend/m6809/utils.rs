// Utils - Helper functions for M6809 backend
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::BTreeSet;
use crate::ast::{Expr, Stmt};

/// Generate a unique label with the given prefix
pub fn fresh_label(prefix: &str) -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, id)
}

/// Check if an expression is a power of two constant
pub fn power_of_two_const(expr: &Expr) -> Option<u32> {
    if let Expr::Number(n) = expr {
        let val = *n as u32 & 0xFFFF;
        if val >= 2 && (val & (val - 1)) == 0 {
            return (0..16).find(|s| (1u32 << s) == val);
        }
    }
    None
}

/// Format an expression reference for debugging
pub fn format_expr_ref(e: &Expr) -> String {
    match e {
        Expr::Ident(n) => format!("I:{}", n.name),
        Expr::Number(v) => format!("N:{}", v),
        Expr::StringLit(s) => format!("S:{}", s),
        Expr::Call(ci) => format!("C:{}", ci.name),
        _ => "?".to_string(),
    }
}

/// Collect all expression identifiers
pub fn collect_expr_syms(expr: &Expr, set: &mut BTreeSet<String>) {
    match expr {
        Expr::Ident(n) => { set.insert(n.name.clone()); }
        Expr::Call(ci) => { for a in &ci.args { collect_expr_syms(a, set); } }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => {
            collect_expr_syms(left, set);
            collect_expr_syms(right, set);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => collect_expr_syms(inner, set),
        Expr::Number(_) | Expr::StringLit(_) => {}
        Expr::List(elements) => {
            for elem in elements {
                collect_expr_syms(elem, set);
            }
        }
        Expr::Index { target, index } => {
            collect_expr_syms(target, set);
            collect_expr_syms(index, set);
        }
    }
}

/// Collect all statement symbols
pub fn collect_stmt_syms(stmt: &Stmt, set: &mut BTreeSet<String>) {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            match target {
                crate::ast::AssignTarget::Ident { name, .. } => {
                    set.insert(name.clone());
                }
                crate::ast::AssignTarget::Index { target: array_expr, index, .. } => {
                    if let Expr::Ident(id) = &**array_expr {
                        set.insert(id.name.clone());
                    }
                    collect_expr_syms(array_expr, set);
                    collect_expr_syms(index, set);
                }
            }
            collect_expr_syms(value, set);
        }
        Stmt::Let { name, value, .. } => {
            set.insert(name.clone());
            collect_expr_syms(value, set);
        }
        Stmt::Expr(e, ..) | Stmt::Return(Some(e), _) => {
            collect_expr_syms(e, set);
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            collect_expr_syms(cond, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
            for (elif_cond, elif_body) in elifs {
                collect_expr_syms(elif_cond, set);
                for s in elif_body {
                    collect_stmt_syms(s, set);
                }
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    collect_stmt_syms(s, set);
                }
            }
        }
        Stmt::While { cond, body, .. } => {
            collect_expr_syms(cond, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::For { body, .. } => {
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::ForIn { iterable, body, .. } => {
            collect_expr_syms(iterable, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            collect_expr_syms(expr, set);
            for (val, case_body) in cases {
                collect_expr_syms(val, set);
                for s in case_body {
                    collect_stmt_syms(s, set);
                }
            }
            if let Some(default_body) = default {
                for s in default_body {
                    collect_stmt_syms(s, set);
                }
            }
        }
        _ => {}
    }
}

/// Collect local variables from statements
pub fn collect_locals(stmts: &[Stmt]) -> Vec<String> {
    fn walk(s: &Stmt, set: &mut BTreeSet<String>) {
        if let Stmt::Let { name, .. } = s {
            set.insert(name.clone());
        }
        match s {
            Stmt::If { body, elifs, else_body, .. } => {
                for b in body { walk(b, set); }
                for (_, elif_body) in elifs {
                    for b in elif_body { walk(b, set); }
                }
                if let Some(else_stmts) = else_body {
                    for b in else_stmts { walk(b, set); }
                }
            }
            Stmt::While { body, .. } | Stmt::For { body, .. } | Stmt::ForIn { body, .. } => {
                for b in body { walk(b, set); }
            }
            Stmt::Switch { cases, default, .. } => {
                for (_, case_body) in cases {
                    for b in case_body { walk(b, set); }
                }
                if let Some(default_body) = default {
                    for b in default_body { walk(b, set); }
                }
            }
            _ => {}
        }
    }
    let mut set = BTreeSet::new();
    for s in stmts { walk(s, &mut set); }
    set.into_iter().collect()
}

/// Loop context for break/continue handling
#[derive(Default, Clone)]
pub struct LoopCtx { 
    pub start: Option<String>, 
    pub end: Option<String> 
}

/// Function context with local variables
#[derive(Clone)]
pub struct FuncCtx { 
    pub locals: Vec<String>, 
    pub frame_size: i32 
}

impl FuncCtx {
    pub fn offset_of(&self, name: &str) -> Option<i32> {
        self.locals.iter().position(|n| n == name).map(|i| (i as i32)*2)
    }
}
