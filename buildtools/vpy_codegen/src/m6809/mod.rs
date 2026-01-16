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
    
    // For multibank: Start Bank 0
    if is_multibank {
        asm.push_str("; === BANK 0 ===\n");
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
    
    // RAM variables (system)
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; SYSTEM RAM VARIABLES\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("CURRENT_ROM_BANK EQU $C880\n");
    asm.push_str("RESULT EQU $CF00\n");
    asm.push_str("TMPPTR EQU $CF02\n");
    asm.push_str("TMPPTR2 EQU $CF04\n\n");  // For array indexed assignments
    
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
    
    // Generate user functions
    let functions_asm = functions::generate_functions(module)?;
    asm.push_str(&functions_asm);
    
    // Collect and emit PRINT_TEXT string data (MUST be in Bank 0 for references to work)
    let print_text_strings = builtins::collect_print_text_strings(module);
    builtins::emit_print_text_strings(&print_text_strings, &mut asm);
    
    // For multibank: Switch to fixed bank (last bank) for helpers
    // This bank is always visible at $4000-$7FFF, so helpers can be called from any bank
    // Two-pass assembly in vpy_assembler will:
    //   1. Assemble fixed bank first
    //   2. Extract helper symbols (VECTREX_PRINT_TEXT, etc.)
    //   3. Inject EQU declarations into other banks
    //   4. Assemble other banks with symbol references resolved
    if is_multibank {
        asm.push_str(&format!("\n; === BANK {} ===\n", helpers_bank));
        asm.push_str("    ORG $4000\n");
        asm.push_str(&format!("    ; Fixed bank (always visible at $4000-$7FFF)\n"));
        asm.push_str(&format!("    ; Contains runtime helpers for all banks\n\n"));
        let helpers_asm = helpers::generate_helpers()?;
        asm.push_str(&helpers_asm);
    }
    
    // For single-bank: Emit helpers normally
    if !is_multibank {
        let helpers_asm = helpers::generate_helpers()?;
        asm.push_str(&helpers_asm);
    }
    
    Ok(asm)
}
