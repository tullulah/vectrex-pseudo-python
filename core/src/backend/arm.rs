use crate::ast::{BinOp, CmpOp, Expr, Function, Item, Module, Stmt};
use super::string_literals::{collect_string_literals, escape_ascii};
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};

// emit: entry point for ARM backend assembly generation.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    let string_map = collect_string_literals(module); // literal text -> label
    out.push_str(&format!("; --- ARM backend (PiTrex) --- title='{}' origin={} ---\n", opts.title, ti.origin));
    out.push_str(";***************************************************************************\n; DEFINE SECTION (ARM)\n;***************************************************************************\n");
    out.push_str("; (No BIOS header required for PiTrex)\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    out.push_str("; Entry point\n.global _start\n_start:\n    BL pitrex_init ; engine init placeholder\n    BL main\n1:  B 1b ; loop\n\n");
    for item in &module.items {
        match item {
            Item::Function(f) => emit_function(f, &mut out, &string_map),
            Item::Const { name, value } => if let Expr::Number(n) = value { out.push_str(&format!(".equ {} , {}\n", name, n & 0xFFFF)); },
            Item::GlobalLet { name, value } => if let Expr::Number(n) = value { out.push_str(&format!("GVAR_{}: .word {}\n", name.to_uppercase(), n & 0xFFFF)); } else { out.push_str(&format!("GVAR_{}: .word 0\n", name.to_uppercase())); },
            Item::VectorList { name, .. } => { out.push_str(&format!("; vectorlist {} ignored on ARM backend (NYI)\n", name)); },
            Item::ExprStatement(_) => { out.push_str("; ExprStatement ignored on ARM backend (TODO: implement top-level execution)\n"); },
            Item::Export(_) => { /* Export declarations are metadata, no code needed */ },
        }
    }
    out.push_str(";***************************************************************************\n; RUNTIME SECTION\n;***************************************************************************\n; Runtime helpers\n");
    out.push_str(
    "__mul32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __mul32_done\n__mul32_loop:\n    AND r3,r1,#1\n    CMP r3,#0\n    BEQ __mul32_skip\n    ADD r2,r2,r0\n__mul32_skip:\n    LSR r1,r1,#1\n    LSL r0,r0,#1\n    CMP r1,#0\n    BNE __mul32_loop\n__mul32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str(
        "__div32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __div32_done\n    MOV r3,r0\n__div32_loop:\n    CMP r3,r1\n    BLT __div32_done\n    SUB r3,r3,r1\n    ADD r2,r2,#1\n    B __div32_loop\n__div32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str(";***************************************************************************\n; DATA SECTION\n;***************************************************************************\n; Data segment (prototype)\n.data\n");
    // Shared trig tables
    out.push_str("; Trig tables (shared)\n");
    emit_trig_tables(&mut out, ".hword");
    for v in syms { out.push_str(&format!("VAR_{}: .word 0\n", v.to_uppercase())); }
    if !string_map.is_empty() { out.push_str("; String literals (null-terminated)\n"); }
    for (lit, label) in &string_map {
        out.push_str(&format!("{}: .ascii \"{}\"\n    .byte 0\n", label, escape_ascii(lit)));
    }
    out.push_str("; Call arg scratch (if needed by future ABI changes)\nVAR_ARG0: .word 0\nVAR_ARG1: .word 0\nVAR_ARG2: .word 0\nVAR_ARG3: .word 0\n");
    out
}

// emit_function: outputs assembly for a single function including label and tail return.
fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>) {
    out.push_str(&format!(".global {0}\n{0}:\n", f.name));
    // Collect local variables declared via 'let'
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local
    if frame_size > 0 { out.push_str(&format!("    SUB sp, sp, #{}\n", frame_size)); }
    // Parameter prologue (still stored in globals for now)
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDR r4, =VAR_{}\n    STR r{} , [r4]\n", p.to_uppercase(), i));
    }
    let fctx = FuncCtx { locals: locals.clone(), frame_size };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map); }
    if !matches!(f.body.last(), Some(Stmt::Return(_, _))) {
        if frame_size > 0 { out.push_str(&format!("    ADD sp, sp, #{}\n", frame_size)); }
        out.push_str("    BX LR\n");
    }
    out.push('\n');
}

#[derive(Default, Clone)]
struct LoopCtx { start: Option<String>, end: Option<String>, }

struct FuncCtx { locals: Vec<String>, frame_size: i32 }
impl FuncCtx {
    fn offset_of(&self, name: &str) -> Option<i32> { self.locals.iter().position(|n| n == name).map(|i| (i as i32)*2) }
}

// emit_stmt: lowers high-level statements into ARM instructions with structured labels.
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            match target {
                crate::ast::AssignTarget::Ident { name, .. } => {
                    emit_expr(value, out, fctx, string_map);
                    if let Some(off) = fctx.offset_of(name) {
                        out.push_str(&format!("    STRH r0, [sp, #{}]\n", off));
                    } else {
                        out.push_str(&format!("    LDR r1, =VAR_{0}\n    STR r0, [r1]\n", name.to_uppercase()));
                    }
                }
                crate::ast::AssignTarget::Index { .. } => panic!("Array assignment not implemented for ARM backend"),
            }
        },
        Stmt::Let { name, value, .. } => {
            emit_expr(value, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    STRH r0, [sp, #{}]\n", off)); }
        },
        Stmt::Expr(e, _) => emit_expr(e, out, fctx, string_map),
        Stmt::Return(expr_opt, _) => {
            if let Some(e) = expr_opt { emit_expr(e, out, fctx, string_map); }
            if fctx.frame_size > 0 { out.push_str(&format!("    ADD sp, sp, #{}\n", fctx.frame_size)); }
            out.push_str("    BX LR\n");
        },
        Stmt::Break { .. } => {
            if let Some(end) = &loop_ctx.end { out.push_str(&format!("    B {}\n", end)); }
            else { out.push_str("    ; break outside loop\n"); }
        },
        Stmt::Continue { .. } => {
            if let Some(start) = &loop_ctx.start { out.push_str(&format!("    B {}\n", start)); }
            else { out.push_str("    ; continue outside loop\n"); }
        },
        Stmt::While { cond, body, .. } => {
            let lbl_start = fresh_label("WH");
            let lbl_end = fresh_label("WH_END");
            out.push_str(&format!("{}:\n", lbl_start));
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", lbl_end));
            let inner = LoopCtx { start: Some(lbl_start.clone()), end: Some(lbl_end.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            out.push_str(&format!("    B {}\n{}:\n", lbl_start, lbl_end));
        },
        Stmt::For { var, start, end, step, body, .. } => {
            let loop_label = fresh_label("FOR");
            let end_label = fresh_label("FOR_END");
            emit_expr(start, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    STRH r0, [sp, #{}]\n", off)); }
            else { out.push_str(&format!("    LDR r1, =VAR_{0}\n    STR r0, [r1]\n", var.to_uppercase())); }
            out.push_str(&format!("{}:\n", loop_label));
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDRH r1, [sp, #{}]\n", off)); }
            else { out.push_str(&format!("    LDR r1, =VAR_{}\n    LDR r1, [r1]\n", var.to_uppercase())); }
            emit_expr(end, out, fctx, string_map);
            out.push_str(&format!("    CMP r1, r0\n    BGE {}\n", end_label));
            let inner = LoopCtx { start: Some(loop_label.clone()), end: Some(end_label.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            if let Some(se) = step { emit_expr(se, out, fctx, string_map); } else { out.push_str("    MOV r0, #1\n"); }
            if let Some(off) = fctx.offset_of(var) {
                out.push_str(&format!("    LDRH r3, [sp, #{}]\n    ADD r3, r3, r0\n    AND r3, r3, #0xFFFF\n    STRH r3, [sp, #{}]\n", off, off));
            } else {
                out.push_str(&format!("    LDR r2, =VAR_{}\n    LDR r3, [r2]\n    ADD r3, r3, r0\n    STR r3, [r2]\n", var.to_uppercase()));
            }
            out.push_str(&format!("    B {}\n{}:\n", loop_label, end_label));
        },
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let end_label = fresh_label("IF_END");
            let mut next_label = fresh_label("IF_NEXT");
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", next_label));
            for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            out.push_str(&format!("    B {}\n", end_label));
            for (i, (econd, ebody)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next_label));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end_label.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(econd, out, fctx, string_map);
                out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", new_next));
                for s in ebody { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    B {}\n", end_label));
                next_label = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next_label));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            } else if !elifs.is_empty() {
                out.push_str(&format!("{}:\n", next_label));
            }
            out.push_str(&format!("{}:\n", end_label));
        },
        Stmt::Switch { expr, cases, default, .. } => {
            emit_expr(expr, out, fctx, string_map);
            out.push_str("    MOV r4, r0\n");
            let end_label = fresh_label("SW_END");
            let default_label = if default.is_some() { Some(fresh_label("SW_DEF")) } else { None };
            let mut numeric_cases: Vec<(i32,&Vec<Stmt>)> = Vec::new();
            let mut all_numeric = true;
            for (ce, body) in cases { if let Expr::Number(n) = ce { numeric_cases.push((*n, body)); } else { all_numeric = false; break; } }
            if all_numeric && numeric_cases.len() >= 3 {
                numeric_cases.sort_by_key(|(v,_)| *v);
                let min = numeric_cases.first().unwrap().0 & 0xFFFF;
                let max = numeric_cases.last().unwrap().0 & 0xFFFF;
                let span = (max - min) as usize + 1;
                if span <= numeric_cases.len()*2 { // density heuristic
                    let table_label = fresh_label("SW_JT");
                    out.push_str(&format!("    MOV r5,#{}\n    CMP r4,r5\n    BLT {}\n", min, default_label.as_ref().unwrap_or(&end_label)));
                    out.push_str(&format!("    MOV r5,#{}\n    CMP r4,r5\n    BGT {}\n", max, default_label.as_ref().unwrap_or(&end_label)));
                    if min != 0 { out.push_str(&format!("    SUB r4,r4,#{}\n", min)); }
                    out.push_str(&format!("    LDR r5, ={}\n    LSL r4,r4,#2\n    ADD r5,r5,r4\n    LDR r5, [r5]\n    MOV r15,r5 ; jump indirect\n", table_label));
                    let mut label_map: std::collections::BTreeMap<i32,String> = std::collections::BTreeMap::new();
                    for (val, _b) in &numeric_cases { label_map.insert(*val, fresh_label("SW_CASE")); }
                    for (val, body) in &numeric_cases {
                        let lbl = label_map.get(val).unwrap();
                        out.push_str(&format!("{}:\n", lbl));
                        for s in *body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                        out.push_str(&format!("    B {}\n", end_label));
                    }
                    if let Some(dl) = &default_label {
                        out.push_str(&format!("{}:\n", dl));
                        for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                    }
                    out.push_str(&format!("{}:\n", end_label));
                    out.push_str(&format!("{}:\n", table_label));
                    for offset in 0..span as i32 {
                        let actual = (min + offset) & 0xFFFF;
                        if let Some(lbl) = label_map.get(&actual) { out.push_str(&format!("    .word {}\n", lbl)); }
                        else if let Some(dl) = &default_label { out.push_str(&format!("    .word {}\n", dl)); }
                        else { out.push_str(&format!("    .word {}\n", end_label)); }
                    }
                    return;
                }
            }
            let mut case_labels = Vec::new();
            for _ in cases { case_labels.push(fresh_label("SW_CASE")); }
            for ((case_expr, _), lbl) in cases.iter().zip(case_labels.iter()) {
                emit_expr(case_expr, out, fctx, string_map);
                out.push_str("    MOV r5, r0\n    CMP r4, r5\n");
                out.push_str(&format!("    BEQ {}\n", lbl));
            }
            if let Some(dl) = &default_label { out.push_str(&format!("    B {}\n", dl)); } else { out.push_str(&format!("    B {}\n", end_label)); }
            for ((_, body), lbl) in cases.iter().zip(case_labels.iter()) {
                out.push_str(&format!("{}:\n", lbl));
                for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    B {}\n", end_label));
            }
            if let Some(dl) = default_label {
                out.push_str(&format!("{}:\n", dl));
                for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            }
            out.push_str(&format!("{}:\n", end_label));
        },
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before emit_stmt"),
        _ => panic!("Unsupported statement type in ARM backend"),
    }
}

// emit_expr: produces expression value in r0 with 16-bit masking semantics.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match expr {
        Expr::Number(n) => out.push_str(&format!("    MOV r0, #{}\n", *n)),
        Expr::StringLit(s) => {
            if let Some(label) = string_map.get(s) {
                out.push_str(&format!("    LDR r0, ={}\n", label));
            } else {
                out.push_str("    MOV r0,#0 ; missing string label (should not happen)\n");
            }
        }
        Expr::Ident(name) => {
            if let Some(off) = fctx.offset_of(&name.name) {
                out.push_str(&format!("    LDRH r0, [sp, #{}]\n", off));
            } else {
                out.push_str(&format!("    LDR r0, =VAR_{}\n    LDR r0, [r0]\n", name.name.to_uppercase()));
            }
        }
        Expr::List(_) => panic!("Array literals not implemented for ARM backend"),
        Expr::Index { .. } => panic!("Array indexing not implemented for ARM backend"),
        Expr::Call(ci) => {
            if emit_builtin_call_arm(&ci.name, &ci.args, out, fctx, string_map) { return; }
            let limit = ci.args.len().min(4);
            for idx in (0..limit).rev() { emit_expr(&ci.args[idx], out, fctx, string_map); if idx!=0 { out.push_str(&format!("    MOV r{} , r0\n", idx)); } }
            out.push_str(&format!("    BL {}\n", ci.name));
        },
        Expr::Binary { op, left, right } => {
            if let BinOp::Mul = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out, fctx, string_map);
                            out.push_str(&format!("    LSL r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
                if let Expr::Number(n) = &**left {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(right, out, fctx, string_map);
                            out.push_str(&format!("    LSL r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            } else if let BinOp::Div = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out, fctx, string_map);
                            out.push_str(&format!("    LSR r0, r0, #{}\n    AND r0, r0, #0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            }
            emit_expr(left, out, fctx, string_map);
            out.push_str("    MOV r4, r0\n");
            emit_expr(right, out, fctx, string_map);
            out.push_str("    MOV r5, r0\n");
            match op {
                BinOp::Add => out.push_str("    ADD r0, r4, r5\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Sub => out.push_str("    SUB r0, r4, r5\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Mul => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __mul32\n    AND r0, r0, #0xFFFF\n"),
                BinOp::Div => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    AND r0, r0, #0xFFFF\n"),
                BinOp::FloorDiv => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    AND r0, r0, #0xFFFF\n"), // División entera igual que Div
                BinOp::Mod => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    ; quotient now in r0 -> compute remainder r4 - r0*r5\n    MOV r2,r0\n    MUL r2,r2,r5\n    RSBS r0,r2,r4\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shl => out.push_str("    MOV r0,r4,LSL r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shr => out.push_str("    MOV r0,r4,LSR r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::BitAnd => out.push_str("    AND r0, r4, r5\n"),
                BinOp::BitOr  => out.push_str("    ORR r0, r4, r5\n"),
                BinOp::BitXor => out.push_str("    EOR r0, r4, r5\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr(inner, out, fctx, string_map);
            out.push_str("    MVN r0,r0\n    AND r0,r0,#0xFFFF\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out, fctx, string_map);
            out.push_str("    MOV r4, r0\n");
            emit_expr(right, out, fctx, string_map);
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
                emit_expr(left, out, fctx, string_map);
                let false_lbl = fresh_label("AND_FALSE");
                let end_lbl = fresh_label("AND_END");
                out.push_str(&format!("    CMP r0,#0\n    BEQ {}\n", false_lbl));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BEQ {}\n    MOV r0,#1\n    B {}\n{}:\n    MOV r0,#0\n{}:\n",
                    false_lbl, end_lbl, false_lbl, end_lbl
                ));
            }
            crate::ast::LogicOp::Or => {
                let true_lbl = fresh_label("OR_TRUE");
                let end_lbl = fresh_label("OR_END");
                emit_expr(left, out, fctx, string_map);
                out.push_str(&format!("    CMP r0,#0\n    BNE {}\n", true_lbl));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BNE {}\n    MOV r0,#0\n    B {}\n{}:\n    MOV r0,#1\n{}:\n",
                    true_lbl, end_lbl, true_lbl, end_lbl
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out, fctx, string_map);
            out.push_str("    CMP r0,#0\n    MOVEQ r0,#1\n    MOVNE r0,#0\n");
        }
    }
}

// emit_builtin_call_arm: inline lowering for math intrinsics on ARM
fn emit_builtin_call_arm(name: &str, args: &[Expr], out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) -> bool {
    let up = name.to_ascii_uppercase();
    let is = matches!(up.as_str(),
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|
        "ABS"|"MATH_ABS"|"MIN"|"MATH_MIN"|"MAX"|"MATH_MAX"|"CLAMP"|"MATH_CLAMP"|
        "VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"
    );
    if !is {
        // Backward or namespace translation
        let translated = match up.as_str() {
            "PRINT_TEXT" => Some("VECTREX_PRINT_TEXT"),
            "MOVE" => Some("VECTREX_MOVE_TO"),        // Unificado: MOVE -> VECTREX_MOVE_TO
            "MOVE_TO" => Some("VECTREX_MOVE_TO"),     // Compatibilidad hacia atrás
            "DRAW_TO" => Some("VECTREX_DRAW_TO"),
            "DRAW_LINE" => Some("VECTREX_DRAW_LINE"),
            "SET_ORIGIN" => Some("VECTREX_SET_ORIGIN"),
            "SET_INTENSITY" => Some("VECTREX_SET_INTENSITY"),
            _ => None
        };
        if let Some(new_up) = translated { return emit_builtin_call_arm(new_up, args, out, fctx, string_map); }
        return false;
    }
    // For non-6809 targets treat Vectrex-specific functions as inert no-ops (return 0) for portability
    if matches!(up.as_str(),"VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY") {
        // Evaluate arguments for side-effects
        for a in args { emit_expr(a, out, fctx, string_map); }
        out.push_str("    MOV r0,#0\n");
        return true;
    }
    // ABS
    if matches!(up.as_str(), "ABS"|"MATH_ABS") {
        if let Some(arg)=args.first() { emit_expr(arg,out,fctx,string_map); } else { out.push_str("    MOV r0,#0\n"); return true; }
        let done = fresh_label("ABS_DONE");
        out.push_str(&format!("    CMP r0,#0\n    BGE {}\n    RSBS r0,r0,#0\n{}:\n", done, done));
        return true;
    }
    // MIN(a,b)
    if matches!(up.as_str(), "MIN"|"MATH_MIN") {
        if args.len()<2 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0], out, fctx, string_map); out.push_str("    MOV r4,r0\n");
        emit_expr(&args[1], out, fctx, string_map); out.push_str("    MOV r5,r0\n");
        out.push_str("    CMP r4,r5\n    BLE 1f\n    MOV r0,r5\n    B 2f\n1:  MOV r0,r4\n2:\n");
        return true;
    }
    // MAX(a,b)
    if matches!(up.as_str(), "MAX"|"MATH_MAX") {
        if args.len()<2 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0], out, fctx, string_map); out.push_str("    MOV r4,r0\n");
        emit_expr(&args[1], out, fctx, string_map); out.push_str("    MOV r5,r0\n");
        out.push_str("    CMP r4,r5\n    BGE 1f\n    MOV r0,r5\n    B 2f\n1:  MOV r0,r4\n2:\n");
        return true;
    }
    // CLAMP(v,lo,hi)
    if matches!(up.as_str(), "CLAMP"|"MATH_CLAMP") {
        if args.len()<3 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0], out, fctx, string_map); out.push_str("    MOV r4,r0\n"); // v
        emit_expr(&args[1], out, fctx, string_map); out.push_str("    MOV r5,r0\n"); // lo
        emit_expr(&args[2], out, fctx, string_map); out.push_str("    MOV r6,r0\n"); // hi
        out.push_str("    CMP r4,r5\n    BLT 1f\n    CMP r4,r6\n    BGT 2f\n    MOV r0,r4\n    B 3f\n1:  MOV r0,r5\n    B 3f\n2:  MOV r0,r6\n3:\n");
        return true;
    }
    // Trig via tables (index masked 7 bits, table of .hword)
    if matches!(up.as_str(), "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN") {
        if let Some(arg)=args.first() { emit_expr(arg,out,fctx,string_map); } else { out.push_str("    MOV r0,#0\n"); return true; }
        out.push_str("    AND r0,r0,#0x7F\n    LSL r0,r0,#1\n");
        out.push_str("    LDR r1, =SIN_TABLE\n");
        if up.ends_with("COS") { out.push_str("    LDR r1, =COS_TABLE\n"); }
        if up.ends_with("TAN") { out.push_str("    LDR r1, =TAN_TABLE\n"); }
        out.push_str("    ADD r1,r1,r0\n    LDRH r0,[r1]\n");
        return true;
    }
    false
}

// (string literal collection & escaping now delegated to backend::string_literals)

// collect_symbols: gather all variable identifiers across module for data section.
fn collect_symbols(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut globals = BTreeSet::new();
    let mut locals = BTreeSet::new();
    for item in &module.items {
        if let Item::Function(f) = item {
            for stmt in &f.body { collect_stmt_syms(stmt, &mut globals); }
            for l in collect_locals(&f.body) { locals.insert(l); }
        }
    }
    // remove locals from globals
    for l in &locals { globals.remove(l); }
    globals.into_iter().collect()
}

// collect_stmt_syms: scan a statement for variable references.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            if let crate::ast::AssignTarget::Ident { name, .. } = target {
                set.insert(name.clone());
            }
            collect_expr_syms(value, set);
        }
        Stmt::Let { .. } => { /* locals excluded */ }
        Stmt::Expr(e, _) => collect_expr_syms(e, set),
    Stmt::For { var: _var, start, end, step, body, .. } => {
            // Do not insert loop var here; it may be a local 'let' elsewhere. We'll rely on absence of Let to keep it global.
            collect_expr_syms(start, set);
            collect_expr_syms(end, set);
            if let Some(se) = step { collect_expr_syms(se, set); }
            for s in body { collect_stmt_syms(s, set); }
        }
        Stmt::While { cond, body, .. } => {
            collect_expr_syms(cond, set);
            for s in body {
                collect_stmt_syms(s, set);
            }
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
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
        Stmt::Return(o, _) => {
            if let Some(e) = o {
                collect_expr_syms(e, set);
            }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            collect_expr_syms(expr, set);
            for (ce, cb) in cases { collect_expr_syms(ce, set); for s in cb { collect_stmt_syms(s, set); } }
            if let Some(db) = default { for s in db { collect_stmt_syms(s, set); } }
        }
        Stmt::Break { .. } | Stmt::Continue { .. } => {},
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before collect_stmt_syms"),
        _ => {}, // Catch-all for unsupported statements (e.g., ForIn)
    }
}

// collect_expr_syms: scan an expression for identifiers.
fn collect_expr_syms(expr: &Expr, set: &mut std::collections::BTreeSet<String>) {
    match expr {
    Expr::Ident(n) => { set.insert(n.name.clone()); }
    Expr::StringLit(_) => {}
    Expr::Call(ci) => { for a in &ci.args { collect_expr_syms(a, set); } }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => {
            collect_expr_syms(left, set);
            collect_expr_syms(right, set);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => collect_expr_syms(inner, set),
        Expr::List(elements) => {
            for elem in elements {
                collect_expr_syms(elem, set);
            }
        }
        Expr::Index { target, index } => {
            collect_expr_syms(target, set);
            collect_expr_syms(index, set);
        }
        Expr::Number(_) => {}
    }
}

fn collect_locals(stmts: &[Stmt]) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut set = BTreeSet::new();
    fn walk(s: &Stmt, set: &mut std::collections::BTreeSet<String>) {
        match s {
            Stmt::Let { name, .. } => { set.insert(name.clone()); }
            Stmt::If { body, elifs, else_body, .. } => {
                for b in body { walk(b, set); }
                for (_, eb) in elifs { for b in eb { walk(b, set); } }
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
    for s in stmts { walk(s, &mut set); }
    set.into_iter().collect()
}

// fresh_label: produce unique assembler labels with a given prefix.
fn fresh_label(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static C: AtomicUsize = AtomicUsize::new(0);
    let id = C.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, id)
}
