use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::{collect_string_literals, escape_ascii};
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};

// emit: entry point for Cortex-M backend (VecFever/Vextreme)
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    let string_map = collect_string_literals(module);
    out.push_str(&format!("; --- Cortex-M backend ({}) title='{}' origin={} ---\n", ti.name, opts.title, ti.origin));
    out.push_str(";***************************************************************************\n; DEFINE SECTION (Cortex-M)\n;***************************************************************************\n; (No Vectrex header needed)\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    out.push_str("; Vector table (prototype)\n    .section .isr_vector\n    .word _estack\n    .word Reset_Handler\n\nReset_Handler:\n    BL engine_init\n    BL main\n1:  B 1b\n\n");
    for item in &module.items {
        if let Item::Function(f) = item {
            emit_function(f, &mut out, &string_map);
        } else if let Item::Const { name, value: Expr::Number(n) } = item {
            out.push_str(&format!(".equ {} , {}\n", name, n & 0xFFFF));
        } else if let Item::ExprStatement(_) = item {
            // TODO: Manejar expresiones top-level en cortexm backend
            // Por ahora las ignoramos ya que no tenemos un "main" automático
        }
    }
    out.push_str(";***************************************************************************\n; RUNTIME SECTION\n;***************************************************************************\n; Runtime helpers\n");
    out.push_str(
        "__mul32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __mul32_done\n__mul32_loop:\n    AND r3,r1,#1\n    CMP r3,#0\n    BEQ __mul32_skip\n    ADD r2,r2,r0\n__mul32_skip:\n    LSR r1,r1,#1\n    LSL r0,r0,#1\n    CMP r1,#0\n    BNE __mul32_loop\n__mul32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str(
        "__div32:\n    PUSH {r2,r3,lr}\n    MOV r2,#0\n    CMP r1,#0\n    BEQ __div32_done\n    MOV r3,r0\n__div32_loop:\n    CMP r3,r1\n    BLT __div32_done\n    SUB r3,r3,r1\n    ADD r2,r2,#1\n    B __div32_loop\n__div32_done:\n    MOV r0,r2\n    POP {r2,r3,lr}\n    BX lr\n\n",
    );
    out.push_str(";***************************************************************************\n; DATA SECTION\n;***************************************************************************\n; Data\n    .section .data\n");
    // Shared trig tables
    out.push_str("; Trig tables (shared)\n");
    emit_trig_tables(&mut out, ".hword");
    for v in syms { out.push_str(&format!("VAR_{}: .word 0\n", v.to_uppercase())); }
    if !string_map.is_empty() { out.push_str("; String literals (null-terminated)\n"); }
    for (lit,label) in &string_map { out.push_str(&format!("{}: .ascii \"{}\"\n    .byte 0\n", label, escape_ascii(lit))); }
    out.push_str("; Call arg scratch\nVAR_ARG0: .word 0\nVAR_ARG1: .word 0\nVAR_ARG2: .word 0\nVAR_ARG3: .word 0\n");
    out
}

// emit_function: outputs code for a function body.
fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>) {
    out.push_str(&format!(".global {0}\n{0}:\n", f.name));
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local (compact like ARM)
    if frame_size > 0 { out.push_str(&format!("    SUB sp, sp, #{}\n", frame_size)); }
    // Parameter handling (still via globals for now)
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
struct LoopCtx { start: Option<String>, end: Option<String> }

#[derive(Clone)]
struct FuncCtx { locals: Vec<String>, frame_size: i32 }
impl FuncCtx { fn offset_of(&self, name: &str) -> Option<i32> { self.locals.iter().position(|n| n == name).map(|i| (i as i32) * 2) } }

// emit_stmt: lowers high-level statements to Cortex-M instructions.
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            match target {
                crate::ast::AssignTarget::Ident { name, .. } => {
                    emit_expr(value, out, fctx, string_map);
                    if let Some(off) = fctx.offset_of(name) {
                        out.push_str(&format!("    STRH r0, [sp, #{}]\n", off));
                    } else {
                        out.push_str(&format!("    LDR r1, =VAR_{}\n    STR r0, [r1]\n", name.to_uppercase()));
                    }
                }
                crate::ast::AssignTarget::Index { .. } => panic!("Array assignment not implemented for Cortex-M backend"),
                crate::ast::AssignTarget::FieldAccess { .. } => panic!("Field access assignment not implemented for Cortex-M backend (Phase 3)"),
            }
        },
        Stmt::Let { name, value, .. } => {
            emit_expr(value, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(name) {
                out.push_str(&format!("    STRH r0, [sp, #{}]\n", off));
            }
        },
        Stmt::Expr(e, _) => emit_expr(e, out, fctx, string_map),
        Stmt::Return(o, _) => {
            if let Some(e) = o { emit_expr(e, out, fctx, string_map); }
            if fctx.frame_size > 0 { out.push_str(&format!("    ADD sp, sp, #{}\n", fctx.frame_size)); }
            out.push_str("    BX LR\n");
        },
        Stmt::Break { .. } => {
            if let Some(end) = &loop_ctx.end {
                out.push_str(&format!("    B {}\n", end));
            }
        },
        Stmt::Continue { .. } => {
            if let Some(st) = &loop_ctx.start {
                out.push_str(&format!("    B {}\n", st));
            }
        },
        Stmt::Pass { .. } => {
            out.push_str("    ; pass (no-op)\n");
        },
        Stmt::While { cond, body, .. } => {
            let ls = fresh_label("WH");
            let le = fresh_label("WH_END");
            out.push_str(&format!("{}:\n", ls));
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    CMP r0, #0\n    BEQ {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            out.push_str(&format!("    B {}\n{}:\n", ls, le));
        },
        Stmt::For { var, start, end, step, body, .. } => {
            let ls = fresh_label("FOR");
            let le = fresh_label("FOR_END");
            emit_expr(start, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(var) {
                out.push_str(&format!("    STRH r0, [sp, #{}]\n", off));
            } else {
                out.push_str(&format!("    LDR r1, =VAR_{}\n    STR r0, [r1]\n", var.to_uppercase()));
            }
            out.push_str(&format!("{}:\n", ls));
            if let Some(off) = fctx.offset_of(var) {
                out.push_str(&format!("    LDRH r1, [sp, #{}]\n", off));
            } else {
                out.push_str(&format!("    LDR r1, =VAR_{}\n    LDR r1, [r1]\n", var.to_uppercase()));
            }
            emit_expr(end, out, fctx, string_map);
            out.push_str(&format!("    CMP r1, r0\n    BGE {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            if let Some(se) = step { emit_expr(se, out, fctx, string_map); } else { out.push_str("    MOV r0, #1\n"); }
            if let Some(off) = fctx.offset_of(var) {
                out.push_str(&format!("    LDRH r3, [sp, #{}]\n    ADD r3, r3, r0\n    AND r3, r3, #0xFFFF\n    STRH r3, [sp, #{}]\n", off, off));
            } else {
                out.push_str(&format!("    LDR r2, =VAR_{}\n    LDR r3, [r2]\n    ADD r3, r3, r0\n    STR r3, [r2]\n", var.to_uppercase()));
            }
            out.push_str(&format!("    B {}\n{}:\n", ls, le));
        },
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let end = fresh_label("IF_END");
            let mut next = fresh_label("IF_NEXT");
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    CMP r0,#0\n    BEQ {}\n", next));
            for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            out.push_str(&format!("    B {}\n", end));
            for (i, (c, b)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(c, out, fctx, string_map);
                out.push_str(&format!("    CMP r0,#0\n    BEQ {}\n", new_next));
                for s in b { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    B {}\n", end));
                next = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            } else if !elifs.is_empty() {
                out.push_str(&format!("{}:\n", next));
            }
            out.push_str(&format!("{}:\n", end));
        },
        Stmt::Switch { expr, cases, default, .. } => {
            emit_expr(expr, out, fctx, string_map);
            out.push_str("    MOV r4,r0\n");
            let end = fresh_label("SW_END");
            let def_label = if default.is_some() { Some(fresh_label("SW_DEF")) } else { None };
            let mut numeric_cases: Vec<(i32,&Vec<Stmt>)> = Vec::new();
            let mut all_numeric = true;
            for (ce, body) in cases {
                if let Expr::Number(n) = ce { numeric_cases.push((*n, body)); } else { all_numeric = false; break; }
            }
            if all_numeric && numeric_cases.len() >= 3 {
                numeric_cases.sort_by_key(|(v,_)| *v);
                let min = numeric_cases.first().unwrap().0 & 0xFFFF;
                let max = numeric_cases.last().unwrap().0 & 0xFFFF;
                let span = (max - min) as usize + 1;
                if span <= numeric_cases.len()*2 { // density heuristic
                    let table_label = fresh_label("SW_JT");
                    let mut label_map: std::collections::BTreeMap<i32,String> = std::collections::BTreeMap::new();
                    for (val, _) in &numeric_cases { label_map.insert(*val, fresh_label("SW_CASE")); }
                    out.push_str(&format!("    MOV r5,#{}\n    CMP r4,r5\n    BLT {}\n", min, def_label.as_ref().unwrap_or(&end)));
                    out.push_str(&format!("    MOV r5,#{}\n    CMP r4,r5\n    BGT {}\n", max, def_label.as_ref().unwrap_or(&end)));
                    if min != 0 { out.push_str(&format!("    SUB r4,r4,#{}\n", min)); }
                    out.push_str(&format!("    LDR r5, ={}\n    LSL r4,r4,#2\n    ADD r5,r5,r4\n    LDR r5, [r5]\n    BX r5\n", table_label));
                    for (val, body) in &numeric_cases {
                        let lbl = label_map.get(val).unwrap();
                        out.push_str(&format!("{}:\n", lbl));
                        for s in *body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                        out.push_str(&format!("    B {}\n", end));
                    }
                    if let Some(dl) = &def_label {
                        out.push_str(&format!("{}:\n", dl));
                        for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                    }
                    out.push_str(&format!("{}:\n", end));
                    out.push_str(&format!("{}:\n", table_label));
                    for offset in 0..span as i32 {
                        let actual = (min + offset) & 0xFFFF;
                        if let Some(lbl) = label_map.get(&actual) { out.push_str(&format!("    .word {}\n", lbl)); }
                        else if let Some(dl) = &def_label { out.push_str(&format!("    .word {}\n", dl)); }
                        else { out.push_str(&format!("    .word {}\n", end)); }
                    }
                    return;
                }
            }
            // fallback linear chain
            let mut labels = Vec::new();
            for _ in cases { labels.push(fresh_label("SW_CASE")); }
            for ((cv,_), lbl) in cases.iter().zip(labels.iter()) {
                emit_expr(cv, out, fctx, string_map);
                out.push_str(&format!("    CMP r4,r0\n    BEQ {}\n", lbl));
            }
            if let Some(dl) = &def_label { out.push_str(&format!("    B {}\n", dl)); } else { out.push_str(&format!("    B {}\n", end)); }
            for ((_, body), lbl) in cases.iter().zip(labels.iter()) {
                out.push_str(&format!("{}:\n", lbl));
                for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    B {}\n", end));
            }
            if let Some(dl) = def_label {
                out.push_str(&format!("{}:\n", dl));
                for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            }
            out.push_str(&format!("{}:\n", end));
        },
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before emit_stmt"),
        _ => panic!("Unsupported statement type in Cortex-M backend"),
    }
}

// emit_expr: generates expression into r0 with masking.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match expr {
    Expr::Number(n) => out.push_str(&format!("    MOV r0,#{}\n", *n)),
    Expr::StringLit(s) => { if let Some(label)=string_map.get(s){ out.push_str(&format!("    LDR r0, ={}\n", label)); } else { out.push_str("    MOV r0,#0 ; missing string label\n"); } }
        Expr::Ident(name) => {
            if let Some(off) = fctx.offset_of(&name.name) { out.push_str(&format!("    LDR r0, [sp, #{}]\n", off)); }
            else { out.push_str(&format!("    LDR r0, =VAR_{}\n    LDR r0,[r0]\n", name.name.to_uppercase())); }
        }
        Expr::List(_) => panic!("Array literals not implemented for Cortex-M backend"),
        Expr::Index { .. } => panic!("Array indexing not implemented for Cortex-M backend"),
        Expr::Call(ci) => {
            if emit_builtin_call_cm(&ci.name, &ci.args, out, fctx, string_map) { return; }
            let limit = ci.args.len().min(4);
            for idx in (0..limit).rev() { emit_expr(&ci.args[idx], out, fctx, string_map); if idx!=0 { out.push_str(&format!("    MOV r{} , r0\n", idx)); } }
            out.push_str(&format!("    BL {}\n", ci.name));
        }
        Expr::Binary { op, left, right } => {
            if let BinOp::Mul = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out, fctx, string_map);
                            out.push_str(&format!("    LSL r0,r0,#{}\n    AND r0,r0,#0xFFFF\n", shift));
                            return;
                        }
                    }
                }
                if let Expr::Number(n) = &**left {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(right, out, fctx, string_map);
                            out.push_str(&format!("    LSL r0,r0,#{}\n    AND r0,r0,#0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            } else if let BinOp::Div = op {
                if let Expr::Number(n) = &**right {
                    if *n > 0 && (*n & (*n - 1)) == 0 {
                        if let Some(shift) = (0..=16).find(|s| (1 << s) == *n) {
                            emit_expr(left, out, fctx, string_map);
                            out.push_str(&format!("    LSR r0,r0,#{}\n    AND r0,r0,#0xFFFF\n", shift));
                            return;
                        }
                    }
                }
            }
            emit_expr(left, out, fctx, string_map);
            out.push_str("    MOV r4,r0\n");
            emit_expr(right, out, fctx, string_map);
            out.push_str("    MOV r5,r0\n");
            match op {
                BinOp::Add => out.push_str("    ADD r0,r4,r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Sub => out.push_str("    SUB r0,r4,r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Mul => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __mul32\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Div => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    AND r0,r0,#0xFFFF\n"),
                BinOp::FloorDiv => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    AND r0,r0,#0xFFFF\n"), // División entera igual que Div
                BinOp::Mod => out.push_str("    MOV r0,r4\n    MOV r1,r5\n    BL __div32\n    MOV r2,r0\n    MUL r2,r2,r5\n    RSBS r0,r2,r4\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shl => out.push_str("    MOV r0,r4,LSL r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::Shr => out.push_str("    MOV r0,r4,LSR r5\n    AND r0,r0,#0xFFFF\n"),
                BinOp::BitAnd => out.push_str("    AND r0,r4,r5\n"),
                BinOp::BitOr  => out.push_str("    ORR r0,r4,r5\n"),
                BinOp::BitXor => out.push_str("    EOR r0,r4,r5\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr(inner, out, fctx, string_map);
            out.push_str("    MVN r0,r0\n    AND r0,r0,#0xFFFF\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out, fctx, string_map);
            out.push_str("    MOV r4,r0\n");
            emit_expr(right, out, fctx, string_map);
            out.push_str("    MOV r5,r0\n    CMP r4,r5\n    MOV r0,#0\n");
            let lt = fresh_label("CT");
            let end = fresh_label("CE");
            let br = match op {
                CmpOp::Eq => "BEQ",
                CmpOp::Ne => "BNE",
                CmpOp::Lt => "BLT",
                CmpOp::Le => "BLE",
                CmpOp::Gt => "BGT",
                CmpOp::Ge => "BGE",
            };
            out.push_str(&format!("    {} {}\n    B {}\n{}:\n    MOV r0,#1\n{}:\n", br, lt, end, lt, end));
        }
        Expr::Logic { op, left, right } => match op {
            LogicOp::And => {
                emit_expr(left, out, fctx, string_map);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    CMP r0,#0\n    BEQ {}\n", fl));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BEQ {}\n    MOV r0,#1\n    B {}\n{}:\n    MOV r0,#0\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr(left, out, fctx, string_map);
                out.push_str(&format!("    CMP r0,#0\n    BNE {}\n", tr));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    CMP r0,#0\n    BNE {}\n    MOV r0,#0\n    B {}\n{}:\n    MOV r0,#1\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out, fctx, string_map);
            out.push_str("    CMP r0,#0\n    MOVEQ r0,#1\n    MOVNE r0,#0\n");
        }
        Expr::StructInit { .. } => panic!("Struct initialization not implemented for Cortex-M backend (Phase 3)"),
        Expr::FieldAccess { .. } => panic!("Field access not implemented for Cortex-M backend (Phase 3)"),
    }
}

fn emit_builtin_call_cm(name: &str, args: &[Expr], out: &mut String, fctx:&FuncCtx, string_map:&std::collections::BTreeMap<String,String>) -> bool {
    let up = name.to_ascii_uppercase();
    let is = matches!(up.as_str(),
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|
        "ABS"|"MATH_ABS"|"MIN"|"MATH_MIN"|"MAX"|"MATH_MAX"|"CLAMP"|"MATH_CLAMP"|
        "VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"
    );
    if !is {
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
        if let Some(new_up)=translated { return emit_builtin_call_cm(new_up, args, out, fctx, string_map); }
        return false;
    }
    if matches!(up.as_str(),"VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY") {
        for a in args { emit_expr(a,out,fctx,string_map); }
        out.push_str("    MOV r0,#0\n");
        return true;
    }
    if matches!(up.as_str(),"ABS"|"MATH_ABS") {
        if let Some(a)=args.first(){ emit_expr(a,out,fctx,string_map);} else { out.push_str("    MOV r0,#0\n"); return true; }
        let done = fresh_label("ABS_DONE");
        out.push_str(&format!("    CMP r0,#0\n    BGE {}\n    RSBS r0,r0,#0\n{}:\n",done,done));
        return true;
    }
    if matches!(up.as_str(),"MIN"|"MATH_MIN") {
        if args.len()<2 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0],out,fctx,string_map); out.push_str("    MOV r4,r0\n");
        emit_expr(&args[1],out,fctx,string_map); out.push_str("    MOV r5,r0\n");
        out.push_str("    CMP r4,r5\n    BLE 1f\n    MOV r0,r5\n    B 2f\n1:  MOV r0,r4\n2:\n");
        return true;
    }
    if matches!(up.as_str(),"MAX"|"MATH_MAX") {
        if args.len()<2 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0],out,fctx,string_map); out.push_str("    MOV r4,r0\n");
        emit_expr(&args[1],out,fctx,string_map); out.push_str("    MOV r5,r0\n");
        out.push_str("    CMP r4,r5\n    BGE 1f\n    MOV r0,r5\n    B 2f\n1:  MOV r0,r4\n2:\n");
        return true;
    }
    if matches!(up.as_str(),"CLAMP"|"MATH_CLAMP") {
        if args.len()<3 { out.push_str("    MOV r0,#0\n"); return true; }
        emit_expr(&args[0],out,fctx,string_map); out.push_str("    MOV r4,r0\n");
        emit_expr(&args[1],out,fctx,string_map); out.push_str("    MOV r5,r0\n");
        emit_expr(&args[2],out,fctx,string_map); out.push_str("    MOV r6,r0\n");
        out.push_str("    CMP r4,r5\n    BLT 1f\n    CMP r4,r6\n    BGT 2f\n    MOV r0,r4\n    B 3f\n1:  MOV r0,r5\n    B 3f\n2:  MOV r0,r6\n3:\n");
        return true;
    }
    if matches!(up.as_str(),"SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN") {
        if let Some(a)=args.first(){ emit_expr(a,out,fctx,string_map); } else { out.push_str("    MOV r0,#0\n"); return true; }
        out.push_str("    AND r0,r0,#0x7F\n    LSL r0,r0,#1\n    LDR r1,=SIN_TABLE\n");
        if up.ends_with("COS") { out.push_str("    LDR r1,=COS_TABLE\n"); }
        if up.ends_with("TAN") { out.push_str("    LDR r1,=TAN_TABLE\n"); }
        out.push_str("    ADD r1,r1,r0\n    LDRH r0,[r1]\n");
        return true;
    }
    false
}

// (string literal utilities refactored to backend::string_literals)

// collect_symbols: scan module for variables.
fn collect_symbols(module: &Module) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut globals = BTreeSet::new();
    let mut locals = BTreeSet::new();
    for item in &module.items {
        if let Item::Function(f) = item {
            for stmt in &f.body { collect_stmt_syms(stmt, &mut globals); }
            for l in collect_locals(&f.body) { locals.insert(l); }
        } else if let Item::ExprStatement(expr) = item {
            // Scan expression statements for symbol usage
            collect_expr_syms(expr, &mut globals);
        }
    }
    for l in &locals { globals.remove(l); }
    globals.into_iter().collect()
}

// collect_stmt_syms: collect variable names in statement.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
    Stmt::Assign { target, value, .. } => {
            if let crate::ast::AssignTarget::Ident { name, .. } = target {
                set.insert(name.clone());
            }
            collect_expr_syms(value, set);
        }
    Stmt::Let { name: _, value, .. } => { collect_expr_syms(value, set); } // locals excluded
        Stmt::Expr(e, _) => collect_expr_syms(e, set),
        Stmt::For { var: _, start, end, step, body, .. } => {
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
        Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => {},
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before collect_stmt_syms"),
        _ => {}, // Catch-all for unsupported statements
    }
}

// collect_expr_syms: collect identifiers in expression.
fn collect_expr_syms(expr: &Expr, set: &mut std::collections::BTreeSet<String>) {
    match expr {
        Expr::Ident(n) => {
            set.insert(n.name.clone());
        }
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
    Expr::StringLit(_) => {}
    }
}

// collect_locals: gather unique let-declared identifiers within function body
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
            Stmt::For { var, body, .. } => { set.insert(var.clone()); for b in body { walk(b, set); } }
            _ => {}
        }
    }
    let mut set = BTreeSet::new();
    for s in stmts { walk(s, &mut set); }
    set.into_iter().collect()
}

// fresh_label: create unique label.
fn fresh_label(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static C: AtomicUsize = AtomicUsize::new(0);
    let id = C.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, id)
}
