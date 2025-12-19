use crate::ast::{Expr, Item, Module, Stmt};

// Shared utilities for collecting and escaping string literals across backends.

pub fn collect_string_literals(module: &Module) -> std::collections::BTreeMap<String,String> {
    use std::collections::{BTreeSet, BTreeMap};
    let mut set = BTreeSet::new();
    for item in &module.items {
        if let Item::Function(f) = item { 
            for s in &f.body { gather_stmt_strings(s, &mut set); } 
        } else if let Item::Const { value, .. } = item { 
            gather_expr_strings(value, &mut set); 
        } else if let Item::ExprStatement(expr) = item {
            gather_expr_strings(expr, &mut set);
        } else if let Item::GlobalLet { value, .. } = item {
            gather_expr_strings(value, &mut set);
        }
    }
    let mut map = BTreeMap::new();
    for (i, lit) in set.iter().enumerate() { map.insert(lit.clone(), format!("STR_{}", i)); }
    map
}

fn gather_stmt_strings(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
        Stmt::Assign { value, .. } | Stmt::Let { value, .. } => gather_expr_strings(value, set),
        Stmt::Expr(value, _) => gather_expr_strings(value, set),
        Stmt::While { cond, body, .. } => { gather_expr_strings(cond,set); for s in body { gather_stmt_strings(s,set); } }
        Stmt::For { start, end, step, body, .. } => { gather_expr_strings(start,set); gather_expr_strings(end,set); if let Some(se)=step { gather_expr_strings(se,set); } for s in body { gather_stmt_strings(s,set); } }
        Stmt::ForIn { iterable, body, .. } => { gather_expr_strings(iterable,set); for s in body { gather_stmt_strings(s,set); } }
        Stmt::If { cond, body, elifs, else_body, .. } => { gather_expr_strings(cond,set); for s in body { gather_stmt_strings(s,set); } for (c,b) in elifs { gather_expr_strings(c,set); for s in b { gather_stmt_strings(s,set); } } if let Some(eb)=else_body { for s in eb { gather_stmt_strings(s,set); } } }
        Stmt::Return(o, _) => { if let Some(e)=o { gather_expr_strings(e,set); } }
        Stmt::Switch { expr, cases, default, .. } => { gather_expr_strings(expr,set); for (ce,cb) in cases { gather_expr_strings(ce,set); for s in cb { gather_stmt_strings(s,set); } } if let Some(db)=default { for s in db { gather_stmt_strings(s,set); } } }
        Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => {},
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before gather_stmt_strings"),
    }
}

fn gather_expr_strings(expr: &Expr, set: &mut std::collections::BTreeSet<String>) {
    match expr {
        Expr::StringLit(s) => { set.insert(s.clone()); }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => { gather_expr_strings(left,set); gather_expr_strings(right,set); }
    Expr::Call(ci) => { for a in &ci.args { gather_expr_strings(a,set); } }
        Expr::Not(inner) | Expr::BitNot(inner) => gather_expr_strings(inner,set),
        Expr::List(elements) => {
            for elem in elements {
                gather_expr_strings(elem, set);
            }
        }
        Expr::Index { target, index } => {
            gather_expr_strings(target, set);
            gather_expr_strings(index, set);
        }
        Expr::Ident(_) | Expr::Number(_) => {}
        Expr::StructInit { .. } => {} // Phase 3 - no string literals
        Expr::FieldAccess { target, .. } => gather_expr_strings(target, set),
    }
}

pub fn escape_ascii(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() { match b { b'"' => out.push_str("\\\""), b'\\' => out.push_str("\\\\"), 0x20..=0x7E => out.push(b as char), _ => out.push_str(&format!("\\x{:02X}", b)), } }
    out
}
