//! vpy_assembler: Assemble to object files
//!
//! Phase 6 of the compilation pipeline.
//! Assembles M6809 ASM to relocatable object format.

pub mod m6809;
mod error;

pub use error::AssemblyError;
pub use m6809::{assemble_m6809, set_include_dir, BinaryEmitter, load_vectrex_symbols};

use std::collections::HashMap;
// Note: Will be used when assembler is fully implemented
#[allow(unused_imports)]
use vpy_codegen::GeneratedASM;

/// Bank section extracted from unified ASM
#[derive(Debug, Clone)]
pub struct BankSection {
    pub bank_id: usize,
    pub org_address: u16,
    pub asm_lines: Vec<String>,
}

/// Assembled binary for one bank
#[derive(Debug, Clone)]
pub struct BankBinary {
    pub bank_id: usize,
    pub bytes: Vec<u8>,
    pub symbols: HashMap<String, SymbolDef>,
}

#[derive(Debug, Clone)]
pub struct SymbolDef {
    pub offset: u16,
    pub is_export: bool,
}

/// Multi-bank ROM output
#[derive(Debug, Clone)]
pub struct MultiROM {
    pub banks: Vec<BankBinary>,
    pub total_size: usize,
}

/// Parse unified ASM and extract bank sections
pub fn parse_unified_asm(asm_source: &str) -> Result<Vec<BankSection>, AssemblyError> {
    let mut sections = Vec::new();
    let mut current_bank_id: Option<usize> = None;
    let mut current_org: Option<u16> = None;
    let mut current_lines = Vec::new();
    let mut equ_definitions = Vec::new(); // Collect EQUs before first bank
    
    for (_line_num, line) in asm_source.lines().enumerate() {
        let trimmed = line.trim();
        
        // Collect EQU definitions before first bank
        if current_bank_id.is_none() && trimmed.to_uppercase().contains(" EQU ") {
            equ_definitions.push(line.to_string());
            continue;
        }
        
        // Detect bank separator: "; === BANK N ==="
        if let Some(bank_marker) = trimmed.strip_prefix("; === BANK ") {
            if let Some(end_pos) = bank_marker.find(" ===") {
                let bank_str = &bank_marker[..end_pos];
                if let Ok(bank_id) = bank_str.parse::<usize>() {
                    // Save previous bank section if any
                    if let (Some(bid), Some(org)) = (current_bank_id, current_org) {
                        sections.push(BankSection {
                            bank_id: bid,
                            org_address: org,
                            asm_lines: current_lines.clone(),
                        });
                        current_lines.clear();
                    }
                    
                    // Start new bank (prepend EQU definitions)
                    current_bank_id = Some(bank_id);
                    current_org = None;
                    current_lines.extend(equ_definitions.clone());
                    continue;
                }
            }
        }
        
        // Detect ORG directive
        if trimmed.starts_with("ORG ") || trimmed.starts_with("    ORG ") {
            if let Some(addr_str) = trimmed.split_whitespace().nth(1) {
                if let Some(hex) = addr_str.strip_prefix("$") {
                    if let Ok(addr) = u16::from_str_radix(hex, 16) {
                        current_org = Some(addr);
                    }
                }
            }
        }
        
        // Skip BANK_START/BANK_END markers (decorative only)
        if trimmed.starts_with("BANK") && (trimmed.ends_with("_START:") || trimmed.ends_with("_END:")) {
            continue;
        }
        
        // Collect line for current bank
        if current_bank_id.is_some() {
            current_lines.push(line.to_string());
        }
    }
    
    // Save last bank section
    if let (Some(bid), Some(org)) = (current_bank_id, current_org) {
        sections.push(BankSection {
            bank_id: bid,
            org_address: org,
            asm_lines: current_lines,
        });
    }
    
    // If no bank separators found, treat entire ASM as Bank 0 (single-bank mode)
    if sections.is_empty() {
        // Collect all lines and find ORG
        let mut all_lines = Vec::new();
        let mut org_address = 0x0000; // Default ORG
        
        all_lines.extend(equ_definitions);
        
        for line in asm_source.lines() {
            let trimmed = line.trim();
            
            // Detect ORG directive
            if trimmed.starts_with("ORG ") || trimmed.starts_with("    ORG ") {
                if let Some(addr_str) = trimmed.split_whitespace().nth(1) {
                    if let Some(hex) = addr_str.strip_prefix("$") {
                        if let Ok(addr) = u16::from_str_radix(hex, 16) {
                            org_address = addr;
                        }
                    }
                }
            }
            
            all_lines.push(line.to_string());
        }
        
        sections.push(BankSection {
            bank_id: 0,
            org_address,
            asm_lines: all_lines,
        });
    }
    
    Ok(sections)
}

/// Assemble all bank sections with two-pass algorithm for cross-bank symbol resolution
/// 
/// Two-Pass Assembly Algorithm:
/// 1. Identify fixed bank (last bank) which contains helpers
/// 2. Extract EQU definitions from Bank 0 (shared RAM variables)
/// 3. Assemble fixed bank first with injected EQU definitions
/// 4. Extract helper symbols from assembled fixed bank
/// 5. Inject both EQU definitions and helper symbols into other banks
/// 6. Assemble other banks with all symbol references resolved
pub fn assemble_banks(sections: Vec<BankSection>) -> Result<Vec<BankBinary>, AssemblyError> {
    if sections.is_empty() {
        return Ok(Vec::new());
    }
    
    // Determine if this is multibank by checking for multiple bank IDs
    let bank_ids: std::collections::HashSet<_> = sections.iter().map(|s| s.bank_id).collect();
    let is_multibank = bank_ids.len() > 1;
    
    if !is_multibank {
        // Single-bank: assemble directly (no cross-bank references)
        return assemble_banks_simple(sections);
    }
    
    // === TWO-PASS ASSEMBLY FOR MULTIBANK ===
    
    // Step 1: Find fixed bank (last bank ID) and Bank 0
    let max_bank_id = *bank_ids.iter().max().unwrap();
    let fixed_bank_id = max_bank_id;
    
    // Step 2: Extract EQU definitions from Bank 0 (shared RAM symbols)
    let bank0_section = sections.iter().find(|s| s.bank_id == 0);
    let mut shared_equ_definitions = Vec::new();
    
    if let Some(bank0) = bank0_section {
        shared_equ_definitions.push("; === Shared RAM symbols from Bank 0 ===".to_string());
        for line in &bank0.asm_lines {
            let trimmed = line.trim();
            // Extract EQU definitions for shared symbols
            if trimmed.contains(" EQU ") && 
               (trimmed.starts_with("CURRENT_ROM_BANK") || 
                trimmed.starts_with("RESULT") ||
                trimmed.starts_with("TMPPTR") ||
                trimmed.starts_with("VAR_ARG") ||
                trimmed.starts_with("VAR_")) {
                shared_equ_definitions.push(line.clone());
            }
        }
        shared_equ_definitions.push(String::new()); // Blank line
    }
    
    // Step 3: Separate fixed bank from other banks
    let mut fixed_bank_section: Option<BankSection> = None;
    let mut other_sections = Vec::new();
    
    for section in sections {
        if section.bank_id == fixed_bank_id {
            fixed_bank_section = Some(section);
        } else {
            other_sections.push(section);
        }
    }
    
    let mut fixed_bank_section = fixed_bank_section
        .ok_or_else(|| AssemblyError::Failed(format!("Fixed bank {} not found", fixed_bank_id)))?;
    
    // Step 4: Inject shared EQU definitions into fixed bank
    let mut augmented_fixed_lines = shared_equ_definitions.clone();
    augmented_fixed_lines.extend(fixed_bank_section.asm_lines);
    fixed_bank_section.asm_lines = augmented_fixed_lines;
    
    // Step 5: Assemble fixed bank (PASS 1)
    let asm_source = fixed_bank_section.asm_lines.join("\n");
    let (bytes, _line_map, symbol_table, _unresolved) = m6809::assemble_m6809(
        &asm_source, 
        fixed_bank_section.org_address, 
        false, 
        false
    ).map_err(|e| AssemblyError::Failed(format!("Failed to assemble fixed bank {}: {}", fixed_bank_id, e)))?;
    
    // Convert symbol table for fixed bank
    let fixed_symbols: HashMap<String, SymbolDef> = symbol_table
        .iter()
        .map(|(name, &offset)| (name.clone(), SymbolDef { offset, is_export: true }))
        .collect();
    
    let fixed_bank_binary = BankBinary {
        bank_id: fixed_bank_id,
        bytes,
        symbols: fixed_symbols.clone(),
    };
    
    // Step 6: Generate EQU declarations for helper symbols from fixed bank
    let mut helper_equ_declarations = Vec::new();
    helper_equ_declarations.push(format!("; === Cross-bank helper symbols from Bank {} (fixed bank) ===", fixed_bank_id));
    
    // Filter for helper symbols (typically uppercase, exclude internal labels starting with underscore or dot)
    for (name, def) in &fixed_symbols {
        if name.chars().next().map_or(false, |c| c.is_uppercase()) && 
           !name.starts_with('_') && 
           !name.starts_with('.') &&
           !name.starts_with("VAR_") &&  // Skip VAR_ symbols (already in shared EQUs)
           name != "CURRENT_ROM_BANK" &&
           name != "RESULT" &&
           name != "TMPPTR" {
            // Symbol offsets from fixed bank assembly are already absolute (ORG $4000)
            // Do NOT re-add $4000 or we would double the base (0x4000 -> 0x8000)
            let absolute_addr = def.offset;
            helper_equ_declarations.push(format!("{} EQU ${:04X}", name, absolute_addr));
        }
    }
    helper_equ_declarations.push(String::new()); // Blank line after EQUs
    
    // Step 7: Assemble other banks with injected EQU declarations (PASS 2)
    let mut binaries = vec![fixed_bank_binary];
    
    for section in other_sections {
        // Inject both shared EQUs and helper EQUs at the beginning of the bank's ASM
        let mut augmented_lines = Vec::new();
        augmented_lines.extend(shared_equ_definitions.clone());
        augmented_lines.extend(helper_equ_declarations.clone());
        augmented_lines.extend(section.asm_lines);
        
        let asm_source = augmented_lines.join("\n");
        
        // Assemble with injected symbols
        match m6809::assemble_m6809(&asm_source, section.org_address, false, false) {
            Ok((bytes, _line_map, symbol_table, _unresolved)) => {
                let symbols: HashMap<String, SymbolDef> = symbol_table
                    .into_iter()
                    .map(|(name, offset)| (name, SymbolDef { offset, is_export: false }))
                    .collect();
                
                binaries.push(BankBinary {
                    bank_id: section.bank_id,
                    bytes,
                    symbols,
                });
            }
            Err(e) => {
                return Err(AssemblyError::Failed(format!(
                    "Failed to assemble bank {} (pass 2): {}", 
                    section.bank_id, e
                )));
            }
        }
    }
    
    // Sort binaries by bank_id for consistent output
    binaries.sort_by_key(|b| b.bank_id);
    
    Ok(binaries)
}

/// Simple assembly for single-bank projects (no cross-bank references)
fn assemble_banks_simple(sections: Vec<BankSection>) -> Result<Vec<BankBinary>, AssemblyError> {
    let mut binaries = Vec::new();
    
    for section in sections {
        // Reconstruct ASM source for this bank
        let asm_source = section.asm_lines.join("\n");
        
        // Assemble with M6809 native assembler
        match m6809::assemble_m6809(&asm_source, section.org_address, false, false) {
            Ok((bytes, _line_map, symbol_table, _unresolved)) => {
                // Convert symbol_table (HashMap<String, u16>) to SymbolDef format
                let symbols: HashMap<String, SymbolDef> = symbol_table
                    .into_iter()
                    .map(|(name, offset)| (name, SymbolDef { offset, is_export: false }))
                    .collect();
                
                binaries.push(BankBinary {
                    bank_id: section.bank_id,
                    bytes,
                    symbols,
                });
            }
            Err(e) => {
                return Err(AssemblyError::Failed(format!(
                    "Failed to assemble bank {}: {}", 
                    section.bank_id, e
                )));
            }
        }
    }
    
    Ok(binaries)
}

// This will be expanded with real assembler logic
