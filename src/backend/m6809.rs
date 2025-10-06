// (Removed duplicated legacy block above during refactor)
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::collect_string_literals;
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};
use std::sync::atomic::{AtomicBool, Ordering};

static LAST_END_SET: AtomicBool = AtomicBool::new(false);

// Helper function to map legacy function names to their modern counterparts
fn resolve_function_name(name: &str) -> Option<String> {
    match name {
        "PRINT_TEXT" => Some("VECTREX_PRINT_TEXT".to_string()),
        "MOVE" => Some("VECTREX_MOVE_TO".to_string()),        // Unificado: MOVE -> VECTREX_MOVE_TO
        "MOVE_TO" => Some("VECTREX_MOVE_TO".to_string()),     // Compatibilidad hacia atrás
        "DRAW_TO" => Some("VECTREX_DRAW_TO".to_string()),
        // DRAW_LINE -> removed from auto-mapping, handled by inline optimization or explicit wrapper call
        "DRAW_POLYGON" => Some("DRAW_POLYGON".to_string()),   // already handled if constants; allow pass-through if dynamic (future)
        "DRAW_VL" => Some("VECTREX_DRAW_VL".to_string()),
        "FRAME_BEGIN" => Some("VECTREX_FRAME_BEGIN".to_string()),
        "VECTOR_PHASE_BEGIN" => Some("VECTREX_VECTOR_PHASE_BEGIN".to_string()),
        "SET_ORIGIN" => Some("VECTREX_SET_ORIGIN".to_string()),
        "SET_INTENSITY" => Some("VECTREX_SET_INTENSITY".to_string()),
        "WAIT_RECAL" => Some("VECTREX_WAIT_RECAL".to_string()),
        "PLAY_MUSIC1" => Some("VECTREX_PLAY_MUSIC1".to_string()),
        "DBG_STATIC_VL" => Some("VECTREX_DBG_STATIC_VL".to_string()),
        name if name.starts_with("VECTREX_") => Some(name.to_string()),
        _ => None
    }
}

// emit: entry point for Motorola 6809 backend assembly generation.
// Produces a simple Vectrex-style header, calls platform init + MAIN, then infinite loop.
pub fn emit(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let mut out = String::new();
    let syms = collect_symbols(module);
    let string_map = collect_string_literals(module);
        let rt_usage = analyze_runtime_usage(module);
    
    // Locate required 'main' and 'loop' functions
    let mut user_main: Option<&Function> = None;
    let mut user_loop: Option<&Function> = None;
    
    for item in &module.items { 
        if let Item::Function(f) = item { 
            if f.name.eq_ignore_ascii_case("main") { 
                user_main = Some(f); 
            } else if f.name.eq_ignore_ascii_case("loop") { 
                user_loop = Some(f); 
            }
        } 
    }
    
    // Validate that both required functions exist and emit errors in assembly
    if user_main.is_none() {
        out.push_str("; ERROR: Missing required function 'def main():'\n");
        out.push_str("; The main() function is required for initialization code that runs once at startup.\n");
        out.push_str("        ; Compilation failed - please add def main(): function\n");
        return out;
    }
    if user_loop.is_none() {
        out.push_str("; ERROR: Missing required function 'def loop():'\n");
        out.push_str("; The loop() function is required and contains code that runs every frame (60 FPS).\n");
        out.push_str("        ; Compilation failed - please add def loop(): function\n");
        return out;
    }
    
    // Track whether we ended up inlining 'main'
    let main_inlined = false; // not currently toggled (kept for future inlining logic)
    // Initially suppress runtime only if main was inlined (will be re-evaluated later)
    let mut suppress_runtime = main_inlined;
    // Detect if any vector list already carries intensity commands; if so skip per-frame Intensity_5F
    // NOTE: previously used to skip per-frame Intensity_5F; currently unused. If reinstated, re-enable.
    // let vectorlists_have_intensity = module.items.iter().any(|it| matches!(it, Item::VectorList { .. }));
    // Origin is fixed at $0000 for Vectrex cartridge space. Using a configurable origin caused
    // loader mismatches with the emulator; keep this constant and adjust the emulator loader base
    // instead of relocating here.
    out.push_str(&format!("; --- Motorola 6809 backend ({}) title='{}' origin=$0000 ---\n", ti.name, opts.title));
    out.push_str("        ORG $0000\n");
    out.push_str(";***************************************************************************\n; DEFINE SECTION\n;***************************************************************************\n");
    // Classic include; no manual EQU needed.
        out.push_str("    INCLUDE \"include/VECTREX.I\"\n\n");
    // (Include already added above)
    out.push_str(";***************************************************************************\n; HEADER SECTION\n;***************************************************************************\n");
    // Header (emulator-compatible variant):
    //  - 'g GCE 1998' + $80 (year per reference example)
    //  - music pointer (word) (set 0 if no custom music)
    //  - height, width, rel y, rel x
    //  - title bytes (plain ASCII, sanitized, length<=24)
    //  - $80 terminator for title
    //  - reserved 0 byte
    //  - pad with zeros to $0030
    // Legacy header year to match classic examples
    // Resolve meta overrides
    // Canonical Vectrex cart signature must start with 'g GCE 1982' so BIOS detects cartridge.
    // Allow override, but warn (implicitly) that changing it may break detection.
    let copyright = module.meta.copyright_override.as_deref().unwrap_or("g GCE 1982");
    let music_raw = module.meta.music_override.as_deref().unwrap_or("music1");
    // Force exact lowercase 'g' prefix required by BIOS heuristic.
    let canonical_prefix = "g GCE 1982";
    let mut effective_copy = copyright.to_string();
    if !effective_copy.starts_with(canonical_prefix) {
        // Replace only the starting segment, keep rest if any.
        effective_copy = canonical_prefix.to_string();
    }
    let music_fdb = if music_raw == "0" { String::from("$0000") } else { music_raw.to_string() };
    // Single classic header only
    out.push_str(&format!("    FCC \"{}\"\n    FCB $80\n    FDB {}\n    FCB $F8,$50,$20,-$45\n", effective_copy, music_fdb));
    let mut title = opts.title.to_uppercase();
    if title.len() > 24 { title.truncate(24); }
    title = title.chars().map(|c| if c.is_ascii_alphanumeric() || c==' ' { c } else { ' ' }).collect();
    if title.is_empty() { title.push(' '); }
    out.push_str(&format!("    FCC \"{}\"\n", title));
    out.push_str("    FCB $80\n    FCB 0\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    // Jump over code to START like smartlist_demo pattern
    out.push_str("    JMP START\n\n");
    out.push_str("START:\n    LDA #$80\n    STA VIA_t1_cnt_lo\n    LDX #Vec_Default_Stk\n    TFR X,S\n\n");
    // No explicit init routine defined yet for Vectrex; skip calling ti.init_label if undefined.
    // Execution falls through to MAIN directly.
    // Entry sequence: call MAIN then loop forever (Vectrex BIOS expects cartridge not to return).
    // Precompute flags
    let do_blink = opts.blink_intensity;
    let _do_per_frame_silence = opts.per_frame_silence;
    let jsr_ext = if opts.force_extended_jsr { ">" } else { "" };
    let _wait_recal_call = if opts.fast_wait { "JSR VECTREX_WAIT_RECAL" } else { &format!("JSR {}Wait_Recal", jsr_ext) };

    if opts.auto_loop {
        // Vectrex tutorial structure: main() code inline in START, loop() code inline in MAIN
        out.push_str("    ; *** DEBUG *** main() function code inline (initialization)\n");
        
        // Emit main() function body inline
        if let Some(main_func) = user_main {
            let fctx = FuncCtx { locals: Vec::new(), frame_size: 0 };
            for stmt in &main_func.body {
                emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts);
            }
        }
        
        out.push_str("\nMAIN:\n    JSR Wait_Recal\n    LDA #$D0\n    TFR A,DP\n    JSR Intensity_5F\n    JSR Reset0Ref\n");
        
        // Emit loop() function body inline
        if let Some(loop_func) = user_loop {
            out.push_str("    ; *** DEBUG *** loop() function code inline (executed every frame)\n");
            let fctx = FuncCtx { locals: Vec::new(), frame_size: 0 };
            for stmt in &loop_func.body {
                emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts);
            }
        }
        
        out.push_str("    BRA MAIN\n\n");
    } else {
        out.push_str("; Init without implicit loop (auto_loop disabled)\n");
    let intensity_init: String = if do_blink { "    JSR VECTREX_BLINK_INT\n".into() } else { format!("    JSR {}Intensity_5F\n", jsr_ext) };
    out.push_str(&format!("ENTRY_START: LDS #Vec_Default_Stk ; set default stack like BIOS examples\n    JSR {}Wait_Recal\n{}    JSR MAIN ; user initialization\n    JSR LOOP ; user loop\nHANG: BRA HANG\n\n", jsr_ext, intensity_init));
    }
    // Emit all functions so code exists (MAIN label will resolve).
    let mut global_mutables: Vec<(String,i32)> = Vec::new();
    use std::collections::BTreeSet;
    let mut emitted_consts: BTreeSet<String> = BTreeSet::new();
    for item in &module.items {
        match item {
            Item::Function(f) => {
                if opts.auto_loop && (f.name == "main" || f.name == "loop") {
                    // Skip main/loop functions in auto_loop mode - they're inlined in START/MAIN
                    continue;
                } else {
                    // Emit other functions normally
                    emit_function(f, &mut out, &string_map, opts);
                }
            }
                Item::VectorList { name, entries } => {
                    // Emit compact data-only vector list consumed by Run_VectorList.
                    // Format: count, then 'count' triples (y,x,cmd). For CMD_INT an extra intensity byte follows. Terminator triple CMD_END added automatically.
                    // Map: Move -> START, Rect -> START + 4 LINE, Polygon -> START + n LINE, Origin -> ZERO (Reset0Ref), Intensity -> INT (with mapped byte).
                    let label = format!("VL_{}", name.to_ascii_uppercase());
                    struct Cmd { y:i32, x:i32, cmd:u8, extra:Option<u8> }
                    let mut cmds: Vec<Cmd> = Vec::new();
                    for e in entries {
                        match e {
                            crate::ast::VlEntry::Intensity(v) => {
                                // Same friendly mapping as before
                                let mut val = (*v & 0xFF) as u8;
                                if (0..=7).contains(v) {
                                    val = match *v { 0|1 => 0x1F, 2 => 0x3F, 3 => 0x5F, 4..=7 => 0x7F, _ => 0x5F };
                                }
                                cmds.push(Cmd{ y:0, x:0, cmd:3, extra:Some(val) });
                            }
                            crate::ast::VlEntry::Origin => { cmds.push(Cmd{ y:0,x:0,cmd:4,extra:None}); }
                            crate::ast::VlEntry::Move(x,y) => { cmds.push(Cmd{ y:*y,x:*x,cmd:0,extra:None}); }
                            crate::ast::VlEntry::Rect(x1,y1,x2,y2) => {
                                cmds.push(Cmd{ y:*y1,x:*x1,cmd:0,extra:None});
                                let pts = [(*x1,*y1),(*x2,*y1),(*x2,*y2),(*x1,*y2)];
                                for w in 0..4 { let (sx,sy)=pts[w]; let (ex,ey)=pts[(w+1)%4]; cmds.push(Cmd{ y: (ey - sy), x:(ex - sx), cmd:1, extra:None}); }
                            }
                            crate::ast::VlEntry::Polygon(verts) => {
                                if verts.len()>1 {
                                    let (first_x, first_y) = verts[0];
                                    cmds.push(Cmd{ y:first_y, x:first_x, cmd:0, extra:None});
                                    for w in 0..verts.len() { let (sx,sy)=verts[w]; let (ex,ey)=verts[(w+1)%verts.len()]; cmds.push(Cmd{ y:(ey - sy), x:(ex - sx), cmd:1, extra:None}); }
                                }
                            }
                            crate::ast::VlEntry::Circle { cx, cy, r, segs } => {
                                use std::f64::consts::PI;
                                let n = (*segs).max(3) as usize;
                                let radius = *r as f64;
                                let mut verts: Vec<(i32,i32)> = Vec::new();
                                for k in 0..n { let ang = 2.0*PI*(k as f64)/(n as f64); let x = *cx as f64 + radius*ang.cos(); let y = *cy as f64 + radius*ang.sin(); verts.push((x.round() as i32, y.round() as i32)); }
                                if let Some((fx,fy))=verts.first().copied() { cmds.push(Cmd{ y:fy,x:fx,cmd:0,extra:None}); }
                                for i in 0..n { let (sx,sy)=verts[i]; let (ex,ey)=verts[(i+1)%n]; cmds.push(Cmd{ y:(ey-sy), x:(ex-sx), cmd:1, extra:None}); }
                            }
                            crate::ast::VlEntry::Arc { cx, cy, r, start_deg, sweep_deg, segs } => {
                                use std::f64::consts::PI;
                                let n = (*segs).max(2) as usize;
                                let radius = *r as f64;
                                let start = (*start_deg as f64) * PI / 180.0;
                                let sweep = (*sweep_deg as f64) * PI / 180.0;
                                let mut last: Option<(i32,i32)>=None;
                                for k in 0..n {
                                    let ang = start + sweep*(k as f64)/(n as f64 - 1.0);
                                    let x = *cx as f64 + radius*ang.cos();
                                    let y = *cy as f64 + radius*ang.sin();
                                    let xi = x.round() as i32; let yi = y.round() as i32;
                                    if k==0 { cmds.push(Cmd{ y:yi,x:xi,cmd:0,extra:None}); }
                                    if let Some((lx,ly))=last { cmds.push(Cmd{ y: yi-ly, x: xi-lx, cmd:1, extra:None}); }
                                    last=Some((xi,yi));
                                }
                            }
                            crate::ast::VlEntry::Spiral { cx, cy, r_start, r_end, turns, segs } => {
                                use std::f64::consts::PI;
                                let n = (*segs).max(4) as usize;
                                let rs = *r_start as f64; let re = *r_end as f64;
                                let total_ang = 2.0*PI*(*turns as f64);
                                let mut last: Option<(i32,i32)>=None;
                                for k in 0..n {
                                    let t = k as f64 / (n-1) as f64;
                                    let r = rs + (re - rs)*t;
                                    let ang = total_ang * t;
                                    let x = *cx as f64 + r*ang.cos();
                                    let y = *cy as f64 + r*ang.sin();
                                    let xi = x.round() as i32; let yi = y.round() as i32;
                                    if k==0 { cmds.push(Cmd{ y:yi,x:xi,cmd:0,extra:None}); }
                                    if let Some((lx,ly))=last { cmds.push(Cmd{ y: yi-ly, x: xi-lx, cmd:1, extra:None}); }
                                    last=Some((xi,yi));
                                }
                            }
                        }
                    }
                    // --- Post-processing cleanup (steps 1,2,3) ---
                    // 1. Remove duplicate consecutive START at same coords (MOVE followed by RECT/POLYGON emitting same START)
                    // 2. Drop leading ZERO (Origin) if immediately followed by a START (frame Reset0Ref already done each loop)
                    // 3. Collapse consecutive ZERO entries into one.
                    let mut cleaned: Vec<Cmd> = Vec::with_capacity(cmds.len());
                    for c in cmds.into_iter() {
                        if let Some(last) = cleaned.last() {
                            // Duplicate START at same absolute position
                            if c.cmd==0 && last.cmd==0 && c.x==last.x && c.y==last.y { continue; }
                            // Consecutive ZERO -> skip
                            if c.cmd==4 && last.cmd==4 { continue; }
                        }
                        cleaned.push(c);
                    }
                    // Drop initial ZERO if followed by START
                    if cleaned.len()>=2 && cleaned[0].cmd==4 && cleaned[1].cmd==0 { cleaned.remove(0); }
                    // Ensure an initial explicit center START like smartlist_demo (0,0) if first cmd is not START at (0,0)
                    if cleaned.first().map(|c| !(c.cmd==0 && c.x==0 && c.y==0)).unwrap_or(true) {
                        cleaned.insert(0, Cmd{ y:0,x:0,cmd:0,extra:None});
                    }
                    // Append END terminator
                    cleaned.push(Cmd{ y:0,x:0,cmd:2,extra:None});
                    // Move first INT (if any) immediately after the initial center START
                    let first_start_pos = cleaned.iter().position(|c| c.cmd==0).unwrap_or(0);
                    if let Some(int_pos) = cleaned.iter().position(|c| c.cmd==3) {
                        if int_pos > first_start_pos + 1 {
                            let int_cmd = cleaned.remove(int_pos);
                            cleaned.insert(first_start_pos+1, int_cmd);
                        }
                    }
                    let count = cleaned.len() as u8; // count excludes extra INT bytes
                    out.push_str(&format!("{}:\n    FCB {}\n", label, count));
                    let mut abs_x: i32 = 0; let mut abs_y: i32 = 0; // track absolute for comments
                    for c in cleaned {
                        let cmd_name = match c.cmd {0=>"CMD_START",1=>"CMD_LINE",2=>"CMD_END",3=>"CMD_INT",4=>"CMD_ZERO", _=>"?"};
                        // Compute signed 8-bit representations
                        let sy = ((c.y & 0xFF) as u8) as i8 as i32;
                        let sx = ((c.x & 0xFF) as u8) as i8 as i32;
                        let comment = match c.cmd {
                            0 => { // START absolute
                                abs_x = sx; abs_y = sy;
                                format!("; START to ({:+}, {:+})", abs_x, abs_y)
                            }
                            1 => { // LINE delta
                                abs_y += sy; abs_x += sx;
                                format!("; LINE dy={:+} dx={:+} -> ({:+}, {:+})", sy, sx, abs_x, abs_y)
                            }
                            3 => {
                                format!("; INTENSITY (next byte) at current ({:+}, {:+})", abs_x, abs_y)
                            }
                            4 => { // ZERO resets origin
                                abs_x = 0; abs_y = 0; 
                                "; ZERO (Reset0Ref) => origin (0,0)".to_string()
                            }
                            2 => "; END".to_string(),
                            _ => String::from("; ?")
                        };
                        out.push_str(&format!("    FCB ${:02X},${:02X},{} {}\n", (c.y & 0xFF), (c.x & 0xFF), cmd_name, comment));
                        if let Some(e) = c.extra { out.push_str(&format!("    FCB ${:02X} ; intensity value\n", e)); }
                    }
                }
            Item::Const { name, value } => {
                let up = name.to_uppercase();
                if !emitted_consts.contains(&up) {
                    if let Expr::Number(n) = value { out.push_str(&format!("{} EQU {}\n", up, n & 0xFFFF)); }
                    emitted_consts.insert(up);
                }
            }
            Item::GlobalLet { name, value } => {
                if let Expr::Number(n) = value { global_mutables.push((name.clone(), *n)); } else { global_mutables.push((name.clone(), 0)); }
            }
            Item::ExprStatement(_expr) => {
                // TODO: Handle top-level expression statements in m6809 backend
                // For now, these would need to be executed in a generated main function
                // or as part of initialization code. Skip for now.
            }
        }
    }
    // In classic minimal, ensure first string literal gets label STR_0 for inlined reference
    // Classic mode: don't duplicate string literals; rely on collected emission below
    // (Legacy tail loop removed; entry sequence already loops.)
    // Move runtime include AFTER vector lists like smartlist_demo - but only if needed
    if rt_usage.needs_vectorlist_runtime {
        out.push_str("    INCLUDE \"runtime/vectorlist_runtime.asm\"\n");
    }
    if !suppress_runtime {
        if rt_usage.needs_mul_helper { emit_mul_helper(&mut out); }
        if rt_usage.needs_div_helper { emit_div_helper(&mut out); }
        emit_builtin_helpers(&mut out, &rt_usage, opts);
    }
    out.push_str(";***************************************************************************\n; DATA SECTION\n;***************************************************************************\n");
    // Align ROM size to next 4K boundary: compute remainder via assembler can't do complex IF here, approximate with macro-style logic.
    // Fallback: emit a padding block sized by repeating labels (simple approach): not portable across all assemblers, so disabled for now.
    // NOTE: External packer should align to desired bank size (4K/8K). No internal alignment performed.
    // Optional bank alignment (4K/8K). Use ALIGN macro if bank_size is power-of-two.
    // Bank padding handled at end of file now.
    
    // Determine max args used (0..5) BEFORE evaluating suppress_runtime
    let max_args = compute_max_args_used(module);
    // Re-evaluate suppress_runtime now that we know max_args
    let no_runtime_vars_needed = !rt_usage.needs_tmp_left && !rt_usage.needs_tmp_right && 
                                 !rt_usage.needs_tmp_ptr && 
                                 !rt_usage.needs_mul_helper && !rt_usage.needs_div_helper && 
                                 !rt_usage.needs_line_vars && !rt_usage.needs_vcur_vars &&
                                 string_map.is_empty() && max_args == 0;
    suppress_runtime = main_inlined || no_runtime_vars_needed;
    
    // RAM variables: either emit ORG or symbolic EQU addresses.
    if suppress_runtime { /* skip RAM ORG and temp vars entirely */ }
    else if !opts.exclude_ram_org {
        out.push_str("    ORG $C880 ; begin runtime variables in RAM\n");
    }
    if !suppress_runtime { out.push_str("; Variables (in RAM)\n"); }
    // Runtime temporaries used by expression lowering and helpers
    if !suppress_runtime { if opts.exclude_ram_org { out.push_str("RESULT    EQU $C880\n"); } else { out.push_str("RESULT:   FDB 0\n"); } }
    if !suppress_runtime && rt_usage.needs_tmp_left { if opts.exclude_ram_org { out.push_str("TMPLEFT   EQU RESULT+2\n"); } else { out.push_str("TMPLEFT:  FDB 0\n"); } }
    if !suppress_runtime && rt_usage.needs_tmp_right { if opts.exclude_ram_org { out.push_str("TMPRIGHT  EQU RESULT+4\n"); } else { out.push_str("TMPRIGHT: FDB 0\n"); } }
    if !suppress_runtime && rt_usage.needs_tmp_ptr { if opts.exclude_ram_org { out.push_str("TMPPTR    EQU RESULT+6\n"); } else { out.push_str("TMPPTR:   FDB 0\n"); } }
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
    // Global mutables already allocated via symbol list; (future) could emit non-zero inits via a small startup routine.
    if !suppress_runtime && !string_map.is_empty() { out.push_str("; String literals (classic FCC + $80 terminator)\n"); }
    if !string_map.is_empty() {
        if string_map.len()==1 {
            let (lit,_label) = string_map.iter().next().unwrap();
            out.push_str("STR_0:\n");
            out.push_str(&format!("    FCC \"{}\"\n    FCB $80\n", lit.to_ascii_uppercase()));
        } else {
            // Each entry already has a unique label (STR_n) from string_literals.rs; emit them directly
            // Avoid emitting an unlabeled duplicated STR_0 header.
            for (lit,label) in &string_map {
                out.push_str(&format!("{}:\n    FCC \"{}\"\n    FCB $80\n", label, lit.to_ascii_uppercase()));
            }
        }
    }
    if !suppress_runtime {
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
    if opts.fast_wait {
        if opts.exclude_ram_org {
            out.push_str(&format!("FAST_WAIT_HIT EQU RESULT+{}\n", var_offset)); var_offset += 1;
        } else {
            out.push_str("FAST_WAIT_HIT: FCB 0\n");
        }
    }
    // Shared trig tables (emit only if used)
    if module_uses_trig(module) {
        out.push_str("; Trig tables (shared)\n");
        emit_trig_tables(&mut out, "FDB");
    }
    // Touch var_offset so compiler sees it used when EQU mode enabled
    #[allow(unused_variables)]
    { let _vo = var_offset; }
    // NOTE: No cartridge vector table emitted (raw snippet). Emulator that needs full 32K must wrap externally.
    out
}
fn expr_has_trig(e: &Expr) -> bool {
    match e {
        Expr::Call(ci) => {
            let u = ci.name.to_ascii_lowercase();
            u == "sin" || u == "cos" || u == "tan" || u == "math.sin" || u == "math.cos" || u == "math.tan"
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => expr_has_trig(left) || expr_has_trig(right),
        Expr::Not(inner) | Expr::BitNot(inner) => expr_has_trig(inner),
        _ => false,
    }
}

fn module_uses_trig(module: &Module) -> bool {
    for item in &module.items {
        if let Item::Function(f) = item {
            for s in &f.body { if stmt_has_trig(s) { return true; } }
        } else if let Item::ExprStatement(expr) = item {
            if expr_has_trig(expr) { return true; }
        }
    }
    false
}

fn stmt_has_trig(s: &Stmt) -> bool {
    match s {
        Stmt::Assign { value, .. } => expr_has_trig(value),
        Stmt::Let { value, .. } => expr_has_trig(value),
        Stmt::Expr(e) => expr_has_trig(e),
    Stmt::For { start, end, step, body, .. } => expr_has_trig(start) || expr_has_trig(end) || step.as_ref().map(expr_has_trig).unwrap_or(false) || body.iter().any(stmt_has_trig),
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
        } else if let Item::ExprStatement(expr) = item {
            maxa = maxa.max(scan_expr_args(expr));
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
        Expr::Call(ci) => {
            // Check if this call can be optimized inline (no VAR_ARG usage)
            let up = ci.name.to_ascii_uppercase();
            
            // DRAW_LINE with all constants doesn't use VAR_ARG (optimized inline)
            if up == "DRAW_LINE" && ci.args.len() == 5 && 
               ci.args.iter().all(|a| matches!(a, Expr::Number(_))) {
                // This call will be optimized inline - doesn't use VAR_ARG
                return ci.args.iter().map(scan_expr_args).max().unwrap_or(0);
            }
            
            // Other calls use VAR_ARG normally
            ci.args.len().min(5).max(ci.args.iter().map(scan_expr_args).max().unwrap_or(0))
        },
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
    needs_vectorlist_runtime: bool,
    wrappers_used: HashSet<String>,
}

fn analyze_runtime_usage(module: &Module) -> RuntimeUsage {
    let mut usage = RuntimeUsage::default();
    for item in &module.items {
        if let Item::Function(f) = item {
            for s in &f.body { scan_stmt_runtime(s, &mut usage); }
        } else if let Item::ExprStatement(expr) = item {
            scan_expr_runtime(expr, &mut usage);
        }
    }
    // Derive grouped variable needs from wrappers
    if usage.wrappers_used.contains("DRAW_LINE_WRAPPER") || usage.wrappers_used.contains("VECTREX_DRAW_VL") || usage.wrappers_used.contains("VECTREX_DRAW_TO") {
        usage.needs_line_vars = true;
    }
    if usage.wrappers_used.contains("VECTREX_MOVE_TO") || usage.wrappers_used.contains("VECTREX_DRAW_TO") {
        usage.needs_vcur_vars = true;
    }
    usage
}

fn scan_stmt_runtime(s: &Stmt, usage: &mut RuntimeUsage) {
    match s {
        Stmt::Assign { value, .. } => { usage.needs_tmp_ptr = true; scan_expr_runtime(value, usage); },
        Stmt::Let { value, .. } => scan_expr_runtime(value, usage),
        Stmt::Expr(value) => scan_expr_runtime(value, usage),
        Stmt::For { start, end, step, body, .. } => {
            scan_expr_runtime(start, usage);
            scan_expr_runtime(end, usage);
            if let Some(se) = step { scan_expr_runtime(se, usage); }
            for st in body { scan_stmt_runtime(st, usage); }
        }
        Stmt::While { cond, body } => { scan_expr_runtime(cond, usage); for st in body { scan_stmt_runtime(st, usage); } }
        Stmt::If { cond, body, elifs, else_body } => {
            scan_expr_runtime(cond, usage);
            for st in body { scan_stmt_runtime(st, usage); }
            for (c, b) in elifs { scan_expr_runtime(c, usage); for st in b { scan_stmt_runtime(st, usage); } }
            if let Some(eb) = else_body { for st in eb { scan_stmt_runtime(st, usage); } }
        }
        Stmt::Return(o) => { if let Some(e) = o { scan_expr_runtime(e, usage); } }
        Stmt::Switch { expr, cases, default } => {
            scan_expr_runtime(expr, usage);
            for (ce, cb) in cases { scan_expr_runtime(ce, usage); for st in cb { scan_stmt_runtime(st, usage); } }
            if let Some(db) = default { for st in db { scan_stmt_runtime(st, usage); } }
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
        Expr::Call(ci) => { 
            // Track wrapper usage (normalize like emit_builtin_call)
            let up = ci.name.to_ascii_uppercase();
            let resolved = resolve_function_name(&up);
            if let Some(r) = resolved {
                // Always use wrapper (no inline optimization)
                usage.wrappers_used.insert(r);
            }
            // Check if this function needs vectorlist runtime
            if up == "VECTREX_DRAW_VECTORLIST" || up == "DRAW_VECTORLIST" {
                usage.needs_vectorlist_runtime = true;
            }
            // DRAW_LINE: only mark wrapper as needed if it can't be optimized inline
            if up == "DRAW_LINE" {
                // Check if this call can be optimized inline (all 5 args are constants)
                let can_optimize_inline = ci.args.len() == 5 && 
                    ci.args.iter().all(|a| matches!(a, Expr::Number(_)));
                
                if !can_optimize_inline {
                    // Only mark wrapper as needed if inline optimization isn't possible
                    usage.wrappers_used.insert("DRAW_LINE_WRAPPER".to_string());
                }
            }
            for a in &ci.args { scan_expr_runtime(a, usage); }
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
    // Reset end position tracking for each function
    LAST_END_SET.store(false, Ordering::Relaxed);
    
    // Map special VPy functions to proper ASM labels
    let label_name = if f.name == "main" {
        "MAIN_INT_BLABLA".to_string()
    } else if f.name == "loop" {
        "MAIN_LOOP_CODE".to_string()
    } else {
        f.name.to_uppercase()
    };
    
    out.push_str(&format!("{}: ; function\n", label_name));
    out.push_str(&format!("; --- function {} ---\n", f.name));
    let locals = collect_locals(&f.body);
    let frame_size = (locals.len() as i32) * 2; // 2 bytes per 16-bit local
    if frame_size > 0 { out.push_str(&format!("    LEAS -{},S ; allocate locals\n", frame_size)); }
    for (i, p) in f.params.iter().enumerate().take(4) {
        out.push_str(&format!("    LDD VAR_ARG{}\n    STD VAR_{}\n", i, p.to_uppercase()));
    }
    let fctx = FuncCtx { locals: locals.clone(), frame_size };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map, opts); }
    if !matches!(f.body.last(), Some(Stmt::Return(_))) {
    if frame_size > 0 { out.push_str(&format!("    LEAS {},S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push('\n');
}

// emit_builtin_helpers: simple placeholder wrappers for Vectrex intrinsics.
fn emit_builtin_helpers(out: &mut String, usage: &RuntimeUsage, opts: &CodegenOptions) {
    let w = &usage.wrappers_used;
    // Only emit vector phase helper if referenced
    if w.contains("VECTREX_VECTOR_PHASE_BEGIN") {
        if opts.fast_wait {
            out.push_str("VECTREX_VECTOR_PHASE_BEGIN:\n    JSR DP_to_C8\n    JSR VECTREX_RESET0_FAST\n    RTS\n");
        } else {
            out.push_str("VECTREX_VECTOR_PHASE_BEGIN:\n    JSR DP_to_C8\n    JSR Reset0Ref\n    RTS\n");
        }
    }
    if w.contains("VECTREX_DBG_STATIC_VL") {
        out.push_str("VECTREX_DBG_STATIC_VL:\n    JSR DP_to_C8\n    LDU #DBG_STATIC_LIST\n    LDA #$5F\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDBG_STATIC_LIST:\n    FCB $80,$20\n");
    }
    if opts.blink_intensity {
        out.push_str("VECTREX_BLINK_INT:\n    LDA BLINK_STATE\n    EORA #$01\n    STA BLINK_STATE\n    BEQ BLINK_LOW\nBLINK_HIGH: LDA #$5F\n    BRA BLINK_SET\nBLINK_LOW:  LDA #$10\nBLINK_SET:  JSR Intensity_a\n    RTS\n");
    }
    if opts.debug_init_draw {
        out.push_str("VECTREX_DEBUG_DRAW:\n    JSR DP_to_C8\n    LDU #DEBUG_DRAW_LIST\n    LDA #$40\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDEBUG_DRAW_LIST:\n    FCB $80,$40\n");
    }
    if opts.per_frame_silence {
        out.push_str("VECTREX_SILENCE:\n    LDA #0\n    STA $D001\n    CLR $D000\n    LDA #1\n    STA $D001\n    CLR $D000\n    LDA #2\n    STA $D001\n    CLR $D000\n    LDA #3\n    STA $D001\n    CLR $D000\n    LDA #4\n    STA $D001\n    CLR $D000\n    LDA #5\n    STA $D001\n    CLR $D000\n    LDA #6\n    STA $D001\n    CLR $D000\n    LDA #7\n    STA $D001\n    LDA #$3F\n    STA $D000\n    LDA #8\n    STA $D001\n    CLR $D000\n    LDA #9\n    STA $D001\n    CLR $D000\n    LDA #10\n    STA $D001\n    CLR $D000\n    RTS\n");
    }
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
            "; Draw from current (VCUR_X,VCUR_Y) to new (x,y) provided in low bytes VAR_ARG0/1.\n; Semántica: igual a MOVE_TO seguido de línea, pero preserva origen previo como punto inicial.\n; Limita dx/dy a rango BIOS (-64..63) antes de invocar Draw_Line_d.\nVECTREX_DRAW_TO:\n    ; Cargar destino (x,y)\n    LDA VAR_ARG0+1  ; Xdest en A temporalmente\n    STA VLINE_DX    ; reutilizar buffer temporal (bajo) para Xdest\n    LDA VAR_ARG1+1  ; Ydest en A\n    STA VLINE_DY    ; reutilizar buffer temporal para Ydest\n    ; Calcular dx = Xdest - VCUR_X\n    LDA VLINE_DX\n    SUBA VCUR_X\n    STA VLINE_DX\n    ; Calcular dy = Ydest - VCUR_Y\n    LDA VLINE_DY\n    SUBA VCUR_Y\n    STA VLINE_DY\n    ; Clamp dx\n    LDA VLINE_DX\n    CMPA #63\n    BLE D2_DX_HI_OK\n    LDA #63\nD2_DX_HI_OK: CMPA #-64\n    BGE D2_DX_LO_OK\n    LDA #-64\nD2_DX_LO_OK: STA VLINE_DX\n    ; Clamp dy\n    LDA VLINE_DY\n    CMPA #63\n    BLE D2_DY_HI_OK\n    LDA #63\nD2_DY_HI_OK: CMPA #-64\n    BGE D2_DY_LO_OK\n    LDA #-64\nD2_DY_LO_OK: STA VLINE_DY\n    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)\n    LDA VCUR_Y\n    LDB VCUR_X\n    JSR Moveto_d\n    ; Dibujar línea usando deltas (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    ; Actualizar posición actual al destino exacto original (no clamped)\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("DRAW_LINE_WRAPPER") {
        out.push_str(
            "; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly\n; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.\n; ALWAYS sets intensity and handles Reset0Ref properly.\nDRAW_LINE_WRAPPER:\n    ; Set DP to hardware registers\n    LDA #$D0\n    TFR A,DP\n    ; ALWAYS set intensity (no optimization)\n    LDA VAR_ARG4+1\n    JSR Intensity_a\n    ; CRITICAL: Reset integrator origin before each line\n    JSR Reset0Ref\n    ; Move to start (y in A, x in B) - use signed byte values\n    LDA VAR_ARG1+1  ; Y start (signed byte)\n    LDB VAR_ARG0+1  ; X start (signed byte)\n    JSR Moveto_d\n    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes\n    ; dx = x1 - x0 (treating as signed)\n    LDD VAR_ARG2    ; x1 (16-bit)\n    SUBD VAR_ARG0   ; subtract x0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DX_CLAMP_HI_OK\n    LDD #127\nDLW_DX_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DX_CLAMP_LO_OK\n    LDD #-128\nDLW_DX_CLAMP_LO_OK:\n    STB VLINE_DX    ; Store dx as signed byte\n    ; dy = y1 - y0 (treating as signed)\n    LDD VAR_ARG3    ; y1 (16-bit)\n    SUBD VAR_ARG1   ; subtract y0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DY_CLAMP_HI_OK\n    LDD #127\nDLW_DY_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DY_CLAMP_LO_OK\n    LDD #-128\nDLW_DY_CLAMP_LO_OK:\n    STB VLINE_DY    ; Store dy as signed byte\n    ; Further clamp to Vectrex hardware limits (-64 to +63)\n    LDA VLINE_DX\n    CMPA #63\n    BLE DLW_DX_HI_OK\n    LDA #63\nDLW_DX_HI_OK: CMPA #-64\n    BGE DLW_DX_LO_OK\n    LDA #-64\nDLW_DX_LO_OK: STA VLINE_DX\n    ; Clamp dy to Vectrex limits\n    LDA VLINE_DY\n    CMPA #63\n    BLE DLW_DY_HI_OK\n    LDA #63\nDLW_DY_HI_OK: CMPA #-64\n    BGE DLW_DY_LO_OK\n    LDA #-64\nDLW_DY_LO_OK: STA VLINE_DY\n    ; Clear Vec_Misc_Count for proper timing\n    CLR Vec_Misc_Count\n    ; Draw line (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    RTS\n"
        );
    }
    if w.contains("VECTREX_FRAME_BEGIN") {
        if opts.fast_wait {
            out.push_str(
                "VECTREX_FRAME_BEGIN:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    JSR VECTREX_RESET0_FAST\n    RTS\n"
            );
        } else {
            out.push_str(
                "VECTREX_FRAME_BEGIN:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    JSR Reset0Ref\n    RTS\n"
            );
        }
    }
    if w.contains("VECTREX_DRAW_VL") {
        out.push_str(
            "VECTREX_DRAW_VL:\n    LDU VAR_ARG0\n    LDA VAR_ARG1+1\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\n"
        );
    }
    if w.contains("VECTREX_SET_ORIGIN") {
        if opts.fast_wait {
            out.push_str("VECTREX_SET_ORIGIN:\n    JSR VECTREX_RESET0_FAST\n    RTS\n");
        } else {
            out.push_str("VECTREX_SET_ORIGIN:\n    JSR Reset0Ref\n    RTS\n");
        }
    }
    if w.contains("VECTREX_SET_INTENSITY") {
    out.push_str("VECTREX_SET_INTENSITY:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    RTS\n");
    }
    if w.contains("VECTREX_WAIT_RECAL") || opts.fast_wait {
        if opts.fast_wait { out.push_str("VECTREX_WAIT_RECAL:\n    LDA #$D0\n    TFR A,DP\n    LDA FAST_WAIT_HIT\n    INCA\n    STA FAST_WAIT_HIT\n    RTS\n");
            out.push_str("VECTREX_RESET0_FAST:\n    LDA #$D0\n    TFR A,DP\n    CLR Vec_Dot_Dwell\n    CLR Vec_Loop_Count\n    RTS\n"); } else { out.push_str("VECTREX_WAIT_RECAL:\n    JSR WAIT_RECAL\n    RTS\n"); }
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
        "VECTREX_PRINT_TEXT"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"DRAW_LINE_WRAPPER"|"VECTREX_DRAW_VL"|"VECTREX_FRAME_BEGIN"|"VECTREX_VECTOR_PHASE_BEGIN"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"|"VECTREX_WAIT_RECAL"|
    "VECTREX_PLAY_MUSIC1"|
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|
    "ABS"|"MATH_ABS"|"MIN"|"MATH_MIN"|"MAX"|"MATH_MAX"|"CLAMP"|"MATH_CLAMP"|
    "DRAW_CIRCLE"|"DRAW_CIRCLE_SEG"|"DRAW_ARC"|"DRAW_SPIRAL"|"DRAW_VECTORLIST"
    );
    if up == "VECTREX_DRAW_VECTORLIST" { // alias to compact list runtime
        if args.len()==1 {
            if let Expr::StringLit(s) = &args[0] {
                out.push_str(&format!("    LDX #VL_{}\n    JSR Run_VectorList\n", s.to_ascii_uppercase()));
                return true;
            } else if let Expr::Ident(id) = &args[0] {
                out.push_str(&format!("    LDX #VL_{}\n    JSR Run_VectorList\n", id.name.to_ascii_uppercase()));
                return true;
            }
        }
    }
    if up == "DRAW_VECTORLIST" {
        if args.is_empty() { return true; }
        if let Expr::Ident(v) = &args[0] {
            out.push_str(&format!("    JSR VL_{}\n", v.name.to_ascii_uppercase()));
            return true;
        } else if let Expr::StringLit(s) = &args[0] {
            out.push_str(&format!("    JSR VL_{}\n", s.to_ascii_uppercase()));
            return true;
        }
    }
    
    // DRAW_LINE optimization: when all args are numeric constants, generate inline code
    if up == "DRAW_LINE" && args.len() == 5 && args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(x0), Expr::Number(y0), Expr::Number(x1), Expr::Number(y1), Expr::Number(intensity)) 
            = (&args[0], &args[1], &args[2], &args[3], &args[4]) {
            // Calculate deltas with proper signed handling
            let dx_total = *x1 - *x0;
            let dy_total = *y1 - *y0;
            
            // Clamp to Vectrex hardware limits (-64 to +63)
            let dx = dx_total.max(-64).min(63);
            let dy = dy_total.max(-64).min(63);
            
            // Generate optimized inline assembly
            out.push_str("    LDA #$D0\n    TFR A,DP\n");
            if *intensity == 0x5F { 
                out.push_str("    JSR Intensity_5F\n"); 
            } else { 
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", *intensity & 0xFF)); 
            }
            out.push_str("    JSR Reset0Ref\n");
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (*y0 & 0xFF), (*x0 & 0xFF)));
            out.push_str("    CLR Vec_Misc_Count\n");
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", (dy & 0xFF), (dx & 0xFF)));
            return true;
        }
    }
    
    // DRAW_LINE fallback: if not all constants, use wrapper
    if up == "DRAW_LINE" && args.len() == 5 {
        // Generate standard call sequence with wrapper
        for (i, arg) in args.iter().enumerate() {
            emit_expr(arg, out, fctx, string_map, opts);
            out.push_str("    LDD RESULT\n");
            out.push_str(&format!("    STD VAR_ARG{}\n", i));
        }
        out.push_str("    JSR DRAW_LINE_WRAPPER\n");
        out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
        return true;
    }
    
    // Custom macro: DRAW_POLYGON(N, x0,y0,x1,y1,...,x_{N-1},y_{N-1}) all numeric constants -> inline lines with origin resets
    if up == "DRAW_POLYGON" && !args.is_empty() {
        if let Expr::Number(nv) = &args[0] {
                let n = *nv as usize;
                // Two accepted forms:
                //  Form A: DRAW_POLYGON(N, x0,y0, x1,y1, ..., xN-1,yN-1)
                //  Form B: DRAW_POLYGON(N, INTENS, x0,y0, ...)
                // All numeric constants. Optimized (single Reset0Ref + intensity) to reduce flicker.
                let form_a_len = 1 + 2*n;
                let form_b_len = 2 + 2*n;
                let mut intensity: i32 = 0x5F; // default
                let (start_index, total_len_ok) = if args.len() == form_a_len { (1usize, true) } else if args.len() == form_b_len { (2usize, true) } else { (0,false) };
                if total_len_ok {
                    if start_index == 2 { // intensity provided
                        if let Expr::Number(iv) = &args[1] { intensity = *iv; }
                    }
                    if args[start_index..].iter().all(|a| matches!(a, Expr::Number(_))) {
                        let mut verts: Vec<(i32,i32)> = Vec::new();
                        for i in 0..n { if let (Expr::Number(xv), Expr::Number(yv)) = (&args[start_index+2*i], &args[start_index+2*i+1]) { verts.push((*xv, *yv)); } }
                        if verts.len()==n {
                            // DEBUG / SAFE MODE: draw each edge independently with a Reset0Ref + Moveto to start vertex.
                            // This is less efficient and may flicker more, but isolates any integrator drift issues.
                            if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                            for i in 0..n {
                                let (x0,y0)=verts[i];
                                let (x1,y1)=verts[(i+1)%n];
                                let dx_total = x1 - x0;
                                let dy_total = y1 - y0;
                                // Split only once if out of range (>127) into two halves.
                                let need_split = dx_total.abs().max(dy_total.abs()) > 127;
                                let (first_dx, first_dy, second_dx, second_dy, second) = if need_split {
                                    (dx_total/2, dy_total/2, dx_total - dx_total/2, dy_total - dy_total/2, true)
                                } else { (dx_total, dy_total, 0, 0, false) };
                                out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (y0 & 0xFF), (x0 & 0xFF)));
                                out.push_str("    CLR Vec_Misc_Count\n");
                                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", (first_dy & 0xFF), (first_dx & 0xFF)));
                                if second {
                                    out.push_str("    CLR Vec_Misc_Count\n");
                                    out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", (second_dy & 0xFF), (second_dx & 0xFF)));
                                }
                            }
                            out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
                            return true;
                        }
                    }
                }
            }
        }
    // DRAW_CIRCLE(xc,yc,diam) or DRAW_CIRCLE(xc,yc,diam,intensity) all numeric constants -> approximate with 16-gon
    if up == "DRAW_CIRCLE" && (args.len()==3 || args.len()==4) && args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(xc),Expr::Number(yc),Expr::Number(diam)) = (&args[0],&args[1],&args[2]) {
                    let mut intensity: i32 = 0x5F;
                    if args.len()==4 { if let Expr::Number(i) = &args[3] { intensity = *i; } }
                    let segs = 16; // fixed approximation
                    let r = (*diam as f64)/2.0;
                    use std::f64::consts::PI;
                    let mut verts: Vec<(i32,i32)> = Vec::new();
                    for k in 0..segs {
                        let ang = 2.0*PI*(k as f64)/(segs as f64);
                        let x = (*xc as f64) + r*ang.cos();
                        let y = (*yc as f64) + r*ang.sin();
                        verts.push((x.round() as i32, y.round() as i32));
                    }
                    // Emit optimized similar to polygon
                    out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                    if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                    let (sx,sy)=verts[0];
                    out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (sy & 0xFF), (sx & 0xFF)));
                    for i in 0..segs {
                        let (x0,y0)=verts[i];
                        let (x1,y1)=verts[(i+1)%segs];
                        let dx = (x1 - x0) & 0xFF;
                        let dy = (y1 - y0) & 0xFF;
                        out.push_str("    CLR Vec_Misc_Count\n");
                        out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", dy, dx));
                    }
                    out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
                    return true;
        }
    }
    // DRAW_CIRCLE_SEG(nseg, xc,yc,diam[,intensity]) variable segments
    if up == "DRAW_CIRCLE_SEG" && (args.len()==4 || args.len()==5) && args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(nseg),Expr::Number(xc),Expr::Number(yc),Expr::Number(diam)) = (&args[0],&args[1],&args[2],&args[3]) {
                    let mut intensity: i32 = 0x5F; if args.len()==5 { if let Expr::Number(i)=&args[4] { intensity = *i; }}
                    let segs = (*nseg).clamp(3, 64);
                    let r = (*diam as f64)/2.0;
                    use std::f64::consts::PI;
                    let mut verts: Vec<(i32,i32)> = Vec::new();
                    for k in 0..segs { let ang = 2.0*PI*(k as f64)/(segs as f64); let x = (*xc as f64)+r*ang.cos(); let y= (*yc as f64)+r*ang.sin(); verts.push((x.round() as i32,y.round() as i32)); }
                    out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                    if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                    let (sx,sy)=verts[0]; out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (sy & 0xFF),(sx & 0xFF)));
                    for i in 0..segs { let (x0,y0)=verts[i as usize]; let (x1,y1)=verts[((i+1)%segs) as usize]; let dx=(x1-x0)&0xFF; let dy=(y1-y0)&0xFF; out.push_str("    CLR Vec_Misc_Count\n"); out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", dy, dx)); }
                    out.push_str("    CLRA\n    CLRB\n    STD RESULT\n"); return true;
        }
    }
    // DRAW_ARC(nseg, xc,yc,radius,start_deg,sweep_deg[,intensity]) open arc
    if up == "DRAW_ARC" && (6..=7).contains(&args.len()) && args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(nseg),Expr::Number(xc),Expr::Number(yc),Expr::Number(rad),Expr::Number(startd),Expr::Number(sweepd)) = (&args[0],&args[1],&args[2],&args[3],&args[4],&args[5]) {
                let mut intensity: i32 = 0x5F; if args.len()==7 { if let Expr::Number(i)=&args[6] { intensity = *i; }}
                let segs = (*nseg).clamp(1, 96);
                let start = *startd as f64 * std::f64::consts::PI / 180.0; let sweep = *sweepd as f64 * std::f64::consts::PI / 180.0;
                // Clamp radius to keep inside safe display range (~ +-120)
                let r = (*rad as f64).clamp(4.0, 110.0);
                let steps = segs;
                let mut verts: Vec<(i32,i32)> = Vec::new();
                for k in 0..=steps { let t = k as f64 / steps as f64; let ang = start + sweep * t; let mut x= (*xc as f64)+ r*ang.cos(); let mut y= (*yc as f64)+ r*ang.sin(); x = x.clamp(-120.0,120.0); y = y.clamp(-120.0,120.0); verts.push((x.round() as i32,y.round() as i32)); }
                out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                let (sx,sy)=verts[0]; out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (sy & 0xFF),(sx & 0xFF)));
                for i in 0..steps { let (x0,y0)=verts[i as usize]; let (x1,y1)=verts[(i+1) as usize]; let dx=(x1-x0)&0xFF; let dy=(y1-y0)&0xFF; out.push_str("    CLR Vec_Misc_Count\n"); out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", dy, dx)); }
                out.push_str("    CLRA\n    CLRB\n    STD RESULT\n"); return true;
        }
    }
    // DRAW_SPIRAL(nseg, xc,yc,r_start,r_end,turns[,intensity]) open spiral
    if up == "DRAW_SPIRAL" && (6..=8).contains(&args.len()) && args.iter().all(|a| matches!(a, Expr::Number(_))) {
            // Accept forms with optional explicit intensity; if more than 6 args, last is intensity.
            let last_idx = args.len()-1;
            let (has_intensity, inten_expr_idx) = if args.len() > 6 { (true, last_idx) } else { (false, 0) };
            if let (Expr::Number(nseg),Expr::Number(xc),Expr::Number(yc),Expr::Number(r0),Expr::Number(r1),Expr::Number(turns)) = (&args[0],&args[1],&args[2],&args[3],&args[4],&args[5]) {
                let mut intensity: i32 = 0x5F; if has_intensity { if let Expr::Number(iv)=&args[inten_expr_idx] { intensity = *iv; } }
                let segs = (*nseg).clamp(4, 120);
                // Clamp turns to avoid huge angle wrap distortions
                let turns_f = (*turns as f64).clamp(0.1, 4.0);
                let total_ang = turns_f * 2.0 * std::f64::consts::PI;
                let start_r = (*r0 as f64).clamp(1.0, 110.0); let end_r = (*r1 as f64).clamp(1.0, 110.0);
                let steps = segs;
                let mut verts: Vec<(i32,i32)> = Vec::new();
                for k in 0..=steps { let t = k as f64 / steps as f64; let ang = total_ang * t; let r = start_r + (end_r - start_r)*t; let mut x= (*xc as f64)+ r*ang.cos(); let mut y= (*yc as f64)+ r*ang.sin(); x = x.clamp(-120.0,120.0); y = y.clamp(-120.0,120.0); verts.push((x.round() as i32,y.round() as i32)); }
                out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                let (sx,sy)=verts[0]; out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (sy & 0xFF),(sx & 0xFF)));
                for i in 0..steps { let (x0,y0)=verts[i as usize]; let (x1,y1)=verts[(i+1) as usize]; let dx=(x1-x0)&0xFF; let dy=(y1-y0)&0xFF; out.push_str("    CLR Vec_Misc_Count\n"); out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", dy, dx)); }
                out.push_str("    CLRA\n    CLRB\n    STD RESULT\n"); return true;
        }
    }
    if !is {
        // Backward compatibility: map legacy short names to vectrex-prefixed versions
        let translated = resolve_function_name(&up);
        if let Some(new_up) = translated {
            // Re-dispatch using new name recursively (avoid infinite loop by guarding is set)
            return emit_builtin_call(&new_up, args, out, fctx, string_map, opts);
        }
        return false;
    }
    // ABS
    if matches!(up.as_str(), "ABS"|"MATH_ABS") {
    if let Some(arg) = args.first() { emit_expr(arg, out, fctx, string_map, opts); } else { out.push_str("    LDD #0\n    STD RESULT\n"); return true; }
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
    if let Some(arg) = args.first() {
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
            if let Some(off) = fctx.offset_of(&target.name) {
                out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off));
            } else {
                out.push_str(&format!("    LDX RESULT\n    LDU #VAR_{}\n    STU TMPPTR\n    STX ,U\n", target.name.to_uppercase()));
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
                        let actual = (min + offset) & 0xFFFF;
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
                out.push('\n');
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
            if let Some(off) = fctx.offset_of(&name.name) { out.push_str(&format!("    LDD {} ,S\n    STD RESULT\n", off)); }
            else { out.push_str(&format!("    LDD VAR_{}\n    STD RESULT\n", name.name.to_uppercase())); }
        }
        Expr::Call(ci) => {
            if emit_builtin_call(&ci.name, &ci.args, out, fctx, string_map, opts) { return; }
            for (i, arg) in ci.args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr(arg, out, fctx, string_map, opts);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            if opts.force_extended_jsr { out.push_str(&format!("    JSR >{}\n", ci.name.to_uppercase())); } else { out.push_str(&format!("    JSR {}\n", ci.name.to_uppercase())); }
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
    Expr::Ident(n) => format!("I:{}", n.name),
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
        } else if let Item::GlobalLet { name, .. } = item { 
            globals.insert(name.clone()); 
        } else if let Item::ExprStatement(expr) = item {
            collect_expr_syms(expr, &mut globals);
        }
    }
    for l in &locals { globals.remove(l); }
    globals.into_iter().collect()
}

// collect_stmt_syms: process statement symbols.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
    match stmt {
    Stmt::Assign { target, value } => {
            set.insert(target.name.clone());
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
    Expr::Ident(n) => { set.insert(n.name.clone()); }
        Expr::Call(ci) => { for a in &ci.args { collect_expr_syms(a, set); } }
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
