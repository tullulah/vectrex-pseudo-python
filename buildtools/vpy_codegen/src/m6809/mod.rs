//! M6809 Code Generator for Vectrex
//! 
//! Modular architecture:
//! - header: Vectrex cartridge header generation
//! - variables: RAM variable allocation
//! - functions: Function code generation
//! - expressions: Expression compilation
//! - builtins: Builtin function code
//! - helpers: Runtime helpers (MUL16, DIV16, etc.)

pub mod header;
pub mod variables;
pub mod functions;
pub mod expressions;
pub mod builtins;
pub mod helpers;
pub mod math;
pub mod joystick;
pub mod debug;
pub mod math_extended;
pub mod drawing;
pub mod level;
pub mod utilities;

use vpy_parser::{Item, Expr, Stmt, CallInfo};
use std::collections::HashSet;

// Determine which RAM variables are needed based on used helpers
fn get_needed_ram_vars(needed_helpers: &HashSet<String>, is_multibank: bool) -> HashSet<String> {
    let mut vars = HashSet::new();
    
    // Always needed
    vars.insert("RESULT".to_string());
    
    // Multibank needs bank tracking
    if is_multibank {
        vars.insert("CURRENT_ROM_BANK".to_string());
    }
    
    // Helper-specific variables
    if needed_helpers.contains("PRINT_TEXT") || needed_helpers.contains("PRINT_NUMBER") {
        vars.insert("VAR_ARG0".to_string());
        vars.insert("VAR_ARG1".to_string());
        vars.insert("VAR_ARG2".to_string());
    }
    
    if needed_helpers.contains("PRINT_NUMBER") {
        vars.insert("NUM_STR".to_string());
    }
    
    if needed_helpers.contains("DRAW_CIRCLE_RUNTIME") {
        vars.insert("DRAW_CIRCLE_XC".to_string());
        vars.insert("DRAW_CIRCLE_YC".to_string());
        vars.insert("DRAW_CIRCLE_DIAM".to_string());
        vars.insert("DRAW_CIRCLE_INTENSITY".to_string());
        vars.insert("DRAW_CIRCLE_TEMP".to_string());
        vars.insert("TMPPTR".to_string());
    }
    
    if needed_helpers.contains("DRAW_RECT_RUNTIME") {
        vars.insert("DRAW_RECT_X".to_string());
        vars.insert("DRAW_RECT_Y".to_string());
        vars.insert("DRAW_RECT_WIDTH".to_string());
        vars.insert("DRAW_RECT_HEIGHT".to_string());
        vars.insert("DRAW_RECT_INTENSITY".to_string());
    }
    
    if needed_helpers.contains("DRAW_LINE_WRAPPER") {
        vars.insert("VLINE_DX_16".to_string());
        vars.insert("VLINE_DY_16".to_string());
        vars.insert("VLINE_DX".to_string());
        vars.insert("VLINE_DY".to_string());
        vars.insert("VLINE_DY_REMAINING".to_string());
        vars.insert("TMPPTR".to_string());
    }
    
    if needed_helpers.contains("SHOW_LEVEL_RUNTIME") {
        vars.insert("LEVEL_PTR".to_string());
        vars.insert("LEVEL_WIDTH".to_string());
        vars.insert("LEVEL_HEIGHT".to_string());
        vars.insert("LEVEL_TILE_SIZE".to_string());
        vars.insert("TMPPTR".to_string());
        vars.insert("TMPPTR2".to_string());
    }
    
    if needed_helpers.contains("FADE_IN_RUNTIME") || needed_helpers.contains("FADE_OUT_RUNTIME") {
        vars.insert("FRAME_COUNTER".to_string());
        vars.insert("CURRENT_INTENSITY".to_string());
    }
    
    if needed_helpers.contains("RAND_HELPER") || needed_helpers.contains("RAND_RANGE_HELPER") {
        vars.insert("RAND_SEED".to_string());
    }
    
    vars
}

/// Check if trigonometric functions (SIN, COS, TAN) are used in statements
fn check_trig_usage(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        if check_stmt_trig(stmt) {
            return true;
        }
    }
    false
}

fn check_stmt_trig(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => check_expr_trig(expr),
        Stmt::Assign { value, .. } => check_expr_trig(value),
        Stmt::If { cond, body, elifs, else_body, .. } => {
            check_expr_trig(cond)
                || check_trig_usage(body)
                || elifs.iter().any(|(c, b)| check_expr_trig(c) || check_trig_usage(b))
                || else_body.as_ref().map_or(false, |b| check_trig_usage(b))
        }
        Stmt::While { cond, body, .. } => check_expr_trig(cond) || check_trig_usage(body),
        _ => false,
    }
}

fn check_expr_trig(expr: &Expr) -> bool {
    match expr {
        Expr::Call(CallInfo { name, args, .. }) => {
            let upper = name.to_uppercase();
            upper == "SIN" || upper == "COS" || upper == "TAN"
                || args.iter().any(check_expr_trig)
        }
        Expr::Binary { left, right, .. } => check_expr_trig(left) || check_expr_trig(right),
        Expr::Not(operand) | Expr::BitNot(operand) => check_expr_trig(operand),
        Expr::Index { target, index, .. } => check_expr_trig(target) || check_expr_trig(index),
        Expr::List(elements) => elements.iter().any(check_expr_trig),
        _ => false,
    }
}

use vpy_parser::Module;

/// Main entry point for M6809 code generation
pub fn generate_m6809_asm(
    module: &Module,
    title: &str,
    rom_size: usize,
    _bank_size: usize,
) -> Result<String, String> {
    let mut asm = String::new();
    
    // Detect if this is a multibank ROM (>32KB)
    let is_multibank = rom_size > 32768;
    
    // Calculate bank configuration dynamically
    let bank_size = 16384; // Standard Vectrex bank size (16KB)
    let num_banks = if is_multibank { rom_size / bank_size } else { 1 };
    let helpers_bank = if is_multibank { num_banks - 1 } else { 0 };
    
    // Generate header comments
    asm.push_str(&format!("; VPy M6809 Assembly (Vectrex)\n"));
    asm.push_str(&format!("; ROM: {} bytes\n", rom_size));
    if is_multibank {
        asm.push_str(&format!("; Multibank cartridge: {} banks ({}KB each)\n", num_banks, bank_size / 1024));
        asm.push_str(&format!("; Helpers bank: {} (fixed bank at $4000-$7FFF)\n", helpers_bank));
    }
    asm.push_str("\n");
    
    // For multibank: Emit Bank 0 marker
    if is_multibank {
        asm.push_str("; ================================================\n");
        asm.push_str("; BANK #0 - Entry point and main code\n");
        asm.push_str("; ================================================\n");
    }
    
    // Start of ROM
    asm.push_str("    ORG $0000\n\n");
    
    // Include VECTREX.I
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; DEFINE SECTION\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("    INCLUDE \"VECTREX.I\"\n\n");
    
    // Generate Vectrex header
    let header_asm = header::generate_header(title, &module.meta)?;
    asm.push_str(&header_asm);
    
    // Analyze which helpers are needed (for conditional RAM variables)
    let needed_helpers = helpers::analyze_module_helpers(module);
    let needed_ram_vars = get_needed_ram_vars(&needed_helpers, is_multibank);
    eprintln!("[DEBUG RAM] Needed {} RAM variables: {:?}", needed_ram_vars.len(), needed_ram_vars);
    
    // RAM variables (conditional based on usage)
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; SYSTEM RAM VARIABLES\n");
    asm.push_str(";***************************************************************************\n");
    
    if needed_ram_vars.contains("CURRENT_ROM_BANK") {
        asm.push_str("CURRENT_ROM_BANK EQU $C880\n");
    }
    if needed_ram_vars.contains("RESULT") {
        asm.push_str("RESULT EQU $CF00\n");
    }
    if needed_ram_vars.contains("TMPPTR") {
        asm.push_str("TMPPTR EQU $CF02\n");
    }
    if needed_ram_vars.contains("TMPPTR2") {
        asm.push_str("TMPPTR2 EQU $CF04\n");
    }
    if needed_ram_vars.contains("NUM_STR") {
        asm.push_str("NUM_STR EQU $CF06   ; 2-byte buffer for PRINT_NUMBER hex output\n");
    }
    if needed_ram_vars.contains("RAND_SEED") {
        asm.push_str("RAND_SEED EQU $CF08 ; 2-byte random seed for RAND()\n");
    }
    
    // Drawing builtins parameters
    if needed_ram_vars.contains("DRAW_CIRCLE_XC") {
        asm.push_str("\n; Drawing builtins parameters (bytes in RAM)\n");
        asm.push_str("DRAW_CIRCLE_XC EQU $CF0A\n");
        asm.push_str("DRAW_CIRCLE_YC EQU $CF0B\n");
        asm.push_str("DRAW_CIRCLE_DIAM EQU $CF0C\n");
        asm.push_str("DRAW_CIRCLE_INTENSITY EQU $CF0D\n");
        asm.push_str("DRAW_CIRCLE_TEMP EQU $CF0E ; 6 bytes for runtime calculations\n");
    }
    
    if needed_ram_vars.contains("DRAW_RECT_X") {
        asm.push_str("\nDRAW_RECT_X EQU $CF14\n");
        asm.push_str("DRAW_RECT_Y EQU $CF15\n");
        asm.push_str("DRAW_RECT_WIDTH EQU $CF16\n");
        asm.push_str("DRAW_RECT_HEIGHT EQU $CF17\n");
        asm.push_str("DRAW_RECT_INTENSITY EQU $CF18\n");
    }
    
    if needed_ram_vars.contains("VLINE_DX_16") {
        asm.push_str("\n; DRAW_LINE helper variables (16-bit deltas + 8-bit clamped + remaining)\n");
        asm.push_str("VLINE_DX_16 EQU $CF19         ; 16-bit dx (2 bytes)\n");
        asm.push_str("VLINE_DY_16 EQU $CF1B         ; 16-bit dy (2 bytes)\n");
        asm.push_str("VLINE_DX EQU $CF1D            ; 8-bit clamped dx (1 byte)\n");
        asm.push_str("VLINE_DY EQU $CF1E            ; 8-bit clamped dy (1 byte)\n");
        asm.push_str("VLINE_DY_REMAINING EQU $CF1F  ; Remaining dy for segment 2 (2 bytes)\n");
    }
    
    if needed_ram_vars.contains("LEVEL_PTR") {
        asm.push_str("\n; Level system variables\n");
        asm.push_str("LEVEL_PTR EQU $CF20           ; Pointer to current level data (2 bytes)\n");
        asm.push_str("LEVEL_WIDTH EQU $CF22          ; Level width in tiles (1 byte)\n");
        asm.push_str("LEVEL_HEIGHT EQU $CF23         ; Level height in tiles (1 byte)\n");
        asm.push_str("LEVEL_TILE_SIZE EQU $CF24      ; Tile size in pixels (1 byte)\n");
    }
    
    if needed_ram_vars.contains("FRAME_COUNTER") {
        asm.push_str("\n; Fade effects\n");
        asm.push_str("FRAME_COUNTER EQU $CF26        ; Frame counter (2 bytes)\n");
        asm.push_str("CURRENT_INTENSITY EQU $CF28    ; Current intensity for fade effects (1 byte)\n");
    }
    
    if needed_ram_vars.contains("VAR_ARG0") {
        asm.push_str("\n; Function argument slots (16-bit each, 3 slots = 6 bytes)\n");
        asm.push_str("VAR_ARG0 EQU $CFE0+0\n");
        asm.push_str("VAR_ARG1 EQU $CFE0+2\n");
        asm.push_str("VAR_ARG2 EQU $CFE0+4\n");
    }
    asm.push_str("\n");
    
    // Generate user variables
    let vars_asm = variables::generate_variables(module)?;
    asm.push_str(&vars_asm);
    
    // Generate code section
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; CODE SECTION\n");
    asm.push_str(";***************************************************************************\n\n");
    
    // Generate START initialization
    asm.push_str("START:\n");
    asm.push_str("    LDA #$D0\n");
    asm.push_str("    TFR A,DP        ; Set Direct Page for BIOS\n");
    asm.push_str("    CLR $C80E        ; Initialize Vec_Prev_Btns\n");
    asm.push_str("    LDA #$80\n");
    asm.push_str("    STA VIA_t1_cnt_lo\n");
    asm.push_str("    LDS #$CBFF       ; Initialize stack\n");
    
    // For multibank: Fixed bank is ALWAYS visible at $4000-$7FFF
    // No need to write bank register - cartridge hardware has it configured
    // from factory. Bank 0 is at $0000, fixed bank at $4000.
    if is_multibank {
        asm.push_str(&format!("; Bank 0 ($0000) is active; fixed bank {} ($4000-$7FFF) always visible\n", helpers_bank));
    }
    
    asm.push_str("    JMP MAIN\n\n");
    
    // Generate user functions in Bank 0
    let functions_asm = functions::generate_functions(module)?;
    asm.push_str(&functions_asm);
    
    // CRITICAL FIX (2026-01-17): Collect PRINT_TEXT strings here but emit LATER
    // Problem: If strings are emitted immediately after functions, they get addresses
    // in the middle of code, and LDX #PRINT_TEXT_STR references fail in assembler
    // Solution: Collect now, emit at END (after helpers, before vectors) like CORE does
    let print_text_strings = builtins::collect_print_text_strings(module);
    
    // For multibank: Emit ALL intermediate banks as empty placeholders
    // multi_bank_linker requires ALL banks to be marked in the ASM
    // Format: "; ================================================"
    //         "; BANK #N - 0 function(s) [EMPTY]"
    //         "; ================================================"
    //         "    ORG $0000  ; Sequential bank model"
    if is_multibank {
        // Emit banks 1 through (helpers_bank - 1) as empty
        for bank_id in 1..(helpers_bank as usize) {
            asm.push_str(&format!("\n; ================================================\n"));
            asm.push_str(&format!("; BANK #{} - 0 function(s) [EMPTY]\n", bank_id));
            asm.push_str(&format!("; ================================================\n"));
            asm.push_str("    ORG $0000  ; Sequential bank model\n");
            asm.push_str(&format!("    ; Reserved for future code overflow\n\n"));
        }
        
        // Emit helpers bank (last bank) with proper marker
        asm.push_str(&format!("\n; ================================================\n"));
        asm.push_str(&format!("; BANK #{} - 0 function(s) [HELPERS ONLY]\n", helpers_bank));
        asm.push_str(&format!("; ================================================\n"));
        asm.push_str("    ORG $0000  ; Sequential bank model\n");
        asm.push_str(&format!("    ; Runtime helpers (accessible from all banks)\n\n"));
        let helpers_asm = helpers::generate_helpers(module)?;
        asm.push_str(&helpers_asm);
    }
    
    // For single-bank: Emit helpers normally
    if !is_multibank {
        let helpers_asm = helpers::generate_helpers(module)?;
        asm.push_str(&helpers_asm);
    }
    
    // Emit trigonometry lookup tables (SIN, COS, TAN) - CONDITIONAL
    // Only emit if SIN, COS, or TAN functions are actually used
    if module.items.iter().any(|item| {
        if let Item::Function(f) = item {
            check_trig_usage(&f.body)
        } else {
            false
        }
    }) {
        asm.push_str(&math_extended::generate_trig_tables());
    }
    
    // CRITICAL FIX (2026-01-17): Emit PRINT_TEXT strings AFTER all code/helpers
    // This ensures labels have stable final addresses that assembler can resolve
    // Matches CORE architecture where strings are emitted at end
    if !print_text_strings.is_empty() {
        builtins::emit_print_text_strings(&print_text_strings, &mut asm);
    }
    
    // NOTE: Cartridge ROM ($0000-$7FFF) does NOT contain interrupt vectors
    // Hardware vectors ($FFF0-$FFFF) are in BIOS ROM
    // BIOS vectors point to RAM vectors ($CBF2-$CBFB) as defined in VECTREX.I
    // Cartridge starts at $0000 and BIOS jumps there after verification
    
    Ok(asm)
}
