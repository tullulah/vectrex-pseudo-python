//! vpy_codegen: Generate M6809 assembly
//!
//! Phase 5 of the compilation pipeline.
//! Produces assembly code per bank with metadata.

pub mod m6809;

use std::collections::HashMap;
use thiserror::Error;
// Note: These imports will be used when codegen is fully implemented
#[allow(unused_imports)]
use vpy_unifier::UnifiedModule;
#[allow(unused_imports)]
use vpy_bank_allocator::BankLayout;
use vpy_parser::Module;

/// Get BIOS function address from VECTREX.I
/// Returns the address as a hex string (e.g., "$F192")
/// Falls back to hardcoded value if VECTREX.I cannot be read
fn get_bios_address(symbol_name: &str, fallback_address: &str) -> String {
    // Try to get from VECTREX.I
    let possible_paths = vec![
        "ide/frontend/public/include/VECTREX.I",
        "../ide/frontend/public/include/VECTREX.I",
        "../../ide/frontend/public/include/VECTREX.I",
        "./ide/frontend/public/include/VECTREX.I",
    ];
    
    for path in &possible_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            // Parse VECTREX.I to find the symbol
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with(';') {
                    continue;
                }
                
                // Parse lines like: "Wait_Recal  EQU     $F192"
                if let Some(equ_pos) = line.find("EQU") {
                    let name_part = line[..equ_pos].trim();
                    let value_part = line[equ_pos + 3..].trim();
                    
                    if name_part.eq_ignore_ascii_case(symbol_name) {
                        // Extract just the address (e.g., "$F192" or "$F192   ; comment")
                        if let Some(addr) = value_part.split_whitespace().next() {
                            if addr.starts_with('$') || addr.starts_with("0x") {
                                return addr.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to hardcoded value
    fallback_address.to_string()
}

#[derive(Debug, Clone, Error)]
pub enum CodegenError {
    #[error("Codegen error: {0}")]
    Error(String),
}

/// Configuration for multibank ROM generation
#[derive(Debug, Clone)]
pub struct BankConfig {
    pub rom_total_size: usize,
    pub rom_bank_size: usize,
    pub rom_bank_count: usize,
    pub helpers_bank: usize, // Always last bank (rom_bank_count - 1)
}

impl BankConfig {
    /// Create bank config from total size and bank size
    pub fn new(rom_total_size: usize, rom_bank_size: usize) -> Self {
        let rom_bank_count = rom_total_size / rom_bank_size;
        let helpers_bank = rom_bank_count.saturating_sub(1);
        
        Self {
            rom_total_size,
            rom_bank_size,
            rom_bank_count,
            helpers_bank,
        }
    }
    
    /// Single bank configuration (32KB cartridge)
    pub fn single_bank() -> Self {
        Self {
            rom_total_size: 32768,
            rom_bank_size: 32768,
            rom_bank_count: 1,
            helpers_bank: 0,
        }
    }
}

/// Generated assembly output - UNIFIED format
/// Single ASM file with bank separators
#[derive(Debug, Clone)]
pub struct GeneratedASM {
    /// Complete unified ASM source with bank markers
    pub asm_source: String,
    
    /// Bank configuration (single or multibank)
    pub bank_config: BankConfig,
    
    /// Symbol table (name → bank_id, offset)
    pub symbols: HashMap<String, SymbolInfo>,
    
    /// External references that need linking
    pub external_refs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub bank_id: usize,
    pub offset: u16,
    pub metadata: String,
}

/// Generate unified ASM with bank markers (NEW - uses real M6809 codegen)
pub fn generate_from_module(
    module: &Module,
    bank_config: &BankConfig,
    title: &str,
) -> Result<GeneratedASM, CodegenError> {
    // Use real M6809 backend
    let asm_source = m6809::generate_m6809_asm(
        module,
        title,
        bank_config.rom_total_size,
        bank_config.rom_bank_size,
    ).map_err(|e| CodegenError::Error(e))?;
    
    Ok(GeneratedASM {
        asm_source,
        bank_config: bank_config.clone(),
        symbols: HashMap::new(), // TODO: Extract from generated ASM
        external_refs: Vec::new(),
    })
}

/// Generate unified ASM with bank markers (OLD - placeholder version)
/// Kept for backward compatibility with existing code
pub fn generate_unified_asm(
    bank_config: &BankConfig,
    functions: &[String], // Placeholder: will be real function data
    title: &str, // Game title from META TITLE
) -> Result<GeneratedASM, CodegenError> {
    let mut asm = String::new();
    let mut symbols = HashMap::new();
    
    // Generate header comment
    asm.push_str(&format!("; VPy Unified Assembly\n"));
    asm.push_str(&format!("; Total ROM: {} bytes ({} banks x {} bytes)\n",
        bank_config.rom_total_size,
        bank_config.rom_bank_count,
        bank_config.rom_bank_size));
    asm.push_str(&format!("; Helpers Bank: {} (DYNAMIC - not hardcoded)\n\n",
        bank_config.helpers_bank));
    
    // Define RAM variables (EQU directives must come before code)
    asm.push_str("; === RAM Variables ===\n");
    asm.push_str("CURRENT_ROM_BANK EQU $C880\n");
    asm.push_str("RESULT EQU $CF00\n");
    asm.push_str("TMPPTR EQU $CF02\n");
    asm.push_str("\n");
    
    // Generate Bank 0 (boot + main code)
    asm.push_str("; === BANK 0 ===\n");
    asm.push_str("    ORG $0000\n");
    asm.push_str("BANK0_START:\n");
    asm.push_str("\n");
    
    // Include Vectrex BIOS definitions
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; DEFINE SECTION\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("    INCLUDE \"VECTREX.I\"\n");
    asm.push_str("\n");
    
    // Vectrex cartridge header (CRITICAL - must be at $0000)
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; HEADER SECTION\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("    FCC \"g GCE 2025\"\n");
    asm.push_str("    FCB $80              ; String terminator\n");
    asm.push_str("    FDB $0000            ; Music pointer (no music)\n");
    asm.push_str("    FCB $F8,$50,$20,$BB  ; Height, width, rel Y, rel X\n");
    asm.push_str(&format!("    FCC \"{}\"      ; Game title\n", title));
    asm.push_str("    FCB $80              ; String terminator\n");
    asm.push_str("    FCB 0                ; End marker\n");
    asm.push_str("\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; CODE SECTION\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("\n");
    asm.push_str("START:\n");
    asm.push_str("    ; Initialize BIOS\n");
    asm.push_str("    LDA #$D0\n");
    asm.push_str("    TFR A,DP        ; Set Direct Page to $D0 (BIOS requirement)\n");
    asm.push_str("    LDS #$CBFF      ; Initialize stack\n");
    asm.push_str("\n");
    
    symbols.insert("BANK0_START".to_string(), SymbolInfo {
        bank_id: 0,
        offset: 0x0000,
        metadata: "Boot section".to_string(),
    });
    
    symbols.insert("START".to_string(), SymbolInfo {
        bank_id: 0,
        offset: 0x0027, // Offset after header (14 bytes copyright + 4 music + 4 box + 9 title + 1 end = ~32 bytes)
        metadata: "Entry point".to_string(),
    });
    
    // If multibank, add bank switch code
    if bank_config.rom_bank_count > 1 {
        asm.push_str("    ; === Multibank Boot Sequence ===\n");
        asm.push_str(&format!("    ; Switch to Bank {} (helpers bank)\n", bank_config.helpers_bank));
        asm.push_str(&format!("    LDA #{}\n", bank_config.helpers_bank));
        asm.push_str("    STA >CURRENT_ROM_BANK\n");
        asm.push_str("    STA $DF00       ; Hardware bank register\n");
        asm.push_str(&format!("    JMP MAIN        ; Jump to main in Bank {}\n\n", bank_config.helpers_bank));
    } else {
        // Single-bank: MAIN is in the same bank, jump directly
        asm.push_str("    JMP MAIN        ; Jump to main (single-bank)\n\n");
    }
    
    // Generate placeholder user functions (only in single-bank)
    if bank_config.rom_bank_count == 1 {
        asm.push_str("    ; User functions\n");
        for (i, func_name) in functions.iter().enumerate() {
            // Skip main/loop - they're handled specially below
            if func_name.eq_ignore_ascii_case("main") || func_name.eq_ignore_ascii_case("loop") {
                continue;
            }
            
            asm.push_str(&format!("{}:\n", func_name.to_uppercase()));
            asm.push_str(&format!("    ; Function {} code here\n", func_name));
            asm.push_str("    RTS\n\n");
            
            symbols.insert(func_name.to_uppercase(), SymbolInfo {
                bank_id: 0,
                offset: 0x0050 + (i * 10) as u16,
                metadata: format!("User function {}", func_name),
            });
        }
        
        // In single-bank, MAIN/LOOP_BODY come right after user functions
        asm.push_str("\n");
        asm.push_str("MAIN:\n");
        asm.push_str("    ; Main initialization\n");
        asm.push_str("    JSR LOOP_BODY\n");
        asm.push_str("MAIN_LOOP:\n");
        asm.push_str("    JSR LOOP_BODY\n");
        asm.push_str("    BRA MAIN_LOOP\n");
        asm.push_str("\n");
        
        symbols.insert("MAIN".to_string(), SymbolInfo {
            bank_id: 0,
            offset: 0x0100,
            metadata: "Main entry point".to_string(),
        });
        
        // Loop body
        let wait_recal = get_bios_address("Wait_Recal", "$F192");
        asm.push_str("LOOP_BODY:\n");
        asm.push_str("    ; User loop code\n");
        asm.push_str(&format!("    JSR {}       ; BIOS Wait_Recal\n", wait_recal));
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        // Runtime helpers
        asm.push_str("    ; === Runtime Helpers ===\n");
        asm.push_str("MUL16:\n");
        asm.push_str("    ; 16-bit multiplication helper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        asm.push_str("DIV16:\n");
        asm.push_str("    ; 16-bit division helper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        asm.push_str("DRAW_LINE_WRAPPER:\n");
        asm.push_str("    ; Line drawing wrapper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        symbols.insert("MUL16".to_string(), SymbolInfo {
            bank_id: 0,
            offset: 0x0150,
            metadata: "Multiplication helper".to_string(),
        });
    }
    
    asm.push_str("BANK0_END:\n\n");
    
    // Generate Helpers Bank (only for multibank)
    if bank_config.rom_bank_count > 1 {
        let helpers_bank = bank_config.helpers_bank;
        asm.push_str(&format!("; === BANK {} ===\n", helpers_bank));
        asm.push_str("    ORG $4000       ; Fixed bank window\n");
        asm.push_str(&format!("BANK{}_START:\n", helpers_bank));
        asm.push_str("\n");
        
        // Main function
        asm.push_str("MAIN:\n");
        asm.push_str("    ; Main initialization\n");
        asm.push_str("    JSR LOOP_BODY\n");
        asm.push_str("MAIN_LOOP:\n");
        asm.push_str("    JSR LOOP_BODY\n");
        asm.push_str("    BRA MAIN_LOOP\n");
        asm.push_str("\n");
        
        symbols.insert("MAIN".to_string(), SymbolInfo {
            bank_id: helpers_bank,
            offset: 0x4000,
            metadata: "Main entry point".to_string(),
        });
        
        // Loop body
        let wait_recal_2 = get_bios_address("Wait_Recal", "$F192");
        asm.push_str("LOOP_BODY:\n");
        asm.push_str("    ; User loop code\n");
        asm.push_str(&format!("    JSR {}       ; BIOS Wait_Recal\n", wait_recal_2));
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        // Runtime helpers
        asm.push_str("    ; === Runtime Helpers ===\n");
        asm.push_str("MUL16:\n");
        asm.push_str("    ; 16-bit multiplication helper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        asm.push_str("DIV16:\n");
        asm.push_str("    ; 16-bit division helper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        asm.push_str("DRAW_LINE_WRAPPER:\n");
        asm.push_str("    ; Line drawing wrapper\n");
        asm.push_str("    RTS\n");
        asm.push_str("\n");
        
        symbols.insert("MUL16".to_string(), SymbolInfo {
            bank_id: helpers_bank,
            offset: 0x4050,
            metadata: "Multiplication helper".to_string(),
        });
        
        asm.push_str(&format!("BANK{}_END:\n", helpers_bank));
    }
    
    Ok(GeneratedASM {
        asm_source: asm,
        bank_config: bank_config.clone(),
        symbols,
        external_refs: vec!["Wait_Recal".to_string()],
    })
}

// This will be expanded with real codegen logic when AST types are ready
