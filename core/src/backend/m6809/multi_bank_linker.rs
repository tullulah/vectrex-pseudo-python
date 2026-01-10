/// Multi-Bank Linker - Generate 512KB multi-bank ROM from sectioned ASM
///
/// This module processes ASM files with multiple ORG directives (one per bank)
/// and generates a single multi-bank ROM file with all banks concatenated.
///
/// Bank Layout:
/// - Bank #0:  Offset 0x00000 (16KB) - ORG $0000 (banked window)
/// - Bank #1:  Offset 0x04000 (16KB) - ORG $0000 (banked window)
/// - ...
/// - Bank #30: Offset 0x78000 (16KB) - ORG $0000 (banked window)
/// - Bank #31: Offset 0x7C000 (16KB) - ORG $4000 (fixed bank)
///
/// Each bank is assembled separately with its ORG directive, then concatenated
/// to form the final ROM. The Vectrex hardware uses register $4000 to switch
/// between banks in the 0x0000-0x3FFF window.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents a single bank section in the ASM
#[derive(Debug, Clone)]
pub struct BankSection {
    pub bank_id: u8,
    pub org: u16,
    pub asm_code: String,
    pub size_estimate: usize,
}

/// Multi-bank linker configuration
#[derive(Debug, Clone)]
pub struct MultiBankLinker {
    pub rom_bank_size: u32,      // 16KB per bank
    pub rom_bank_count: u8,       // 32 banks total
    pub fixed_bank_id: u8,        // Bank #31 (fixed)
    pub use_native_assembler: bool, // Use vecasm vs lwasm
}

impl MultiBankLinker {
    pub fn new(rom_bank_size: u32, rom_bank_count: u8, use_native_assembler: bool) -> Self {
        MultiBankLinker {
            rom_bank_size,
            rom_bank_count,
            fixed_bank_id: rom_bank_count.saturating_sub(1),
            use_native_assembler,
        }
    }
    
    /// Split ASM file into bank sections based on ORG directives
    ///
    /// Input ASM format:
    /// ```asm
    /// ; Common header
    /// 
    /// ; ================================================
    /// ; BANK #31 - 2 function(s)
    /// ; ================================================
    ///     ORG $4000  ; Fixed bank
    /// 
    /// LOOP_BODY:
    ///     ; ... code ...
    /// 
    /// ; ================================================
    /// ; BANK #0 - 13 function(s)
    /// ; ================================================
    ///     ORG $0000  ; Banked window
    /// 
    /// INIT_GAME:
    ///     ; ... code ...
    /// ```
    ///
    /// Returns: HashMap<bank_id, BankSection>
    pub fn split_asm_by_bank(&self, asm_content: &str) -> Result<HashMap<u8, BankSection>, String> {
        let mut sections: HashMap<u8, BankSection> = HashMap::new();
        let mut current_bank_id: Option<u8> = None;
        let mut current_org: Option<u16> = None;
        let mut current_code = String::new();
        let mut header = String::new();
        let mut definitions = String::new(); // EQU definitions - needed by ALL banks
        let mut in_bank_section = false;
        let mut in_definitions = false;
        
        for line in asm_content.lines() {
            // Detect RAM definitions section
            if line.contains("=== RAM VARIABLE DEFINITIONS") {
                in_definitions = true;
                definitions.push_str(line);
                definitions.push('\n');
                continue;
            }
            
            // Collect EQU definitions
            if in_definitions {
                definitions.push_str(line);
                definitions.push('\n');
                // End of definitions when we hit empty line or non-EQU line
                if line.trim().is_empty() || (!line.contains("EQU") && !line.starts_with(';')) {
                    in_definitions = false;
                }
                continue;
            }
            

            // Detect bank header: "; BANK #N - M function(s)"
            if line.starts_with("; BANK #") {
                // Save previous bank if exists
                if let (Some(bank_id), Some(org)) = (current_bank_id, current_org) {
                    // Prepend definitions to bank code (ALL banks need EQU definitions)
                    let full_code = format!("{}\n{}", definitions, current_code);
                    sections.insert(bank_id, BankSection {
                        bank_id,
                        org,
                        asm_code: full_code,
                        size_estimate: current_code.len(),
                    });
                }
                
                // Parse new bank ID
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let bank_str = parts[2].trim_matches(|c| c == '#' || c == ' ' || c == '-');
                    if let Ok(bank_id) = bank_str.parse::<u8>() {
                        current_bank_id = Some(bank_id);
                        current_code.clear();
                        in_bank_section = true;
                        continue;
                    }
                }
            }
            
            // Detect ORG directive
            if line.trim().starts_with("ORG ") && in_bank_section {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let org_str = parts[1].trim_start_matches('$');
                    if let Ok(org) = u16::from_str_radix(org_str, 16) {
                        current_org = Some(org);
                        current_code.push_str(line);
                        current_code.push('\n');
                        continue;
                    }
                }
            }
            
            // Accumulate code
            if in_bank_section {
                current_code.push_str(line);
                current_code.push('\n');
            } else {
                // Header (before first bank section)
                header.push_str(line);
                header.push('\n');
            }
        }
        
        // Save last bank
        if let (Some(bank_id), Some(org)) = (current_bank_id, current_org) {
            // Prepend definitions to bank code
            let full_code = format!("{}\n{}", definitions, current_code);
            let size = full_code.len();
            sections.insert(bank_id, BankSection {
                bank_id,
                org,
                asm_code: full_code,
                size_estimate: size,
            });
        }
        
        // CRITICAL: Header (code before first bank) belongs to Bank #31 (fixed bank)
        // The header contains START, strings, constants - all must be in fixed bank
        // Prepend header to Bank #31's code
        if !header.is_empty() {
            if let Some(bank31) = sections.get_mut(&31) {
                // Bank #31 already has definitions prepended, now add header between definitions and code
                // Structure: [definitions] + [header] + [Bank #31 code]
                let existing_code = bank31.asm_code.clone();
                // Remove definitions from existing code (they're already there)
                let code_without_defs = existing_code.trim_start_matches(&definitions);
                let combined = format!("{}\n{}\n{}", definitions, header, code_without_defs);
                bank31.asm_code = combined;
                bank31.size_estimate += header.len();
            } else {
                // No Bank #31 section found - create from header + definitions
                let full_code = format!("{}\n{}", definitions, header);
                let size = full_code.len();
                sections.insert(31, BankSection {
                    bank_id: 31,
                    org: 0x4000,
                    asm_code: full_code,
                    size_estimate: size,
                });
            }
        }
        
        Ok(sections)
    }
    
    /// Assemble a single bank section to binary
    ///
    /// Creates temporary ASM file with:
    /// - Common header (from pseudo-bank 255)
    /// - Bank-specific ORG and code
    /// - Assembles with vecasm or lwasm
    ///
    /// Returns: Binary data (padded to bank size)
    pub fn assemble_bank(
        &self,
        bank_section: &BankSection,
        temp_dir: &Path,
    ) -> Result<Vec<u8>, String> {
        // Bank ASM already contains everything (header prepended for Bank #31)
        let full_asm = &bank_section.asm_code;
        
        // Assemble with native assembler (integrated, object_mode=false for final build)
        let (binary, _line_map, _symbol_table, _unresolved) = crate::backend::asm_to_binary::assemble_m6809(
            full_asm,
            bank_section.org,
            false
        ).map_err(|e| format!("Failed to assemble bank {}: {}", bank_section.bank_id, e))?;
        
        // Get binary data (already Vec<u8>, no .0 field)
        let mut binary_data = binary;
        
        // Pad to bank size (16KB)
        let bank_size = self.rom_bank_size as usize;
        if binary_data.len() > bank_size {
            return Err(format!("Bank {} overflow: {} bytes (max: {} bytes)", 
                bank_section.bank_id, binary_data.len(), bank_size));
        }
        
        // Pad with 0xFF (standard for unused ROM)
        binary_data.resize(bank_size, 0xFF);
        
        Ok(binary_data)
    }
    
    /// Generate multi-bank ROM from sectioned ASM
    ///
    /// Process:
    /// 1. Split ASM by bank sections
    /// 2. Assemble each bank separately
    /// 3. Concatenate in order (0, 1, ..., 31)
    /// 4. Write to output ROM file
    ///
    /// Output: 512KB ROM file with 32 banks
    pub fn generate_multibank_rom(
        &self,
        asm_path: &Path,
        output_rom_path: &Path,
    ) -> Result<(), String> {
        eprintln!("   [Multi-Bank Linker] Generating 512KB ROM...");
        
        // Read ASM
        let asm_content = fs::read_to_string(asm_path)
            .map_err(|e| format!("Failed to read ASM: {}", e))?;
        
        // Split by bank
        let sections = self.split_asm_by_bank(&asm_content)?;
        eprintln!("     - Found {} bank section(s)", sections.len());
        
        // Create temp directory for bank assemblies
        let temp_dir = output_rom_path.parent()
            .ok_or("Invalid output path")?
            .join("multibank_temp");
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp dir: {}", e))?;
        
        // Assemble each bank in order
        let mut rom_data = Vec::new();
        for bank_id in 0..self.rom_bank_count {
            if let Some(section) = sections.get(&bank_id) {
                eprintln!("     - Assembling Bank #{} ({} bytes code)...", bank_id, section.size_estimate);
                let binary = self.assemble_bank(section, &temp_dir)?;
                rom_data.extend_from_slice(&binary);
            } else {
                // Empty bank - fill with 0xFF
                eprintln!("     - Bank #{} empty (padding with 0xFF)", bank_id);
                rom_data.resize(rom_data.len() + self.rom_bank_size as usize, 0xFF);
            }
        }
        
        // Verify total size
        let expected_size = self.rom_bank_size as usize * self.rom_bank_count as usize;
        if rom_data.len() != expected_size {
            return Err(format!("ROM size mismatch: {} bytes (expected {} bytes)", 
                rom_data.len(), expected_size));
        }
        
        // Write ROM
        fs::write(output_rom_path, rom_data)
            .map_err(|e| format!("Failed to write ROM: {}", e))?;
        
        eprintln!("     ✓ Multi-bank ROM written: {} KB ({} bytes)", 
            expected_size / 1024, expected_size);
        
        // Cleanup temp directory
        let _ = fs::remove_dir_all(&temp_dir);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_asm_by_bank() {
        let asm = r#"
; Header
    ORG $0000

; ================================================
; BANK #31 - 2 function(s)
; ================================================
    ORG $4000  ; Fixed bank

LOOP_BODY:
    RTS

; ================================================
; BANK #0 - 13 function(s)
; ================================================
    ORG $0000  ; Banked window

INIT_GAME:
    RTS
"#;
        
        let linker = MultiBankLinker::new(16384, 32, true);
        let sections = linker.split_asm_by_bank(asm).unwrap();
        
        assert_eq!(sections.len(), 3); // 2 banks + 1 header
        assert!(sections.contains_key(&31));
        assert!(sections.contains_key(&0));
        assert!(sections.contains_key(&255)); // header
        
        let bank31 = sections.get(&31).unwrap();
        assert_eq!(bank31.org, 0x4000);
        assert!(bank31.asm_code.contains("LOOP_BODY"));
        
        let bank0 = sections.get(&0).unwrap();
        assert_eq!(bank0.org, 0x0000);
        assert!(bank0.asm_code.contains("INIT_GAME"));
    }
}
