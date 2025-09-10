use crate::ast::{BinOp, CmpOp, Expr, Function, Item, Module, Stmt};
use crate::codegen::CodegenOptions;
use crate::target::{Target, TargetInfo};

// emit: entry point for ARM backend assembly generation.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    out.push_str(&format!(
        "; --- ARM backend (PiTrex) --- title='{}' origin={} ---\n",
        opts.title, ti.origin
    ));
    out.push_str(
        "; Entry point\n.global _start\n_start:\n    BL pitrex_init ; engine init placeholder\n    BL main\n1:  B 1b ; loop\n\n",
    );
    for Item::Function(f) in &module.items { emit_function(f, &mut out); }
    out.push_str("; Runtime helpers\n");
    out.push_str(
        "__mul32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __mul32_done\n__mul32_loop:\n    AND r3,r1,#1\n    CMP r3,#0\n    BEQ __mul32_skip\n    ADD r2,r2,r0\n__mul32_skip:\n    LSR r1,r1,#1\n    LSL r0,r0,#1\n    CMP r1,#0\n    BNE __mul32_loop\n__mul32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str(
        "__div32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __div32_done\n    MOV r3,r0\n__div32_loop:\n    CMP r3,r1\n    BLT __div32_done\n    SUB r3,r3,r1\n    ADD r2,r2,#1\n    B __div32_loop\n__div32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str("; Data segment (prototype)\n.data\n");
    for v in syms { out.push_str(&format!("VAR_{}: .word 0\n", v.to_uppercase())); }
    out.push_str("; Call arg scratch (if needed by future ABI changes)\nVAR_ARG0: .word 0\nVAR_ARG1: .word 0\nVAR_ARG2: .word 0\nVAR_ARG3: .word 0\n");
    out
}

// emit_function: outputs assembly for a single function including label and tail return.
fn emit_function(f: &Function, out: &mut String) {
    out.push_str(&format!(".global {0}\n{0}:\n", f.name));
    // Parameter prologue: copy up to 4 incoming registers r0-r3 into globals.
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDR r4, =VAR_{}\n    STR r{} , [r4]\n", p.to_uppercase(), i));
    }
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default()); }
    // Suppress epilogue if last stmt is an explicit return.
    if !matches!(f.body.last(), Some(Stmt::Return(_))) {
        out.push_str("    BX LR\n");
    }
    out.push_str("\n");
}

#[derive(Default, Clone)]
struct LoopCtx {
    start: Option<String>,
    end: Option<String>,
}

// emit_stmt: lowers high-level statements into ARM instructions with structured labels.
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx) {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_expr(value, out);
            out.push_str(&format!("    LDR r1, =VAR_{0}\n    STR r0, [r1]\n", target.to_uppercase()));
        }
        Stmt::Expr(e) => emit_expr(e, out),
        Stmt::Return(expr_opt) => {
            if let Some(e) = expr_opt {
                emit_expr(e, out);
            }
            out.push_str("    BX LR\n");
        }
        Stmt::Break => {
            if let Some(end) = &loop_ctx.end {
                out.push_str(&format!("    B {}\n", end));
            } else {
                out.push_str("    ; break outside loop\n");
            }
        }
        Stmt::Continue => {
            if let Some(start) = &loop_ctx.start {
                out.push_str(&format!("    B {}\n", start));
            } else {
                out.push_str("    ; continue outside loop\n");
            }
        }
        Stmt::While { cond, body } => {
            let lbl_start = fresh_label("WH");
            let lbl_end = fresh_label("WH_END");
            out.push_str(&format!("{}:\n", lbl_start));
            emit_expr(cond, out); // r0 = cond
            out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", lbl_end));
            let inner = LoopCtx {
                start: Some(lbl_start.clone()),
                end: Some(lbl_end.clone()),
            };
            for s in body {
                emit_stmt(s, out, &inner);
            }
            out.push_str(&format!("    B {}\n{}:\n", lbl_start, lbl_end));
        }
        Stmt::For { var, start, end, step, body } => {
            let loop_label = fresh_label("FOR");
            let end_label = fresh_label("FOR_END");
            emit_expr(start, out);
            out.push_str(&format!("    LDR r1, =VAR_{0}\n    STR r0, [r1]\n", var.to_uppercase()));
            out.push_str(&format!("{}:\n", loop_label));
            out.push_str(&format!("    LDR r1, =VAR_{}\n    LDR r1, [r1]\n", var.to_uppercase()));
            emit_expr(end, out);
            out.push_str(&format!("    CMP r1, r0\n    BGE {}\n", end_label));
            let inner = LoopCtx {
                start: Some(loop_label.clone()),
                end: Some(end_label.clone()),
            };
            for s in body {
                emit_stmt(s, out, &inner);
            }
            if let Some(se) = step {
                emit_expr(se, out);
            } else {
                out.push_str("    MOV r0, #1\n");
            }
            out.push_str(&format!(
                "    LDR r2, =VAR_{}\n    LDR r3, [r2]\n    ADD r3, r3, r0\n    STR r3, [r2]\n",
                var.to_uppercase()
            ));
            out.push_str(&format!("    B {}\n{}:\n", loop_label, end_label));
        }
        Stmt::If { cond, body, elifs, else_body } => {
            let end_label = fresh_label("IF_END");
            let mut next_label = fresh_label("IF_NEXT");
            emit_expr(cond, out);
            out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", next_label));
            for s in body {
                emit_stmt(s, out, loop_ctx);
            }
            out.push_str(&format!("    B {}\n", end_label));
            for (i, (econd, ebody)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next_label));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() {
                    end_label.clone()
                } else {
                    fresh_label("IF_NEXT")
                };
                emit_expr(econd, out);
                out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", new_next));
                for s in ebody {
                    emit_stmt(s, out, loop_ctx);
                }
                out.push_str(&format!("    B {}\n", end_label));
                next_label = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next_label));
                for s in eb {
                    emit_stmt(s, out, loop_ctx);
                }
            } else if !elifs.is_empty() {
                out.push_str(&format!("{}:\n", next_label));
            }
            out.push_str(&format!("{}:\n", end_label));
        }
    }
}

// emit_expr: produces expression value in r0 with 16-bit masking semantics.
fn emit_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Number(n) => out.push_str(&format!("    MOV r0, #{}\n", *n)),
        Expr::Ident(name) => out.push_str(&format!("    LDR r0, =VAR_{}\n    LDR r0, [r0]\n", name.to_uppercase())),
        Expr::Call { name, args } => {
            let limit = args.len().min(4);
            for idx in (0..limit).rev() { // evaluate right-to-left
                emit_expr(&args[idx], out);
                if idx != 0 { out.push_str(&format!("    MOV r{} , r0\n", idx)); }
            }
            out.push_str(&format!("    BL {}\n", name));
        },
    Expr::Binary { op, left, right } => {
            if let BinOp::Mul = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out);
                            out.push_str(&format!("    LSL r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
                if let Expr::Number(n) = &**left {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(right, out);
                            out.push_str(&format!("    LSL r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            } else if let BinOp::Div = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out);
                            out.push_str(&format!("    LSR r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            }
            emit_expr(left, out);
            out.push_str("    MOV r4, r0\n");
            emit_expr(right, out);
            out.push_str("    MOV r5, r0\n");
            match op {
                BinOp::Add => out.push_str("    ADD r0, r4, r5\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Sub => out.push_str("    SUB r0, r4, r5\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Mul => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __mul32\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Div => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Mod => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    ; quotient now in r0 -> compute remainder r4 - r0*r5\n    MOV r2,r0\n    MUL r2,r2,r5\n    RSBS r0,r2,r4\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shl => out.push_str("    MOV r0,r4,LSL r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shr => out.push_str("    MOV r0,r4,LSR r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::BitAnd => out.push_str("    AND r0, r4, r5\n"),
                BinOp::BitOr  => out.push_str("    ORR r0, r4, r5\n"),
                BinOp::BitXor => out.push_str("    EOR r0, r4, r5\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr(inner, out);
            out.push_str("    MVN r0,r0\n    AND r0,r0,#0xFFFF\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out);
            out.push_str("    MOV r4, r0\n");
            emit_expr(right, out);
            out.push_str("    MOV r5, r0\n    CMP r4, r5\n    MOV r0, #0\n");
            let lbl_true = fresh_label("CMP_T");
            let lbl_end = fresh_label("CMP_E");
            let branch = match op {
                CmpOp::Eq => "BEQ",
                CmpOp::Ne => "BNE",
                CmpOp::Lt => "BLT",
                CmpOp::Le => "BLE",
                CmpOp::Gt => "BGT",
                CmpOp::Ge => "BGE",
            };
            out.push_str(&format!(
                "    {} {}\n    B {}\n{}:\n    MOV r0, #1\n{}:\n",
                branch, lbl_true, lbl_end, lbl_true, lbl_end
            ));
        }
        Expr::Logic { op, left, right } => match op {
            crate::ast::LogicOp::And => {
                emit_expr(left, out);
                let false_lbl = fresh_label("AND_FALSE");
                let end_lbl = fresh_label("AND_END");
                out.push_str(&format!("    CMP r0,#0\n    BEQ {}\n", false_lbl));
                emit_expr(right, out);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BEQ {}\n    MOV r0,#1\n    B {}\n{}:\n    MOV r0,#0\n{}:\n",
                    false_lbl, end_lbl, false_lbl, end_lbl
                ));
            }
            crate::ast::LogicOp::Or => {
                let true_lbl = fresh_label("OR_TRUE");
                let end_lbl = fresh_label("OR_END");
                emit_expr(left, out);
                out.push_str(&format!("    CMP r0,#0\n    BNE {}\n", true_lbl));
                emit_expr(right, out);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BNE {}\n    MOV r0,#0\n    B {}\n{}:\n    MOV r0,#1\n{}:\n",
                    true_lbl, end_lbl, true_lbl, end_lbl
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out);
            out.push_str("    CMP r0,#0\n    MOVEQ r0,#1\n    MOVNE r0,#0\n");
        }
    }
}

// collect_symbols: gather all variable identifiers across module for data section.
fn collect_symbols(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut set = BTreeSet::new();
    for Item::Function(f) in &module.items {
        for stmt in &f.body {
            collect_stmt_syms(stmt, &mut set);
        }
    }
    set.into_iter().collect()
}

// collect_stmt_syms: scan a statement for variable references.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
        Stmt::Assign { target, value } => {
            set.insert(target.clone());
            collect_expr_syms(value, set);
        }
        Stmt::Expr(e) => collect_expr_syms(e, set),
        Stmt::For { var, start, end, step, body } => {
            set.insert(var.clone());
            collect_expr_syms(start, set);
            collect_expr_syms(end, set);
            if let Some(se) = step {
                collect_expr_syms(se, set);
            }
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::While { cond, body } => {
            collect_expr_syms(cond, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::If { cond, body, elifs, else_body } => {
            collect_expr_syms(cond, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
            for (c, b) in elifs {
                collect_expr_syms(c, set);
                for s in b {
                    collect_stmt_syms(s, set);
                }
            }
            if let Some(eb) = else_body {
                for s in eb {
                    collect_stmt_syms(s, set);
                }
            }
        }
        Stmt::Return(o) => {
            if let Some(e) = o {
                collect_expr_syms(e, set);
            }
        }
        Stmt::Break | Stmt::Continue => {}
    }
}

// collect_expr_syms: scan an expression for identifiers.
fn collect_expr_syms(expr: &Expr, set: &mut std::collections::BTreeSet<String>) {
    match expr {
        Expr::Ident(n) => { set.insert(n.clone()); }
        Expr::Call { args, .. } => { for a in args { collect_expr_syms(a, set); } }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => {
            collect_expr_syms(left, set);
            collect_expr_syms(right, set);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => collect_expr_syms(inner, set),
        Expr::Number(_) => {}
    }
}

// fresh_label: produce unique assembler labels with a given prefix.
fn fresh_label(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static C: AtomicUsize = AtomicUsize::new(0);
    let id = C.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, id)
}
