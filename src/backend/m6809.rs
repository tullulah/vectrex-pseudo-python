// (Removed duplicated legacy block above during refactor)
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::collect_string_literals;
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};

// emit: entry point for Motorola 6809 backend assembly generation.
// Produces a simple Vectrex-style header, calls platform init + MAIN, then infinite loop.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    let string_map = collect_string_literals(module);
        let rt_usage = analyze_runtime_usage(module);
    out.push_str(&format!("; --- Motorola 6809 backend ({}) title='{}' origin={} ---\n", ti.name, opts.title, ti.origin));
    out.push_str(&format!("        ORG {}\n", ti.origin));
    out.push_str(";***************************************************************************\n; DEFINE SECTION\n;***************************************************************************\n");
    // Always include official equates from ../include (assembler invoked from project root, asm in examples/)
    out.push_str("    INCLUDE \"../include/VECTREX.I\"\n\n");
    out.push_str(";***************************************************************************\n; HEADER SECTION\n;***************************************************************************\n");
    // Header (emulator-compatible variant):
    //  - 'g GCE 1982' + $80
    //  - music pointer (word) (set 0 if no custom music)
    //  - height, width, rel y, rel x
    //  - title bytes (plain ASCII, sanitized, length<=24)
    //  - $80 terminator for title
    //  - reserved 0 byte
    //  - pad with zeros to $0030
    out.push_str("    FCC \"g GCE 1982\"\n");
    out.push_str("    FCB $80\n");
    out.push_str("    FDB $0000 ; music pointer (0 = none)\n");
    out.push_str("    FCB $F8 ; height\n    FCB $50 ; width\n    FCB $20 ; rel y\n    FCB $D0 ; rel x (-$30)\n");
    let mut title = opts.title.to_uppercase();
    if title.len() > 24 { title.truncate(24); }
    title = title.chars().map(|c| if c.is_ascii_alphanumeric() || c==' ' { c } else { ' ' }).collect();
    if title.is_empty() { title.push(' '); }
    out.push_str(&format!("    FCC \"{}\"\n", title));
    out.push_str("    FCB $80 ; title terminator\n");
    out.push_str("    FCB 0 ; reserved\n");
    // Pad to 0x30 bytes (Vectrex BIOS begins executing at $0030)
    out.push_str("    RMB $0030-* ; pad header to $30\n");
    // Start code at $0030 (Vectrex expects execution here after BIOS header scan)
    out.push_str("    ORG $0030\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    // No explicit init routine defined yet for Vectrex; skip calling ti.init_label if undefined.
    // Execution falls through to MAIN directly.
    // Entry stub: call MAIN then loop forever (Vectrex BIOS expects cartridge not to return).
    // Precompute flags
    let do_blink = opts.blink_intensity;
    let do_per_frame_silence = opts.per_frame_silence;
    let jsr_ext = if opts.force_extended_jsr { ">" } else { "" };

    if opts.auto_loop {
        // TODO: Refactor init/loop below to remove DP_TO_C8 usage; header + wrappers now assume DP stays $D0 until VECTOR_PHASE_BEGIN.
        out.push_str("; Init then implicit frame loop (auto_loop enabled)\n");
    // Detect if user code uses FRAME_BEGIN so we can emit a lean wrapper and avoid double Wait_Recal / intensity.
    let frame_begin_used = rt_usage.wrappers_used.contains("VECTREX_FRAME_BEGIN");
        if opts.minimal_init {
            if frame_begin_used {
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
                let debug_line = if opts.debug_init_draw { "JSR VECTREX_DEBUG_DRAW\n    ".to_string() } else { String::new() };
                out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    JSR VECTREX_SILENCE\n    ; minimal_init + frame_begin_used: FRAME_BEGIN will Reset0Ref.\n    {}BRA ENTRY_LOOP\n",
                    jsr_ext,
                    intensity_line,
                    debug_line
                ));
                out.push_str("ENTRY_LOOP: ");
                if opts.diag_freeze { out.push_str("INC DIAG_COUNTER\n    "); }
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n    ".to_string() } else { format!("JSR {}INTENSITY_5F\n    ", jsr_ext) };
                out.push_str(&format!("JSR {}Wait_Recal\n    {}{}    ; FRAME_BEGIN may still adjust intensity/reset\n    JSR MAIN\n    BRA ENTRY_LOOP\n\n",
                    jsr_ext,
                    silence_line,
                    intensity_line
                ));
            } else {
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let debug_line = if opts.debug_init_draw { "JSR VECTREX_DEBUG_DRAW\n    " } else { "" };
                out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    {}; minimal_init: skip Reset0Ref here\n    {}BRA ENTRY_LOOP\n",
                    jsr_ext,
                    intensity_line,
                    silence_line,
                    debug_line
                ));
                out.push_str("ENTRY_LOOP: ");
                if opts.diag_freeze { out.push_str("INC DIAG_COUNTER\n    "); }
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n    ".to_string() } else { format!("JSR {}INTENSITY_5F\n    ", jsr_ext) };
                out.push_str(&format!("JSR {}Wait_Recal\n    {}{}    JSR MAIN\n    BRA ENTRY_LOOP\n\n",
                    jsr_ext,
                    silence_line,
                    intensity_line
                ));
            }
        } else {
            if frame_begin_used {
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
                let debug_line = if opts.debug_init_draw { "JSR VECTREX_DEBUG_DRAW\n    " } else { "" };
                out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    JSR {}Reset0Ref\n    {}BRA ENTRY_LOOP\n",
                    jsr_ext,
                    intensity_line,
                    jsr_ext,
                    debug_line
                ));
                out.push_str("ENTRY_LOOP: ");
                if opts.diag_freeze { out.push_str("INC DIAG_COUNTER\n    "); }
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n    ".to_string() } else { format!("JSR {}INTENSITY_5F\n    ", jsr_ext) };
                out.push_str(&format!("JSR {}Wait_Recal\n    {}{}    ; FRAME_BEGIN wrapper (called by user) may set different intensity / Reset0Ref.\n    JSR MAIN\n    BRA ENTRY_LOOP\n\n",
                    jsr_ext,
                    silence_line,
                    intensity_line
                ));
            } else {
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let debug_line = if opts.debug_init_draw { "JSR VECTREX_DEBUG_DRAW\n    " } else { "" };
                out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    JSR {}Reset0Ref\n    {}{}BRA ENTRY_LOOP\n",
                    jsr_ext,
                    intensity_line,
                    jsr_ext,
                    silence_line,
                    debug_line
                ));
                out.push_str("ENTRY_LOOP: ");
                if opts.diag_freeze { out.push_str("INC DIAG_COUNTER\n    "); }
                let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
                let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n    ".to_string() } else { format!("JSR {}INTENSITY_5F\n    ", jsr_ext) };
                out.push_str(&format!("JSR {}Wait_Recal\n    {}{}    ; DP & origin assumed stable from Reset0Ref\n    JSR MAIN\n    BRA ENTRY_LOOP\n\n",
                    jsr_ext,
                    silence_line,
                    intensity_line
                ));
            }
        }
    } else {
        out.push_str("; Init without implicit loop (auto_loop disabled)\n");
        if opts.minimal_init {
            let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
            let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
            out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    {}; minimal_init: skipping Reset0Ref, user must call if needed\n    JSR MAIN ; user must implement its own frame loop\nHANG: BRA HANG ; prevent fallthrough\n\n",
                jsr_ext,
                intensity_line,
                silence_line
            ));
        } else {
            let intensity_line = if do_blink { "JSR VECTREX_BLINK_INT\n".to_string() } else { format!("JSR {}INTENSITY_5F\n", jsr_ext) };
            let silence_line = if do_per_frame_silence { "JSR VECTREX_SILENCE\n    " } else { "" };
            out.push_str(&format!("INIT_START: JSR {}Wait_Recal\n    {}    JSR {}Reset0Ref\n    {}JSR MAIN ; user must implement its own frame loop\nHANG: BRA HANG ; prevent fallthrough\n\n",
                jsr_ext,
                intensity_line,
                jsr_ext,
                silence_line
            ));
        }
    }
    // Emit all functions so code exists (MAIN label will resolve).
    for item in &module.items {
        match item {
            Item::Function(f) => emit_function(f, &mut out, &string_map, opts),
            Item::Const { name, value } => { if let Expr::Number(n) = value { out.push_str(&format!("{} EQU {}\n", name.to_uppercase(), n & 0xFFFF)); } }
        }
    }
    // (Legacy tail loop removed; entry stub already loops.)
    out.push_str(";***************************************************************************\n; RUNTIME SECTION\n;***************************************************************************\n");
    if rt_usage.needs_mul_helper { emit_mul_helper(&mut out); }
    if rt_usage.needs_div_helper { emit_div_helper(&mut out); }
    emit_builtin_helpers(&mut out, &rt_usage); // Built-in Vectrex wrappers (conditional)
    out.push_str(";***************************************************************************\n; DATA SECTION\n;***************************************************************************\n");
    // Align ROM size to next 4K boundary: compute remainder via assembler can't do complex IF here, approximate with macro-style logic.
    // Fallback: emit a padding block sized by repeating labels (simple approach): not portable across all assemblers, so disabled for now.
    // NOTE: External packer should align to desired bank size (4K/8K). No internal alignment performed.
    // Optional bank alignment (4K/8K). Use ALIGN macro if bank_size is power-of-two.
    // Bank padding handled at end of file now.
    // RAM variables: either emit ORG or symbolic EQU addresses.
    if !opts.exclude_ram_org {
        out.push_str("    ORG $C880 ; begin runtime variables in RAM\n");
    }
    out.push_str("; Variables (in RAM)\n");
    // Runtime temporaries used by expression lowering and helpers
    if opts.exclude_ram_org { out.push_str("RESULT    EQU $C880\n"); } else { out.push_str("RESULT:   FDB 0\n"); }
    if rt_usage.needs_tmp_left { if opts.exclude_ram_org { out.push_str("TMPLEFT   EQU RESULT+2\n"); } else { out.push_str("TMPLEFT:  FDB 0\n"); } }
    if rt_usage.needs_tmp_right { if opts.exclude_ram_org { out.push_str("TMPRIGHT  EQU RESULT+4\n"); } else { out.push_str("TMPRIGHT: FDB 0\n"); } }
    if rt_usage.needs_tmp_ptr { if opts.exclude_ram_org { out.push_str("TMPPTR    EQU RESULT+6\n"); } else { out.push_str("TMPPTR:   FDB 0\n"); } }
    if rt_usage.needs_mul_helper || rt_usage.needs_div_helper { // MUL vars shared with MOD too
        if rt_usage.needs_mul_helper {
            if opts.exclude_ram_org { out.push_str("MUL_A    EQU RESULT+8\nMUL_B    EQU RESULT+10\nMUL_RES  EQU RESULT+12\nMUL_TMP  EQU RESULT+14\nMUL_CNT  EQU RESULT+16\n"); }
            else { out.push_str("MUL_A:    FDB 0\nMUL_B:    FDB 0\nMUL_RES:  FDB 0\nMUL_TMP:  FDB 0\nMUL_CNT:  FDB 0\n"); }
        }
    }
    if rt_usage.needs_div_helper {
        if opts.exclude_ram_org { out.push_str("DIV_A   EQU RESULT+18\nDIV_B   EQU RESULT+20\nDIV_Q   EQU RESULT+22\nDIV_R   EQU RESULT+24\n"); }
        else { out.push_str("DIV_A:    FDB 0\nDIV_B:    FDB 0\nDIV_Q:   FDB 0\nDIV_R:   FDB 0\n"); }
    }
    let mut var_offset = 26; // after reserved temps above when exclude_ram_org
    for v in syms {
        if opts.exclude_ram_org {
            out.push_str(&format!("VAR_{} EQU RESULT+{}\n", v.to_uppercase(), var_offset));
            var_offset += 2;
        } else {
            out.push_str(&format!("VAR_{}: FDB 0\n", v.to_uppercase()));
        }
    }
    if !string_map.is_empty() { out.push_str("; String literals (high-bit terminated for Vectrex PRINT_STR_D)\n"); }
    for (lit,label) in &string_map {
        // Build FCB list with last char high bit set
        let mut bytes: Vec<u8> = lit.bytes().collect();
        if let Some(last) = bytes.last_mut() { *last |= 0x80; } else { bytes.push(0x80); }
        // Emit as FCB sequence (avoid FCC so we can set high bit explicitly)
    out.push_str(&format!("{}:", label));
    for b in bytes { out.push_str(&format!("\n    FCB ${:02X}", b)); }
    out.push_str("\n");
    }
    // Determine max args used (0..5)
    let max_args = compute_max_args_used(module);
    out.push_str("; Call argument scratch space\n");
    if opts.exclude_ram_org {
        for i in 0..max_args { out.push_str(&format!("VAR_ARG{} EQU RESULT+{}\n", i, var_offset)); var_offset += 2; }
    } else {
        if max_args >=1 { out.push_str("VAR_ARG0: FDB 0\n"); }
        if max_args >=2 { out.push_str("VAR_ARG1: FDB 0\n"); }
        if max_args >=3 { out.push_str("VAR_ARG2: FDB 0\n"); }
        if max_args >=4 { out.push_str("VAR_ARG3: FDB 0\n"); }
        if max_args >=5 { out.push_str("VAR_ARG4: FDB 0\n"); }
    }
    if opts.diag_freeze { if opts.exclude_ram_org { out.push_str(&format!("DIAG_COUNTER EQU RESULT+{}\n", var_offset)); var_offset += 1; } else { out.push_str("DIAG_COUNTER: FCB 0\n"); } }
    if rt_usage.needs_vcur_vars {
        if opts.exclude_ram_org {
            out.push_str(&format!("VCUR_X EQU RESULT+{}\n", var_offset)); var_offset += 1;
            out.push_str(&format!("VCUR_Y EQU RESULT+{}\n", var_offset)); var_offset += 1;
        } else { out.push_str("; Current beam position (low byte storage)\nVCUR_X: FCB 0\nVCUR_Y: FCB 0\n"); }
    }
    if rt_usage.needs_line_vars {
        if opts.exclude_ram_org {
            out.push_str(&format!("VLINE_DX EQU RESULT+{}\n", var_offset)); var_offset += 1;
            out.push_str(&format!("VLINE_DY EQU RESULT+{}\n", var_offset)); var_offset += 1;
            out.push_str(&format!("VLINE_STEPS EQU RESULT+{}\n", var_offset)); var_offset += 1;
            out.push_str(&format!("VLINE_LIST EQU RESULT+{}\n", var_offset)); var_offset += 2; // 2 bytes
        } else { out.push_str("; Line drawing temps\nVLINE_DX: FCB 0\nVLINE_DY: FCB 0\nVLINE_STEPS: FCB 0\nVLINE_LIST: FCB 0,0 ; 2-byte vector list (Y|endbit, X)\n"); }
    }
    // Blink state variable (1 byte) must live in RAM, not ROM.
    if opts.blink_intensity {
        if opts.exclude_ram_org {
            out.push_str(&format!("BLINK_STATE EQU RESULT+{}\n", var_offset)); var_offset += 1;
        } else {
            out.push_str("BLINK_STATE: FCB 0\n");
        }
    }
    // Shared trig tables (emit only if used)
    if module_uses_trig(module) {
        out.push_str("; Trig tables (shared)\n");
        emit_trig_tables(&mut out, "FDB");
    }
    // Emit blink helper & debug draw last (code section earlier referenced them)
    if do_blink {
    out.push_str("; Blink intensity helper (toggles two intensity levels each call)\nVECTREX_BLINK_INT:\n    LDA BLINK_STATE\n    EORA #1\n    STA BLINK_STATE\n    BEQ BLINK_LOW\n    LDA #$5F\n    BRA BLINK_SET\nBLINK_LOW:\n    LDA #$20\nBLINK_SET:\n    JSR Intensity_a\n    RTS\n");
    }
    if opts.debug_init_draw {
    out.push_str("; Debug initial draw: tiny horizontal line to prove we left title screen\nVECTREX_DEBUG_DRAW:\n    JSR DP_to_C8\n    LDU #DBG_INIT_LINE\n    LDA #$4F\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDBG_INIT_LINE: FCB $80,$10\n");
    }
    // Bank padding: ensure final size reaches opts.bank_size by filling with $FF.
    if opts.bank_size > 0 {
        out.push_str(&format!("; Bank padding to {} bytes (fill with $FF)\n", opts.bank_size));
        out.push_str(&format!("    IF * < ${:04X}\n", opts.bank_size));
        out.push_str("PADSIZE SET ");
        out.push_str(&format!("${:04X}-*\n", opts.bank_size));
        out.push_str("    FILL $FF,PADSIZE\n");
        out.push_str("    ENDC\n");
    }
    // Touch var_offset so compiler sees it used when EQU mode enabled
    #[allow(unused_variables)]
    { let _vo = var_offset; }
    out
}

// Detect usage of sin/cos/tan (any alias) in the module to decide if tables are needed
fn module_uses_trig(module: &Module) -> bool {
    for item in &module.items {
        if let Item::Function(f) = item {
            for s in &f.body { if stmt_has_trig(s) { return true; } }
        }
    }
    false
}

fn expr_has_trig(e: &Expr) -> bool {
    match e {
        Expr::Call { name, .. } => {
            let u = name.to_ascii_lowercase();
            u == "sin" || u == "cos" || u == "tan" || u == "math.sin" || u == "math.cos" || u == "math.tan"
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => expr_has_trig(left) || expr_has_trig(right),
        Expr::Not(inner) | Expr::BitNot(inner) => expr_has_trig(inner),
        _ => false,
    }
}

fn stmt_has_trig(s: &Stmt) -> bool {
    match s {
        Stmt::Assign { value, .. } => expr_has_trig(value),
        Stmt::Let { value, .. } => expr_has_trig(value),
        Stmt::Expr(e) => expr_has_trig(e),
        Stmt::For { start, end, step, body, .. } => expr_has_trig(start) || expr_has_trig(end) || step.as_ref().map(|e| expr_has_trig(e)).unwrap_or(false) || body.iter().any(stmt_has_trig),
        Stmt::While { cond, body } => expr_has_trig(cond) || body.iter().any(stmt_has_trig),
        Stmt::If { cond, body, elifs, else_body } => expr_has_trig(cond) || body.iter().any(stmt_has_trig) || elifs.iter().any(|(c,b)| expr_has_trig(c) || b.iter().any(stmt_has_trig)) || else_body.as_ref().map(|eb| eb.iter().any(stmt_has_trig)).unwrap_or(false),
        Stmt::Return(o) => o.as_ref().map(expr_has_trig).unwrap_or(false),
        Stmt::Switch { expr, cases, default } => expr_has_trig(expr) || cases.iter().any(|(ce, cb)| expr_has_trig(ce) || cb.iter().any(stmt_has_trig)) || default.as_ref().map(|db| db.iter().any(stmt_has_trig)).unwrap_or(false),
        Stmt::Break | Stmt::Continue => false,
    }
}

fn compute_max_args_used(module: &Module) -> usize {
    let mut maxa = 0usize;
    for item in &module.items {
        if let Item::Function(f) = item {
            for s in &f.body { maxa = maxa.max(scan_stmt_args(s)); }
        }
    }
    maxa
}

fn scan_stmt_args(s: &Stmt) -> usize {
    match s {
        Stmt::Assign { value, .. } | Stmt::Let { value, .. } | Stmt::Expr(value) => scan_expr_args(value),
        Stmt::For { start, end, step, body, .. } => {
            let mut m = scan_expr_args(start).max(scan_expr_args(end));
            if let Some(se) = step { m = m.max(scan_expr_args(se)); }
            for st in body { m = m.max(scan_stmt_args(st)); }
            m
        }
        Stmt::While { cond, body } => {
            let mut m = scan_expr_args(cond);
            for st in body { m = m.max(scan_stmt_args(st)); }
            m
        }
        Stmt::If { cond, body, elifs, else_body } => {
            let mut m = scan_expr_args(cond);
            for st in body { m = m.max(scan_stmt_args(st)); }
            for (c, b) in elifs { m = m.max(scan_expr_args(c)); for st in b { m = m.max(scan_stmt_args(st)); } }
            if let Some(eb) = else_body { for st in eb { m = m.max(scan_stmt_args(st)); } }
            m
        }
        Stmt::Return(o) => o.as_ref().map(scan_expr_args).unwrap_or(0),
        Stmt::Switch { expr, cases, default } => {
            let mut m = scan_expr_args(expr);
            for (ce, cb) in cases { m = m.max(scan_expr_args(ce)); for st in cb { m = m.max(scan_stmt_args(st)); } }
            if let Some(db) = default { for st in db { m = m.max(scan_stmt_args(st)); } }
            m
        }
        Stmt::Break | Stmt::Continue => 0,
    }
}

fn scan_expr_args(e: &Expr) -> usize {
    match e {
        Expr::Call { args, .. } => args.len().min(5).max(args.iter().map(scan_expr_args).max().unwrap_or(0)),
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => scan_expr_args(left).max(scan_expr_args(right)),
        Expr::Not(inner) | Expr::BitNot(inner) => scan_expr_args(inner),
        _ => 0,
    }
}

use std::collections::HashSet;

#[derive(Default)]
struct RuntimeUsage {
    needs_mul_helper: bool,
    needs_div_helper: bool,
    needs_tmp_left: bool,
    needs_tmp_right: bool,
    needs_tmp_ptr: bool,
    needs_line_vars: bool,
    needs_vcur_vars: bool,
    wrappers_used: HashSet<String>,
}

fn analyze_runtime_usage(module: &Module) -> RuntimeUsage {
    let mut usage = RuntimeUsage::default();
    for item in &module.items {
        if let Item::Function(f) = item {
            for s in &f.body { scan_stmt_runtime(s, &mut usage, &f.body); }
        }
    }
    // Derive grouped variable needs from wrappers
    if usage.wrappers_used.contains("VECTREX_DRAW_LINE") || usage.wrappers_used.contains("VECTREX_DRAW_VL") {
        usage.needs_line_vars = true;
    }
    if usage.wrappers_used.contains("VECTREX_MOVE_TO") || usage.wrappers_used.contains("VECTREX_DRAW_TO") {
        usage.needs_vcur_vars = true;
    }
    usage
}

fn scan_stmt_runtime(s: &Stmt, usage: &mut RuntimeUsage, fn_body: &Vec<Stmt>) {
    match s {
        Stmt::Assign { value, .. } => { usage.needs_tmp_ptr = true; scan_expr_runtime(value, usage); },
        Stmt::Let { value, .. } => scan_expr_runtime(value, usage),
        Stmt::Expr(value) => scan_expr_runtime(value, usage),
        Stmt::For { start, end, step, body, .. } => {
            scan_expr_runtime(start, usage);
            scan_expr_runtime(end, usage);
            if let Some(se) = step { scan_expr_runtime(se, usage); }
            for st in body { scan_stmt_runtime(st, usage, fn_body); }
        }
        Stmt::While { cond, body } => { scan_expr_runtime(cond, usage); for st in body { scan_stmt_runtime(st, usage, fn_body); } }
        Stmt::If { cond, body, elifs, else_body } => {
            scan_expr_runtime(cond, usage);
            for st in body { scan_stmt_runtime(st, usage, fn_body); }
            for (c, b) in elifs { scan_expr_runtime(c, usage); for st in b { scan_stmt_runtime(st, usage, fn_body); } }
            if let Some(eb) = else_body { for st in eb { scan_stmt_runtime(st, usage, fn_body); } }
        }
        Stmt::Return(o) => { if let Some(e) = o { scan_expr_runtime(e, usage); } }
        Stmt::Switch { expr, cases, default } => {
            scan_expr_runtime(expr, usage);
            for (ce, cb) in cases { scan_expr_runtime(ce, usage); for st in cb { scan_stmt_runtime(st, usage, fn_body); } }
            if let Some(db) = default { for st in db { scan_stmt_runtime(st, usage, fn_body); } }
            usage.needs_tmp_left = true; usage.needs_tmp_right = true; // switch lowering uses TMPLEFT
        }
        Stmt::Break | Stmt::Continue => {}
    }
}

fn scan_expr_runtime(e: &Expr, usage: &mut RuntimeUsage) {
    match e {
        Expr::Binary { op, left, right } => {
            // Only mark if not optimized away (non power-of-two cases handled later)
            match op {
                BinOp::Mul => { usage.needs_mul_helper = true; }
                BinOp::Div | BinOp::Mod => { usage.needs_div_helper = true; }
                _ => {}
            }
            usage.needs_tmp_left = true; usage.needs_tmp_right = true; // general binary op temps
            scan_expr_runtime(left, usage);
            scan_expr_runtime(right, usage);
        }
        Expr::Call { name, args } => { 
            // Track wrapper usage (normalize like emit_builtin_call)
            let up = name.to_ascii_uppercase();
            let resolved = match up.as_str() {
                "PRINT_TEXT" => Some("VECTREX_PRINT_TEXT"),
                "MOVE_TO" => Some("VECTREX_MOVE_TO"),
                "DRAW_TO" => Some("VECTREX_DRAW_TO"),
                "DRAW_LINE" => Some("VECTREX_DRAW_LINE"),
                "DRAW_VL" => Some("VECTREX_DRAW_VL"),
                "FRAME_BEGIN" => Some("VECTREX_FRAME_BEGIN"),
                "SET_ORIGIN" => Some("VECTREX_SET_ORIGIN"),
                "SET_INTENSITY" => Some("VECTREX_SET_INTENSITY"),
                "WAIT_RECAL" => Some("VECTREX_WAIT_RECAL"),
                "PLAY_MUSIC1" => Some("VECTREX_PLAY_MUSIC1"),
                "DBG_STATIC_VL" => Some("VECTREX_DBG_STATIC_VL"),
                _ if up.starts_with("VECTREX_") => Some(up.as_str()),
                _ => None
            };
            if let Some(r) = resolved { usage.wrappers_used.insert(r.to_string()); }
            for a in args { scan_expr_runtime(a, usage); }
        }
        Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
            scan_expr_runtime(left, usage);
            scan_expr_runtime(right, usage);
            usage.needs_tmp_left = true; usage.needs_tmp_right = true;
        }
        Expr::Not(inner) | Expr::BitNot(inner) => scan_expr_runtime(inner, usage),
        _ => {}
    }
}

// emit_function: outputs code for a function.
fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
    out.push_str(&format!("{}: ; function\n", f.name.to_uppercase()));
    out.push_str(&format!("; --- function {} ---\n", f.name));
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local
    if frame_size > 0 { out.push_str(&format!("    LEAS -{} ,S ; allocate locals\n", frame_size)); }
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDD VAR_ARG{}\n    STD VAR_{}\n", i, p.to_uppercase()));
    }
    let fctx = FuncCtx { locals: locals.clone(), frame_size };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map, opts); }
    if !matches!(f.body.last(), Some(Stmt::Return(_))) {
        if frame_size > 0 { out.push_str(&format!("    LEAS {} ,S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push_str("\n");
}

// emit_builtin_helpers: simple placeholder wrappers for Vectrex intrinsics.
fn emit_builtin_helpers(out: &mut String, usage: &RuntimeUsage) {
    out.push_str("; --- Vectrex built-in wrappers ---\n");
    let w = &usage.wrappers_used;
    // Emit vector phase begin unconditionally to avoid undefined symbol if optimizer misses usage
    out.push_str(
    "VECTREX_VECTOR_PHASE_BEGIN:\n    ; Cambia a DP=$C8 para rutinas de lista de vectores y recentra.\n    JSR DP_to_C8\n    JSR Reset0Ref\n    RTS\n"
    );
    // Emit debug static vector list drawer unconditionally (used for diagnostics)
    out.push_str(
    "VECTREX_DBG_STATIC_VL:\n    ; Draw static debug vector list (one horizontal line).\n    JSR DP_to_C8\n    LDU #DBG_STATIC_LIST\n    LDA #$5F\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDBG_STATIC_LIST:\n    FCB $80,$20 ; end bit set, dy=0, dx=32\n"
    );
    // Always emit silence helper; cheap.
    out.push_str(
    "VECTREX_SILENCE:\n    ; Comprehensive AY silence: zero tone periods (0-5), noise (6), mixer (7=0x3F), vols (8-10).\n    LDA #0\n    STA $D001 ; reg 0 select\n    CLR $D000 ; tone A coarse/low (write twice for 0 & 1)\n    LDA #1\n    STA $D001\n    CLR $D000\n    LDA #2\n    STA $D001\n    CLR $D000 ; tone B low\n    LDA #3\n    STA $D001\n    CLR $D000 ; tone B high\n    LDA #4\n    STA $D001\n    CLR $D000 ; tone C low\n    LDA #5\n    STA $D001\n    CLR $D000 ; tone C high\n    LDA #6\n    STA $D001\n    CLR $D000 ; noise period\n    LDA #7\n    STA $D001\n    LDA #$3F ; disable tone+noise all channels\n    STA $D000\n    LDA #8\n    STA $D001\n    CLR $D000 ; vol A\n    LDA #9\n    STA $D001\n    CLR $D000 ; vol B\n    LDA #10\n    STA $D001\n    CLR $D000 ; vol C\n    RTS\n"
    );
    if w.contains("VECTREX_PRINT_TEXT") {
        out.push_str(
            "VECTREX_PRINT_TEXT:\n    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS\n    LDU VAR_ARG2   ; string pointer (high-bit terminated)\n    LDA VAR_ARG1+1 ; Y\n    LDB VAR_ARG0+1 ; X\n    JSR Print_Str_d\n    RTS\n"
        );
    }
    if w.contains("VECTREX_MOVE_TO") {
        out.push_str(
            "VECTREX_MOVE_TO:\n    LDA VAR_ARG1+1 ; Y\n    LDB VAR_ARG0+1 ; X\n    JSR Moveto_d\n    ; store new current position\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("VECTREX_DRAW_TO") {
        out.push_str(
            "; TODO: implement DRAW_TO using vector list (Draw_VL) or incremental delta steps.\nVECTREX_DRAW_TO:\n    ; update current position (no actual drawing yet)\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("VECTREX_DRAW_LINE") {
        out.push_str(
            "; Draw single line using vector list. Args: (x0,y0,x1,y1,intensity) low bytes.\n; Assumes WAIT_RECAL already left DP at $D0. Only switches to $C8 for Draw_VL.\nVECTREX_DRAW_LINE:\n    ; Set intensity\n    LDA VAR_ARG4+1\n    JSR Intensity_a\n    LDA VAR_ARG1+1\n    LDB VAR_ARG0+1\n    JSR Moveto_d\n    ; Compute deltas (end - start) using low bytes\n    LDA VAR_ARG2+1\n    SUBA VAR_ARG0+1\n    STA VLINE_DX\n    LDA VAR_ARG3+1\n    SUBA VAR_ARG1+1\n    STA VLINE_DY\n    ; Clamp to +/-63\n    LDA VLINE_DX\n    CMPA #63\n    BLE VLX_OK_HI\n    LDA #63\nVLX_OK_HI: CMPA #-64\n    BGE VLX_OK_LO\n    LDA #-64\nVLX_OK_LO: STA VLINE_DX\n    LDA VLINE_DY\n    CMPA #63\n    BLE VLY_OK_HI\n    LDA #63\nVLY_OK_HI: CMPA #-64\n    BGE VLY_OK_LO\n    LDA #-64\nVLY_OK_LO: STA VLINE_DY\n    ; Build 2-byte vector list (Y|endbit, X)\n    LDA VLINE_DY\n    ORA #$80\n    STA VLINE_LIST\n    LDA VLINE_DX\n    STA VLINE_LIST+1\n    ; Switch to vector DP and draw, no restore (next WAIT_RECAL resets)\n    JSR DP_to_C8\n    LDU #VLINE_LIST\n    JSR Draw_VL\n    RTS\n"
        );
    }
    if w.contains("VECTREX_FRAME_BEGIN") {
        out.push_str(
            "VECTREX_FRAME_BEGIN:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    JSR Reset0Ref\n    RTS\n"
        );
    }
    if w.contains("VECTREX_DRAW_VL") {
        out.push_str(
            "VECTREX_DRAW_VL:\n    LDU VAR_ARG0\n    LDA VAR_ARG1+1\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\n"
        );
    }
    if w.contains("VECTREX_SET_ORIGIN") {
    out.push_str("VECTREX_SET_ORIGIN:\n    JSR Reset0Ref\n    RTS\n");
    }
    if w.contains("VECTREX_SET_INTENSITY") {
    out.push_str("VECTREX_SET_INTENSITY:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    RTS\n");
    }
    if w.contains("VECTREX_WAIT_RECAL") {
        out.push_str("VECTREX_WAIT_RECAL:\n    JSR WAIT_RECAL\n    RTS\n");
    }
    if w.contains("VECTREX_PLAY_MUSIC1") {
        // Simple wrapper to restart the default MUSIC1 tune each frame or once. BIOS expects U to point to music data table at (?), but calling MUSIC1 vector reinitializes tune.
        out.push_str("VECTREX_PLAY_MUSIC1:\n    JSR MUSIC1\n    RTS\n");
    }
    // Trig tables are emitted later in data section.
}

// emit_builtin_call: inline lowering for intrinsic names; returns true if handled
fn emit_builtin_call(name: &str, args: &Vec<Expr>, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) -> bool {
    let up = name.to_ascii_uppercase();
    let is = matches!(up.as_str(),
        "VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"VECTREX_DRAW_LINE"|"VECTREX_DRAW_VL"|"VECTREX_FRAME_BEGIN"|"VECTREX_VECTOR_PHASE_BEGIN"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"|"VECTREX_WAIT_RECAL"|
    "VECTREX_PLAY_MUSIC1"|
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|
        "ABS"|"MATH_ABS"|"MIN"|"MATH_MIN"|"MAX"|"MATH_MAX"|"CLAMP"|"MATH_CLAMP"
    );
    if !is {
        // Backward compatibility: map legacy short names to vectrex-prefixed versions
        let translated = match up.as_str() {
            "PRINT_TEXT" => Some("VECTREX_PRINT_TEXT"),
            "MOVE_TO" => Some("VECTREX_MOVE_TO"),
            "DRAW_TO" => Some("VECTREX_DRAW_TO"),
            "DRAW_LINE" => Some("VECTREX_DRAW_LINE"),
            "DRAW_VL" => Some("VECTREX_DRAW_VL"),
            "FRAME_BEGIN" => Some("VECTREX_FRAME_BEGIN"),
            "VECTOR_PHASE_BEGIN" => Some("VECTREX_VECTOR_PHASE_BEGIN"),
            "SET_ORIGIN" => Some("VECTREX_SET_ORIGIN"),
            "SET_INTENSITY" => Some("VECTREX_SET_INTENSITY"),
            "WAIT_RECAL" => Some("VECTREX_WAIT_RECAL"),
            "PLAY_MUSIC1" => Some("VECTREX_PLAY_MUSIC1"),
            "DBG_STATIC_VL" => Some("VECTREX_DBG_STATIC_VL"),
            _ => None
        };
        if let Some(new_up) = translated {
            // Re-dispatch using new name recursively (avoid infinite loop by guarding is set)
            return emit_builtin_call(new_up, args, out, fctx, string_map, opts);
        }
        return false;
    }
    // ABS
    if matches!(up.as_str(), "ABS"|"MATH_ABS") {
    if let Some(arg) = args.get(0) { emit_expr(arg, out, fctx, string_map, opts); } else { out.push_str("    LDD #0\n    STD RESULT\n"); return true; }
        let done = fresh_label("ABS_DONE");
        out.push_str(&format!("    LDD RESULT\n    TSTA\n    BPL {}\n    COMA\n    COMB\n    ADDD #1\n{}: STD RESULT\n", done, done));
        return true;
    }
    // MIN(a,b)
    if matches!(up.as_str(), "MIN"|"MATH_MIN") {
        if args.len() < 2 { out.push_str("    LDD #0\n    STD RESULT\n"); return true; }
    emit_expr(&args[0], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
    emit_expr(&args[1], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
        let use_right = fresh_label("MIN_USE_R");
        let done = fresh_label("MIN_DONE");
        out.push_str(&format!("    LDD TMPLEFT\n    SUBD TMPRIGHT\n    BGT {}\n    LDD TMPLEFT\n    BRA {}\n{}: LDD TMPRIGHT\n{}: STD RESULT\n", use_right, done, use_right, done));
        return true;
    }
    // MAX(a,b)
    if matches!(up.as_str(), "MAX"|"MATH_MAX") {
        if args.len() < 2 { out.push_str("    LDD #0\n    STD RESULT\n"); return true; }
    emit_expr(&args[0], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
    emit_expr(&args[1], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
        let use_right = fresh_label("MAX_USE_R");
        let done = fresh_label("MAX_DONE");
        out.push_str(&format!("    LDD TMPLEFT\n    SUBD TMPRIGHT\n    BLT {}\n    LDD TMPLEFT\n    BRA {}\n{}: LDD TMPRIGHT\n{}: STD RESULT\n", use_right, done, use_right, done));
        return true;
    }
    // CLAMP(v, lo, hi)
    if matches!(up.as_str(), "CLAMP"|"MATH_CLAMP") {
        if args.len() < 3 { out.push_str("    LDD #0\n    STD RESULT\n"); return true; }
        // v
    emit_expr(&args[0], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
        // lo
    emit_expr(&args[1], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
        // hi -> reuse DIV_A
    emit_expr(&args[2], out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n    STD DIV_A\n");
        let use_lo = fresh_label("CLAMP_USE_LO");
        let check_hi = fresh_label("CLAMP_CHECK_HI");
        let use_hi = fresh_label("CLAMP_USE_HI");
        let done = fresh_label("CLAMP_DONE");
        out.push_str(&format!(
            "    LDD TMPLEFT\n    SUBD TMPRIGHT\n    BLT {}\n    BRA {}\n{}: LDD TMPRIGHT\n    BRA {}\n{}: LDD TMPLEFT\n    SUBD DIV_A\n    BGT {}\n    LDD TMPLEFT\n    BRA {}\n{}: LDD DIV_A\n{}: STD RESULT\n",
            use_lo, check_hi, use_lo, done, check_hi, use_hi, done, use_hi, done
        ));
        return true;
    }
    // Trig functions
    if matches!(up.as_str(), "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN") {
        // Expect 1 arg
        if let Some(arg) = args.get(0) {
            emit_expr(arg, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n    ANDB #$7F\n    CLRA\n    ASLB\n    ROLA\n    LDX #SIN_TABLE\n");
            if up.ends_with("COS") { out.push_str("    LDX #COS_TABLE\n"); }
            if up.ends_with("TAN") { out.push_str("    LDX #TAN_TABLE\n"); }
            out.push_str("    ABX\n    LDD ,X\n    STD RESULT\n");
            return true;
        }
        // No arg: return 0
        out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
        return true;
    }
    for (i, a) in args.iter().enumerate() {
        if i >= 5 { break; }
    emit_expr(a, out, fctx, string_map, opts);
        out.push_str("    LDD RESULT\n");
        out.push_str(&format!("    STD VAR_ARG{}\n", i));
    }
    if opts.force_extended_jsr { out.push_str(&format!("    JSR >{}\n", up)); } else { out.push_str(&format!("    JSR {}\n", up)); }
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
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_expr(value, out, fctx, string_map, opts);
            if let Some(off) = fctx.offset_of(target) {
                out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off));
            } else {
                // Global/store: load 16-bit value from RESULT into X then store via U pointer
                out.push_str(&format!("    LDX RESULT\n    LDU #VAR_{}\n    STU TMPPTR\n    STX ,U\n", target.to_uppercase()));
            }
        }
        Stmt::Let { name, value } => {
            emit_expr(value, out, fctx, string_map, opts);
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off)); }
        }
    Stmt::Expr(e) => emit_expr(e, out, fctx, string_map, opts),
        Stmt::Return(o) => {
            if let Some(e) = o { emit_expr(e, out, fctx, string_map, opts); }
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
            emit_expr(cond, out, fctx, string_map, opts);
            // Long branch to end
            out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map, opts); }
            out.push_str(&format!("    LBRA {}\n{}: ; while end\n", ls, le));
        }
        Stmt::For { var, start, end, step, body } => {
            let ls = fresh_label("FOR");
            let le = fresh_label("FOR_END");
            emit_expr(start, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n");
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    STD {} ,S\n", off)); }
            else { out.push_str(&format!("    STD VAR_{}\n", var.to_uppercase())); }
            out.push_str(&format!("{}: ; for loop\n", ls));
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDD {} ,S\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n", var.to_uppercase())); }
            emit_expr(end, out, fctx, string_map, opts);
            out.push_str("    LDX RESULT\n    CPD RESULT\n");
            out.push_str(&format!("    LBCC {}\n", le)); // unsigned >= end => exit
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map, opts); }
            if let Some(se) = step {
                emit_expr(se, out, fctx, string_map, opts);
                out.push_str("    LDX RESULT\n");
            } else {
                out.push_str("    LDX #1\n");
            }
            if let Some(off) = fctx.offset_of(var) { out.push_str(&format!("    LDD {} ,S\n    ADDD ,X\n    STD {} ,S\n", off, off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n    ADDD ,X\n    STD VAR_{}\n", var.to_uppercase(), var.to_uppercase())); }
            out.push_str(&format!("    LBRA {}\n{}: ; for end\n", ls, le));
        }
        Stmt::If { cond, body, elifs, else_body } => {
            let end = fresh_label("IF_END");
            let mut next = fresh_label("IF_NEXT");
            let simple_if = elifs.is_empty() && else_body.is_none();
            emit_expr(cond, out, fctx, string_map, opts);
            out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", next));
            for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
            out.push_str(&format!("    LBRA {}\n", end));
            for (i, (c, b)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(c, out, fctx, string_map, opts);
                out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", new_next));
                for s in b { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
                out.push_str(&format!("    LBRA {}\n", end));
                next = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
            } else if !elifs.is_empty() || simple_if {
                out.push_str(&format!("{}:\n", next));
            }
            out.push_str(&format!("{}:\n", end));
        }
        Stmt::Switch { expr, cases, default } => {
            emit_expr(expr, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n    STD TMPLEFT ; switch value\n");
            let end = fresh_label("SW_END");
            let def_label = if default.is_some() { Some(fresh_label("SW_DEF")) } else { None };
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
                if span <= numeric_cases.len()*2 && span*2 <= 254 {
                    let table_label = fresh_label("SW_JT");
                    use std::collections::BTreeMap;
                    let mut label_map: BTreeMap<i32,String> = BTreeMap::new();
                    for (val, _) in &numeric_cases { label_map.insert(*val & 0xFFFF, fresh_label("SW_CASE")); }
                    out.push_str(&format!("    LDD TMPLEFT\n    SUBD #{}\n    LBLT {}\n", min, def_label.as_ref().unwrap_or(&end)));
                    out.push_str(&format!("    CPD #{}\n    LBHI {}\n", span as i32 - 1, def_label.as_ref().unwrap_or(&end)));
                    out.push_str("    ASLB\n    ROLA\n");
                    out.push_str(&format!("    LDX #{}\n    ABX\n", table_label));
                    out.push_str("    LDD ,X\n    TFR D,X\n    JMP ,X\n");
                    for (val, body) in &numeric_cases {
                        let lbl = label_map.get(&(*val & 0xFFFF)).unwrap();
                        out.push_str(&format!("{}:\n", lbl));
                        for s in *body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
                        out.push_str(&format!("    LBRA {}\n", end));
                    }
                    if let Some(dl) = &def_label {
                        out.push_str(&format!("{}:\n", dl));
                        for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
                    }
                    out.push_str(&format!("{}:\n", end));
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
            let mut labels = Vec::new();
            for _ in cases { labels.push(fresh_label("SW_CASE")); }
            for ((cv,_), lbl) in cases.iter().zip(labels.iter()) {
                emit_expr(cv, out, fctx, string_map, opts);
                out.push_str("    LDD RESULT\n    SUBD TMPLEFT\n    LBEQ ");
                out.push_str(lbl);
                out.push_str("\n");
            }
            if let Some(dl) = &def_label { out.push_str(&format!("    LBRA {}\n", dl)); } else { out.push_str(&format!("    LBRA {}\n", end)); }
            for ((_, body), lbl) in cases.iter().zip(labels.iter()) {
                out.push_str(&format!("{}:\n", lbl));
                for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
                out.push_str(&format!("    LBRA {}\n", end));
            }
            if let Some(dl) = def_label {
                out.push_str(&format!("{}:\n", dl));
                for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map, opts); }
            }
            out.push_str(&format!("{}:\n", end));
        }
    }
}

// emit_expr: lower expressions; result placed in RESULT.
// Nota: En 6809 las operaciones sobre D ya limitan a 16 bits; no hace falta 'mask' explícito.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
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
            if emit_builtin_call(name, args, out, fctx, string_map, opts) { return; }
            for (i, arg) in args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr(arg, out, fctx, string_map, opts);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            if opts.force_extended_jsr { out.push_str(&format!("    JSR >{}\n", name.to_uppercase())); } else { out.push_str(&format!("    JSR {}\n", name.to_uppercase())); }
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr(left, out, fctx, string_map, opts);
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
                    emit_expr(left, out, fctx, string_map, opts);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr(right, out, fctx, string_map, opts);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr(left, out, fctx, string_map, opts);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr(left, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx, string_map, opts);
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
            emit_expr(inner, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr(left, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr(right, out, fctx, string_map, opts);
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
                emit_expr(left, out, fctx, string_map, opts);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr(right, out, fctx, string_map, opts);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr(left, out, fctx, string_map, opts);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr(right, out, fctx, string_map, opts);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr(inner, out, fctx, string_map, opts);
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
