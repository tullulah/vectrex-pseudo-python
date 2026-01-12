/// Multi-Bank Linker - Generate 512KB multi-bank ROM from sectioned ASM
///
/// Sequential Bank Model (2025-01-02):
/// - Banks #0 to #(N-2): Code fills sequentially (address $0000 per bank)
/// - Bank #(N-1): Reserved for runtime helpers (address $0000)
///
/// Each bank is assembled with ORG $0000, then concatenated to form final ROM.
/// No "fixed bank" concept - all banks have same addressing model.

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
    pub use_native_assembler: bool, // Use vecasm vs lwasm
}

impl MultiBankLinker {
    pub fn new(rom_bank_size: u32, rom_bank_count: u8, use_native_assembler: bool) -> Self {
        MultiBankLinker {
            rom_bank_size,
            rom_bank_count,
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
        // Helper: detect start of shared data (arrays/consts) inside a bank block
        let extract_shared_tail = |code: &str| -> Option<String> {
            let markers = [
                "; Array literal for variable",
                "; Const array literal",
                "; Inline array",
                "; CONST ARRAY",
                "; === CONST ARRAY",
            ];

            markers
                .iter()
                .filter_map(|m| code.find(m))
                .min()
                .map(|idx| code[idx..].to_string())
        };
        
        // Helper: extract wrappers section from code
        let extract_wrappers = |code: &str| -> (String, String) {
            let marker = "; ===== CROSS-BANK CALL WRAPPERS =====";
            if let Some(idx) = code.find(marker) {
                let before = code[..idx].to_string();
                let after = code[idx..].to_string();
                (before, after)
            } else {
                (code.to_string(), String::new())
            }
        };

        let mut sections: HashMap<u8, BankSection> = HashMap::new();
        let mut current_bank_id: Option<u8> = None;
        let mut current_org: Option<u16> = None;
        let mut current_code = String::new();
        let mut header = String::new();
        let mut include_directives = String::new(); // INCLUDE directives - needed by ALL banks
        let mut definitions = String::new(); // EQU definitions - needed by ALL banks
        let mut runtime_helpers = String::new(); // Runtime helper functions - needed by ALL banks
        let mut shared_tail = String::new(); // Data tail (arrays/consts) from last bank
        let mut data_bank_id: Option<u8> = None; // Bank that originally contained shared_tail
        let mut in_bank_section = false;
        let mut in_definitions = false;
        let mut definitions_ended = false;  // Track when EQU section ends
        let mut post_bank_code = String::new(); // Code AFTER all bank sections (wrappers, etc.)
        
        for line in asm_content.lines() {
            // Collect INCLUDE directives (before bank sections)
            if !in_bank_section {
                let trimmed = line.trim();
                if trimmed.to_uppercase().starts_with("INCLUDE") {
                    include_directives.push_str(line);
                    include_directives.push('\n');
                    continue;
                }
            }
            
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
                    definitions_ended = true;  // Mark that definitions section has ended
                }
                continue;
            }
            

            // Detect bank header: "; BANK #N - M function(s)"
            if line.starts_with("; BANK #") {
                // If we're coming from a bank section, save it first
                // NOTE: Don't add runtime_helpers here - they'll be added later at the end
                if in_bank_section {
                    if let (Some(bank_id), Some(org)) = (current_bank_id, current_org) {
                        // Prepend INCLUDE + definitions ONLY (runtime helpers added at EOF after extraction)
                        let full_code = format!("{}\n{}\n{}", include_directives, definitions, current_code);
                        sections.insert(bank_id, BankSection {
                            bank_id,
                            org,
                            asm_code: full_code,
                            size_estimate: current_code.len(),
                        });
                    }
                }
                
                // Parse new bank ID
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let bank_str = parts[2].trim_matches(|c| c == '#' || c == ' ' || c == '-');
                    if let Ok(bank_id) = bank_str.parse::<u8>() {
                        current_bank_id = Some(bank_id);
                        current_code.clear();
                        in_bank_section = true;
                        post_bank_code.clear();  // Reset post-bank code when entering new bank
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
            
            // Detect end of bank sections: wrappers section or other post-bank code
            // This allows us to exit in_bank_section when we encounter these markers
            if in_bank_section && line.starts_with("; ===== ") {
                // Save current bank before exiting in_bank_section
                // But DO NOT clear current_bank_id/org - let EOF handler save it properly
                in_bank_section = false;
                
                // Now accumulate this line and remaining lines in post_bank_code
                post_bank_code.push_str(line);
                post_bank_code.push('\n');
                continue;
            }
            
            // Accumulate code
            if in_bank_section {
                current_code.push_str(line);
                current_code.push('\n');
            } else if definitions_ended {
                // After definitions end, before first bank - this is runtime helpers
                // Or after last bank section ends - this is also runtime helpers (wrappers, etc.)
                post_bank_code.push_str(line);
                post_bank_code.push('\n');
            } else {
                // Before definitions - this is header
                header.push_str(line);
                header.push('\n');
            }
        }
        
        // ===== FINALIZE RUNTIME HELPERS =====
        // Merge post_bank_code (wrappers and other helpers after banks)
        runtime_helpers.push_str(&post_bank_code);
        
        // Also extract wrappers from current_code if they're there (in case marker detection didn't split them)
        let (bank_code_only, extracted_wrappers) = extract_wrappers(&current_code);
        if !extracted_wrappers.is_empty() {
            runtime_helpers.push_str(&extracted_wrappers);
        }
        
        // ===== SAVE LAST BANK WITH RUNTIME HELPERS =====
        if let (Some(bank_id), Some(org)) = (current_bank_id, current_org) {
            // Capture shared tail (arrays/consts) from bank_code_only
            if let Some(tail) = extract_shared_tail(&bank_code_only) {
                shared_tail = tail;
                data_bank_id = Some(bank_id);
            }
            
            // Prepend INCLUDE + definitions + runtime helpers to last bank
            let full_code = format!("{}\n{}\n{}\n{}", include_directives, definitions, runtime_helpers, bank_code_only);
            let size = full_code.len();
            sections.insert(bank_id, BankSection {
                bank_id,
                org,
                asm_code: full_code,
                size_estimate: size,
            });
        }
        
        // ===== ADD RUNTIME HELPERS TO ALL PREVIOUSLY SAVED BANKS =====
        // All banks that were saved earlier (in the loop) don't have runtime_helpers yet
        // We need to add them now that we know what they are
        for (_, section) in sections.iter_mut() {
            // Check if this bank already has runtime_helpers (only the last bank saved above will)
            if !section.asm_code.contains("; ===== CROSS-BANK CALL WRAPPERS =====") && !runtime_helpers.is_empty() {
                // Insert runtime_helpers AFTER definitions, BEFORE bank code
                // Find where the bank code starts (look for first ORG directive)
                let lines: Vec<&str> = section.asm_code.lines().collect();
                let mut new_code = String::new();
                let mut org_found = false;
                
                for line in lines {
                    if !org_found && line.trim().starts_with("ORG ") {
                        // Insert runtime_helpers BEFORE ORG
                        new_code.push_str(&runtime_helpers);
                        new_code.push('\n');
                        org_found = true;
                    }
                    new_code.push_str(line);
                    new_code.push('\n');
                }
                
                section.asm_code = new_code;
                section.size_estimate = section.asm_code.len();
            }
        }
        // CRITICAL: Header (code before first bank) belongs to Bank #31 (fixed bank)
        // The header contains START, strings, constants - all must be in fixed bank
        // Insert header at the BEGINNING of Bank #31's code (before ORG directive)
        if !header.is_empty() {
            if let Some(bank31) = sections.get_mut(&31) {
                // Bank #31's asm_code has: [INCLUDE] + [definitions] + [runtime_helpers] + [bank code with ORG]
                // We need to insert header BEFORE the ORG line
                let lines: Vec<&str> = bank31.asm_code.lines().collect();
                let mut new_code = String::new();
                let mut org_found = false;
                
                for line in lines {
                    if !org_found && line.trim().starts_with("ORG") {
                        // Insert header BEFORE ORG directive
                        new_code.push_str(&header);
                        new_code.push('\n');
                        org_found = true;
                    }
                    new_code.push_str(line);
                    new_code.push('\n');
                }
                
                bank31.asm_code = new_code;
                bank31.size_estimate += header.len();
            } else {
                // No Bank #31 section found - create one with header
                let full_code = format!("{}\n{}\n{}\n{}", 
                    include_directives,
                    definitions,
                    runtime_helpers,
                    header
                );
                let size = full_code.len();
                sections.insert(31, BankSection {
                    bank_id: 31,
                    org: 0x4000,
                    asm_code: full_code,
                    size_estimate: size,
                });
            }
        }

        // Propagate shared tail (arrays/consts) to all banks except the one that already contains it
        if !shared_tail.is_empty() {
            for (bank_id, section) in sections.iter_mut() {
                if Some(*bank_id) != data_bank_id {
                    section.asm_code.push_str("\n");
                    section.asm_code.push_str(&shared_tail);
                    section.size_estimate = section.asm_code.len();
                }
            }
        }
        
        Ok(sections)
    }
    
    /// Assemble a single bank section to binary
    ///
    /// Creates temporary ASM file with:
    /// - Common header (from pseudo-bank 255)
    /// - Bank-specific ORG and code
    /// - External symbols from fixed bank (if not fixed bank itself)
    /// - Assembles with vecasm or lwasm
    ///
    /// Returns: Binary data (padded to bank size)
    pub fn assemble_bank(
        &self,
        bank_section: &BankSection,
        temp_dir: &Path,
        helper_symbols: &HashMap<String, u16>,
    ) -> Result<Vec<u8>, String> {
        // Bank ASM already contains everything
        let mut full_asm = bank_section.asm_code.clone();
        
        // Prepend external symbol definitions from helper bank and shared data
        // This is needed for all banks to reference symbols from helpers and arrays/consts
        if !helper_symbols.is_empty() {
            let mut external_symbols = String::from("; External symbols (helpers and shared data)\n");
            for (symbol, address) in helper_symbols {
                external_symbols.push_str(&format!("{} EQU ${:04X}\n", symbol, address));
            }
            external_symbols.push_str("\n");
            
            // Insert after INCLUDE directive
            let lines: Vec<&str> = full_asm.lines().collect();
            let mut new_asm = String::new();
            let mut include_found = false;
            
            for line in lines {
                new_asm.push_str(line);
                new_asm.push('\n');
                
                if !include_found && line.trim().to_uppercase().starts_with("INCLUDE") {
                    new_asm.push_str(&external_symbols);
                    include_found = true;
                }
            }
            
            full_asm = new_asm;
        }
        
        let full_asm = &full_asm;
        
        // CRITICAL: For multi-bank, convert all short branches to long branches in helpers
        // This is needed because DIV16, MUL16, etc. have branches that exceed ±127 bytes
        // in banks beyond #0. The assembler cannot emit long branches based on final
        // offsets (they're not known yet), so we convert at codegen time.
        let full_asm_longbranch = convert_short_to_long_branches(full_asm);
        
        // CRITICAL: Assemble with ORG $0000 for multi-bank
        // The bank's actual ORG ($0000 or $4000) is just for logical addressing
        // Physical ROM layout: each bank starts at offset 0 in its binary
        // Later, the linker places each bank at correct ROM offset (bank_id * 16KB)
        let (binary, _line_map, _symbol_table, _unresolved) = crate::backend::asm_to_binary::assemble_m6809(
            &full_asm_longbranch,
            0x0000,  // Force ORG 0 - each bank is a separate 16KB chunk
            false,   // Not object mode
            false    // Don't auto-convert (we already did it above)
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
    /// 2. Extract symbols from fixed bank (Bank #31)
    /// 3. Assemble each bank separately (with external symbols if needed)
    /// 4. Concatenate in order (0, 1, ..., 31)
    /// 5. Write to output ROM file
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
        for (bank_id, section) in &sections {
            eprintln!("       Bank #{}: {} bytes ASM", bank_id, section.asm_code.len());
        }
        
        // Extract symbols from fixed bank for cross-bank references
        let _helper_bank_id = (self.rom_bank_count - 1) as u8;
        let mut helper_bank_symbols = HashMap::new();
        
        // ALSO extract symbols from FULL ASM (includes arrays, consts, runtime helpers)
        // This is needed because arrays/data are defined OUTSIDE bank sections
        eprintln!("     - Extracting symbols from Full ASM (including arrays/data)...");
        for line in asm_content.lines() {
            let trimmed = line.trim();
            // Look for labels (ending with ':')
            if let Some(colon_pos) = trimmed.find(':') {
                let label = trimmed[..colon_pos].trim();
                if !label.is_empty() 
                    && !label.starts_with(';') 
                    && !label.starts_with('.')
                    && label.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && !label.starts_with("DSL_")  // Skip Draw Sync List internal labels
                    && !label.starts_with("PMUSIC_")  // Skip music player internal labels
                {
                    // Store with placeholder address 0x0000 (Helper bank start)
                    // Actual address will be resolved during linking
                    helper_bank_symbols.insert(label.to_string(), 0x0000);
                }
            }
        }
        
        eprintln!("     - Found {} external symbols for cross-bank references", 
            helper_bank_symbols.len());
        
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
                // IMPORTANT: Pass external symbols to ALL banks (helpers available everywhere)
                let binary = self.assemble_bank(section, &temp_dir, &helper_bank_symbols)?;
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
        
        // Patch header: copy header bytes from single-bank .bin so ROM starts with valid Vectrex header
        // Derive .bin path next to the .rom
        let bin_path = output_rom_path.with_extension("bin");
        let mut final_rom = rom_data;

        if let Ok(bin_bytes) = fs::read(&bin_path) {
            // Typical Vectrex header is within the first 0x100 bytes (copyright string: "g GCE 1982\x80")
            let header_len = 0x100usize;
            if bin_bytes.len() >= header_len && final_rom.len() >= header_len {
                final_rom[0..header_len].copy_from_slice(&bin_bytes[0..header_len]);
                eprintln!("     ✓ Patched ROM header from {} ({} bytes)", bin_path.display(), header_len);
            } else if !bin_bytes.is_empty() && !final_rom.is_empty() {
                let n = bin_bytes.len().min(final_rom.len()).min(0x100);
                final_rom[0..n].copy_from_slice(&bin_bytes[0..n]);
                eprintln!("     ✓ Patched ROM header from {} ({} bytes)", bin_path.display(), n);
            } else {
                eprintln!("     ⚠ Skipped header patch: insufficient data (bin={}, rom={})", bin_bytes.len(), final_rom.len());
            }

            // Validate header signature: expect "g GCE 1982" + 0x80 at start
            let expected_sig = b"g GCE 1982";
            if final_rom.len() >= expected_sig.len() + 1 {
                let sig_ok = &final_rom[0..expected_sig.len()] == expected_sig && final_rom[expected_sig.len()] == 0x80;
                if sig_ok {
                    eprintln!("     ✓ Header validation: signature 'g GCE 1982' + $80 present at offset 0");
                } else {
                    // Print a short hex preview to aid debugging
                    let preview_len = 12.min(final_rom.len());
                    let mut hex = String::new();
                    for b in &final_rom[0..preview_len] {
                        use std::fmt::Write as _;
                        let _ = write!(hex, "{:02X} ", b);
                    }
                    eprintln!("     ⚠ Header validation: unexpected signature at start (bytes: {})", hex.trim_end());
                }
            } else {
                eprintln!("     ⚠ Header validation: ROM too small to validate signature ({} bytes)", final_rom.len());
            }
        } else {
            eprintln!("     ⚠ Skipped header patch: {} not found", bin_path.display());
        }

        // Write ROM
        fs::write(output_rom_path, final_rom)
            .map_err(|e| format!("Failed to write ROM: {}", e))?;
        
        eprintln!("     ✓ Multi-bank ROM written: {} KB ({} bytes)", 
            expected_size / 1024, expected_size);
        
        // Cleanup temp directory
        // DEBUG: Don't remove temp dir so we can inspect ASM files
        // let _ = fs::remove_dir_all(&temp_dir);
        eprintln!("     [DEBUG] Temp ASM files kept in: {:?}", temp_dir);
        
        Ok(())
    }
}

/// Convert all short branches to long branches in assembly code
/// This is required for multi-bank compilation because helper functions
/// (DIV16, MUL16, etc.) contain branches that may exceed ±127 bytes when
/// split across banks.
/// 
/// Conversions:
/// - BRA label → LBRA label  (0x20 → 0x16)
/// - BEQ label → LBEQ label  (0x27 → 0x10 0x27)
/// - BNE label → LBNE label  (0x26 → 0x10 0x26)
/// - BCS label → LBCS label  (0x25 → 0x10 0x25)
/// - BCC label → LBCC label  (0x24 → 0x10 0x24)
/// - And many other conditional branches...
fn convert_short_to_long_branches(asm: &str) -> String {
    let mut result = String::new();
    
    for line in asm.lines() {
        // Don't touch comments or labels
        if line.trim().starts_with(';') || line.trim().starts_with('*') {
            result.push_str(line);
            result.push('\n');
            continue;
        }
        
        let trimmed = line.trim();
        
        // Check for short branch instructions
        let converted = if trimmed.starts_with("BRA ") {
            let operand = trimmed.strip_prefix("BRA ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBRA {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BEQ ") {
            let operand = trimmed.strip_prefix("BEQ ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBEQ {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BNE ") {
            let operand = trimmed.strip_prefix("BNE ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBNE {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BCS ") {
            let operand = trimmed.strip_prefix("BCS ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBCS {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BCC ") {
            let operand = trimmed.strip_prefix("BCC ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBCC {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BLO ") {
            let operand = trimmed.strip_prefix("BLO ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBLO {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BHS ") {
            let operand = trimmed.strip_prefix("BHS ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBHS {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BLT ") {
            let operand = trimmed.strip_prefix("BLT ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBLT {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BGE ") {
            let operand = trimmed.strip_prefix("BGE ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBGE {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BLE ") {
            let operand = trimmed.strip_prefix("BLE ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBLE {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BGT ") {
            let operand = trimmed.strip_prefix("BGT ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBGT {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BVS ") {
            let operand = trimmed.strip_prefix("BVS ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBVS {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BVC ") {
            let operand = trimmed.strip_prefix("BVC ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBVC {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BMI ") {
            let operand = trimmed.strip_prefix("BMI ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBMI {}", " ".repeat(indent), operand)
        } else if trimmed.starts_with("BPL ") {
            let operand = trimmed.strip_prefix("BPL ").unwrap_or("");
            let indent = line.len() - line.trim_start().len();
            format!("{}LBPL {}", " ".repeat(indent), operand)
        } else {
            // No branch instruction - keep as is
            line.to_string()
        };
        
        result.push_str(&converted);
        result.push('\n');
    }
    
    result
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
