// (Removed duplicated legacy block above during refactor)
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::collect_string_literals;
use super::debug_info::{DebugInfo, LineTracker, parse_asm_addresses, parse_native_call_comments};
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};
use std::sync::atomic::{AtomicBool, Ordering};

static LAST_END_SET: AtomicBool = AtomicBool::new(false);

// Macro to check recursion depth and prevent stack overflow
macro_rules! check_depth {
    ($depth:expr, $max:expr, $context:expr) => {
        if $depth > $max {
            panic!("Maximum recursion depth ({}) exceeded in {}. Please simplify your code.", $max, $context);
        }
    };
}

// Helper function to map legacy function names to their modern counterparts
fn resolve_function_name(name: &str) -> Option<String> {
    match name {
        "PRINT_TEXT" => Some("VECTREX_PRINT_TEXT".to_string()),
        "DEBUG_PRINT" => Some("VECTREX_DEBUG_PRINT".to_string()),
        "DEBUG_PRINT_LABELED" => Some("VECTREX_DEBUG_PRINT_LABELED".to_string()),
        "MOVE" => Some("VECTREX_MOVE_TO".to_string()),        // Unificado: MOVE -> VECTREX_MOVE_TO
        "MOVE_TO" => Some("VECTREX_MOVE_TO".to_string()),     // Compatibilidad hacia atrás
        "DRAW_TO" => Some("VECTREX_DRAW_TO".to_string()),
        "POKE" => Some("VECTREX_POKE".to_string()),
        "PEEK" => Some("VECTREX_PEEK".to_string()),
        "PRINT_NUMBER" => Some("VECTREX_PRINT_NUMBER".to_string()),
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

// Helper function to calculate the correct include path based on source file location
fn calculate_include_path(_opts: &CodegenOptions) -> String {
    // For lwasm compatibility: use "VECTREX.I" and pass -Iinclude to lwasm
    // Native assembler: reads from project root, so both "VECTREX.I" and "include/VECTREX.I" work
    "VECTREX.I".to_string()
}

// Helper function to calculate the correct runtime path based on source file location  
fn calculate_runtime_path(_opts: &CodegenOptions) -> String {
    // Always use relative path from project root - assembler should be run from project root
    "runtime/vectorlist_runtime.asm".to_string()
}

// analyze_used_assets: Scan module for DRAW_VECTOR() and PLAY_MUSIC() calls
// Returns set of asset names that are actually used in the code
fn analyze_used_assets(module: &Module) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut used = HashSet::new();
    
    fn scan_expr(expr: &Expr, used: &mut HashSet<String>, depth: usize) {
        const MAX_DEPTH: usize = 500;
        if depth > MAX_DEPTH {
            panic!("Maximum expression nesting depth ({}) exceeded during asset analysis.", MAX_DEPTH);
        }
        match expr {
            Expr::Call(call_info) => {
                let name_upper = call_info.name.to_uppercase();
                // Check for DRAW_VECTOR("asset_name", x, y), PLAY_MUSIC("asset_name"), or PLAY_SFX("asset_name")
                if (name_upper == "DRAW_VECTOR" && call_info.args.len() == 3) || 
                   (name_upper == "PLAY_MUSIC" && call_info.args.len() == 1) ||
                   (name_upper == "PLAY_SFX" && call_info.args.len() == 1) {
                    if let Expr::StringLit(asset_name) = &call_info.args[0] {
                        eprintln!("[DEBUG] Found asset usage: {} ({})", asset_name, name_upper);
                        used.insert(asset_name.clone());
                    }
                }
                // Recursively scan arguments
                for arg in &call_info.args {
                    scan_expr(arg, used, depth + 1);
                }
            },
            Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
                scan_expr(left, used, depth + 1);
                scan_expr(right, used, depth + 1);
            },
            Expr::Not(inner) | Expr::BitNot(inner) => scan_expr(inner, used, depth + 1),
            Expr::List(elements) => {
                for elem in elements {
                    scan_expr(elem, used, depth + 1);
                }
            },
            Expr::Index { target, index } => {
                scan_expr(target, used, depth + 1);
                scan_expr(index, used, depth + 1);
            },
            _ => {}
        }
    }
    
    fn scan_stmt(stmt: &Stmt, used: &mut HashSet<String>, depth: usize) {
        const MAX_DEPTH: usize = 500;
        if depth > MAX_DEPTH {
            panic!("Maximum statement nesting depth ({}) exceeded during asset analysis.", MAX_DEPTH);
        }
        match stmt {
            Stmt::Assign { value, .. } => scan_expr(value, used, depth + 1),
            Stmt::Let { value, .. } => scan_expr(value, used, depth + 1),
            Stmt::CompoundAssign { value, .. } => scan_expr(value, used, depth + 1),
            Stmt::Expr(expr, _line) => scan_expr(expr, used, depth + 1),
            Stmt::If { cond, body, elifs, else_body, .. } => {
                scan_expr(cond, used, depth + 1);
                for s in body { scan_stmt(s, used, depth + 1); }
                for (elif_cond, elif_body) in elifs {
                    scan_expr(elif_cond, used, depth + 1);
                    for s in elif_body { scan_stmt(s, used, depth + 1); }
                }
                if let Some(els) = else_body {
                    for s in els { scan_stmt(s, used, depth + 1); }
                }
            },
            Stmt::While { cond, body, .. } => {
                scan_expr(cond, used, depth + 1);
                for s in body { scan_stmt(s, used, depth + 1); }
            },
            Stmt::For { start, end, step, body, .. } => {
                scan_expr(start, used, depth + 1);
                scan_expr(end, used, depth + 1);
                if let Some(step_expr) = step {
                    scan_expr(step_expr, used, depth + 1);
                }
                for s in body { scan_stmt(s, used, depth + 1); }
            },
            Stmt::Switch { expr, cases, default, .. } => {
                scan_expr(expr, used, depth + 1);
                for (case_expr, case_body) in cases {
                    scan_expr(case_expr, used, depth + 1);
                    for s in case_body { scan_stmt(s, used, depth + 1); }
                }
                if let Some(default_body) = default {
                    for s in default_body { scan_stmt(s, used, depth + 1); }
                }
            },
            Stmt::Return(Some(expr), _line) => scan_expr(expr, used, depth + 1),
            _ => {}
        }
    }
    
    // Scan all functions and top-level items in module
    for item in &module.items {
        match item {
            Item::Function(func) => {
                for stmt in &func.body {
                    scan_stmt(stmt, &mut used, 0);
                }
            },
            Item::Const { value, .. } | Item::GlobalLet { value, .. } => {
                scan_expr(value, &mut used, 0);
            },
            Item::ExprStatement(expr) => {
                scan_expr(expr, &mut used, 0);
            },
            _ => {}
        }
    }
    
    used
}

// emit: entry point for Motorola 6809 backend assembly generation.
// Produces a simple Vectrex-style header, calls platform init + MAIN, then infinite loop.
pub fn emit(module: &Module, t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> String {
    let (asm, _debug_info) = emit_with_debug(module, t, ti, opts);
    asm
}

// emit_with_debug: Same as emit but also returns debug information for .pdb generation
pub fn emit_with_debug(module: &Module, _t: Target, ti: &TargetInfo, opts: &CodegenOptions) -> (String, DebugInfo) {
    // Initialize debug info with source and binary names
    let source_name = opts.source_path.as_ref()
        .and_then(|p| std::path::Path::new(p).file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.vpy")
        .to_string();
    
    let binary_name = source_name.replace(".vpy", ".bin");
    let mut debug_info = DebugInfo::new(source_name.clone(), binary_name.clone());
    
    // Create LineTracker to populate lineMap during code generation  
    // Start address: Parse from ti.origin or use Vectrex RAM start (0xC800)
    let start_address = u16::from_str_radix(ti.origin.trim_start_matches("0x").trim_start_matches("$"), 16)
        .unwrap_or(0xC800);
    let mut tracker = LineTracker::new(source_name.clone(), binary_name, start_address);
    
    let mut out = String::new();
    let syms = collect_all_vars(module); // Use ALL vars (including locals) for assembly generation
    let global_vars = collect_global_vars(module); // NEW: Collect variables with initial values
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
        return (out, debug_info);
    }
    if user_loop.is_none() {
        out.push_str("; ERROR: Missing required function 'def loop():'\n");
        out.push_str("; The loop() function is required and contains code that runs every frame (60 FPS).\n");
        out.push_str("        ; Compilation failed - please add def loop(): function\n");
        return (out, debug_info);
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
    let include_path = calculate_include_path(opts);
    out.push_str(&format!("    INCLUDE \"{}\"\n\n", include_path));
    
    // NOTE: BIOS symbols (Vec_Music_Flag, etc.) are defined in VECTREX.I
    // Do NOT duplicate them here to maintain lwasm compatibility
    
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
    // Single classic header only (tutorial format)
    out.push_str(&format!("    FCC \"{}\"\n    FCB $80\n    FDB {}\n", effective_copy, music_fdb));
    out.push_str("    FCB $F8\n    FCB $50\n    FCB $20\n    FCB $BB\n"); // -$45 = $BB
    let mut title = opts.title.to_uppercase();
    if title.len() > 24 { title.truncate(24); }
    title = title.chars().map(|c| if c.is_ascii_alphanumeric() || c==' ' { c } else { ' ' }).collect();
    if title.is_empty() { title.push(' '); }
    out.push_str(&format!("    FCC \"{}\"\n", title));
    out.push_str("    FCB $80\n    FCB 0\n\n");
    out.push_str(";***************************************************************************\n; CODE SECTION\n;***************************************************************************\n");
    
    // ========================================================================
    // CRITICAL FIX: Emit RAM EQU definitions EARLY (before helpers)
    // This ensures symbols like PSG_MUSIC_PTR are defined before being used
    // ========================================================================
    out.push_str("\n; === RAM VARIABLE DEFINITIONS (EQU) ===\n");
    out.push_str("; Must be defined BEFORE builtin helpers that reference them\n");
    out.push_str("RESULT         EQU $C880   ; Main result temporary\n");
    
    // PSG_MUSIC_PTR: Only if music assets exist
    let has_music_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Music)
    });
    if has_music_assets {
        out.push_str("PSG_MUSIC_PTR    EQU $C89C   ; Pointer to current PSG music position (RESULT+$1C, 2 bytes)\n");
        out.push_str("PSG_MUSIC_START  EQU $C89E   ; Pointer to start of PSG music for loops (RESULT+$1E, 2 bytes)\n");
        out.push_str("PSG_IS_PLAYING   EQU $C8A0   ; Playing flag (RESULT+$20, 1 byte)\n");
        out.push_str("PSG_MUSIC_ACTIVE EQU $C8A1   ; Set=1 during UPDATE_MUSIC_PSG (for logging, 1 byte)\n");
        out.push_str("PSG_FRAME_COUNT  EQU $C8A2   ; Current frame register write count (RESULT+$22, 1 byte)\n");
        out.push_str("PSG_MUSIC_PTR_DP   EQU $9C  ; DP-relative offset (for lwasm compatibility)\n");
        out.push_str("PSG_MUSIC_START_DP EQU $9E  ; DP-relative offset (for lwasm compatibility)\n");
        out.push_str("PSG_IS_PLAYING_DP  EQU $A0  ; DP-relative offset (for lwasm compatibility)\n");
        out.push_str("PSG_MUSIC_ACTIVE_DP EQU $A1 ; DP-relative offset (for lwasm compatibility)\n");
        out.push_str("PSG_FRAME_COUNT_DP EQU $A2  ; DP-relative offset (for lwasm compatibility)\n");
    }
    
    // SFX_PTR: Only if SFX assets exist
    let has_sfx_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Sfx)
    });
    if has_sfx_assets {
        out.push_str("SFX_PTR        EQU $C8A8   ; Current SFX pointer (RESULT+$28, 2 bytes)\n");
        out.push_str("SFX_TICK       EQU $C8AA   ; 32-bit tick counter (RESULT+$2A, 4 bytes)\n");
        out.push_str("SFX_EVENT      EQU $C8AE   ; Current event pointer (RESULT+$2E, 2 bytes)\n");
        out.push_str("SFX_ACTIVE     EQU $C8B0   ; Playback state (RESULT+$30, 1 byte)\n");
    }
    out.push_str("\n");
    
    // Check if main() has real content (not just return)
    let main_has_content = if let Some(main_func) = user_main {
        !main_func.body.is_empty() && !main_func.body.iter().all(|stmt| {
            matches!(stmt, Stmt::Return { .. })
        })
    } else {
        false
    };
    
    if main_has_content {
        // main() has real content - use START structure
        out.push_str("    JMP START\n\n");
        
        // Emit builtin helpers BEFORE program code (fixes forward reference issues)
        if !suppress_runtime {
            emit_builtin_helpers(&mut out, &rt_usage, opts, &mut debug_info);
        }
        
        out.push_str("START:\n    LDA #$D0\n    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)\n    LDA #$80\n    STA VIA_t1_cnt_lo\n    LDX #Vec_Default_Stk\n    TFR X,S\n");
        
        // Check if we have music/sfx assets that need PSG initialization
        let has_audio_assets = opts.assets.iter().any(|a| {
            matches!(a.asset_type, crate::codegen::AssetType::Music | crate::codegen::AssetType::Sfx)
        });
        
        // BIOS music system: Initialize music buffer to silence
        if has_audio_assets {
            out.push_str("    JSR $F533       ; Init_Music_Buf - Initialize BIOS music system to silence\n");
        }
        
        out.push_str("\n");
    }
    
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
        
        // Emit main() function body inline only if it has real content
        if main_has_content {
            out.push_str("    ; *** DEBUG *** main() function code inline (initialization)\n");
            
            // NEW: Initialize global variables with their initial values (ONCE at startup)
            for (name, value) in &global_vars {
                if let Expr::Number(n) = value {
                    out.push_str(&format!("    LDD #{}\n", n));
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                } else {
                    // For non-constant initial values, evaluate the expression
                    emit_expr(value, &mut out, &FuncCtx { locals: Vec::new(), frame_size: 0 }, &string_map, opts);
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
            }
            
            if let Some(main_func) = user_main {
                let fctx = FuncCtx { locals: Vec::new(), frame_size: 0 };
                for stmt in &main_func.body {
                    emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts, &mut tracker, 0);
                }
            }
        }
        
        // Choose label based on whether we have START or not
        let main_label = if main_has_content { "MAIN" } else { "main" };
        out.push_str(&format!("\n{}:\n", main_label));
        
        out.push_str("    JSR Wait_Recal\n    LDA #$80\n    STA VIA_t1_cnt_lo\n");
        // NOTE: UPDATE_MUSIC_PSG now called at START of LOOP_BODY, not here
        
        // CRITICAL: Initialize global variables even if main() has no content
        // This must happen ONCE before the loop starts
        if !main_has_content && !global_vars.is_empty() {
            out.push_str("    ; Initialize global variables\n");
            for (name, value) in &global_vars {
                if let Expr::Number(n) = value {
                    out.push_str(&format!("    LDD #{}\n", n));
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                } else {
                    // For non-constant initial values, evaluate the expression
                    emit_expr(value, &mut out, &FuncCtx { locals: Vec::new(), frame_size: 0 }, &string_map, opts);
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
            }
        }
        
        // Call loop() function as subroutine to avoid code duplication and overflow
        if let Some(_loop_func) = user_loop {
            out.push_str("    ; *** Call loop() as subroutine (executed every frame)\n");
            out.push_str("    JSR LOOP_BODY\n");
        }
        
        out.push_str(&format!("    BRA {}\n\n", main_label));
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
                if opts.auto_loop && f.name == "main" {
                    // Skip main function in auto_loop mode - it's inlined in START/MAIN
                    continue;
                } else if opts.auto_loop && f.name == "loop" {
                    // Emit loop function as LOOP_BODY subroutine to avoid code duplication
                    out.push_str("LOOP_BODY:\n");
                    // NOTE: Do NOT auto-insert UPDATE_MUSIC_PSG here - user must call MUSIC_UPDATE() explicitly
                    // This gives user control over when PSG updates happen (important for Print_Str_d compatibility)
                    
                    out.push_str(&format!("    ; DEBUG: Processing {} statements in loop() body\n", f.body.len()));
                    let fctx = FuncCtx { locals: Vec::new(), frame_size: 0 };
                    for (i, stmt) in f.body.iter().enumerate() {
                        out.push_str(&format!("    ; DEBUG: Statement {} - {:?}\n", i, std::mem::discriminant(stmt)));
                        emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts, &mut tracker, 0);
                    }
                    
                    out.push_str("    RTS\n\n");
                } else {
                    // Emit other functions normally
                    emit_function(f, &mut out, &string_map, opts, &mut tracker);
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
            Item::Export(_) => {
                // Export declarations are metadata for multi-file compilation.
                // No code generation needed at this stage.
            }
        }
    }
    // In classic minimal, ensure first string literal gets label STR_0 for inlined reference
    // Classic mode: don't duplicate string literals; rely on collected emission below
    // (Legacy tail loop removed; entry sequence already loops.)
    // Move runtime include AFTER vector lists like smartlist_demo - but only if needed
    if rt_usage.needs_vectorlist_runtime {
        let runtime_path = calculate_runtime_path(opts);
        out.push_str(&format!("    INCLUDE \"{}\"\n", runtime_path));
    }
    if !suppress_runtime {
        if rt_usage.needs_mul_helper { emit_mul_helper(&mut out); }
        if rt_usage.needs_div_helper { emit_div_helper(&mut out); }
        // NOTE: emit_builtin_helpers moved BEFORE program code (line ~268) to fix forward references
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
    // NOTE: RESULT EQU is now defined earlier (before helpers)
    // Only emit storage allocation here (FDB) if not using EQU mode
    if !opts.exclude_ram_org { out.push_str("RESULT:   FDB 0\n"); }
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
    
    // TEMP_YX: Temporary storage for y,x coordinates (used by Draw_Sync_List)
    if opts.exclude_ram_org {
        out.push_str("TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)\n");
    } else {
        out.push_str("TEMP_YX:   FDB 0   ; Temporary y,x storage\n");
    }
    
    // NOTE: PSG_MUSIC_PTR/PSG_IS_PLAYING EQU definitions moved earlier (before helpers)
    // Only emit storage allocation here (FDB/FCB) if not using EQU mode
    let has_music_assets_storage = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Music)
    });
    if has_music_assets_storage && !opts.exclude_ram_org {
        out.push_str("PSG_MUSIC_PTR:     FDB 0          ; Pointer to current PSG music position\n");
        out.push_str("PSG_MUSIC_START:   FDB 0          ; Pointer to start of PSG music (for loops)\n");
        out.push_str("PSG_IS_PLAYING:    FCB 0          ; Playing flag ($00=stopped, $01=playing)\n");
        out.push_str("PSG_MUSIC_ACTIVE:  FCB 0          ; Set=1 during UPDATE_MUSIC_PSG (for logging)\n");
    }
    
    // NOTE: SFX EQU definitions moved earlier (before helpers)
    // Only emit storage allocation here if not using EQU mode
    let has_sfx_assets_storage = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Sfx)
    });
    if has_sfx_assets_storage && !opts.exclude_ram_org {
        out.push_str("SFX_PTR:       FDB 0   ; Current SFX pointer\n");
        out.push_str("SFX_TICK:      FDB 0,0  ; Current playback tick (32-bit)\n");
        out.push_str("SFX_EVENT:     FDB 0   ; Pointer to current event\n");
        out.push_str("SFX_ACTIVE:    FCB 0   ; 0=stopped, 1=playing\n");
    }
    
    // VL_: Vector list variables for DRAW_VECTOR_LIST (Malban algorithm)
    // Always define if any code might use DRAW_VECTOR_LIST
    if opts.exclude_ram_org {
        out.push_str("VL_PTR     EQU $CF80      ; Current position in vector list\n");
        out.push_str("VL_Y       EQU $CF82      ; Y position (1 byte)\n");
        out.push_str("VL_X       EQU $CF83      ; X position (1 byte)\n");
        out.push_str("VL_SCALE   EQU $CF84      ; Scale factor (1 byte)\n");
    } else {
        out.push_str("VL_PTR:    FDB 0    ; Current position in vector list\n");
        out.push_str("VL_Y:      FCB 0    ; Y position\n");
        out.push_str("VL_X:      FCB 0    ; X position\n");
        out.push_str("VL_SCALE:  FCB 0    ; Scale factor\n");
    }
    
    let mut var_offset = 0; // Variables start at separate base, not after RESULT temps
    for v in syms {
        if opts.exclude_ram_org {
            // FIX: Use separate memory area for global variables, not RESULT+offset
            // Global variables should persist between function calls, but RESULT is temporary
            out.push_str(&format!("VAR_{} EQU $CF00+{}\n", v.to_uppercase(), var_offset));
            var_offset += 2;
        } else {
            out.push_str(&format!("VAR_{}: FDB 0\n", v.to_uppercase()));
        }
    }
    // Global mutables already allocated via symbol list; (future) could emit non-zero inits via a small startup routine.
    
    // ✅ Emit VAR_ARG EQU definitions HERE (before assets/strings)
    // This ensures ALL EQU definitions are together and don't interrupt FCB/FCC data
    if !suppress_runtime {
        out.push_str("; Call argument scratch space\n");
        if opts.exclude_ram_org {
            // Function arguments can use RESULT area since they're temporary
            // RESULT is defined as $C880, calculate absolute addresses
            // CRITICAL: Must be AFTER all PSG/SFX variables to avoid corruption
            // PSG vars: +28 to +34 ($C89C-$C8A2), SFX vars: +40 to +48 ($C8A8-$C8B0)
            let result_base = 0xC880u16;
            let mut arg_offset = 50; // Start after SFX_ACTIVE at +48 (0xC8B0)
            for i in 0..max_args { 
                let abs_addr = result_base + arg_offset;
                out.push_str(&format!("VAR_ARG{} EQU ${:04X}\n", i, abs_addr)); 
                arg_offset += 2; 
            }
        } else {
            if max_args >=1 { out.push_str("VAR_ARG0: FDB 0\n"); }
            if max_args >=2 { out.push_str("VAR_ARG1: FDB 0\n"); }
            if max_args >=3 { out.push_str("VAR_ARG2: FDB 0\n"); }
            if max_args >=4 { out.push_str("VAR_ARG3: FDB 0\n"); }
            if max_args >=5 { out.push_str("VAR_ARG4: FDB 0\n"); }
        }
    }
    
    // ✅ CRITICAL FIX: Embed assets HERE (after ALL EQU definitions, before strings)
    // The native assembler processes ALL lines but EQU doesn't generate bytes
    // We need FCB data AFTER all EQUs but BEFORE strings to ensure it's included
    if !opts.assets.is_empty() {
        // Analyze which assets are actually used in the code
        let used_assets = analyze_used_assets(module);
        eprintln!("[DEBUG] Used assets: {:?}", used_assets);
        eprintln!("[DEBUG] Available assets: {:?}", opts.assets.iter().map(|a| &a.name).collect::<Vec<_>>());
        
        // Filter assets to only include used ones
        let assets_to_embed: Vec<_> = opts.assets.iter()
            .filter(|asset| used_assets.contains(&asset.name))
            .collect();
        eprintln!("[DEBUG] Assets to embed: {}", assets_to_embed.len());
        
        if !assets_to_embed.is_empty() {
            out.push_str("\n; ========================================\n");
            out.push_str("; ASSET DATA SECTION\n");
            out.push_str(&format!("; Embedded {} of {} assets (unused assets excluded)\n", 
                assets_to_embed.len(), opts.assets.len()));
            out.push_str("; ========================================\n\n");
            
            for asset in assets_to_embed {
                match asset.asset_type {
                    crate::codegen::AssetType::Vector => {
                        use crate::vecres::VecResource;
                        if let Ok(resource) = VecResource::load(std::path::Path::new(&asset.path)) {
                            let asm = resource.compile_to_asm_with_name(Some(&asset.name));
                            out.push_str(&format!("; Vector asset: {}\n", asset.name));
                            out.push_str(&asm);
                            out.push_str("\n");
                        } else {
                            out.push_str(&format!("; ERROR: Failed to load vector asset: {}\n", asset.path));
                        }
                    },
                    crate::codegen::AssetType::Music => {
                        // Use MusicResource to generate proper ASM data (usando asset.name, NO nombre del JSON)
                        match crate::musres::MusicResource::load(std::path::Path::new(&asset.path)) {
                            Ok(resource) => {
                                let asm = resource.compile_to_asm(&asset.name);
                                out.push_str(&asm);
                                out.push_str("\n");
                            },
                            Err(e) => {
                                out.push_str(&format!("; ERROR: Failed to load/generate music asset {}: {}\n", asset.path, e));
                            }
                        }
                    },
                    crate::codegen::AssetType::Sfx => {
                        // SFX uses same format as music but labeled differently
                        match crate::musres::MusicResource::load(std::path::Path::new(&asset.path)) {
                            Ok(resource) => {
                                // Generate with "_SFX" suffix instead of "_MUSIC"
                                let symbol = format!("_{}_SFX", asset.name.to_uppercase().replace("-", "_").replace(" ", "_"));
                                out.push_str(&format!("; ========================================\n"));
                                out.push_str(&format!("; SFX Asset: {} (from {})\n", asset.name, asset.path));
                                out.push_str(&format!("; ========================================\n"));
                                out.push_str(&format!("{}:\n", symbol));
                                
                                // Emit same structure as music (tempo, ticks, events)
                                // MusicResource::compile_to_asm already handles full structure
                                // Just need to extract the data part without the label
                                let full_asm = resource.compile_to_asm(&asset.name);
                                // Skip first 4 lines (comments) and label line
                                let lines: Vec<&str> = full_asm.lines().collect();
                                if lines.len() > 5 {
                                    for line in &lines[5..] {
                                        out.push_str(line);
                                        out.push_str("\n");
                                    }
                                } else {
                                    // Fallback: just emit the whole thing
                                    out.push_str(&full_asm);
                                }
                                out.push_str("\n");
                            },
                            Err(e) => {
                                out.push_str(&format!("; ERROR: Failed to load/generate SFX asset {}: {}\n", asset.path, e));
                            }
                        }
                    }
                }
            }
        } else {
            out.push_str("\n; ========================================\n");
            out.push_str("; NO ASSETS EMBEDDED\n");
            out.push_str(&format!("; All {} discovered assets are unused in code\n", opts.assets.len()));
            out.push_str("; ========================================\n\n");
        }
    }
    
    // Filter out asset names from string_map (they are resolved to symbols, not string data)
    let asset_names: std::collections::HashSet<_> = opts.assets.iter().map(|a| a.name.as_str()).collect();
    let filtered_strings: Vec<_> = string_map.iter()
        .filter(|(lit, _)| !asset_names.contains(lit.as_str()))
        .collect();
    
    if !suppress_runtime && !filtered_strings.is_empty() { out.push_str("; String literals (classic FCC + $80 terminator)\n"); }
    if !filtered_strings.is_empty() {
        if filtered_strings.len()==1 {
            let (lit,_label) = filtered_strings[0];
            out.push_str("STR_0:\n");
            out.push_str(&format!("    FCC \"{}\"\n    FCB $80\n", lit.to_ascii_uppercase()));
        } else {
            // Each entry already has a unique label (STR_n) from string_literals.rs; emit them directly
            // Avoid emitting an unlabeled duplicated STR_0 header.
            for (lit,label) in filtered_strings {
                out.push_str(&format!("{}:\n    FCC \"{}\"\n    FCB $80\n", label, lit.to_ascii_uppercase()));
            }
        }
    }
    // VAR_ARG definitions moved earlier (before assets/strings) to keep all EQUs together
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
    // DRAW_VECTOR offset position storage (always needed for DRAW_VECTOR)
    if opts.exclude_ram_org {
        out.push_str(&format!("DRAW_VEC_X EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_VEC_Y EQU RESULT+{}\n", var_offset)); var_offset += 1;
    } else {
        out.push_str("; DRAW_VECTOR position offset\nDRAW_VEC_X: FCB 0\nDRAW_VEC_Y: FCB 0\n");
    }
    // Vector drawing temporary storage - NO LONGER NEEDED (removed old DRAW_VECTOR_RUNTIME)
    // Now using inline code with BIOS Draw_VLc function
    
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
    
    // Populate debug info with function symbols using REAL addresses from ASM parsing
    // Parse the generated ASM to extract label addresses
    let label_addresses = parse_asm_addresses(&out, 0x0000);
    
    // Entry point is either START (if main has content) or main label
    debug_info.set_entry_point(0x0000); // Vectrex cartridges start at 0x0000
    
    // Add symbols for main() and loop() functions with REAL addresses
    if user_main.is_some() {
        if main_has_content {
            if let Some(&addr) = label_addresses.get("START") {
                debug_info.add_symbol("START".to_string(), addr);
            }
            if let Some(&addr) = label_addresses.get("MAIN") {
                debug_info.add_symbol("MAIN".to_string(), addr);
            }
        } else {
            if let Some(&addr) = label_addresses.get("main") {
                debug_info.add_symbol("main".to_string(), addr);
            }
        }
    }
    
    if user_loop.is_some() {
        if let Some(&addr) = label_addresses.get("LOOP_BODY") {
            debug_info.add_symbol("LOOP_BODY".to_string(), addr);
        }
    }
    
    // Add symbols for all other functions with REAL addresses
    for item in &module.items {
        if let Item::Function(f) = item {
            if f.name != "main" && f.name != "loop" {
                let label_name = f.name.to_uppercase();
                if let Some(&addr) = label_addresses.get(&label_name) {
                    debug_info.add_symbol(label_name.clone(), addr);
                    
                    // Add function metadata
                    // Note: Line numbers will be 0 until AST is extended with line tracking
                    let start_line = 0; // TODO: f.line when available
                    let end_line = 0;   // TODO: Calculate from body when available
                    debug_info.add_function(
                        f.name.clone(),
                        addr,
                        start_line,
                        end_line,
                        "vpy"
                    );
                }
            }
        }
    }
    
    // Add function metadata for main() if present
    if let Some(_) = user_main {
        if main_has_content {
            if let Some(&addr) = label_addresses.get("MAIN") {
                debug_info.add_function(
                    "main".to_string(),
                    addr,
                    0, // TODO: Get from AST when available
                    0, // TODO: Calculate when available
                    "vpy"
                );
            }
        }
    }
    
    // Add function metadata for loop() if present
    if let Some(_) = user_loop {
        if let Some(&addr) = label_addresses.get("LOOP_BODY") {
            debug_info.add_function(
                "loop".to_string(),
                addr,
                0, // TODO: Get from AST when available
                0, // TODO: Calculate when available
                "vpy"
            );
        }
    }
    
    // Phase 4: Parse native call comments from ASM
    let native_calls = parse_native_call_comments(&out);
    for (line_num, function_name) in native_calls {
        debug_info.add_native_call(line_num, function_name);
    }
    
    // ✅ CRITICAL: Parse VPy_LINE markers from generated ASM to get REAL addresses
    // This replaces the tracker lineMap (which has incorrect addresses due to no advance() calls)
    // We parse the entire ASM to calculate actual addresses based on instruction sizes
    use super::debug_info::parse_vpy_line_markers;
    debug_info.line_map = parse_vpy_line_markers(&out, start_address);
    
    // NOTE: No cartridge vector table emitted (raw snippet). Emulator that needs full 32K must wrap externally.
    (out, debug_info)
}
fn expr_has_trig(e: &Expr) -> bool {
    expr_has_trig_depth(e, 0)
}

fn expr_has_trig_depth(e: &Expr, depth: usize) -> bool {
    check_depth!(depth, 500, "expr_has_trig");
    match e {
        Expr::Call(ci) => {
            let u = ci.name.to_ascii_lowercase();
            u == "sin" || u == "cos" || u == "tan" || u == "math.sin" || u == "math.cos" || u == "math.tan"
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => 
            expr_has_trig_depth(left, depth + 1) || expr_has_trig_depth(right, depth + 1),
        Expr::Not(inner) | Expr::BitNot(inner) => expr_has_trig_depth(inner, depth + 1),
        Expr::List(elements) => elements.iter().any(|e| expr_has_trig_depth(e, depth + 1)),
        Expr::Index { target, index } => expr_has_trig_depth(target, depth + 1) || expr_has_trig_depth(index, depth + 1),
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
    stmt_has_trig_depth(s, 0)
}

fn stmt_has_trig_depth(s: &Stmt, depth: usize) -> bool {
    check_depth!(depth, 500, "stmt_has_trig");
    match s {
        Stmt::Assign { value, .. } => expr_has_trig_depth(value, depth + 1),
        Stmt::Let { value, .. } => expr_has_trig_depth(value, depth + 1),
        Stmt::Expr(e, _) => expr_has_trig_depth(e, depth + 1),
    Stmt::For { start, end, step, body, .. } => expr_has_trig_depth(start, depth + 1) || expr_has_trig_depth(end, depth + 1) || step.as_ref().map(|e| expr_has_trig_depth(e, depth + 1)).unwrap_or(false) || body.iter().any(|s| stmt_has_trig_depth(s, depth + 1)),
        Stmt::While { cond, body, .. } => expr_has_trig_depth(cond, depth + 1) || body.iter().any(|s| stmt_has_trig_depth(s, depth + 1)),
        Stmt::If { cond, body, elifs, else_body, .. } => expr_has_trig_depth(cond, depth + 1) || body.iter().any(|s| stmt_has_trig_depth(s, depth + 1)) || elifs.iter().any(|(c,b)| expr_has_trig_depth(c, depth + 1) || b.iter().any(|s| stmt_has_trig_depth(s, depth + 1))) || else_body.as_ref().map(|eb| eb.iter().any(|s| stmt_has_trig_depth(s, depth + 1))).unwrap_or(false),
        Stmt::Return(o, _) => o.as_ref().map(|e| expr_has_trig_depth(e, depth + 1)).unwrap_or(false),
        Stmt::Switch { expr, cases, default, .. } => expr_has_trig(expr) || cases.iter().any(|(ce, cb)| expr_has_trig(ce) || cb.iter().any(stmt_has_trig)) || default.as_ref().map(|db| db.iter().any(stmt_has_trig)).unwrap_or(false),
        Stmt::Break { .. } | Stmt::Continue { .. } => false,
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before stmt_has_trig"),
    }
}

fn compute_max_args_used(module: &Module) -> usize {
    let mut maxa = 0usize;
    for item in &module.items {
        if let Item::Function(f) = item {
            // Count function parameters (they will need VAR_ARG slots)
            maxa = maxa.max(f.params.len());
            // Count arguments in function body calls
            for s in &f.body { maxa = maxa.max(scan_stmt_args(s)); }
        } else if let Item::ExprStatement(expr) = item {
            maxa = maxa.max(scan_expr_args(expr));
        }
    }
    maxa
}

fn scan_stmt_args(s: &Stmt) -> usize {
    match s {
        Stmt::Assign { value, .. } | Stmt::Let { value, .. } | Stmt::Expr(value, _) => scan_expr_args(value),
        Stmt::For { start, end, step, body, .. } => {
            let mut m = scan_expr_args(start).max(scan_expr_args(end));
            if let Some(se) = step { m = m.max(scan_expr_args(se)); }
            for st in body { m = m.max(scan_stmt_args(st)); }
            m
        }
        Stmt::While { cond, body, .. } => {
            let mut m = scan_expr_args(cond);
            for st in body { m = m.max(scan_stmt_args(st)); }
            m
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let mut m = scan_expr_args(cond);
            for st in body { m = m.max(scan_stmt_args(st)); }
            for (c, b) in elifs { m = m.max(scan_expr_args(c)); for st in b { m = m.max(scan_stmt_args(st)); } }
            if let Some(eb) = else_body { for st in eb { m = m.max(scan_stmt_args(st)); } }
            m
        }
        Stmt::Return(o, _) => o.as_ref().map(scan_expr_args).unwrap_or(0),
        Stmt::Switch { expr, cases, default, .. } => {
            let mut m = scan_expr_args(expr);
            for (ce, cb) in cases { m = m.max(scan_expr_args(ce)); for st in cb { m = m.max(scan_stmt_args(st)); } }
            if let Some(db) = default { for st in db { m = m.max(scan_stmt_args(st)); } }
            m
        }
        Stmt::Break { .. } | Stmt::Continue { .. } => 0,
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before scan_stmt_args"),
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
        Expr::List(elements) => elements.iter().map(scan_expr_args).max().unwrap_or(0),
        Expr::Index { target, index } => scan_expr_args(target).max(scan_expr_args(index)),
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
        Stmt::Expr(value, _) => scan_expr_runtime(value, usage),
        Stmt::For { start, end, step, body, .. } => {
            scan_expr_runtime(start, usage);
            scan_expr_runtime(end, usage);
            if let Some(se) = step { scan_expr_runtime(se, usage); }
            for st in body { scan_stmt_runtime(st, usage); }
        }
        Stmt::While { cond, body, .. } => { scan_expr_runtime(cond, usage); for st in body { scan_stmt_runtime(st, usage); } }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            scan_expr_runtime(cond, usage);
            for st in body { scan_stmt_runtime(st, usage); }
            for (c, b) in elifs { scan_expr_runtime(c, usage); for st in b { scan_stmt_runtime(st, usage); } }
            if let Some(eb) = else_body { for st in eb { scan_stmt_runtime(st, usage); } }
        }
        Stmt::Return(o, _) => { if let Some(e) = o { scan_expr_runtime(e, usage); } }
        Stmt::Switch { expr, cases, default, .. } => {
            scan_expr_runtime(expr, usage);
            for (ce, cb) in cases { scan_expr_runtime(ce, usage); for st in cb { scan_stmt_runtime(st, usage); } }
            if let Some(db) = default { for st in db { scan_stmt_runtime(st, usage); } }
            usage.needs_tmp_left = true; usage.needs_tmp_right = true; // switch lowering uses TMPLEFT
        }
        Stmt::Break { .. } | Stmt::Continue { .. } => {},
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before scan_stmt_runtime"),
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
            // Music/SFX system: track runtime helpers needed
            if up == "PLAY_MUSIC" {
                usage.wrappers_used.insert("PLAY_MUSIC_RUNTIME".to_string());
            }
            if up == "PLAY_SFX" {
                usage.wrappers_used.insert("PLAY_SFX_RUNTIME".to_string());
            }
            if up == "STOP_MUSIC" {
                usage.wrappers_used.insert("STOP_MUSIC_RUNTIME".to_string());
            }
            if up == "MUSIC_UPDATE" {
                usage.wrappers_used.insert("UPDATE_MUSIC_PSG".to_string());
            }
            for a in &ci.args { scan_expr_runtime(a, usage); }
        }
        Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
            scan_expr_runtime(left, usage);
            scan_expr_runtime(right, usage);
            usage.needs_tmp_left = true; usage.needs_tmp_right = true;
        }
        Expr::Not(inner) | Expr::BitNot(inner) => scan_expr_runtime(inner, usage),
        Expr::List(elements) => {
            for elem in elements {
                scan_expr_runtime(elem, usage);
            }
            // Array literal creation might need temporary storage
            usage.needs_tmp_ptr = true;
        }
        Expr::Index { target, index } => {
            scan_expr_runtime(target, usage);
            scan_expr_runtime(index, usage);
            usage.needs_tmp_ptr = true; // Array indexing needs address computation
        }
        _ => {}
    }
}

// emit_function: outputs code for a function.
fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, tracker: &mut LineTracker) {
    // Reset end position tracking for each function
    LAST_END_SET.store(false, Ordering::Relaxed);
    
    // Map special VPy functions to proper ASM labels
    let label_name = if f.name == "main" {
        "MAIN".to_string()
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
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map, opts, tracker, 0); }
    if !matches!(f.body.last(), Some(Stmt::Return(_, _))) {
    if frame_size > 0 { out.push_str(&format!("    LEAS {},S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push('\n');
}

// emit_builtin_helpers: simple placeholder wrappers for Vectrex intrinsics.
fn emit_builtin_helpers(out: &mut String, usage: &RuntimeUsage, opts: &CodegenOptions, debug_info: &mut DebugInfo) {
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
        let start_line = out.lines().count() + 1;
        let function_code = "VECTREX_PRINT_TEXT:\n    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)\n    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)\n    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP       ; Set Direct Page to $D0 for BIOS\n    LDU VAR_ARG2   ; string pointer (ARG2 = third param)\n    LDA VAR_ARG1+1 ; Y (ARG1 = second param)\n    LDB VAR_ARG0+1 ; X (ARG0 = first param)\n    JSR Print_Str_d\n    LDA #$C8       ; Restore DP to $C8 for our code\n    TFR A,DP\n    RTS\n";
        out.push_str(function_code);
        let end_line = out.lines().count();
        
        // Register ASM function location for debugging
        debug_info.add_asm_function(
            "VECTREX_PRINT_TEXT".to_string(),
            debug_info.asm.clone(),
            start_line,
            end_line,
            "native"
        );
    }
    if w.contains("VECTREX_DEBUG_PRINT") {
        let start_line = out.lines().count() + 1;
        let function_code = "VECTREX_DEBUG_PRINT:\n    ; Debug print to console - writes to end of RAM (safe area)\n    LDA VAR_ARG0+1   ; Load value to debug print\n    STA $CF00        ; Debug output value at end of RAM\n    LDA #$42         ; Debug marker\n    STA $CF01        ; Debug marker to indicate new output\n    RTS\n";
        out.push_str(function_code);
        let end_line = out.lines().count();
        
        // Register ASM function location for debugging  
        debug_info.add_asm_function(
            "VECTREX_DEBUG_PRINT".to_string(),
            debug_info.asm.clone(),
            start_line,
            end_line,
            "native"
        );
    }
    if w.contains("VECTREX_DEBUG_PRINT_LABELED") {
        out.push_str(
            "VECTREX_DEBUG_PRINT_LABELED:\n    ; Debug print with label - writes to end of RAM (safe area)\n    ; Write label string pointer to end of RAM\n    LDA VAR_ARG0     ; Label string pointer high byte\n    STA $CF02        ; Label pointer high at end of RAM\n    LDA VAR_ARG0+1   ; Label string pointer low byte  \n    STA $CF03        ; Label pointer low at end of RAM\n    ; Write value to debug output\n    LDA VAR_ARG1+1   ; Load value to debug print\n    STA $CF00        ; Debug output value at end of RAM\n    LDA #$FE         ; Labeled debug marker\n    STA $CF01        ; Debug marker to indicate labeled output\n    RTS\n"
        );
    }
    if w.contains("VECTREX_POKE") {
        out.push_str(
            "VECTREX_POKE:\n    ; Write byte to memory address\n    ; ARG0 = address (16-bit), ARG1 = value (8-bit)\n    LDX VAR_ARG0     ; Load address into X\n    LDA VAR_ARG1+1   ; Load value (low byte)\n    STA ,X           ; Store value to address\n    RTS\n"
        );
    }
    if w.contains("VECTREX_PEEK") {
        out.push_str(
            "VECTREX_PEEK:\n    ; Read byte from memory address\n    ; ARG0 = address (16-bit), returns value in VAR_ARG0+1\n    LDX VAR_ARG0     ; Load address into X\n    LDA ,X           ; Load value from address\n    STA VAR_ARG0+1   ; Store result in low byte of ARG0\n    RTS\n"
        );
    }
    if w.contains("VECTREX_PRINT_NUMBER") {
        out.push_str(
            "VECTREX_PRINT_NUMBER:\n    ; Print number at position\n    ; ARG0 = X position, ARG1 = Y position, ARG2 = number value\n    ; Simple implementation: convert number to string and print\n    LDA VAR_ARG1+1   ; Y position\n    LDB VAR_ARG0+1   ; X position\n    JSR Moveto_d     ; Move to position\n    \n    ; Convert number to string (simple: just show low byte as hex)\n    LDA VAR_ARG2+1   ; Load number value\n    \n    ; Convert high nibble to ASCII\n    LSRA\n    LSRA\n    LSRA\n    LSRA\n    ANDA #$0F\n    CMPA #10\n    BLO PN_DIGIT1\n    ADDA #7          ; A-F\nPN_DIGIT1:\n    ADDA #'0'\n    STA NUM_STR      ; Store first digit\n    \n    ; Convert low nibble to ASCII  \n    LDA VAR_ARG2+1\n    ANDA #$0F\n    CMPA #10\n    BLO PN_DIGIT2\n    ADDA #7          ; A-F\nPN_DIGIT2:\n    ADDA #'0'\n    ORA #$80         ; Set high bit for string termination\n    STA NUM_STR+1    ; Store second digit with high bit\n    \n    ; Print the string\n    LDU #NUM_STR     ; Point to our number string\n    JSR Print_Str_d  ; Print using BIOS\n    RTS\n\nNUM_STR: RMB 2      ; Space for 2-digit hex number\n"
        );
    }
    if w.contains("VECTREX_MOVE_TO") {
        out.push_str(
            "VECTREX_MOVE_TO:\n    LDA VAR_ARG1+1 ; Y\n    LDB VAR_ARG0+1 ; X\n    JSR Moveto_d\n    ; store new current position\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("VECTREX_DRAW_TO") {
        out.push_str(
            "; Draw from current (VCUR_X,VCUR_Y) to new (x,y) provided in low bytes VAR_ARG0/1.\n; Semántica: igual a MOVE_TO seguido de línea, pero preserva origen previo como punto inicial.\n; Deltas pueden ser ±127 (hardware Vectrex soporta rango completo).\nVECTREX_DRAW_TO:\n    ; Cargar destino (x,y)\n    LDA VAR_ARG0+1  ; Xdest en A temporalmente\n    STA VLINE_DX    ; reutilizar buffer temporal (bajo) para Xdest\n    LDA VAR_ARG1+1  ; Ydest en A\n    STA VLINE_DY    ; reutilizar buffer temporal para Ydest\n    ; Calcular dx = Xdest - VCUR_X\n    LDA VLINE_DX\n    SUBA VCUR_X\n    STA VLINE_DX\n    ; Calcular dy = Ydest - VCUR_Y\n    LDA VLINE_DY\n    SUBA VCUR_Y\n    STA VLINE_DY\n    ; No clamping needed - signed byte arithmetic handles ±127 correctly\n    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)\n    LDA VCUR_Y\n    LDB VCUR_X\n    JSR Moveto_d\n    ; Dibujar línea usando deltas (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    ; Actualizar posición actual al destino exacto original\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("DRAW_LINE_WRAPPER") {
        out.push_str(
            "; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly\n; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.\n; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).\nDRAW_LINE_WRAPPER:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)\n    STA >$D00C     ; VIA_cntl\n    ; Set DP to hardware registers\n    LDA #$D0\n    TFR A,DP\n    ; ALWAYS set intensity (no optimization)\n    LDA VAR_ARG4+1\n    JSR Intensity_a\n    ; Move to start (y in A, x in B) - use signed byte values\n    LDA VAR_ARG1+1  ; Y start (signed byte)\n    LDB VAR_ARG0+1  ; X start (signed byte)\n    JSR Moveto_d\n    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes\n    ; dx = x1 - x0 (treating as signed)\n    LDD VAR_ARG2    ; x1 (16-bit)\n    SUBD VAR_ARG0   ; subtract x0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DX_CLAMP_HI_OK\n    LDD #127\nDLW_DX_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DX_CLAMP_LO_OK\n    LDD #-128\nDLW_DX_CLAMP_LO_OK:\n    STB VLINE_DX    ; Store dx as signed byte\n    ; dy = y1 - y0 (treating as signed)\n    LDD VAR_ARG3    ; y1 (16-bit)\n    SUBD VAR_ARG1   ; subtract y0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DY_CLAMP_HI_OK\n    LDD #127\nDLW_DY_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DY_CLAMP_LO_OK\n    LDD #-128\nDLW_DY_CLAMP_LO_OK:\n    STB VLINE_DY    ; Store dy as signed byte\n    ; dx and dy are already clamped to ±127 - no further clamping needed\n    ; Vectrex hardware supports full ±127 delta range\n    LDA VLINE_DX\n    STA VLINE_DX    ; Keep full range\n    LDA VLINE_DY\n    STA VLINE_DY    ; Keep full range\n    ; Clear Vec_Misc_Count for proper timing\n    CLR Vec_Misc_Count\n    ; Draw line (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code\n    TFR A,DP\n    RTS\n\n; DRAW_LINE_FAST - optimized version that skips redundant setup\n; Use this for multiple consecutive draws with same intensity\n; Args: (x0,y0,x1,y1) only - intensity must be set beforehand\nDRAW_LINE_FAST:\n    ; Move to start (y in A, x in B) - use signed byte values\n    LDA VAR_ARG1+1  ; Y start (signed byte)\n    LDB VAR_ARG0+1  ; X start (signed byte)\n    JSR Moveto_d\n    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes\n    ; dx = x1 - x0 (treating as signed)\n    LDD VAR_ARG2    ; x1 (16-bit)\n    SUBD VAR_ARG0   ; subtract x0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLF_DX_CLAMP_HI_OK\n    LDD #127\nDLF_DX_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLF_DX_CLAMP_LO_OK\n    LDD #-128\nDLF_DX_CLAMP_LO_OK:\n    STB VLINE_DX    ; Store dx as signed byte\n    ; dy = y1 - y0 (treating as signed)\n    LDD VAR_ARG3    ; y1 (16-bit)\n    SUBD VAR_ARG1   ; subtract y0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLF_DY_CLAMP_HI_OK\n    LDD #127\nDLF_DY_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLF_DY_CLAMP_LO_OK\n    LDD #-128\nDLF_DY_CLAMP_LO_OK:\n    STB VLINE_DY    ; Store dy as signed byte\n    ; dx and dy are already clamped to ±127 - no further clamping needed\n    ; Vectrex hardware supports full ±127 delta range\n    LDA VLINE_DX\n    STA VLINE_DX    ; Keep full range\n    LDA VLINE_DY\n    STA VLINE_DY    ; Keep full range\n    ; Clear Vec_Misc_Count for proper timing\n    CLR Vec_Misc_Count\n    ; Draw line (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    RTS\n"
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
    out.push_str("VECTREX_SET_INTENSITY:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP       ; Set Direct Page to $D0 for BIOS\n    LDA VAR_ARG0+1\n    JSR __Intensity_a\n    RTS\n");
    }
    if w.contains("SETUP_DRAW_COMMON") {
        out.push_str(
            "; Common drawing setup - sets DP register and resets integrator origin\n; Eliminates repetitive LDA #$D0; TFR A,DP; JSR Reset0Ref sequences\nSETUP_DRAW_COMMON:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n    RTS\n"
        );
    }
    if w.contains("VECTREX_WAIT_RECAL") || opts.fast_wait {
        if opts.fast_wait { out.push_str("VECTREX_WAIT_RECAL:\n    LDA #$D0\n    TFR A,DP\n    LDA FAST_WAIT_HIT\n    INCA\n    STA FAST_WAIT_HIT\n    RTS\n");
            out.push_str("VECTREX_RESET0_FAST:\n    LDA #$D0\n    TFR A,DP\n    CLR Vec_Dot_Dwell\n    CLR Vec_Loop_Count\n    RTS\n"); } else { out.push_str("VECTREX_WAIT_RECAL:\n    JSR Wait_Recal\n    RTS\n"); }
    }
    if w.contains("VECTREX_PLAY_MUSIC1") {
        // Simple wrapper to restart the default MUSIC1 tune each frame or once. BIOS expects U to point to music data table at (?), but calling MUSIC1 vector reinitializes tune.
        out.push_str("VECTREX_PLAY_MUSIC1:\n    JSR MUSIC1\n    RTS\n");
    }
    
    // BIOS music system handles all PSG operations internally - no custom helpers needed
    
    // DRAW_VECTOR_RUNTIME: Old helper - NO LONGER USED
    // Now using inline code with Draw_VLc BIOS function
    // (removed to avoid label conflicts with inline code)
    
    // PLAY_MUSIC_RUNTIME: Direct PSG music player (inspired by Christman2024/malbanGit)
    // Writes directly to PSG chip, bypassing BIOS
    // Force generation if music assets exist (for auto-inject UPDATE_MUSIC_PSG)
    let has_music_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Music)
    });
    if w.contains("PLAY_MUSIC_RUNTIME") || w.contains("STOP_MUSIC_RUNTIME") || has_music_assets {
        out.push_str(
            "; ============================================================================\n\
            ; PSG DIRECT MUSIC PLAYER (inspired by Christman2024/malbanGit)\n\
            ; ============================================================================\n\
            ; Writes directly to PSG chip using WRITE_PSG sequence\n\
            ;\n\
            ; Music data format (frame-based):\n\
            ;   FCB count           ; Number of register writes this frame\n\
            ;   FCB reg, val        ; PSG register/value pairs\n\
            ;   ...                 ; Repeat for each register\n\
            ;   FCB $FF             ; End marker\n\
            ;\n\
            ; PSG Registers:\n\
            ;   0-1: Channel A frequency (12-bit)\n\
            ;   2-3: Channel B frequency\n\
            ;   4-5: Channel C frequency\n\
            ;   6:   Noise period\n\
            ;   7:   Mixer control (enable/disable channels)\n\
            ;   8-10: Channel A/B/C volume\n\
            ;   11-12: Envelope period\n\
            ;   13:  Envelope shape\n\
            ; ============================================================================\n\
            \n\
            ; RAM variables (defined in RAM section above)\n\
            ; PSG_MUSIC_PTR    EQU RESULT+26  (2 bytes)\n\
            ; PSG_MUSIC_START  EQU RESULT+28  (2 bytes)\n\
            ; PSG_IS_PLAYING   EQU RESULT+30  (1 byte)\n\
            ; PSG_MUSIC_ACTIVE EQU RESULT+31  (1 byte) - Set=1 during UPDATE_MUSIC_PSG\n\
            \n\
            ; PLAY_MUSIC_RUNTIME - Start PSG music playback\n\
            ; Input: X = pointer to PSG music data\n\
            PLAY_MUSIC_RUNTIME:\n\
            STX >PSG_MUSIC_PTR     ; Store current music pointer (force extended)\n\
            STX >PSG_MUSIC_START   ; Store start pointer for loops (force extended)\n\
            LDA #$01\n\
            STA >PSG_IS_PLAYING ; Mark as playing (extended - var at 0xC8A0)\n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; UPDATE_MUSIC_PSG - Update PSG (call every frame)\n\
            ; ============================================================================\n\
            UPDATE_MUSIC_PSG:\n\
            ; CRITICAL: Set VIA to PSG mode BEFORE accessing PSG (don't assume state)\n\
            LDA #$00       ; VIA_cntl = $00 (PSG mode)\n\
            STA >$D00C     ; VIA_cntl\n\
            LDA #$01\n\
            STA >PSG_MUSIC_ACTIVE  ; Mark music system active (for PSG logging)\n\
            LDA >PSG_IS_PLAYING ; Check if playing (extended - var at 0xC8A0)\n\
            BEQ PSG_update_done    ; Not playing, exit\n\
            \n\
            LDX >PSG_MUSIC_PTR     ; Load pointer (force extended - LDX has no DP mode)\n\
            BEQ PSG_update_done    ; No music loaded\n\
            \n\
            ; Read frame count byte (number of register writes)\n\
            LDB ,X+\n\
            BEQ PSG_music_ended    ; Count=0 means end (no loop)\n\
            CMPB #$FF              ; Check for loop command\n\
            BEQ PSG_music_loop     ; $FF means loop (never valid as count)\n\
            \n\
            ; Process frame - push counter to stack\n\
            PSHS B                 ; Save count on stack\n\
            \n\
            ; Write register/value pairs to PSG\n\
PSG_write_loop:\n\
            LDA ,X+                ; Load register number\n\
            LDB ,X+                ; Load register value\n\
            PSHS X                 ; Save pointer (after reads)\n\
            \n\
            ; WRITE_PSG sequence\n\
            STA VIA_port_a         ; Store register number\n\
            LDA #$19               ; BDIR=1, BC1=1 (LATCH)\n\
            STA VIA_port_b\n\
            LDA #$01               ; BDIR=0, BC1=0 (INACTIVE)\n\
            STA VIA_port_b\n\
            LDA VIA_port_a         ; Read status\n\
            STB VIA_port_a         ; Store data\n\
            LDB #$11               ; BDIR=1, BC1=0 (WRITE)\n\
            STB VIA_port_b\n\
            LDB #$01               ; BDIR=0, BC1=0 (INACTIVE)\n\
            STB VIA_port_b\n\
            \n\
            PULS X                 ; Restore pointer\n\
            PULS B                 ; Get counter\n\
            DECB                   ; Decrement\n\
            BEQ PSG_frame_done     ; Done with this frame\n\
            PSHS B                 ; Save counter back\n\
            BRA PSG_write_loop\n\
            \n\
PSG_frame_done:\n\
            \n\
            ; Frame complete - update pointer and done\n\
            STX >PSG_MUSIC_PTR     ; Update pointer (force extended)\n\
            BRA PSG_update_done\n\
            \n\
PSG_music_ended:\n\
            CLR >PSG_IS_PLAYING ; Stop playback (extended - var at 0xC8A0)\n\
            ; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing\n\
            ; Music will fade naturally as frame data stops updating\n\
            BRA PSG_update_done\n\
            \n\
PSG_music_loop:\n\
            ; Loop command: $FF followed by 2-byte address (FDB)\n\
            ; X points past $FF, read the target address\n\
            LDD ,X                 ; Load 2-byte loop target address\n\
            STD >PSG_MUSIC_PTR     ; Update pointer to loop start\n\
            ; Exit - next frame will start from loop target\n\
            BRA PSG_update_done\n\
            \n\
PSG_update_done:\n\
            CLR >PSG_MUSIC_ACTIVE  ; Clear flag (music system done)\n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; STOP_MUSIC_RUNTIME - Stop music playback\n\
            ; ============================================================================\n\
            STOP_MUSIC_RUNTIME:\n\
            CLR >PSG_IS_PLAYING ; Clear playing flag (extended - var at 0xC8A0)\n\
            CLR >PSG_MUSIC_PTR     ; Clear pointer high byte (force extended)\n\
            CLR >PSG_MUSIC_PTR+1   ; Clear pointer low byte (force extended)\n\
            ; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing\n\
            RTS\n\
            \n"
        );
    }
    
    // PLAY_SFX_RUNTIME: Sound effects player for .vmus assets (one-shot, non-looping)
    // Only emit if PLAY_SFX() builtin is actually used in code
    if w.contains("PLAY_SFX_RUNTIME") {
        out.push_str(
            "; ============================================================================\n\
            ; PSG SOUND EFFECTS PLAYER RUNTIME\n\
            ; ============================================================================\n\
            ; SFX data structure (same as music .vmus format):\n\
            ;   +0: FDB tempo (BPM)\n\
            ;   +2: FDB ticks_per_beat\n\
            ;   +4: FDB total_ticks_hi, total_ticks_lo (32-bit)\n\
            ;   +8: FDB num_events\n\
            ;   +10: Event data (variable length)\n\
            ;\n\
            ; SFX player differences from music:\n\
            ;   - One-shot playback (no looping)\n\
            ;   - Higher priority than music (can interrupt)\n\
            ;   - Multiple SFX can play simultaneously (TODO: implement mixing)\n\
            ;\n\
            ; Current implementation: Simple placeholder\n\
            ; TODO: Implement full SFX queue with mixing\n\
            ; ============================================================================\n\
            \n\
            ; RAM variables for SFX (defined in RAM section)\n\
            ; SFX_PTR       FDB 0  ; Pointer to current SFX data\n\
            ; SFX_TICK      FDB 0, 0  ; Current playback tick (32-bit)\n\
            ; SFX_EVENT     FDB 0  ; Pointer to current event\n\
            ; SFX_ACTIVE    FCB 0  ; 0=stopped, 1=playing\n\
            \n\
            ; PLAY_SFX_RUNTIME - Initialize and start SFX playback\n\
            ; Input: X = pointer to SFX data structure\n\
            PLAY_SFX_RUNTIME:\n\
            ; Store SFX pointer\n\
            STX SFX_PTR\n\
            \n\
            ; Initialize playback state\n\
            CLRA\n\
            CLRB\n\
            STD SFX_TICK        ; Reset tick counter to 0\n\
            STD SFX_TICK+2\n\
            \n\
            ; Point to first event (skip 10-byte header)\n\
            TFR X,D\n\
            ADDD #10\n\
            STD SFX_EVENT\n\
            \n\
            ; Mark as active\n\
            LDA #1\n\
            STA SFX_ACTIVE\n\
            \n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; SFX_UPDATE - Process SFX events (called from MUSIC_UPDATE)\n\
            ; TODO: Integrate into MUSIC_UPDATE or call separately\n\
            ; ============================================================================\n\
            SFX_UPDATE:\n\
            PSHS A,B,X,Y\n\
            \n\
            ; Check if SFX is active\n\
            TST SFX_ACTIVE\n\
            BEQ SFX_UPDATE_done             ; Not playing, skip\n\
            \n\
            ; Increment tick counter (same logic as music)\n\
            LDD SFX_TICK+2\n\
            ADDD #1\n\
            STD SFX_TICK+2\n\
            BCC SFX_UPDATE_no_carry\n\
            LDD SFX_TICK\n\
            ADDD #1\n\
            STD SFX_TICK\n\
            \n\
SFX_UPDATE_no_carry:\n\
            ; TODO: Process SFX events (similar to MUSIC_UPDATE)\n\
            ; For now, just placeholder\n\
            \n\
SFX_UPDATE_done:\n\
            PULS A,B,X,Y\n\
            RTS\n\
            \n"
        );
    }
    
    // Trig tables are emitted later in data section.
    
    // ===========================================================================
    // BIOS WRAPPERS - VIDE/gcc6809 compatible calling convention
    // ===========================================================================
    // These wrappers ensure DP=$D0 is set before each BIOS call, mimicking
    // the behavior of VIDE's auto-generated wrapper functions.
    // Using these wrappers instead of direct BIOS calls eliminates issues
    // with Direct Page register state across multiple calls.
    
    out.push_str("; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)\n");
    
    // __Intensity_a wrapper - VIDE compatible (JMP not JSR)
    out.push_str(
        "__Intensity_a:\n\
        TFR B,A         ; Move B to A (BIOS expects intensity in A)\n\
        JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Reset0Ref wrapper - VIDE compatible (JMP not JSR)
    out.push_str(
        "__Reset0Ref:\n\
        JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Moveto_d wrapper - VIDE compatible (JMP not JSR)
    // Caller pushes Y parameter on stack, X in B register
    out.push_str(
        "__Moveto_d:\n\
        LDA 2,S         ; Get Y from stack (after return address)\n\
        JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Draw_Line_d wrapper - VIDE compatible (JMP not JSR)
    // Caller pushes dy parameter on stack, dx in B register
    out.push_str(
        "__Draw_Line_d:\n\
        LDA 2,S         ; Get dy from stack (after return address)\n\
        JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller\n"
    );

    // Draw_Sync_List - EXACT translation of Malban's draw_synced_list_c
    // Data format: intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
    out.push_str(
        "; ============================================================================\n\
        ; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c\n\
        ; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2\n\
        ; ============================================================================\n\
        Draw_Sync_List:\n\
        ; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)\n\
        LDA ,X+                 ; intensity\n\
        JSR $F2AB               ; BIOS Intensity_a (expects value in A)\n\
        LDB ,X+                 ; y_start\n\
        LDA ,X+                 ; x_start\n\
        STD TEMP_YX             ; Guardar en variable temporal (evita stack)\n\
        ; Reset completo\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move sequence\n\
        LDD TEMP_YX             ; Recuperar y,x\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A                  ; Save x\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore x\n\
        STA VIA_port_a          ; x to DAC\n\
        ; Timing setup\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move to complete\n\
        DSL_W1:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W1\n\
        ; Loop de dibujo\n\
        DSL_LOOP:\n\
        LDA ,X+                 ; Read flag\n\
        CMPA #2                 ; Check end marker\n\
        LBEQ DSL_DONE           ; Exit if end (long branch)\n\
        CMPA #1                 ; Check next path marker\n\
        LBEQ DSL_NEXT_PATH      ; Process next path (long branch)\n\
        ; Draw line\n\
        LDB ,X+                 ; dy\n\
        LDA ,X+                 ; dx\n\
        PSHS A                  ; Save dx\n\
        STB VIA_port_a          ; dy to DAC\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore dx\n\
        STA VIA_port_a          ; dx to DAC\n\
        CLR VIA_t1_cnt_hi\n\
        LDA #$FF\n\
        STA VIA_shift_reg\n\
        ; Wait for line draw\n\
        DSL_W2:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W2\n\
        CLR VIA_shift_reg\n\
        BRA DSL_LOOP\n\
        ; Next path: read new intensity and header, then continue drawing\n\
        DSL_NEXT_PATH:\n\
        ; Save current X position before reading anything\n\
        TFR X,D                 ; D = X (current position)\n\
        PSHS D                  ; Save X address\n\
        LDA ,X+                 ; Read intensity (X now points to y_start)\n\
        PSHS A                  ; Save intensity\n\
        LDB ,X+                 ; y_start\n\
        LDA ,X+                 ; x_start (X now points to next_y)\n\
        STD TEMP_YX             ; Save y,x\n\
        PULS A                  ; Get intensity back\n\
        PSHS A                  ; Save intensity again\n\
        LDA #$D0\n\
        TFR A,DP                ; Set DP=$D0 (BIOS requirement)\n\
        PULS A                  ; Restore intensity\n\
        JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)\n\
        ; Restore X to point to next_y,next_x (after the 3 bytes we read)\n\
        PULS D                  ; Get original X\n\
        ADDD #3                 ; Skip intensity, y_start, x_start\n\
        TFR D,X                 ; X now points to next_y\n\
        ; Reset to zero (same as Draw_Sync_List start)\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move to new start position\n\
        LDD TEMP_YX\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A\n\
        STA VIA_port_a          ; x to DAC\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move\n\
        DSL_W3:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W3\n\
        CLR VIA_shift_reg       ; Clear before continuing\n\
        BRA DSL_LOOP            ; Continue drawing\n\
        DSL_DONE:\n\
        RTS\n\n\
        ; ============================================================================\n\
        ; Draw_Sync_List_At - Draw vector at offset position (DRAW_VEC_X, DRAW_VEC_Y)\n\
        ; Same as Draw_Sync_List but adds offset to y_start, x_start coordinates\n\
        ; Uses: DRAW_VEC_X, DRAW_VEC_Y (set by DRAW_VECTOR before calling this)\n\
        ; ============================================================================\n\
        Draw_Sync_List_At:\n\
        LDA ,X+                 ; intensity\n\
        PSHS A                  ; Save intensity\n\
        LDA #$D0\n\
        PULS A                  ; Restore intensity\n\
        JSR $F2AB               ; BIOS Intensity_a\n\
        LDB ,X+                 ; y_start from .vec\n\
        ADDB DRAW_VEC_Y         ; Add Y offset\n\
        LDA ,X+                 ; x_start from .vec\n\
        ADDA DRAW_VEC_X         ; Add X offset\n\
        STD TEMP_YX             ; Save adjusted position\n\
        ; Reset completo\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move sequence\n\
        LDD TEMP_YX             ; Recuperar y,x ajustado\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A                  ; Save x\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore x\n\
        STA VIA_port_a          ; x to DAC\n\
        ; Timing setup\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move to complete\n\
        DSLA_W1:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W1\n\
        ; Loop de dibujo (same as Draw_Sync_List)\n\
        DSLA_LOOP:\n\
        LDA ,X+                 ; Read flag\n\
        CMPA #2                 ; Check end marker\n\
        LBEQ DSLA_DONE\n\
        CMPA #1                 ; Check next path marker\n\
        LBEQ DSLA_NEXT_PATH\n\
        ; Draw line\n\
        LDB ,X+                 ; dy\n\
        LDA ,X+                 ; dx\n\
        PSHS A                  ; Save dx\n\
        STB VIA_port_a          ; dy to DAC\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore dx\n\
        STA VIA_port_a          ; dx to DAC\n\
        CLR VIA_t1_cnt_hi\n\
        LDA #$FF\n\
        STA VIA_shift_reg\n\
        ; Wait for line draw\n\
        DSLA_W2:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W2\n\
        CLR VIA_shift_reg\n\
        BRA DSLA_LOOP\n\
        ; Next path: add offset to new coordinates too\n\
        DSLA_NEXT_PATH:\n\
        TFR X,D\n\
        PSHS D\n\
        LDA ,X+                 ; Read intensity\n\
        PSHS A\n\
        LDB ,X+                 ; y_start\n\
        ADDB DRAW_VEC_Y         ; Add Y offset to new path\n\
        LDA ,X+                 ; x_start\n\
        ADDA DRAW_VEC_X         ; Add X offset to new path\n\
        STD TEMP_YX\n\
        PULS A                  ; Get intensity back\n\
        JSR $F2AB\n\
        PULS D\n\
        ADDD #3\n\
        TFR D,X\n\
        ; Reset to zero\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move to new start position (already offset-adjusted)\n\
        LDD TEMP_YX\n\
        STB VIA_port_a\n\
        PSHS A\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A\n\
        STA VIA_port_a\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X\n\
        ; Wait for move\n\
        DSLA_W3:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W3\n\
        CLR VIA_shift_reg\n\
        BRA DSLA_LOOP\n\
        DSLA_DONE:\n\
        RTS\n"
    );
    
    // ========== JOYSTICK SUPPORT ==========
    // VPy programs now use REAL BIOS routines just like commercial ROMs:
    // - Joy_Digital ($F1F8) - reads joystick axes, updates Vec_Joy_1_X/Y ($C81B/$C81C)
    // - Read_Btns ($F1BA) - reads button states, updates Vec_Btn_State ($C80F)
    //
    // Benefits:
    // 1. Perfect compatibility with real Vectrex hardware
    // 2. Minestorm and BIOS games work correctly with gamepad
    // 3. No custom memory-mapped registers needed
    // 4. Standard Vectrex programming practice
    //
    // The BIOS calls are inlined directly in emit_builtin_call() for J1_X(), J1_Y(), etc.
    // No helper routines needed - everything goes through official BIOS entry points.
}

// emit_builtin_call: inline lowering for intrinsic names; returns true if handled
fn emit_builtin_call(name: &str, args: &Vec<Expr>, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, line_info: Option<usize>) -> bool {
    let up = name.to_ascii_uppercase();
    let is = matches!(up.as_str(),
        "VECTREX_PRINT_TEXT"|"VECTREX_DEBUG_PRINT"|"VECTREX_DEBUG_PRINT_LABELED"|"VECTREX_POKE"|"VECTREX_PEEK"|"VECTREX_PRINT_NUMBER"|"VECTREX_MOVE_TO"|"VECTREX_DRAW_TO"|"DRAW_LINE_WRAPPER"|"DRAW_LINE_FAST"|"SETUP_DRAW_COMMON"|"VECTREX_DRAW_VL"|"VECTREX_FRAME_BEGIN"|"VECTREX_VECTOR_PHASE_BEGIN"|"VECTREX_SET_ORIGIN"|"VECTREX_SET_INTENSITY"|"VECTREX_WAIT_RECAL"|
    "VECTREX_PLAY_MUSIC1"|"DRAW_VECTOR"|"PLAY_MUSIC"|"PLAY_SFX"|"STOP_MUSIC"|"MUSIC_UPDATE"|
        "J1_X"|"J1_Y"|"J1_BUTTON_1"|"J1_BUTTON_2"|"J1_BUTTON_3"|"J1_BUTTON_4"|
        "J2_X"|"J2_Y"|"J2_BUTTON_1"|"J2_BUTTON_2"|"J2_BUTTON_3"|"J2_BUTTON_4"|
        "SIN"|"COS"|"TAN"|"MATH_SIN"|"MATH_COS"|"MATH_TAN"|
    "ABS"|"MATH_ABS"|"MIN"|"MATH_MIN"|"MAX"|"MATH_MAX"|"CLAMP"|"MATH_CLAMP"|
    "DRAW_CIRCLE"|"DRAW_CIRCLE_SEG"|"DRAW_ARC"|"DRAW_SPIRAL"|"DRAW_VECTORLIST"
    );
    
    // Helper para agregar comentario de tracking cuando es una llamada nativa real
    let add_native_call_comment = |out: &mut String, func_name: &str| {
        if let Some(line) = line_info {
            out.push_str(&format!("; NATIVE_CALL: {} at line {}\n", func_name, line));
        }
    };
    
    // DRAW_VECTOR: Draw vector asset at position
    // Usage: DRAW_VECTOR("player", x, y) -> draws vector at absolute position (x, y)
    if up == "DRAW_VECTOR" && args.len() == 3 {
        if let Expr::StringLit(asset_name) = &args[0] {
            // Check if asset exists in opts.assets
            let asset_exists = opts.assets.iter().any(|a| {
                a.name == *asset_name && matches!(a.asset_type, crate::codegen::AssetType::Vector)
            });
            
            if asset_exists {
                // Find the asset to get path count
                let asset_info = opts.assets.iter()
                    .find(|a| a.name == *asset_name && matches!(a.asset_type, crate::codegen::AssetType::Vector))
                    .unwrap();
                
                // Load the .vec file to count paths
                use crate::vecres::VecResource;
                let path_count = if let Ok(resource) = VecResource::load(std::path::Path::new(&asset_info.path)) {
                    resource.visible_paths().len()
                } else {
                    1 // Fallback to 1 if can't load
                };
                
                let symbol = format!("_{}", asset_name.to_uppercase().replace("-", "_").replace(" ", "_"));
                
                out.push_str(&format!("; DRAW_VECTOR(\"{}\", x, y) - {} path(s) at position\n", asset_name, path_count));
                
                // Evaluate x position (arg 1)
                emit_expr(&args[1], out, fctx, string_map, opts);
                out.push_str("    LDA RESULT+1  ; X position (low byte)\n");
                out.push_str("    STA DRAW_VEC_X\n");
                
                // Evaluate y position (arg 2)
                emit_expr(&args[2], out, fctx, string_map, opts);
                out.push_str("    LDA RESULT+1  ; Y position (low byte)\n");
                out.push_str("    STA DRAW_VEC_Y\n");
                
                // Generate code to draw each path at offset position
                for path_idx in 0..path_count {
                    out.push_str(&format!("    LDX #{}_PATH{}  ; Path {}\n", symbol, path_idx, path_idx));
                    out.push_str("    JSR Draw_Sync_List_At\n");
                }
                
                out.push_str("    LDD #0\n    STD RESULT\n");
                return true;
            } else {
                // Generate helpful compile-time error with list of available assets
                let available: Vec<&str> = opts.assets.iter()
                    .filter(|a| matches!(a.asset_type, crate::codegen::AssetType::Vector))
                    .map(|a| a.name.as_str())
                    .collect();
                
                out.push_str(&format!("; ╔════════════════════════════════════════════════════════════╗\n"));
                out.push_str(&format!("; ║  ❌ COMPILATION ERROR: Vector asset not found             ║\n"));
                out.push_str(&format!("; ╠════════════════════════════════════════════════════════════╣\n"));
                out.push_str(&format!("; ║  DRAW_VECTOR(\"{}\") - asset does not exist{:>width$}║\n", 
                    asset_name, "", width = 61 - asset_name.len() - 32));
                out.push_str(&format!("; ╠════════════════════════════════════════════════════════════╣\n"));
                if available.is_empty() {
                    out.push_str(&format!("; ║  No .vec files found in assets/vectors/                   ║\n"));
                    out.push_str(&format!("; ║  Please create vector assets in that directory.           ║\n"));
                } else {
                    out.push_str(&format!("; ║  Available vector assets ({} found):                     ║\n", available.len()));
                    for (i, name) in available.iter().enumerate() {
                        out.push_str(&format!("; ║    {}. \"{}\"{:>width$}║\n", 
                            i+1, name, "", width = 56 - name.len()));
                    }
                }
                out.push_str(&format!("; ╚════════════════════════════════════════════════════════════╝\n"));
                out.push_str("    ERROR_VECTOR_ASSET_NOT_FOUND  ; Assembly will fail here\n");
                return true;
            }
        }
    }
    
    // PLAY_MUSIC: Play music asset by name
    // Usage: PLAY_MUSIC("theme") -> loads music data and starts playback
    if up == "PLAY_MUSIC" && args.len() == 1 {
        if let Expr::StringLit(asset_name) = &args[0] {
            // Check if asset exists in opts.assets
            let asset_exists = opts.assets.iter().any(|a| {
                a.name == *asset_name && matches!(a.asset_type, crate::codegen::AssetType::Music)
            });
            
            if asset_exists {
                let symbol = format!("_{}_MUSIC", asset_name.to_uppercase().replace("-", "_").replace(" ", "_"));
                out.push_str(&format!("; PLAY_MUSIC(\"{}\") - play music asset\n", asset_name));
                out.push_str(&format!("    LDX #{}\n", symbol));
                out.push_str("    JSR PLAY_MUSIC_RUNTIME\n");
                out.push_str("    LDD #0\n    STD RESULT\n");
                return true;
            } else {
                out.push_str(&format!("; ERROR: Music asset '{}' not found\n", asset_name));
                return true;
            }
        }
    }
    
    if up == "MUSIC_UPDATE" && args.is_empty() {
        add_native_call_comment(out, "UPDATE_MUSIC_PSG");
        out.push_str("    JSR UPDATE_MUSIC_PSG\n");
        out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
        return true;
    }
    
    // PLAY_SFX: Play sound effect asset by name (one-shot, non-looping)
    // Usage: PLAY_SFX("explosion") -> plays SFX once
    if up == "PLAY_SFX" && args.len() == 1 {
        if let Expr::StringLit(asset_name) = &args[0] {
            // Check if asset exists in opts.assets
            let asset_exists = opts.assets.iter().any(|a| {
                a.name == *asset_name && matches!(a.asset_type, crate::codegen::AssetType::Sfx)
            });
            
            if asset_exists {
                let symbol = format!("_{}_SFX", asset_name.to_uppercase().replace("-", "_").replace(" ", "_"));
                out.push_str(&format!("; PLAY_SFX(\"{}\") - play sound effect (one-shot)\n", asset_name));
                out.push_str(&format!("    LDX #{}\n", symbol));
                out.push_str("    JSR PLAY_SFX_RUNTIME\n");
                out.push_str("    LDD #0\n    STD RESULT\n");
                return true;
            } else {
                // Generate helpful compile-time error with list of available SFX
                let available: Vec<&str> = opts.assets.iter()
                    .filter(|a| matches!(a.asset_type, crate::codegen::AssetType::Sfx))
                    .map(|a| a.name.as_str())
                    .collect();
                
                out.push_str(&format!("; ╔════════════════════════════════════════════════════════════╗\n"));
                out.push_str(&format!("; ║  ❌ COMPILATION ERROR: SFX asset not found                ║\n"));
                out.push_str(&format!("; ╠════════════════════════════════════════════════════════════╣\n"));
                out.push_str(&format!("; ║  PLAY_SFX(\"{}\") - asset does not exist{:>width$}║\n", 
                    asset_name, "", width = 64 - asset_name.len() - 29));
                out.push_str(&format!("; ╠════════════════════════════════════════════════════════════╣\n"));
                if available.is_empty() {
                    out.push_str(&format!("; ║  No .vmus files found in assets/sfx/                      ║\n"));
                    out.push_str(&format!("; ║  Please create sound effect assets in that directory.     ║\n"));
                } else {
                    out.push_str(&format!("; ║  Available SFX assets ({} found):                        ║\n", available.len()));
                    for (i, name) in available.iter().enumerate() {
                        out.push_str(&format!("; ║    {}. \"{}\"{:>width$}║\n", 
                            i+1, name, "", width = 56 - name.len()));
                    }
                }
                out.push_str(&format!("; ╚════════════════════════════════════════════════════════════╝\n"));
                out.push_str("    ERROR_SFX_ASSET_NOT_FOUND  ; Assembly will fail here\n");
                return true;
            }
        }
    }
    
    // STOP_MUSIC: Stop currently playing background music
    // Usage: STOP_MUSIC() -> stops music playback
    if up == "STOP_MUSIC" && args.is_empty() {
        add_native_call_comment(out, "STOP_MUSIC");
        out.push_str("; STOP_MUSIC() - stop background music\n");
        out.push_str("    JSR STOP_MUSIC_RUNTIME\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        return true;
    }
    
    // ========== JOYSTICK 1 FUNCTIONS (alg_jch0/jch1) ==========
    
    // J1_X: Default to digital (fast, suitable for 60fps)
    if up == "J1_X" && args.is_empty() {
        add_native_call_comment(out, "J1_X");
        out.push_str("; J1_X() - Read Joystick 1 X axis (BIOS Digital)\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $C81F    ; Vec_Joy_Mux_1_X\n");
        out.push_str("    LDA #3\n");
        out.push_str("    STA $C820    ; Vec_Joy_Mux_1_Y\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP     ; Set DP=$D0 (BIOS requirement)\n");
        out.push_str("    JSR $F1F8    ; Joy_Digital\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP     ; Restore DP=$C8\n");
        out.push_str("    LDB $C81B    ; Vec_Joy_1_X\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }

    // J1_X_DIGITAL: Explicit digital version (-1/0/+1)
    if up == "J1_X_DIGITAL" && args.is_empty() {
        add_native_call_comment(out, "J1_X_DIGITAL");
        out.push_str("; J1_X_DIGITAL() - Read Joystick 1 X axis (BIOS Digital)\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $C81F    ; Vec_Joy_Mux_1_X\n");
        out.push_str("    LDA #3\n");
        out.push_str("    STA $C820    ; Vec_Joy_Mux_1_Y\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP     ; Set DP=$D0 (BIOS requirement)\n");
        out.push_str("    JSR $F1F8    ; Joy_Digital\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP     ; Restore DP=$C8\n");
        out.push_str("    LDB $C81B    ; Vec_Joy_1_X\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }

    // J1_X_ANALOG: Analog version (-127 to +127)
    if up == "J1_X_ANALOG" && args.is_empty() {
        add_native_call_comment(out, "J1_X_ANALOG");
        out.push_str("; J1_X_ANALOG() - Read Joystick 1 X axis (BIOS Analog)\n");
        out.push_str("    LDA #$80\n");
        out.push_str("    STA $C81A    ; Vec_Joy_Resltn (resolution: $80=fast)\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $C81F    ; Vec_Joy_Mux_1_X\n");
        out.push_str("    CLR $C820    ; Vec_Joy_Mux_1_Y (disable Y for speed)\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP     ; Set DP=$D0 (BIOS requirement)\n");
        out.push_str("    JSR $F1F5    ; Joy_Analog\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP     ; Restore DP=$C8\n");
        out.push_str("    LDB $C81B    ; Vec_Joy_1_X\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }

    // J1_Y: Read Joystick 1 Y axis via BIOS Joy_Digital
    // Returns signed 16-bit value: -1 (down), 0 (center), +1 (up)
    // NOTE: Joy_Digital is MUCH faster than Joy_Analog (suitable for 60fps)
    if up == "J1_Y" && args.is_empty() {
        add_native_call_comment(out, "J1_Y");
        out.push_str("; J1_Y() - Read Joystick 1 Y axis (BIOS)\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $C81F    ; Vec_Joy_Mux_1_X\n");
        out.push_str("    LDA #3\n");
        out.push_str("    STA $C820    ; Vec_Joy_Mux_1_Y\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP     ; Set DP=$D0 (BIOS requirement)\n");
        out.push_str("    JSR $F1F8    ; Joy_Digital\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP     ; Restore DP=$C8\n");
        out.push_str("    LDB $C81C    ; Vec_Joy_1_Y\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }

    // J1_Y_ANALOG: Analog version (-127 to +127)
    if up == "J1_Y_ANALOG" && args.is_empty() {
        add_native_call_comment(out, "J1_Y_ANALOG");
        out.push_str("; J1_Y_ANALOG() - Read Joystick 1 Y axis (BIOS Analog)\n");
        out.push_str("    LDA #$80\n");
        out.push_str("    STA $C81A    ; Vec_Joy_Resltn (resolution: $80=fast)\n");
        out.push_str("    CLR $C81F    ; Vec_Joy_Mux_1_X (disable X for speed)\n");
        out.push_str("    LDA #3\n");
        out.push_str("    STA $C820    ; Vec_Joy_Mux_1_Y\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP     ; Set DP=$D0 (BIOS requirement)\n");
        out.push_str("    JSR $F1F5    ; Joy_Analog\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP     ; Restore DP=$C8\n");
        out.push_str("    LDB $C81C    ; Vec_Joy_1_Y\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }

    // J1_BUTTON_1: Read Joystick 1 button 1 via BIOS Read_Btns
    // Returns 0 if released, 1 if pressed
    if up == "J1_BUTTON_1" && args.is_empty() {
        add_native_call_comment(out, "J1_BUTTON_1");
        out.push_str("; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$01\n");
        out.push_str("    BEQ .j1b1_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j1b1_done\n");
        out.push_str(".j1b1_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j1b1_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J1_BUTTON_2: Read Joystick 1 button 2 via BIOS Read_Btns
    if up == "J1_BUTTON_2" && args.is_empty() {
        add_native_call_comment(out, "J1_BUTTON_2");
        out.push_str("; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$02\n");
        out.push_str("    BEQ .j1b2_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j1b2_done\n");
        out.push_str(".j1b2_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j1b2_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J1_BUTTON_3: Read Joystick 1 button 3 via BIOS Read_Btns
    if up == "J1_BUTTON_3" && args.is_empty() {
        add_native_call_comment(out, "J1_BUTTON_3");
        out.push_str("; J1_BUTTON_3() - Read Joystick 1 button 3 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$04\n");
        out.push_str("    BEQ .j1b3_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j1b3_done\n");
        out.push_str(".j1b3_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j1b3_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J1_BUTTON_4: Read Joystick 1 button 4 via BIOS Read_Btns
    if up == "J1_BUTTON_4" && args.is_empty() {
        add_native_call_comment(out, "J1_BUTTON_4");
        out.push_str("; J1_BUTTON_4() - Read Joystick 1 button 4 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$08\n");
        out.push_str("    BEQ .j1b4_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j1b4_done\n");
        out.push_str(".j1b4_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j1b4_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // ========== JOYSTICK 2 FUNCTIONS ==========
    
    // J2_X: Read Joystick 2 X axis via BIOS Joy_Digital
    // Returns signed 16-bit value: -1 (left), 0 (center), +1 (right)
    if up == "J2_X" && args.is_empty() {
        add_native_call_comment(out, "J2_X");
        out.push_str("; J2_X() - Read Joystick 2 X axis (BIOS)\n");
        out.push_str("    LDA #5\n");
        out.push_str("    STA $C821    ; Vec_Joy_Mux_2_X\n");
        out.push_str("    LDA #7\n");
        out.push_str("    STA $C822    ; Vec_Joy_Mux_2_Y\n");
        out.push_str("    JSR $F1F8    ; Joy_Digital\n");
        out.push_str("    LDB $C81D    ; Vec_Joy_2_X\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J2_Y: Read Joystick 2 Y axis via BIOS Joy_Digital
    // Returns signed 16-bit value: -1 (down), 0 (center), +1 (up)
    if up == "J2_Y" && args.is_empty() {
        add_native_call_comment(out, "J2_Y");
        out.push_str("; J2_Y() - Read Joystick 2 Y axis (BIOS)\n");
        out.push_str("    LDA #5\n");
        out.push_str("    STA $C821    ; Vec_Joy_Mux_2_X\n");
        out.push_str("    LDA #7\n");
        out.push_str("    STA $C822    ; Vec_Joy_Mux_2_Y\n");
        out.push_str("    JSR $F1F8    ; Joy_Digital\n");
        out.push_str("    LDB $C81E    ; Vec_Joy_2_Y\n");
        out.push_str("    SEX\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J2_BUTTON_1: Read Joystick 2 button 1 via BIOS Read_Btns
    // Returns 0 if released, 1 if pressed
    if up == "J2_BUTTON_1" && args.is_empty() {
        add_native_call_comment(out, "J2_BUTTON_1");
        out.push_str("; J2_BUTTON_1() - Read Joystick 2 button 1 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$10    ; J2 button 1 (bit 4)\n");
        out.push_str("    BEQ .j2b1_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j2b1_done\n");
        out.push_str(".j2b1_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j2b1_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J2_BUTTON_2: Read Joystick 2 button 2 via BIOS Read_Btns
    if up == "J2_BUTTON_2" && args.is_empty() {
        add_native_call_comment(out, "J2_BUTTON_2");
        out.push_str("; J2_BUTTON_2() - Read Joystick 2 button 2 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$20    ; J2 button 2 (bit 5)\n");
        out.push_str("    BEQ .j2b2_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j2b2_done\n");
        out.push_str(".j2b2_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j2b2_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J2_BUTTON_3: Read Joystick 2 button 3 via BIOS Read_Btns
    if up == "J2_BUTTON_3" && args.is_empty() {
        add_native_call_comment(out, "J2_BUTTON_3");
        out.push_str("; J2_BUTTON_3() - Read Joystick 2 button 3 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$40    ; J2 button 3 (bit 6)\n");
        out.push_str("    BEQ .j2b3_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j2b3_done\n");
        out.push_str(".j2b3_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j2b3_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
    // J2_BUTTON_4: Read Joystick 2 button 4 via BIOS Read_Btns
    if up == "J2_BUTTON_4" && args.is_empty() {
        add_native_call_comment(out, "J2_BUTTON_4");
        out.push_str("; J2_BUTTON_4() - Read Joystick 2 button 4 (BIOS)\n");
        out.push_str("    JSR $F1BA    ; Read_Btns\n");
        out.push_str("    LDA $C80F    ; Vec_Btn_State\n");
        out.push_str("    ANDA #$80    ; J2 button 4 (bit 7)\n");
        out.push_str("    BEQ .j2b4_not_pressed\n");
        out.push_str("    LDD #1\n");
        out.push_str("    BRA .j2b4_done\n");
        out.push_str(".j2b4_not_pressed:\n");
        out.push_str("    LDD #0\n");
        out.push_str(".j2b4_done:\n");
        out.push_str("    STD RESULT\n");
        return true;
    }
    
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
    
    // ASM: Inline assembly - emit raw string directly
    if up == "ASM" && args.len() == 1 {
        if let Expr::StringLit(asm_code) = &args[0] {
            out.push_str(&format!("    {}\n", asm_code));
            return true;
        }
    }
    
    // DRAW_VECTOR_LIST: Malban's complete algorithm for processing vector lists
    // Usage: DRAW_VECTOR_LIST(list_label, y, x, scale)
    // Generates the full frame init + vector list iteration code from VIDE
    if up == "DRAW_VECTOR_LIST" && args.len() == 4 {
        // Extract list label (string or ident)
        let list_label = match &args[0] {
            Expr::StringLit(s) => s.clone(),
            Expr::Ident(id) => id.name.clone(),
            _ => {
                out.push_str("; ERROR: DRAW_VECTOR_LIST requires label as first arg\n");
                return true;
            }
        };
        
        // Evaluate other arguments to RESULT/vars (single bytes)
        // y position
        emit_expr(&args[1], out, fctx, string_map, opts);
        out.push_str("    LDA RESULT+1\n    STA VL_Y\n");
        
        // x position  
        emit_expr(&args[2], out, fctx, string_map, opts);
        out.push_str("    LDA RESULT+1\n    STA VL_X\n");
        
        // scale
        emit_expr(&args[3], out, fctx, string_map, opts);
        out.push_str("    LDA RESULT+1\n    STA VL_SCALE\n");
        
        // Generate Malban algorithm inline (replicate VIDE output)
        let list_sym = format!("_{}", list_label.to_uppercase());
        out.push_str(&format!("; DRAW_VECTOR_LIST({}, y, x, scale) - Malban algorithm\n", list_label));
        out.push_str(&format!("    LDX #{}\n", list_sym));
        out.push_str("    STX VL_PTR\n");
        
        // DO-WHILE loop label
        out.push_str("VL_LOOP_START:\n");
        
        // Frame initialization sequence (Malban lines 13-43 from VIDE ASM)
        out.push_str("    CLR $D05A           ; VIA_shift_reg = 0 (blank beam)\n");
        out.push_str("    LDA #$CC\n");
        out.push_str("    STA $D00B           ; VIA_cntl = 0xCC (zero integrators)\n");
        out.push_str("    CLR $D000           ; VIA_port_a = 0 (reset offset)\n");
        out.push_str("    LDA #$82\n");
        out.push_str("    STA $D002           ; VIA_port_b = 0x82\n");
        out.push_str("    LDA VL_SCALE\n");
        out.push_str("    STA $D004           ; VIA_t1_cnt_lo = scale\n");
        
        // Delay loop (5 iterations for beam settling)
        out.push_str("    LDB #5              ; ZERO_DELAY\n");
        out.push_str("VL_DELAY:\n");
        out.push_str("    DECB\n");
        out.push_str("    BNE VL_DELAY\n");
        
        out.push_str("    LDA #$83\n");
        out.push_str("    STA $D002           ; VIA_port_b = 0x83\n");
        
        // Move to initial position (y, x)
        out.push_str("    LDA VL_Y\n");
        out.push_str("    STA $D000           ; VIA_port_a = y\n");
        out.push_str("    LDA #$CE\n");
        out.push_str("    STA $D00B           ; VIA_cntl = 0xCE (integrator mode)\n");
        out.push_str("    CLR $D002           ; VIA_port_b = 0 (mux enable)\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $D002           ; VIA_port_b = 1 (mux disable)\n");
        out.push_str("    LDA VL_X\n");
        out.push_str("    STA $D000           ; VIA_port_a = x\n");
        out.push_str("    CLR $D005           ; VIA_t1_cnt_hi = 0 (start timer)\n");
        
        // Set scale for vector drawing
        out.push_str("    LDA VL_SCALE\n");
        out.push_str("    STA $D004           ; VIA_t1_cnt_lo = scale\n");
        
        // Advance pointer past header (u += 3)
        out.push_str("    LDX VL_PTR\n");
        out.push_str("    LEAX 3,X\n");
        out.push_str("    STX VL_PTR\n");
        
        // Wait for move to complete
        out.push_str("VL_WAIT_MOVE:\n");
        out.push_str("    LDA $D00D           ; VIA_int_flags\n");
        out.push_str("    ANDA #$40\n");
        out.push_str("    BEQ VL_WAIT_MOVE\n");
        
        // Vector list processing loop (WHILE(1))
        out.push_str("VL_PROCESS_LOOP:\n");
        out.push_str("    LDX VL_PTR\n");
        out.push_str("    LDA ,X              ; Load flag byte (*u)\n");
        out.push_str("    TSTA\n");
        out.push_str("    BPL VL_CHECK_MOVE   ; If >= 0, not a draw\n");
        
        // DRAW LINE (*u < 0)
        out.push_str("VL_DRAW:\n");
        out.push_str("    LDA 1,X             ; dy\n");
        out.push_str("    STA $D000           ; VIA_port_a = dy\n");
        out.push_str("    CLR $D002           ; VIA_port_b = 0\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $D002           ; VIA_port_b = 1\n");
        out.push_str("    LDA 2,X             ; dx\n");
        out.push_str("    STA $D000           ; VIA_port_a = dx\n");
        out.push_str("    CLR $D005           ; VIA_t1_cnt_hi = 0\n");
        out.push_str("    LDA #$FF\n");
        out.push_str("    STA $D05A           ; VIA_shift_reg = 0xFF (beam ON)\n");
        out.push_str("VL_WAIT_DRAW:\n");
        out.push_str("    LDA $D00D\n");
        out.push_str("    ANDA #$40\n");
        out.push_str("    BEQ VL_WAIT_DRAW\n");
        out.push_str("    CLR $D05A           ; VIA_shift_reg = 0 (beam OFF)\n");
        out.push_str("    BRA VL_CONTINUE\n");
        
        // MOVE TO (*u == 0)
        out.push_str("VL_CHECK_MOVE:\n");
        out.push_str("    TSTA\n");
        out.push_str("    BNE VL_CHECK_END    ; If != 0, check for end\n");
        out.push_str("    ; MoveTo logic (similar to draw but no beam)\n");
        out.push_str("    LDA 1,X             ; dy\n");
        out.push_str("    BEQ VL_CHECK_DX\n");
        out.push_str("VL_DO_MOVE:\n");
        out.push_str("    STA $D000           ; VIA_port_a = dy\n");
        out.push_str("    LDA #$CE\n");
        out.push_str("    STA $D00B           ; VIA_cntl = 0xCE\n");
        out.push_str("    CLR $D002\n");
        out.push_str("    LDA #1\n");
        out.push_str("    STA $D002\n");
        out.push_str("    LDA 2,X             ; dx\n");
        out.push_str("    STA $D000\n");
        out.push_str("    CLR $D005\n");
        out.push_str("VL_WAIT_MOVE2:\n");
        out.push_str("    LDA $D00D\n");
        out.push_str("    ANDA #$40\n");
        out.push_str("    BEQ VL_WAIT_MOVE2\n");
        out.push_str("    BRA VL_CONTINUE\n");
        out.push_str("VL_CHECK_DX:\n");
        out.push_str("    LDA 2,X\n");
        out.push_str("    BNE VL_DO_MOVE\n");
        out.push_str("    BRA VL_CONTINUE\n");
        
        // Check for end marker (2)
        out.push_str("VL_CHECK_END:\n");
        out.push_str("    CMPA #2\n");
        out.push_str("    BEQ VL_DONE         ; Exit if *u == 2\n");
        
        // Continue to next entry (u += 3)
        out.push_str("VL_CONTINUE:\n");
        out.push_str("    LDX VL_PTR\n");
        out.push_str("    LEAX 3,X\n");
        out.push_str("    STX VL_PTR\n");
        out.push_str("    BRA VL_PROCESS_LOOP\n");
        
        out.push_str("VL_DONE:\n");
        out.push_str("    ; DO-WHILE check: if more lists, loop to VL_LOOP_START\n");
        out.push_str("    ; For single list, we're done\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        
        return true;
    }
    
    // DRAW_LINE optimization: when all args are numeric constants, generate inline BIOS calls
    // NO hace Moveto - el usuario debe posicionarse antes con MOVETO si es necesario
    if up == "DRAW_LINE" && args.len() == 5 && args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(x0), Expr::Number(y0), Expr::Number(x1), Expr::Number(y1), Expr::Number(intensity)) 
            = (&args[0], &args[1], &args[2], &args[3], &args[4]) {
            // Calculate deltas from absolute coordinates
            let dx = (*x1 - *x0) as i8;
            let dy = (*y1 - *y0) as i8;
            
            // Set DP and intensity
            out.push_str("    LDA #$D0\n    TFR A,DP\n");
            if *intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", *intensity as u8));
            }
            // Clear Vec_Misc_Count for proper timing
            out.push_str("    CLR Vec_Misc_Count\n");
            // Draw line using RELATIVE deltas (A=dy, B=dx)
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                dy as u8, dx as u8));
            // Restore DP after BIOS call
            out.push_str("    LDA #$C8\n    TFR A,DP\n");
            out.push_str("    LDD #0\n    STD RESULT\n");
            return true;
        }
    }
    
    // DRAW_LINE fallback: if not all constants, use wrapper
    if up == "DRAW_LINE" && args.len() == 5 {
        // Optimized argument setup - emit directly to VAR_ARG when possible
        for (i, arg) in args.iter().enumerate() {
            match arg {
                Expr::Number(n) => {
                    // Direct constant: skip RESULT, write directly to VAR_ARG
                    out.push_str(&format!("    LDD #{}\n    STD VAR_ARG{}\n", *n & 0xFFFF, i));
                }
                _ => {
                    // Complex expression: use RESULT intermediate
                    emit_expr(arg, out, fctx, string_map, opts);
                    out.push_str(&format!("    LDD RESULT\n    STD VAR_ARG{}\n", i));
                }
            }
        }
        out.push_str("    JSR DRAW_LINE_WRAPPER\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
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
                            // OPTIMIZED MODE: Set intensity and DP once, then draw all edges efficiently
                            // Set intensity once for all edges
                            if intensity == 0x5F { out.push_str("    JSR Intensity_5F\n"); } else { out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF)); }
                            // Set DP once for all VIA operations (inline for now)
                            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
                            
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
                                
                                // Only reset origin for first edge, others are connected
                                if i == 0 {
                                    out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", (y0 & 0xFF), (x0 & 0xFF)));
                                }
                                out.push_str("    CLR Vec_Misc_Count\n");
                                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", (first_dy & 0xFF), (first_dx & 0xFF)));
                                if second {
                                    out.push_str("    CLR Vec_Misc_Count\n");
                                    out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", (second_dy & 0xFF), (second_dx & 0xFF)));
                                }
                            }
                            out.push_str("    LDD #0\n    STD RESULT\n");
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
                    out.push_str("    LDD #0\n    STD RESULT\n");
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
            return emit_builtin_call(&new_up, args, out, fctx, string_map, opts, line_info);
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
    
    // Add native call tracking comment before JSR
    add_native_call_comment(out, &up);
    
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
fn emit_stmt(stmt: &Stmt, out: &mut String, loop_ctx: &LoopCtx, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, tracker: &mut LineTracker, depth: usize) {
    // Safety: Prevent stack overflow with deep recursion
    const MAX_DEPTH: usize = 500;
    if depth > MAX_DEPTH {
        panic!("Maximum statement nesting depth ({}) exceeded. Please simplify your code or split into smaller functions.", MAX_DEPTH);
    }
    
    // ✅ CRITICAL: Record source line BEFORE emitting code
    let line = stmt.source_line();
    tracker.set_line(line);
    
    // Emit line marker comment for ASM parsing to reconstruct accurate lineMap
    out.push_str(&format!("    ; VPy_LINE:{}\n", line));
    
    match stmt {
        Stmt::Assign { target, value, .. } => {
            match target {
                crate::ast::AssignTarget::Ident { name, .. } => {
                    emit_expr(value, out, fctx, string_map, opts);
                    if let Some(off) = fctx.offset_of(name) {
                        out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off));
                    } else {
                        out.push_str(&format!("    LDX RESULT\n    LDU #VAR_{}\n    STU TMPPTR\n    STX ,U\n", name.to_uppercase()));
                    }
                }
                crate::ast::AssignTarget::Index { target: array_expr, index, .. } => {
                    // Array indexed assignment: arr[index] = value
                    // For now, only support simple variable arrays
                    let array_name = if let Expr::Ident(id) = &**array_expr {
                        &id.name
                    } else {
                        panic!("Complex array expressions not yet supported in assignment");
                    };
                    
                    // 1. First, get the array base address
                    let array_addr = if let Some(off) = fctx.offset_of(array_name) {
                        format!("{} ,S", off)
                    } else {
                        format!("VAR_{}", array_name.to_uppercase())
                    };
                    
                    // 2. Evaluate index
                    emit_expr(index, out, fctx, string_map, opts);
                    out.push_str("    LDD RESULT\n    ASLB\n    ROLA\n"); // index * 2
                    
                    // 3. Add to base address (load base into X first)
                    out.push_str(&format!("    LDX #{}\n", array_addr)); // X = &array
                    out.push_str("    LEAX D,X\n"); // X = &array + (index * 2)
                    out.push_str("    STX TMPPTR\n"); // Save computed address
                    
                    // 4. Evaluate value to assign
                    emit_expr(value, out, fctx, string_map, opts);
                    
                    // 5. Store value at computed address
                    out.push_str("    LDX TMPPTR\n    LDD RESULT\n    STD ,X\n");
                }
            }
        }
        Stmt::Let { name, value, .. } => {
            emit_expr(value, out, fctx, string_map, opts);
            if let Some(off) = fctx.offset_of(name) { out.push_str(&format!("    LDX RESULT\n    STX {} ,S\n", off)); }
        }
    Stmt::Expr(e, _) => emit_expr(e, out, fctx, string_map, opts),
        Stmt::Return(o, _) => {
            if let Some(e) = o { emit_expr(e, out, fctx, string_map, opts); }
            if fctx.frame_size > 0 { out.push_str(&format!("    LEAS {} ,S ; free locals\n", fctx.frame_size)); }
            out.push_str("    RTS\n");
        }
        Stmt::Break { .. } => {
            if let Some(end) = &loop_ctx.end {
                out.push_str(&format!("    BRA {}\n", end));
            }
        }
        Stmt::Continue { .. } => {
            if let Some(st) = &loop_ctx.start {
                out.push_str(&format!("    BRA {}\n", st));
            }
        }
        Stmt::While { cond, body, .. } => {
            let ls = fresh_label("WH");
            let le = fresh_label("WH_END");
            out.push_str(&format!("{}: ; while start\n", ls));
            emit_expr(cond, out, fctx, string_map, opts);
            // Long branch to end
            out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", le));
            let inner = LoopCtx { start: Some(ls.clone()), end: Some(le.clone()) };
            for s in body { emit_stmt(s, out, &inner, fctx, string_map, opts, tracker, depth + 1); }
            out.push_str(&format!("    LBRA {}\n{}: ; while end\n", ls, le));
        }
        Stmt::For { var, start, end, step, body, .. } => {
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
            for s in body { emit_stmt(s, out, &inner, fctx, string_map, opts, tracker, depth + 1); }
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
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let end = fresh_label("IF_END");
            let mut next = fresh_label("IF_NEXT");
            let simple_if = elifs.is_empty() && else_body.is_none();
            emit_expr(cond, out, fctx, string_map, opts);
            out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", next));
            for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
            out.push_str(&format!("    LBRA {}\n", end));
            for (i, (c, b)) in elifs.iter().enumerate() {
                out.push_str(&format!("{}:\n", next));
                let new_next = if i == elifs.len() - 1 && else_body.is_none() { end.clone() } else { fresh_label("IF_NEXT") };
                emit_expr(c, out, fctx, string_map, opts);
                out.push_str(&format!("    LDD RESULT\n    LBEQ {}\n", new_next));
                for s in b { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
                out.push_str(&format!("    LBRA {}\n", end));
                next = new_next;
            }
            if let Some(eb) = else_body {
                out.push_str(&format!("{}:\n", next));
                for s in eb { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
            } else if !elifs.is_empty() || simple_if {
                // Only emit next label if it's different from end
                if next != end {
                    out.push_str(&format!("{}:\n", next));
                }
            }
            out.push_str(&format!("{}:\n", end));
        }
        Stmt::Switch { expr, cases, default, .. } => {
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
                        for s in *body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
                        out.push_str(&format!("    LBRA {}\n", end));
                    }
                    if let Some(dl) = &def_label {
                        out.push_str(&format!("{}:\n", dl));
                        for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
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
                for s in body { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
                out.push_str(&format!("    LBRA {}\n", end));
            }
            if let Some(dl) = def_label {
                out.push_str(&format!("{}:\n", dl));
                for s in default.as_ref().unwrap() { emit_stmt(s, out, loop_ctx, fctx, string_map, opts, tracker, depth + 1); }
            }
            out.push_str(&format!("{}:\n", end));
        },
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before emit_stmt"),
    }
}

// emit_expr: lower expressions; result placed in RESULT.
// Nota: En 6809 las operaciones sobre D ya limitan a 16 bits; no hace falta 'mask' explícito.
fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
    emit_expr_depth(expr, out, fctx, string_map, opts, 0);
}

fn emit_expr_depth(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, depth: usize) {
    // Safety: Prevent stack overflow with deep recursion
    const MAX_DEPTH: usize = 500;
    if depth > MAX_DEPTH {
        panic!("Maximum expression nesting depth ({}) exceeded. Please simplify your expressions or split into smaller parts.", MAX_DEPTH);
    }
    if depth > 450 {
        eprintln!("WARNING: Deep recursion at depth {} in emit_expr", depth);
    }
    
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("    LDD #{}\n    STD RESULT\n", *n));
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
            if emit_builtin_call(&ci.name, &ci.args, out, fctx, string_map, opts, Some(ci.source_line)) { return; }
            for (i, arg) in ci.args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr_depth(arg, out, fctx, string_map, opts, depth + 1);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            
            // Special case: in auto_loop mode, loop() function calls should use LOOP_BODY
            let target_name = if opts.auto_loop && ci.name.to_lowercase() == "loop" {
                "LOOP_BODY".to_string()
            } else {
                ci.name.to_uppercase()
            };
            
            if opts.force_extended_jsr { 
                out.push_str(&format!("    JSR >{}\n", target_name)); 
            } else { 
                out.push_str(&format!("    JSR {}\n", target_name)); 
            }
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
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
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
            match op {
                BinOp::Add => out.push_str("    LDD TMPLEFT\n    ADDD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Sub => out.push_str("    LDD TMPLEFT\n    SUBD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Mul => out.push_str("    LDD TMPLEFT\n    STD MUL_A\n    LDD TMPRIGHT\n    STD MUL_B\n    JSR MUL16\n"),
                BinOp::Div => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n"),
                BinOp::FloorDiv => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n"), // División entera igual que Div en enteros
                BinOp::Mod => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n    ; quotient in RESULT, need remainder: A - Q*B\n    LDD DIV_A\n    STD TMPLEFT\n    LDD RESULT\n    STD MUL_A\n    LDD DIV_B\n    STD MUL_B\n    JSR MUL16\n    ; product in RESULT, subtract from original A (TMPLEFT)\n    LDD TMPLEFT\n    SUBD RESULT\n    STD RESULT\n"),
                BinOp::Shl => out.push_str("    LDD TMPLEFT\nSHL_LOOP: LDA TMPRIGHT+1\n    BEQ SHL_DONE\n    ASLB\n    ROLA\n    DEC TMPRIGHT+1\n    BRA SHL_LOOP\nSHL_DONE: STD RESULT\n"),
                BinOp::Shr => out.push_str("    LDD TMPLEFT\nSHR_LOOP: LDA TMPRIGHT+1\n    BEQ SHR_DONE\n    LSRA\n    RORB\n    DEC TMPRIGHT+1\n    BRA SHR_LOOP\nSHR_DONE: STD RESULT\n"),
                BinOp::BitAnd => out.push_str("    LDD TMPLEFT\n    ANDA TMPRIGHT+1\n    ANDB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitOr  => out.push_str("    LDD TMPLEFT\n    ORA TMPRIGHT+1\n    ORB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitXor => out.push_str("    LDD TMPLEFT\n    EORA TMPRIGHT+1\n    EORB TMPRIGHT\n    STD RESULT\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n    LDD TMPLEFT\n    SUBD TMPRIGHT\n");
            // DON'T overwrite result before branch - execute branch immediately after SUBD
            let lt = fresh_label("CT");
            let end = fresh_label("CE");
            let br = match op { CmpOp::Eq => "BEQ", CmpOp::Ne => "BNE", CmpOp::Lt => "BLT", CmpOp::Le => "BLE", CmpOp::Gt => "BGT", CmpOp::Ge => "BGE" };
            out.push_str(&format!(
                "    {} {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                br, lt, end, lt, end
            ));
        }
        Expr::Logic { op, left, right } => match op {
            LogicOp::And => {
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1);
            out.push_str(
                "    LDD RESULT\n    BEQ NOT_TRUE\n    LDD #0\n    STD RESULT\n    BRA NOT_END\nNOT_TRUE:\n    LDD #1\n    STD RESULT\nNOT_END:\n",
            );
        }
        Expr::List(elements) => {
            // Array literal: allocate space and initialize elements
            // For now, we'll allocate a temporary array in DATA section
            // This is a compile-time constant array
            let array_label = fresh_label("ARRAY");
            
            // Generate array data in DATA section (will be moved later)
            // For now, just emit code to load the array address
            out.push_str(&format!("; TODO: Array literal initialization for {} elements\n", elements.len()));
            out.push_str(&format!("    LDX #{}\n    STX RESULT\n", array_label));
            
            // TODO: Proper array allocation and initialization
            // Need to either:
            // 1. Pre-allocate in DATA section (for constants)
            // 2. Allocate on stack (for local arrays)
            // 3. Use a memory pool (for dynamic arrays)
        }
        Expr::Index { target, index } => {
            // Array indexing: arr[index]
            // 1. Evaluate array expression (should return address)
            emit_expr_depth(target, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPPTR\n"); // Save array base address
            
            // 2. Evaluate index expression
            emit_expr_depth(index, out, fctx, string_map, opts, depth + 1);
            
            // 3. Multiply index by 2 (each element is 2 bytes)
            out.push_str("    LDD RESULT\n    ASLB\n    ROLA\n"); // D = index * 2
            
            // 4. Add to base address
            out.push_str("    ADDD TMPPTR\n"); // D = base + (index * 2)
            out.push_str("    TFR D,X\n"); // X = address of element
            
            // 5. Load value from computed address
            out.push_str("    LDD ,X\n    STD RESULT\n");
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

// collect_all_vars: gather ALL variable identifiers used in the module (including locals)
// In the Vectrex context, all variables need DATA section definitions regardless of scope
fn collect_all_vars(module: &Module) -> Vec<String> {
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

// NEW: Collect global variables with their initial values
fn collect_global_vars(module: &Module) -> Vec<(String, Expr)> {
    let mut vars = Vec::new();
    for item in &module.items {
        if let Item::GlobalLet { name, value } = item {
            vars.push((name.clone(), value.clone()));
        }
    }
    vars
}

// collect_stmt_syms: process statement symbols.
fn collect_stmt_syms(stmt: &Stmt, set: &mut std::collections::BTreeSet<String>) {
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
    Stmt::Let { name: _, value, .. } => { collect_expr_syms(value, set); } // exclude locals
        Stmt::Expr(e, _) => collect_expr_syms(e, set),
    Stmt::For { var: _, start, end, step, body, .. } => {
            // treat induction var as global only if not a local (decision deferred to emit)
            collect_expr_syms(start, set);
            collect_expr_syms(end, set);
            if let Some(se) = step { collect_expr_syms(se, set); }
            for s in body { collect_stmt_syms(s, set); }
        }
        Stmt::While { cond, body, .. } => {
            collect_expr_syms(cond, set);
            for s in body { collect_stmt_syms(s, set); }
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
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
        Stmt::Return(o, _) => { if let Some(e) = o { collect_expr_syms(e, set); } }
        Stmt::Switch { expr, cases, default, .. } => {
            collect_expr_syms(expr, set);
            for (ce, cb) in cases { collect_expr_syms(ce, set); for s in cb { collect_stmt_syms(s, set); } }
            if let Some(db) = default { for s in db { collect_stmt_syms(s, set); } }
        }
        Stmt::Break { .. } | Stmt::Continue { .. } => {},
        Stmt::CompoundAssign { target, value, .. } => {
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
