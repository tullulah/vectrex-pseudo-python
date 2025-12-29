// M6809 Backend - Modular structure
// Segregated from original monolithic file for better maintainability

// Sub-modules
mod utils;
mod helpers;
mod builtins;
mod statements;
mod expressions;
mod analysis;
mod emission;
mod collectors;
mod ram_layout;

// Re-export for backward compatibility
pub use utils::*;
pub use helpers::*;
pub use builtins::*;
pub use statements::*;
pub use expressions::*;
pub use analysis::*;
pub use emission::*;
pub use collectors::*;
pub use ram_layout::*;

// Explicit imports for functions used in this module
use emission::{emit_function, emit_builtin_helpers};

// Original imports
use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
use super::string_literals::collect_string_literals;
use super::debug_info::{DebugInfo, LineTracker, parse_asm_addresses, parse_native_call_comments};
use crate::codegen::CodegenOptions;
use crate::backend::trig::emit_trig_tables;
use crate::target::{Target, TargetInfo};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::BTreeMap;

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
                // Check for DRAW_VECTOR("asset_name", x, y), DRAW_VECTOR_EX("asset_name", x, y, mirror, intensity), 
                // PLAY_MUSIC("asset_name"), or PLAY_SFX("asset_name")
                if (name_upper == "DRAW_VECTOR" && call_info.args.len() == 3) || 
                   (name_upper == "DRAW_VECTOR_EX" && call_info.args.len() == 5) ||
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
            Expr::MethodCall(mc) => {
                // Scan target and arguments for nested asset usages
                scan_expr(&mc.target, used, depth + 1);
                for arg in &mc.args {
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
            Stmt::ForIn { iterable, body, .. } => {
                scan_expr(iterable, used, depth + 1);
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
    // CORREGIDO: Usar solo variables GLOBALES, no todas (que incluye locales)
    let global_vars = collect_global_vars(module); // Collect global variables with initial values
    let global_vars_with_line = collect_global_vars_with_line(module); // WITH line numbers for PDB
    
    // Collect const declarations (go to ROM only, NO RAM allocation or initialization)
    let const_vars = collect_const_vars(module);
    let const_vars_with_line = collect_const_vars_with_line(module); // WITH line numbers for PDB
    
    // Build set of const array names to exclude from RAM allocation
    let const_array_names: std::collections::HashSet<String> = const_vars
        .iter()
        .filter_map(|(name, value)| {
            if matches!(value, Expr::List(_)) {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();
    
    // Create non-const variable list for RAM allocation (exclude const arrays)
    let non_const_vars: Vec<(String, Expr)> = global_vars
        .iter()
        .filter(|(name, _)| !const_array_names.contains(name))
        .cloned()
        .collect();
    
    let syms: Vec<String> = non_const_vars.iter().map(|(name, _)| name.clone()).collect(); // Only non-const global names
    let global_names = syms.clone(); // Same list for passing to collectors
    let string_map = collect_string_literals(module);
    
    // Recolectar constantes para inline en expresiones - actualizar opts
    let mut opts_with_consts = opts.clone(); // Clone opts to modify
    for item in &module.items {
        if let Item::Const { name, value, .. } = item {
            if let Expr::Number(n) = value {
                opts_with_consts.const_values.insert(name.to_uppercase(), *n);
            }
        }
    }
    
    // Poblar const_arrays map para indexación en tiempo de compilación
    let mut const_array_index = 0;
    for (name, value) in &const_vars {
        if matches!(value, Expr::List(_)) {
            opts_with_consts.const_arrays.insert(name.clone(), const_array_index);
            const_array_index += 1;
        }
    }
    
    // Poblar const_string_arrays set para identificar arrays de strings
    for (name, value) in &const_vars {
        if let Expr::List(elements) = value {
            // Check if all elements are StringLit
            let is_string_array = elements.iter().all(|e| matches!(e, Expr::StringLit(_)));
            if is_string_array {
                opts_with_consts.const_string_arrays.insert(name.clone());
            }
        }
    }
    
    let opts = &opts_with_consts; // Use the modified opts
    
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
    
    // Check for music/sfx assets (needed for RAM allocation)
    let has_music_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Music)
    });
    let has_sfx_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Sfx)
    });
    
    // ========================================================================
    // AUTOMATIC RAM LAYOUT - Calculates all offsets automatically
    // No more hardcoded offsets, no collisions, always compact
    // ========================================================================
    let mut ram = RamLayout::new(0xC880);
    
    // 1. RESULT (always needed)
    ram.allocate("RESULT", 2, "Main result temporary");
    
    // 2. Runtime temporaries (if needed)
    if !suppress_runtime && rt_usage.needs_tmp_left {
        ram.allocate("TMPLEFT", 2, "Left operand temp");
    }
    if !suppress_runtime && rt_usage.needs_tmp_right {
        ram.allocate("TMPRIGHT", 2, "Right operand temp");
    }
    if !suppress_runtime && rt_usage.needs_tmp_ptr {
        ram.allocate("TMPPTR", 2, "Pointer temp");
    }
    
    // 3. Multiply helper (if needed)
    if rt_usage.needs_mul_helper {
        ram.allocate("MUL_A", 2, "Multiplicand A");
        ram.allocate("MUL_B", 2, "Multiplicand B");
        ram.allocate("MUL_RES", 2, "Multiply result");
        ram.allocate("MUL_TMP", 2, "Multiply temporary");
        ram.allocate("MUL_CNT", 2, "Multiply counter");
    }
    
    // 4. Division helper (if needed)
    if rt_usage.needs_div_helper {
        ram.allocate("DIV_A", 2, "Dividend");
        ram.allocate("DIV_B", 2, "Divisor");
        ram.allocate("DIV_Q", 2, "Quotient");
        ram.allocate("DIV_R", 2, "Remainder");
    }
    
    // 5. Coordinate temporaries (for Draw_Sync_List)
    ram.allocate("TEMP_YX", 2, "Temporary y,x storage");
    ram.allocate("TEMP_X", 1, "Temporary x storage");
    ram.allocate("TEMP_Y", 1, "Temporary y storage");
    
    // 6. PSG Music variables (if music assets exist)
    if has_music_assets {
        ram.allocate("PSG_MUSIC_PTR", 2, "Current music position pointer");
        ram.allocate("PSG_MUSIC_START", 2, "Music start pointer (for loops)");
        ram.allocate("PSG_IS_PLAYING", 1, "Playing flag ($00=stopped, $01=playing)");
        ram.allocate("PSG_MUSIC_ACTIVE", 1, "Set during UPDATE_MUSIC_PSG");
        ram.allocate("PSG_FRAME_COUNT", 1, "Frame register write count");
        ram.allocate("PSG_DELAY_FRAMES", 1, "Frames to wait before next read");
    }
    
    // 7. SFX variables (if SFX assets exist)
    if has_sfx_assets {
        ram.allocate("SFX_PTR", 2, "Current SFX data pointer");
        ram.allocate("SFX_TICK", 2, "Current frame counter");
        ram.allocate("SFX_ACTIVE", 1, "Playback state ($00=stopped, $01=playing)");
        ram.allocate("SFX_PHASE", 1, "Envelope phase (0=A,1=D,2=S,3=R)");
        ram.allocate("SFX_VOL", 1, "Current volume level (0-15)");
    }
    
    // 8. PRINT_NUMBER buffer (always allocate if not suppressed)
    if !suppress_runtime {
        ram.allocate("NUM_STR", 2, "String buffer for PRINT_NUMBER");
    }
    
    out.push_str("\n; === RAM VARIABLE DEFINITIONS (EQU) ===\n");
    out.push_str("; AUTO-GENERATED - All offsets calculated automatically\n");
    out.push_str(&format!("; Total RAM used: {} bytes\n", ram.total_size()));
    out.push_str(&ram.emit_equ_definitions());
    
    // DP-relative offsets for PSG (lwasm compatibility)
    if has_music_assets {
        if let Some(offset) = ram.get_offset("PSG_MUSIC_PTR") {
            out.push_str(&format!("PSG_MUSIC_PTR_DP   EQU ${:02X}  ; DP-relative\n", offset));
        }
        if let Some(offset) = ram.get_offset("PSG_MUSIC_START") {
            out.push_str(&format!("PSG_MUSIC_START_DP EQU ${:02X}  ; DP-relative\n", offset));
        }
        if let Some(offset) = ram.get_offset("PSG_IS_PLAYING") {
            out.push_str(&format!("PSG_IS_PLAYING_DP  EQU ${:02X}  ; DP-relative\n", offset));
        }
        if let Some(offset) = ram.get_offset("PSG_MUSIC_ACTIVE") {
            out.push_str(&format!("PSG_MUSIC_ACTIVE_DP EQU ${:02X}  ; DP-relative\n", offset));
        }
        if let Some(offset) = ram.get_offset("PSG_FRAME_COUNT") {
            out.push_str(&format!("PSG_FRAME_COUNT_DP EQU ${:02X}  ; DP-relative\n", offset));
        }
        if let Some(offset) = ram.get_offset("PSG_DELAY_FRAMES") {
            out.push_str(&format!("PSG_DELAY_FRAMES_DP EQU ${:02X}  ; DP-relative\n", offset));
        }
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
        
        // ✅ Emit line markers for const NUMBER declarations (not arrays)
        // These are inlined in expressions, so we need to record them for PDB coverage
        // Place them here (after ORG, before START) so they have valid addresses
        out.push_str(";**** CONST DECLARATIONS (NUMBER-ONLY) ****\n");
        let mut const_decl_counter = 0;
        for (name, value, source_line) in &const_vars_with_line {
            // Only emit for non-array consts (arrays already handled)
            if !matches!(value, Expr::List(_)) {
                tracker.set_line(*source_line);
                out.push_str(&format!("; VPy_LINE:{}\n", source_line));
                // Emit a dummy label for parser to register the pending marker
                out.push_str(&format!("; _CONST_DECL_{}:  ; const {}\n", const_decl_counter, name));
                const_decl_counter += 1;
            }
        }
        out.push_str("\n");
        
        // Emit builtin helpers BEFORE program code (fixes forward reference issues)
        if !suppress_runtime {
            emit_builtin_helpers(&mut out, &rt_usage, opts, module, &mut debug_info);
        }
        
        out.push_str("START:\n    LDA #$D0\n    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)\n    LDA #$80\n    STA VIA_t1_cnt_lo\n    LDX #Vec_Default_Stk\n    TFR X,S\n");
        
        // Check if we have music/sfx assets that need PSG initialization
        let has_audio_assets = opts.assets.iter().any(|a| {
            matches!(a.asset_type, crate::codegen::AssetType::Music | crate::codegen::AssetType::Sfx)
        });
        
        // Check specifically for SFX assets
        let has_sfx_assets = opts.assets.iter().any(|a| {
            matches!(a.asset_type, crate::codegen::AssetType::Sfx)
        });
        
        // BIOS music system: Initialize music buffer to silence
        if has_audio_assets {
            out.push_str("    JSR $F533       ; Init_Music_Buf - Initialize BIOS music system to silence\n");
        }
        
        // CRITICAL: Initialize SFX system variables to prevent garbage data interference
        if has_sfx_assets {
            out.push_str("    ; Initialize SFX variables to prevent random noise on startup\n");
            out.push_str("    CLR sfx_status         ; Mark SFX as inactive (0=off)\n");
            out.push_str("    LDD #$0000\n");
            out.push_str("    STD sfx_pointer        ; Clear SFX pointer\n");
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
            
            // Initialize global variables with their initial values (ONCE at startup)
            // Use global_vars_with_line to emit line markers for PDB coverage
            let mut array_counter = 0;
            for (name, value, source_line) in &global_vars_with_line {
                // Skip const arrays (they're already in ROM)
                if const_array_names.contains(name) {
                    continue;
                }
                
                // Register line number in PDB
                tracker.set_line(*source_line);
                out.push_str(&format!("    ; VPy_LINE:{}\n", source_line));
                
                if let Expr::List(_elements) = value {
                    // Array literal: load address of pre-generated array data
                    let array_label = format!("ARRAY_{}", array_counter);
                    out.push_str(&format!("    LDX #{}    ; Array literal\n", array_label));
                    out.push_str(&format!("    STX VAR_{}\n", name.to_uppercase()));
                    array_counter += 1;
                } else if let Expr::Number(n) = value {
                    out.push_str(&format!("    LDD #{}\n", n));
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                } else if let Expr::StringLit(s) = value {
                    // String literal: load address of string from string_map
                    if let Some(label) = string_map.get(s) {
                        out.push_str(&format!("    LDX #{}    ; String literal\n", label));
                        out.push_str(&format!("    STX VAR_{}\n", name.to_uppercase()));
                    } else {
                        // Fallback: initialize to 0 if string not in map
                        out.push_str(&format!("    LDD #0    ; String not found in map\n"));
                        out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                    }
                } else {
                    // For non-constant initial values, evaluate the expression
                    emit_expr(value, &mut out, &FuncCtx { locals: Vec::new(), frame_size: 0, var_info: std::collections::HashMap::new(), struct_type: None }, &string_map, opts);
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                }
            }
            
            if let Some(main_func) = user_main {
                let fctx = FuncCtx { locals: Vec::new(), frame_size: 0, var_info: std::collections::HashMap::new(), struct_type: None };
                for stmt in &main_func.body {
                    emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts, &mut tracker, 0);
                }
            }
        }
        
        // Choose label based on whether we have START or not
        let main_label = if main_has_content { "MAIN" } else { "main" };
        
        // ✅ Register main() definition line in .pdb
        if let Some(main_func) = user_main {
            tracker.set_line(main_func.line);
            out.push_str(&format!("; VPy_LINE:{}\n", main_func.line));
        }
        
        out.push_str(&format!("\n{}:\n", main_label));
        
        out.push_str("    JSR Wait_Recal\n    LDA #$80\n    STA VIA_t1_cnt_lo\n");
        // NOTE: UPDATE_MUSIC_PSG now called at START of LOOP_BODY, not here
        
        // CRITICAL: Initialize global variables even if main() has no content
        // This must happen ONCE before the loop starts
        // IMPORTANT: DO NOT initialize const arrays - they only exist in ROM
        // Use global_vars_with_line to emit line markers for PDB coverage
        if !main_has_content && !global_vars_with_line.is_empty() {
            out.push_str("    ; Initialize global variables (excluding const arrays)\n");
            let mut array_counter = 0;
            for (name, value, source_line) in &global_vars_with_line {
                // Skip const arrays (they're already in ROM)
                if const_array_names.contains(name) {
                    continue;
                }
                
                // Register line number in PDB
                tracker.set_line(*source_line);
                out.push_str(&format!("    ; VPy_LINE:{}\n", source_line));
                
                if let Expr::List(_elements) = value {
                    // Array literal: load address of pre-generated array data
                    let array_label = format!("ARRAY_{}", array_counter);
                    out.push_str(&format!("    LDX #{}    ; Array literal\n", array_label));
                    out.push_str(&format!("    STX VAR_{}\n", name.to_uppercase()));
                    array_counter += 1;
                } else if let Expr::Number(n) = value {
                    out.push_str(&format!("    LDD #{}\n", n));
                    out.push_str(&format!("    STD VAR_{}\n", name.to_uppercase()));
                } else {
                    // For non-constant initial values, evaluate the expression
                    emit_expr(value, &mut out, &FuncCtx { locals: Vec::new(), frame_size: 0, var_info: std::collections::HashMap::new(), struct_type: None }, &string_map, opts);
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
                    // ✅ CRITICAL: Record the loop() definition line in .pdb
                    tracker.set_line(f.line);
                    out.push_str(&format!("    ; VPy_LINE:{}\n", f.line));
                    
                    // Emit loop function as LOOP_BODY subroutine to avoid code duplication
                    out.push_str("LOOP_BODY:\n");
                    
                    // Collect locals and allocate stack frame (same as emit_function)
                    let locals = collect_locals(&f.body, &global_names);
                    
                    // Analyze variable types for struct instances
                    let var_info = analyze_var_types(&f.body, &locals, &opts.structs);
                    
                    // Calculate frame size based on actual variable sizes
                    let mut frame_size = 0;
                    for var_name in &locals {
                        let size = var_info.get(var_name)
                            .map(|(_, s)| *s as i32)
                            .unwrap_or(2);
                        frame_size += size;
                    }
                    
                    if frame_size > 0 {
                        out.push_str(&format!("    LEAS -{},S ; allocate locals\n", frame_size));
                    }
                    
                    // Auto-inject AUDIO_UPDATE at START of loop (after WAIT_RECAL, before drawing)
                    // This ensures consistent 50Hz timing regardless of drawing complexity
                    // Music timing no longer fluctuates based on number of vectors drawn
                    if opts.has_audio(module) {
                        out.push_str("    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (consistent timing)\n");
                    }
                    
                    out.push_str(&format!("    ; DEBUG: Processing {} statements in loop() body\n", f.body.len()));
                    let fctx = FuncCtx { locals: locals.clone(), frame_size, var_info, struct_type: None };
                    for (i, stmt) in f.body.iter().enumerate() {
                        out.push_str(&format!("    ; DEBUG: Statement {} - {:?}\n", i, std::mem::discriminant(stmt)));
                        emit_stmt(stmt, &mut out, &LoopCtx::default(), &fctx, &string_map, opts, &mut tracker, 0);
                    }
                    
                    // Free locals before RTS (same as emit_function)
                    if frame_size > 0 {
                        out.push_str(&format!("    LEAS {},S ; free locals\n", frame_size));
                    }
                    out.push_str("    RTS\n\n");
                } else {
                    // Emit other functions normally
                    emit_function(f, &mut out, &string_map, opts, &mut tracker, &global_names);
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
            Item::Const { name, value, .. } => {
                let up = name.to_uppercase();
                if !emitted_consts.contains(&up) {
                    if let Expr::Number(n) = value { out.push_str(&format!("{} EQU {}\n", up, n & 0xFFFF)); }
                    emitted_consts.insert(up);
                }
            }
            Item::GlobalLet { name, value, .. } => {
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
            Item::StructDef(struct_def) => {
                // Phase 3 - struct definitions: emit methods as regular functions with mangled names
                // Method naming convention: StructName_method_name
                for method in &struct_def.methods {
                    let mangled_name = format!("{}_{}", struct_def.name, method.name);
                    
                    // Create a new function with mangled name and self as first parameter
                    let mut method_func = method.clone();
                    method_func.name = mangled_name;
                    
                    // Emit the method as a regular function
                    emit_function(&method_func, &mut out, &string_map, opts, &mut tracker, &global_names);
                }
                
                // If struct has constructor, emit initializer function
                if let Some(constructor) = &struct_def.constructor {
                    // Use same pattern as methods (NOT uppercase here)
                    // emit_function will uppercase the label, but struct_type extraction needs original case
                    let init_name = format!("{}_INIT", struct_def.name);
                    let mut init_func = constructor.clone();
                    init_func.name = init_name;
                    
                    // Constructor params start at ARG1 (ARG0 is struct pointer)
                    // Emit the constructor as a regular function
                    emit_function(&init_func, &mut out, &string_map, opts, &mut tracker, &global_names);
                }
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
    
    // Determine max args used (0..5) BEFORE evaluating suppress_runtime
    let max_args = compute_max_args_used(module);
    // Re-evaluate suppress_runtime now that we know max_args
    let no_runtime_vars_needed = !rt_usage.needs_tmp_left && !rt_usage.needs_tmp_right && 
                                 !rt_usage.needs_tmp_ptr && 
                                 !rt_usage.needs_mul_helper && !rt_usage.needs_div_helper && 
                                 !rt_usage.needs_line_vars && !rt_usage.needs_vcur_vars &&
                                 string_map.is_empty() && max_args == 0;
    suppress_runtime = main_inlined || no_runtime_vars_needed;
    
    // RAM variables: emit storage allocations if using ORG mode
    if suppress_runtime { /* skip RAM ORG and temp vars entirely */ }
    else if !opts.exclude_ram_org {
        out.push_str("    ORG $C880 ; begin runtime variables in RAM\n");
        out.push_str("; Variables (in RAM)\n");
        out.push_str(&ram.emit_storage_allocations());
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
            // Use $CF10 to avoid collision with debug RAM at $CF00-$CF03
            out.push_str(&format!("VAR_{} EQU $CF10+{}\n", v.to_uppercase(), var_offset));
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
            if max_args >=6 { out.push_str("VAR_ARG5: FDB 0\n"); }
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
                        // SFX uses new .vsfx format with parametric sound design
                        match crate::sfxres::SfxResource::load(std::path::Path::new(&asset.path)) {
                            Ok(resource) => {
                                out.push_str(&format!("; ========================================\n"));
                                out.push_str(&format!("; SFX Asset: {} (from {})\n", asset.name, asset.path));
                                out.push_str(&format!("; ========================================\n"));
                                
                                // SfxResource::compile_to_asm() generates full label and data
                                let asm = resource.compile_to_asm();
                                out.push_str(&asm);
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
    
    // ✅ ARRAY LITERAL DATA SECTION
    // Collect all array literals from NON-CONST global variables and generate data
    // (Const arrays are emitted separately in CONST ARRAY DATA SECTION)
    let mut array_counter = 0;
    for (name, value) in &non_const_vars {
        if let Expr::List(elements) = value {
            let array_label = format!("ARRAY_{}", array_counter);
            out.push_str(&format!("; Array literal for variable '{}' ({} elements)\n", name, elements.len()));
            out.push_str(&format!("{}:\n", array_label));
            for (i, elem) in elements.iter().enumerate() {
                if let Expr::Number(n) = elem {
                    out.push_str(&format!("    FDB {}   ; Element {}\n", n, i));
                } else {
                    out.push_str(&format!("    FDB 0    ; Element {} (non-constant - will be initialized at runtime)\n", i));
                }
            }
            out.push_str("\n");
            array_counter += 1;
        }
    }
    
    // ✅ CONST ARRAY DATA SECTION
    // Emit array literals from const declarations (read-only in ROM)
    // ✅ CONST ARRAY DATA SECTION
    // Emit array literals from const declarations (read-only in ROM)
    // Helper closure to evaluate constant expressions (e.g., 0 - 100 = -100)
    let eval_const_expr = |expr: &Expr| -> i32 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Binary { op, left, right } => {
                let left_val = match left.as_ref() {
                    Expr::Number(n) => *n,
                    _ => 0,
                };
                let right_val = match right.as_ref() {
                    Expr::Number(n) => *n,
                    _ => 0,
                };
                match op {
                    BinOp::Add => left_val + right_val,
                    BinOp::Sub => left_val - right_val,
                    BinOp::Mul => left_val * right_val,
                    BinOp::Div => if right_val != 0 { left_val / right_val } else { 0 },
                    BinOp::FloorDiv => if right_val != 0 { left_val / right_val } else { 0 },
                    BinOp::Mod => if right_val != 0 { left_val % right_val } else { 0 },
                    _ => 0,
                }
            }
            _ => 0,
        }
    };
    
    let mut const_array_counter = 0;
    for (name, value, source_line) in &const_vars_with_line {
        if let Expr::List(elements) = value {
            // ✅ Register const array definition line in .pdb
            tracker.set_line(*source_line);
            out.push_str(&format!("; VPy_LINE:{}\n", source_line));
            
            // Check if this is a string array (all elements are StringLit)
            let is_string_array = elements.iter().all(|e| matches!(e, Expr::StringLit(_)));
            
            if is_string_array {
                // String array: emit strings first, then pointer table
                let const_array_label = format!("CONST_ARRAY_{}", const_array_counter);
                out.push_str(&format!("; Const string array for '{}' ({} strings)\n", name, elements.len()));
                
                // Emit individual strings
                let mut string_labels = Vec::new();
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::StringLit(s) = elem {
                        let str_label = format!("{}_STR_{}", const_array_label, i);
                        string_labels.push(str_label.clone());
                        out.push_str(&format!("{}:\n", str_label));
                        out.push_str(&format!("    FCC \"{}\"\n", s.to_ascii_uppercase()));
                        out.push_str("    FCB $80   ; String terminator\n");
                    }
                }
                
                // Emit pointer table
                out.push_str(&format!("{}:  ; Pointer table for {}\n", const_array_label, name));
                for str_label in string_labels {
                    out.push_str(&format!("    FDB {}  ; Pointer to string\n", str_label));
                }
                out.push_str("\n");
            } else {
                // Number array (original code)
                let const_array_label = format!("CONST_ARRAY_{}", const_array_counter);
                out.push_str(&format!("; Const array literal for '{}' ({} elements)\n", name, elements.len()));
                out.push_str(&format!("{}:\n", const_array_label));
                for (i, elem) in elements.iter().enumerate() {
                    let elem_value = eval_const_expr(elem);
                    out.push_str(&format!("    FDB {}   ; Element {}\n", elem_value, i));
                }
                out.push_str("\n");
            }
            const_array_counter += 1;
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
        out.push_str(&format!("MIRROR_X EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("MIRROR_Y EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_VEC_INTENSITY EQU RESULT+{}\n", var_offset)); var_offset += 1;
    } else {
        out.push_str("; DRAW_VECTOR position offset\nDRAW_VEC_X: FCB 0\nDRAW_VEC_Y: FCB 0\n");
        out.push_str("; Mirror flags for DRAW_VECTOR_EX unified function\nMIRROR_X: FCB 0\nMIRROR_Y: FCB 0\n");
        out.push_str("; Intensity override (0=use vector's intensity, non-zero=override)\nDRAW_VEC_INTENSITY: FCB 0\n");
    }
    
    // DRAW_CIRCLE runtime variables (changed to bytes to avoid DP issues)
    if opts.exclude_ram_org {
        out.push_str(&format!("DRAW_CIRCLE_XC EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_CIRCLE_YC EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_CIRCLE_DIAM EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_CIRCLE_INTENSITY EQU RESULT+{}\n", var_offset)); var_offset += 1;
        out.push_str(&format!("DRAW_CIRCLE_TEMP EQU RESULT+{}\n", var_offset)); var_offset += 8; // TEMP is still 8 bytes (radius=2, xc=2, yc=2, spare=2)
    } else {
        out.push_str("; DRAW_CIRCLE runtime variables (bytes, not words - to avoid DP corruption)\n");
        out.push_str("DRAW_CIRCLE_XC: FCB 0\n");
        out.push_str("DRAW_CIRCLE_YC: FCB 0\n");
        out.push_str("DRAW_CIRCLE_DIAM: FCB 0\n");
        out.push_str("DRAW_CIRCLE_INTENSITY: FCB 0\n");
        out.push_str("; DRAW_CIRCLE_TEMP still needs 8 bytes for internal calculations\n");
        out.push_str("DRAW_CIRCLE_TEMP: FDB 0,0,0,0  ; radius(2), xc(2), yc(2), spare(2)\n");
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
// collect_stmt_syms: process statement symbols.

// collect_expr_syms: process expression identifiers.

// (helper functions moved to utils.rs and helpers.rs)
