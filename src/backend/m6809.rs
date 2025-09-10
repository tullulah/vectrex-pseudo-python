// (Removed duplicated legacy block above during refactor)
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use crate::codegen::CodegenOptions;
use crate::target::{Target, TargetInfo};

// emit: entry point for Motorola 6809 backend assembly generation.
// Produces a simple Vectrex-style header, calls platform init + MAIN, then infinite loop.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    out.push_str(&format!(
        "; --- Motorola 6809 backend ({}) title='{}' origin={} ---\n",
        ti.name, opts.title, ti.origin
    ));
    // Origin may already be a properly formatted string (e.g. "$8000").
    out.push_str(&format!("        ORG {}\n", ti.origin));
    out.push_str(
        "; Basic Vectrex header (placeholder)\n    FCB $67,$20,$56,$45,$43,$54,$52,$45,$58,$20,$47,$41,$4D,$45,$20\n    FCB $00,$00,$00,$00\n\n",
    );
    out.push_str(&format!("JSR {}\nJSR MAIN\nEND_LOOP: BRA END_LOOP\n\n", ti.init_label));
    for Item::Function(f) in &module.items {
        emit_function(f, &mut out);
    }
    out.push_str("; Runtime helpers\n");
    emit_mul_helper(&mut out);
    emit_div_helper(&mut out);
    out.push_str("; Variables\n");
    for v in syms { out.push_str(&format!("VAR_{}: FDB 0\n", v.to_uppercase())); }
    out.push_str("; Call argument scratch space\nVAR_ARG0: FDB 0\nVAR_ARG1: FDB 0\nVAR_ARG2: FDB 0\nVAR_ARG3: FDB 0\n");
    out
}

// emit_function: outputs code for a function.
fn emit_function(f: &Function, out: &mut String) {
    out.push_str(&format!("{}: ; function\n", f.name.to_uppercase()));
    out.push_str(&format!("; --- function {} ---\n{}:\n", f.name, f.name));
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local
    if frame_size > 0 { out.push_str(&format!("    LEAS -{} ,S ; allocate locals\n", frame_size)); }
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDD VAR_ARG{}\n    STD VAR_{}\n", i, p.to_uppercase()));
    }
    let fctx = FuncCtx { locals: locals.clone(), frame_size };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx); }
    if !matches!(f.body.last(), Some(Stmt::Return(_))) {
        if frame_size > 0 { out.push_str(&format!("    LEAS {} ,S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push_str("\n");
}

#[derive(Default, Clone)]
struct LoopCtx { start: Option<String>, end: Option<String> }

#[derive(Clone)]
struct FuncCtx { locals: Vec<String>, frame_size: i32 }
impl FuncCtx { fn offset_of(&self, name: &str) -> Option<i32> { self.locals.iter().position(|n| n == name).map(|i| (i as i32)*2) } }

// emit_stmt: lower statements to 6809 instructions.
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx) {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_expr(value, out, fctx);
            if let Some(off) = fctx.offset_of(target) {
                out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off));
            } else {
                out.push_str(&format!("    LDU #VAR_{}\n    STU TMPPTR\n    STX ,U\n", target.to_uppercase()));
            }
        }
        Stmt::Let { name, value } => {
            emit_expr(value, out, fctx);
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off)); }
        }
        Stmt::Expr(e) => emit_expr(e, out, fctx),
        Stmt::Return(o) => {
            if let Some(e) = o { emit_expr(e, out, fctx); }
            if fctx.frame_size > 0 { out.push_str(&format!("    LEAS {} ,S ; free locals\n", fctx.frame_size)); }
            out.push_str("    RTS\n");
        }
        Stmt::Break => {
            if let Some(end) = &loop_ctx.end {
                out.push_str(&format!("    BRA {}\n", end));
            }
        }
        Stmt::Continue => {
            if let Some(st) = &loop_ctx.start {
                out.push_str(&format!("    BRA {}\n", st));
            }
        }
        Stmt::While { cond, body } => {
            let ls = fresh_label("WH");
            let le = fresh_label("WH_END");
            out.push_str(&format!("{}: ; while start\n", ls));
            emit_expr(cond, out, fctx);
            out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx); }
            out.push_str(&format!("    BRA {}\n{}: ; while end\n", ls, le));
        }
    Stmt::For { var, start, end, step, body } => {
            let ls = fresh_label("FOR");
            let le = fresh_label("FOR_END");
            emit_expr(start, out, fctx);
            out.push_str("    LDD RESULT\n");
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    STD {} ,S\n", off)); }
            else { out.push_str(&format!("    STD VAR_{}\n", var.to_uppercase())); }
            out.push_str(&format!("{}: ; for loop\n", ls));
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDD {} ,S\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n", var.to_uppercase())); }
            emit_expr(end, out, fctx);
            out.push_str("    LDX RESULT\n    CPD RESULT\n");
            out.push_str(&format!("    BHS {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx); }
            if let Some(se) = step {
                emit_expr(se, out, fctx);
                out.push_str("    LDX RESULT\n");
            } else {
                out.push_str("    LDX #1\n");
            }
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDD {} ,S\n    ADDD ,X\n    STD {} ,S\n", off, off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n    ADDD ,X\n    STD VAR_{}\n", var.to_uppercase(), var.to_uppercase())); }
            out.push_str(&format!("    BRA {}\n{}: ; for end\n", ls, le));
        }
        Stmt::If { cond, body, elifs, else_body } => {
            let end = fresh_label("IF_END");
            let mut next = fresh_label("IF_NEXT");
            emit_expr(cond, out, fctx);
            out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", next));
            for s in body { emit_stmt(s, out, loop_ctx, fctx); }
            out.push_str(&format!("    BRA {}\n", end));
            for (i, (c, b)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(c, out, fctx);
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", new_next));
                for s in b { emit_stmt(s, out, loop_ctx, fctx); }
                out.push_str(&format!("    BRA {}\n", end));
                next = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx); }
            } else if !elifs.is_empty() {
                out.push_str(&format!("{}:\n", next));
            }
            out.push_str(&format!("{}:\n", end));
        }
    }
}

// emit_expr: lower expressions; result placed in RESULT.
// Nota: En 6809 las operaciones sobre D ya limitan a 16 bits; no hace falta 'mask' explícito.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx) {
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("    LDD #{}\n    STD RESULT\n", *n & 0xFFFF));
        }
        Expr::Ident(name) => {
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDD {} ,S\n    STD RESULT\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n    STD RESULT\n", name.to_uppercase())); }
        }
        Expr::Call { name, args } => {
            for (i, arg) in args.iter().enumerate() {
                if i >= 4 { break; }
                emit_expr(arg, out, fctx);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            out.push_str(&format!("    JSR {}\n", name.to_uppercase()));
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr(left, out, fctx);
                out.push_str("    LDD RESULT\n    ADDD RESULT\n    STD RESULT\n");
                return;
            }
            if matches!(op, BinOp::Sub) && format_expr_ref(left) == format_expr_ref(right) {
                out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
                return;
            }
            // Generalized power-of-two multiply via shifts (any 2^k) using ASLB/ROLA.
            if matches!(op, BinOp::Mul) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr(left, out, fctx);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr(right, out, fctx);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr(left, out, fctx);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr(left, out, fctx);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
            match op {
                BinOp::Add => out.push_str("    LDD TMPLEFT\n    ADDD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Sub => out.push_str("    LDD TMPLEFT\n    SUBD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Mul => out.push_str("    LDD TMPLEFT\n    STD MUL_A\n    LDD TMPRIGHT\n    STD MUL_B\n    JSR MUL16\n"),
                BinOp::Div => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n"),
                BinOp::Mod => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n    ; quotient in RESULT, need remainder: A - Q*B\n    LDD DIV_A\n    STD TMPLEFT\n    LDD RESULT\n    STD MUL_A\n    LDD DIV_B\n    STD MUL_B\n    JSR MUL16\n    ; product in RESULT, subtract from original A (TMPLEFT)\n    LDD TMPLEFT\n    SUBD RESULT\n    STD RESULT\n"),
                BinOp::Shl => out.push_str("    LDD TMPLEFT\nSHL_LOOP: LDA TMPRIGHT+1\n    BEQ SHL_DONE\n    ASLB\n    ROLA\n    DEC TMPRIGHT+1\n    BRA SHL_LOOP\nSHL_DONE: STD RESULT\n"),
                BinOp::Shr => out.push_str("    LDD TMPLEFT\nSHR_LOOP: LDA TMPRIGHT+1\n    BEQ SHR_DONE\n    LSRA\n    RORB\n    DEC TMPRIGHT+1\n    BRA SHR_LOOP\nSHR_DONE: STD RESULT\n"),
                BinOp::BitAnd => out.push_str("    LDD TMPLEFT\n    ANDA TMPRIGHT+1\n    ANDB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitOr  => out.push_str("    LDD TMPLEFT\n    ORA TMPRIGHT+1\n    ORB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitXor => out.push_str("    LDD TMPLEFT\n    EORA TMPRIGHT+1\n    EORB TMPRIGHT\n    STD RESULT\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr(inner, out, fctx);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out, fctx);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n    LDD TMPLEFT\n    SUBD TMPRIGHT\n");
            out.push_str("    LDD #0\n    STD RESULT\n");
            let lt = fresh_label("CT");
            let end = fresh_label("CE");
            let br = match op { CmpOp::Eq => "BEQ", CmpOp::Ne => "BNE", CmpOp::Lt => "BLT", CmpOp::Le => "BLE", CmpOp::Gt => "BGT", CmpOp::Ge => "BGE" };
            out.push_str(&format!(
                "    {} {}\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                br, lt, end, lt, end
            ));
        }
        Expr::Logic { op, left, right } => match op {
            LogicOp::And => {
                emit_expr(left, out, fctx);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr(right, out, fctx);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr(left, out, fctx);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr(right, out, fctx);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out, fctx);
            out.push_str(
                "    LDD RESULT\n    BEQ NOT_TRUE\n    LDD #0\n    STD RESULT\n    BRA NOT_END\nNOT_TRUE:\n    LDD #1\n    STD RESULT\nNOT_END:\n",
            );
        }
    }
}

// power_of_two_const: return shift count if expression is a numeric power-of-two (>1).
fn power_of_two_const(expr: &Expr) -> Option<u32> {
    if let Expr::Number(n) = expr {
        let val = *n as u32 & 0xFFFF;
        if val >= 2 && (val & (val - 1)) == 0 {
            return (0..16).find(|s| (1u32 << s) == val);
        }
    }
    None
}

// format_expr_ref: helper for peephole comparisons.
fn format_expr_ref(e: &Expr) -> String {
    match e {
        Expr::Ident(n) => format!("I:{}", n),
        Expr::Number(n) => format!("N:{}", n),
        _ => "?".into(),
    }
}

// collect_symbols: gather variable identifiers.
fn collect_symbols(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut globals = BTreeSet::new();
    let mut locals = BTreeSet::new();
    for Item::Function(f) in &module.items {
        for stmt in &f.body { collect_stmt_syms(stmt, &mut globals); }
        for l in collect_locals(&f.body) { locals.insert(l); }
    }
    for l in &locals { globals.remove(l); }
    globals.into_iter().collect()
}

// collect_stmt_syms: process statement symbols.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
    Stmt::Assign { target, value } => {
            set.insert(target.clone());
            collect_expr_syms(value, set);
        }
    Stmt::Let { name: _ , value } => { collect_expr_syms(value, set); } // exclude locals
        Stmt::Expr(e) => collect_expr_syms(e, set),
    Stmt::For { var: _, start, end, step, body } => {
            // treat induction var as global only if not a local (decision deferred to emit)
            collect_expr_syms(start, set);
            collect_expr_syms(end, set);
            if let Some(se) = step { collect_expr_syms(se, set); }
            for s in body { collect_stmt_syms(s, set); }
        }
        Stmt::While { cond, body } => {
            collect_expr_syms(cond, set);
            for s in body { collect_stmt_syms(s, set); }
        }
        Stmt::If { cond, body, elifs, else_body } => {
            collect_expr_syms(cond, set);
            for s in body { collect_stmt_syms(s, set); }
            for (c, b) in elifs {
                collect_expr_syms(c, set);
                for s in b { collect_stmt_syms(s, set); }
            }
            if let Some(eb) = else_body {
                for s in eb { collect_stmt_syms(s, set); }
            }
        }
        Stmt::Return(o) => { if let Some(e) = o { collect_expr_syms(e, set); } }
        Stmt::Break | Stmt::Continue => {}
    }
}

// collect_locals: traverse function statements to find let-declared locals
fn collect_locals(stmts: &[Stmt]) -> Vec<String> {
    use std::collections::BTreeSet;
    fn walk(s: &Stmt, set: &mut BTreeSet<String>) {
        match s {
            Stmt::Let { name, .. } => { set.insert(name.clone()); }
            Stmt::If { body, elifs, else_body, .. } => {
                for b in body { walk(b, set); }
                for (_c, eb) in elifs { for b in eb { walk(b, set); } }
                if let Some(eb) = else_body { for b in eb { walk(b, set); } }
            }
            Stmt::While { body, .. } => { for b in body { walk(b, set); } }
            Stmt::For { var, body, .. } => {
                set.insert(var.clone());
                for b in body { walk(b, set); }
            }
            _ => {}
        }
    }
    let mut set = BTreeSet::new();
    for s in stmts { walk(s, &mut set); }
    set.into_iter().collect()
}

// collect_expr_syms: process expression identifiers.
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

// fresh_label: unique label generator.
fn fresh_label(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static C: AtomicUsize = AtomicUsize::new(0);
    let id = C.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, id)
}

// emit_mul_helper: 16-bit multiply routine.
fn emit_mul_helper(out: &mut String) {
    out.push_str(
        "MUL16:\n    LDD MUL_A\n    STD MUL_RES\n    LDD #0\n    STD MUL_TMP\n    LDD MUL_B\n    STD MUL_CNT\nMUL16_LOOP:\n    LDD MUL_CNT\n    BEQ MUL16_DONE\n    LDD MUL_CNT\n    ANDA #1\n    BEQ MUL16_SKIP\n    LDD MUL_RES\n    ADDD MUL_TMP\n    STD MUL_TMP\nMUL16_SKIP:\n    LDD MUL_RES\n    ASLB\n    ROLA\n    STD MUL_RES\n    LDD MUL_CNT\n    LSRA\n    RORB\n    STD MUL_CNT\n    BRA MUL16_LOOP\nMUL16_DONE:\n    LDD MUL_TMP\n    STD RESULT\n    RTS\n\n",
    );
}

// emit_div_helper: 16-bit division routine.
fn emit_div_helper(out: &mut String) {
    out.push_str(
        "DIV16:\n    LDD #0\n    STD DIV_Q\n    LDD DIV_A\n    STD DIV_R\n    LDD DIV_B\n    BEQ DIV16_DONE\nDIV16_LOOP:\n    LDD DIV_R\n    SUBD DIV_B\n    BLO DIV16_DONE\n    STD DIV_R\n    LDD DIV_Q\n    ADDD #1\n    STD DIV_Q\n    BRA DIV16_LOOP\nDIV16_DONE:\n    LDD DIV_Q\n    STD RESULT\n    RTS\n\n",
    );
}
