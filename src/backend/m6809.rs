// (Removed duplicated legacy block above during refactor)
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::{collect_string_literals, escape_ascii};
use crate::codegen::CodegenOptions;
use crate::target::{Target, TargetInfo};

// emit: entry point for Motorola 6809 backend assembly generation.
// Produces a simple Vectrex-style header, calls platform init + MAIN, then infinite loop.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    let string_map = collect_string_literals(module);
    out.push_str(&format!("; --- Motorola 6809 backend ({}) title='{}' origin={} ---\n", ti.name, opts.title, ti.origin));
    out.push_str(&format!("        ORG {}\n", ti.origin));
    out.push_str(";***************************************************************************\n; DEFINE SECTION\n;***************************************************************************\n");
    // BIOS / vector ROM equates (subset)
    out.push_str("WAIT_RECAL    EQU $F192\nINTENSITY_5F EQU $F2A5\nPRINT_STR_D  EQU $F37A\nMUSIC1       EQU $FD0D\n\n");
    out.push_str(";***************************************************************************\n; HEADER SECTION\n;***************************************************************************\n");
    // Standard Vectrex cartridge header
    out.push_str("    DB \"g GCE 2025\", $80\n");
    out.push_str("    DW MUSIC1\n");
    out.push_str("    DB $F8, $50, $20, -$56\n");
    let mut title = opts.title.to_uppercase();
    if title.len() > 24 { title.truncate(24); }
    title = title.chars().map(|c| if c.is_ascii_alphanumeric() || c==' ' { c } else { ' ' }).collect();
    out.push_str(&format!("    DB \"{}\", $80\n", title));
    out.push_str("    DB 0\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    out.push_str(&format!("JSR {}\n", ti.init_label));
    // Emit all functions before calling MAIN so code exists.
    for item in &module.items {
        match item {
            Item::Function(f) => emit_function(f, &mut out, &string_map),
            Item::Const { name, value } => { if let Expr::Number(n) = value { out.push_str(&format!("{} EQU {}\n", name.to_uppercase(), n & 0xFFFF)); } }
        }
    }
    out.push_str("JSR MAIN\nEND_LOOP: BRA END_LOOP\n\n");
    out.push_str(";***************************************************************************\n; RUNTIME SECTION\n;***************************************************************************\n");
    emit_mul_helper(&mut out);
    emit_div_helper(&mut out);
    emit_builtin_helpers(&mut out); // Built-in Vectrex wrappers
    out.push_str(";***************************************************************************\n; DATA SECTION\n;***************************************************************************\n");
    out.push_str("; Variables\n");
    for v in syms { out.push_str(&format!("VAR_{}: FDB 0\n", v.to_uppercase())); }
    if !string_map.is_empty() { out.push_str("; String literals (null-terminated)\n"); }
    for (lit,label) in &string_map { out.push_str(&format!("{}: FCC \"{}\"\n    FCB 0\n", label, escape_ascii(lit))); }
    out.push_str("; Call argument scratch space\nVAR_ARG0: FDB 0\nVAR_ARG1: FDB 0\nVAR_ARG2: FDB 0\nVAR_ARG3: FDB 0\n");
    // Precomputed trig tables (Q7 fixed-point: value *128 ~ amplitude)
    out.push_str("; Trig tables (128 entries, 16-bit signed, scale = 127)\n");
    let mut sin_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; let v = (ang.sin()*127.0).round() as i16; sin_vals.push(v); }
    let mut cos_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; let v = (ang.cos()*127.0).round() as i16; cos_vals.push(v); }
    let mut tan_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; let t = ang.tan(); let v = if t.is_finite() { (t.max(-6.0).min(6.0)*20.0).round() as i16 } else { 0 }; tan_vals.push(v); }
    out.push_str("SIN_TABLE:\n");
    for chunk in sin_vals.chunks(8) { out.push_str("    FDB "); for (ci, val) in chunk.iter().enumerate() { if ci>0 { out.push_str(", "); } out.push_str(&format!("{}", *val as i32 & 0xFFFF)); } out.push_str("\n"); }
    out.push_str("COS_TABLE:\n");
    for chunk in cos_vals.chunks(8) { out.push_str("    FDB "); for (ci, val) in chunk.iter().enumerate() { if ci>0 { out.push_str(", "); } out.push_str(&format!("{}", *val as i32 & 0xFFFF)); } out.push_str("\n"); }
    out.push_str("TAN_TABLE:\n");
    for chunk in tan_vals.chunks(8) { out.push_str("    FDB "); for (ci, val) in chunk.iter().enumerate() { if ci>0 { out.push_str(", "); } out.push_str(&format!("{}", *val as i32 & 0xFFFF)); } out.push_str("\n"); }
    out
}

// emit_function: outputs code for a function.
fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>) {
    out.push_str(&format!("{}: ; function\n", f.name.to_uppercase()));
    out.push_str(&format!("; --- function {} ---\n{}:\n", f.name, f.name));
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local
    if frame_size > 0 { out.push_str(&format!("    LEAS -{} ,S ; allocate locals\n", frame_size)); }
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDD VAR_ARG{}\n    STD VAR_{}\n", i, p.to_uppercase()));
    }
    let fctx = FuncCtx { locals: locals.clone(), frame_size };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map); }
    if !matches!(f.body.last(), Some(Stmt::Return(_))) {
        if frame_size > 0 { out.push_str(&format!("    LEAS {} ,S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push_str("\n");
}

// emit_builtin_helpers: simple placeholder wrappers for Vectrex intrinsics.
fn emit_builtin_helpers(out: &mut String) {
    out.push_str("; --- Vectrex built-in wrappers ---\n");
    // NOTE: coordinate/intensity mapping minimal; real implementation would scale & call vector BIOS.
    // PRINT_TEXT: expects (x,y,ptr). Uses low bytes for position like BIOS expects (A=Y, B=X) then U=ptr
    out.push_str(
        "PRINT_TEXT:\n    LDU VAR_ARG2\n    LDA VAR_ARG1+1\n    LDB VAR_ARG0+1\n    JSR PRINT_STR_D\n    RTS\n"
    );
    // MOVE_TO: placeholder (would position beam) -> currently no-op
    out.push_str("MOVE_TO:\n    RTS\n");
    // DRAW_TO: placeholder for line draw from current to (x,y) with intensity
    out.push_str("DRAW_TO:\n    RTS\n");
    // DRAW_LINE: placeholder using 5 args
    out.push_str("DRAW_LINE:\n    RTS\n");
    // SET_ORIGIN: call WAIT_RECAL (recalibrate) and leave
    out.push_str("SET_ORIGIN:\n    JSR WAIT_RECAL\n    RTS\n");
    // SET_INTENSITY: call fixed intensity BIOS routine INTENSITY_5F (ignores arg for now)
    out.push_str("SET_INTENSITY:\n    JSR INTENSITY_5F\n    RTS\n");
    // Trig tables (128 entries, full circle) and access helpers are emitted in data section below.
}

// emit_builtin_call: inline lowering for intrinsic names; returns true if handled
fn emit_builtin_call(name: &str, args: &Vec<Expr>, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) -> bool {
    let up = name.to_ascii_uppercase();
    let is = matches!(up.as_str(), "PRINT_TEXT"|"MOVE_TO"|"DRAW_TO"|"DRAW_LINE"|"SET_ORIGIN"|"SET_INTENSITY"|"SIN"|"COS"|"TAN");
    if !is { return false; }
    if matches!(up.as_str(), "SIN"|"COS"|"TAN") {
        // Expect 1 arg
        if let Some(arg) = args.get(0) {
            emit_expr(arg, out, fctx, string_map);
            out.push_str("    LDD RESULT\n    ANDB #$7F\n    CLRA\n    ASLB\n    ROLA\n    LDX #SIN_TABLE\n");
            if up == "COS" { out.push_str("    LDX #COS_TABLE\n"); }
            if up == "TAN" { out.push_str("    LDX #TAN_TABLE\n"); }
            out.push_str("    ABX\n    LDD ,X\n    STD RESULT\n");
            return true;
        }
        // No arg: return 0
        out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
        return true;
    }
    for (i, a) in args.iter().enumerate() {
        if i >= 5 { break; }
        emit_expr(a, out, fctx, string_map);
        out.push_str("    LDD RESULT\n");
        out.push_str(&format!("    STD VAR_ARG{}\n", i));
    }
    out.push_str(&format!("    JSR {}\n", up));
    // Return 0
    out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
    true
}

#[derive(Default, Clone)]
struct LoopCtx { start: Option<String>, end: Option<String> }

#[derive(Clone)]
struct FuncCtx { locals: Vec<String>, frame_size: i32 }
impl FuncCtx { fn offset_of(&self, name: &str) -> Option<i32> { self.locals.iter().position(|n| n == name).map(|i| (i as i32)*2) } }

// emit_stmt: lower statements to 6809 instructions.
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_expr(value, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(target) {
                out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off));
            } else {
                out.push_str(&format!("    LDU #VAR_{}\n    STU TMPPTR\n    STX ,U\n", target.to_uppercase()));
            }
        }
        Stmt::Let { name, value } => {
            emit_expr(value, out, fctx, string_map);
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off)); }
        }
    Stmt::Expr(e) => emit_expr(e, out, fctx, string_map),
        Stmt::Return(o) => {
            if let Some(e) = o { emit_expr(e, out, fctx, string_map); }
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
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            out.push_str(&format!("    BRA {}\n{}: ; while end\n", ls, le));
        }
    Stmt::For { var, start, end, step, body } => {
            let ls = fresh_label("FOR");
            let le = fresh_label("FOR_END");
            emit_expr(start, out, fctx, string_map);
            out.push_str("    LDD RESULT\n");
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    STD {} ,S\n", off)); }
            else { out.push_str(&format!("    STD VAR_{}\n", var.to_uppercase())); }
            out.push_str(&format!("{}: ; for loop\n", ls));
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDD {} ,S\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n", var.to_uppercase())); }
            emit_expr(end, out, fctx, string_map);
            out.push_str("    LDX RESULT\n    CPD RESULT\n");
            out.push_str(&format!("    BHS {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map); }
            if let Some(se) = step {
                emit_expr(se, out, fctx, string_map);
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
            emit_expr(cond, out, fctx, string_map);
            out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", next));
            for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            out.push_str(&format!("    BRA {}\n", end));
            for (i, (c, b)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(c, out, fctx, string_map);
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", new_next));
                for s in b { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    BRA {}\n", end));
                next = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            } else if !elifs.is_empty() {
                out.push_str(&format!("{}:\n", next));
            }
            out.push_str(&format!("{}:\n", end));
        }
        Stmt::Switch { expr, cases, default } => {
            emit_expr(expr, out, fctx, string_map);
            out.push_str("    LDD RESULT\n    STD TMPLEFT ; switch value\n");
            let end = fresh_label("SW_END");
            let def_label = if default.is_some() { Some(fresh_label("SW_DEF")) } else { None };
            // Attempt jump table optimization (numeric, dense, >=3 cases, span*2 <= 254)
            let mut numeric_cases: Vec<(i32,&Vec<Stmt>)> = Vec::new();
            let mut all_numeric = true;
            for (ce, body) in cases {
                if let Expr::Number(n) = ce { numeric_cases.push((*n, body)); } else { all_numeric = false; break; }
            }
            let mut used_jump_table = false;
            if all_numeric && numeric_cases.len() >= 3 {
                numeric_cases.sort_by_key(|(v,_)| *v & 0xFFFF);
                let min = numeric_cases.first().unwrap().0 & 0xFFFF;
                let max = numeric_cases.last().unwrap().0 & 0xFFFF;
                let span = (max - min) as usize + 1;
                if span <= numeric_cases.len()*2 && span*2 <= 254 { // density + offset fit
                    let table_label = fresh_label("SW_JT");
                    use std::collections::BTreeMap;
                    let mut label_map: BTreeMap<i32,String> = BTreeMap::new();
                    for (val, _) in &numeric_cases { label_map.insert(*val & 0xFFFF, fresh_label("SW_CASE")); }
                    // Bounds check & index compute
                    out.push_str(&format!("    LDD TMPLEFT\n    SUBD #{}\n    BLT {}\n", min, def_label.as_ref().unwrap_or(&end)));
                    out.push_str(&format!("    CPD #{}\n    BHI {}\n", span as i32 - 1, def_label.as_ref().unwrap_or(&end)));
                    // D holds index (0..span-1); multiply by 2
                    out.push_str("    ASLB\n    ROLA\n");
                    out.push_str(&format!("    LDX #{}\n    ABX\n", table_label));
                    out.push_str("    LDD ,X\n    TFR D,X\n    JMP ,X\n");
                    // Bodies
                    for (val, body) in &numeric_cases {
                        let lbl = label_map.get(&(*val & 0xFFFF)).unwrap();
                        out.push_str(&format!("{}:\n", lbl));
                        for s in *body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                        out.push_str(&format!("    BRA {}\n", end));
                    }
                    if let Some(dl) = &def_label {
                        out.push_str(&format!("{}:\n", dl));
                        for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                    }
                    out.push_str(&format!("{}:\n", end));
                    // Jump table data (word entries)
                    out.push_str(&format!("{}:\n", table_label));
            for offset in 0..span as i32 {
                        let actual = (min as i32 + offset) & 0xFFFF;
                        if let Some(lbl) = label_map.get(&actual) { out.push_str(&format!("    FDB {}\n", lbl)); }
                        else if let Some(dl) = &def_label { out.push_str(&format!("    FDB {}\n", dl)); }
                        else { out.push_str(&format!("    FDB {}\n", end)); }
                    }
            used_jump_table = true;
                }
            }
        if used_jump_table { return; }
        // Fallback linear chain
            let mut labels = Vec::new();
            for _ in cases { labels.push(fresh_label("SW_CASE")); }
            for ((cv,_), lbl) in cases.iter().zip(labels.iter()) {
                emit_expr(cv, out, fctx, string_map);
                out.push_str("    LDD RESULT\n    SUBD TMPLEFT\n    BEQ ");
                out.push_str(lbl);
                out.push_str("\n");
            }
            if let Some(dl) = &def_label { out.push_str(&format!("    BRA {}\n", dl)); } else { out.push_str(&format!("    BRA {}\n", end)); }
            for ((_, body), lbl) in cases.iter().zip(labels.iter()) {
                out.push_str(&format!("{}:\n", lbl));
                for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map); }
                out.push_str(&format!("    BRA {}\n", end));
            }
            if let Some(dl) = def_label {
                out.push_str(&format!("{}:\n", dl));
                for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map); }
            }
            out.push_str(&format!("{}:\n", end));
        }
    }
}

// emit_expr: lower expressions; result placed in RESULT.
// Nota: En 6809 las operaciones sobre D ya limitan a 16 bits; no hace falta 'mask' explícito.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>) {
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("    LDD #{}\n    STD RESULT\n", *n & 0xFFFF));
        }
        Expr::StringLit(s) => {
            if let Some(label) = string_map.get(s) {
                out.push_str(&format!("    LDX #{}\n    STX RESULT\n", label));
            } else {
                out.push_str("    LDD #0\n    STD RESULT\n");
            }
        }
        Expr::Ident(name) => {
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDD {} ,S\n    STD RESULT\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n    STD RESULT\n", name.to_uppercase())); }
        }
        Expr::Call { name, args } => {
            if emit_builtin_call(name, args, out, fctx, string_map) { return; }
            for (i, arg) in args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr(arg, out, fctx, string_map);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            out.push_str(&format!("    JSR {}\n", name.to_uppercase()));
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr(left, out, fctx, string_map);
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
                    emit_expr(left, out, fctx, string_map);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr(right, out, fctx, string_map);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr(left, out, fctx, string_map);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr(left, out, fctx, string_map);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx, string_map);
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
            emit_expr(inner, out, fctx, string_map);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out, fctx, string_map);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx, string_map);
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
                emit_expr(left, out, fctx, string_map);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr(left, out, fctx, string_map);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr(right, out, fctx, string_map);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out, fctx, string_map);
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
    for item in &module.items {
        if let Item::Function(f) = item {
            for stmt in &f.body { collect_stmt_syms(stmt, &mut globals); }
            for l in collect_locals(&f.body) { locals.insert(l); }
        }
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
        Stmt::Switch { expr, cases, default } => {
            collect_expr_syms(expr, set);
            for (ce, cb) in cases { collect_expr_syms(ce, set); for s in cb { collect_stmt_syms(s, set); } }
            if let Some(db) = default { for s in db { collect_stmt_syms(s, set); } }
        }
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
    Expr::StringLit(_) => {}
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

// (string literal utilities now centralized in backend::string_literals)
