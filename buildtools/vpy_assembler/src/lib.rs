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

// Helper function to extract labels from ASM when assembly fails
// Based on core/src/backend/m6809/multi_bank_linker.rs
fn extract_labels_from_asm(asm: &str, org_address: u16) -> HashMap<String, u16> {
    let mut labels = HashMap::new();
    let mut current_offset = 0u16;
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('*') {
            continue;
        }
        
        // Skip INCLUDE and EQU directives
        if trimmed.to_uppercase().starts_with("INCLUDE") || trimmed.to_uppercase().contains(" EQU ") {
            continue;
        }
        
        // Parse label (line ends with :)
        if let Some(label_pos) = trimmed.find(':') {
            let label = trimmed[..label_pos].trim().to_string();
            if !label.is_empty() && !label.starts_with(' ') {
                labels.insert(label, org_address + current_offset);
            }
        }
        
        // Estimate instruction size (rough approximation)
        if !trimmed.is_empty() && !trimmed.ends_with(':') {
            if trimmed.to_uppercase().starts_with("FCB ") {
                current_offset += 1;
            } else if trimmed.to_uppercase().starts_with("FDB ") {
                current_offset += 2;
            } else if trimmed.to_uppercase().starts_with("FCC ") {
                // Count string length roughly
                if let Some(start) = trimmed.find('"') {
                    if let Some(end) = trimmed.rfind('"') {
                        if end > start {
                            current_offset += (end - start - 1) as u16 + 1;
                        }
                    }
                }
            } else if !trimmed.starts_with("ORG ") {
                // Regular instruction estimate
                current_offset += 2;
            }
        }
    }
    
    labels
}

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
    
    // === TWO-PASS ASSEMBLY FOR MULTIBANK (Based on core/multi_bank_linker.rs) ===
    
    let max_bank_id = *bank_ids.iter().max().unwrap();
    let helper_bank_id = max_bank_id;
    
    // **PASS 1**: Iteratively extract global symbol table from all banks
    // This allows cross-bank references (e.g., Bank 31 interrupt vectors referencing Bank 0 START)
    let mut all_symbols: HashMap<String, u16> = HashMap::new();
    let max_iterations = 5;
    
    // Load BIOS symbols first
    use crate::m6809::load_vectrex_symbols;
    let mut bios_symbols = HashMap::new();
    load_vectrex_symbols(&mut bios_symbols);
    for (name, addr) in bios_symbols.iter() {
        all_symbols.insert(name.clone(), *addr);
    }
    
    // Placeholder for START if not found (will be overwritten)
    if !all_symbols.contains_key("START") {
        all_symbols.insert("START".to_string(), 0x0000);
    }
    
    // Create a map of sections by bank_id for easy access
    let mut sections_map: HashMap<usize, BankSection> = HashMap::new();
    for section in sections {
        sections_map.insert(section.bank_id, section);
    }
    
    // Iterative symbol extraction from all banks
    eprintln!("PASS 1: Starting iterative symbol extraction...");
    for iteration in 0..max_iterations {
        let prev_count = all_symbols.len();
        eprintln!("  Iteration {}: {} symbols known", iteration + 1, prev_count);
        
        for bank_id in 0..=max_bank_id {
            if let Some(section) = sections_map.get(&bank_id) {
                // Inject all known symbols as EQU at the beginning
                let mut augmented_lines = Vec::new();
                augmented_lines.push("; === Symbols from other banks (PASS 1) ===".to_string());
                for (symbol, address) in &all_symbols {
                    augmented_lines.push(format!("{} EQU ${:04X}", symbol, address));
                }
                augmented_lines.push(String::new());
                augmented_lines.extend(section.asm_lines.clone());
                
                let asm_source = augmented_lines.join("\n");
                
                // Assemble to extract symbols (ignore errors in early iterations)
                match m6809::assemble_m6809(&asm_source, section.org_address, false, false) {
                    Ok((_bytes, _line_map, symbol_table, _unresolved)) => {
                        // Extract new symbols from successful assembly
                        let mut new_symbols = 0;
                        for (name, &offset) in &symbol_table {
                            // Skip internal labels
                            if !name.starts_with('.') && !name.starts_with('_') {
                                if !all_symbols.contains_key(name) {
                                    new_symbols += 1;
                                }
                                all_symbols.insert(name.clone(), offset);
                            }
                        }
                        eprintln!("    Bank {}: {} new symbols extracted (assembly succeeded)", bank_id, new_symbols);
                    }
                    Err(_e) => {
                        // Assembly failed - extract labels via simple parsing
                        let parsed_labels = extract_labels_from_asm(&asm_source, section.org_address);
                        let mut new_symbols = 0;
                        for (label, offset) in parsed_labels {
                            if !label.starts_with('.') && !label.starts_with('_') {
                                if !all_symbols.contains_key(&label) {
                                    new_symbols += 1;
                                }
                                all_symbols.insert(label, offset);
                            }
                        }
                        eprintln!("    Bank {}: {} new symbols extracted (via parsing, assembly failed)", bank_id, new_symbols);
                    }
                }
            }
        }
        
        // Check convergence
        if all_symbols.len() == prev_count {
            eprintln!("  Convergence reached at iteration {}", iteration + 1);
            break;
        }
    }
    eprintln!("PASS 1 complete: {} total symbols\n", all_symbols.len());
    
    // **PASS 2**: Assemble all banks with complete symbol table
    let mut binaries = Vec::new();
    
    for bank_id in 0..=max_bank_id {
        if let Some(section) = sections_map.get(&bank_id) {
            // Inject complete symbol table
            let mut augmented_lines = Vec::new();
            augmented_lines.push("; === Global symbol table (PASS 2) ===".to_string());
            for (symbol, address) in &all_symbols {
                augmented_lines.push(format!("{} EQU ${:04X}", symbol, address));
            }
            augmented_lines.push(String::new());
            augmented_lines.extend(section.asm_lines.clone());
            
            let asm_source = augmented_lines.join("\n");
            
            // Final assembly with all symbols available
            let (bytes, _line_map, symbol_table, _unresolved) = m6809::assemble_m6809(
                &asm_source,
                section.org_address,
                false,
                false
            ).map_err(|e| AssemblyError::Failed(format!("Failed to assemble bank {}: {}", bank_id, e)))?;
            
            // Convert symbol table
            let symbols: HashMap<String, SymbolDef> = symbol_table
                .into_iter()
                .map(|(name, offset)| (name, SymbolDef { offset, is_export: bank_id == 0 || bank_id == helper_bank_id }))
                .collect();
            
            binaries.push(BankBinary {
                bank_id,
                bytes,
                symbols,
            });
        }
    }
    
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
