// ASM Address Mapper - Post-processing to map ASM lines to binary addresses
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::Result;
use crate::backend::debug_info::DebugInfo;
use crate::backend::m6809_opcodes;

/// Represents the address mapping of one line of ASM code
#[derive(Debug, Clone)]
pub struct AsmLineAddress {
    pub line_number: u32,      // Line number in ASM file (1-indexed)
    pub address: u16,          // Memory address in binary
    pub instruction: String,   // Disassembled instruction for verification
}

/// Generate address mapping by disassembling the binary and correlating with ASM
/// header_offset: START address from binary symbol table (single source of truth)
/// symbol_table: All labels from binary with their absolute addresses
pub fn generate_asm_address_map(
    asm_path: &PathBuf, 
    bin_path: &PathBuf,
    header_offset: u16,
    symbol_table: &HashMap<String, u16>,
    debug_info: &mut DebugInfo
) -> Result<()> {
    eprintln!("Phase 6.5: Generating ASM address map...");
    
    // Read the generated ASM file
    let asm_content = fs::read_to_string(asm_path)?;
    let asm_lines: Vec<&str> = asm_content.lines().collect();
    eprintln!("✓ Read {} lines from ASM file", asm_lines.len());
    
    // Read the binary file
    let bin_data = fs::read(bin_path)?;
    eprintln!("✓ Read {} bytes from binary file", bin_data.len());
    
    // Use header_offset from binary symbol table (already passed as parameter)
    // DO NOT calculate here - single source of truth is BinaryEmitter
    eprintln!("✓ Using header offset from binary START symbol: 0x{:04X} bytes", header_offset);
    
    // Parse ASM to find ALL lines (including comments and empty lines)
    let mut asm_line_map = HashMap::new();
    let mut current_address = 0u16; // Will be set by ORG directive
    let mut bin_offset = 0usize; // Offset into binary data
    let mut last_valid_address = 0u16; // Track last address for non-code lines
    let mut seen_org = false; // Track if we've seen the first ORG (skip boot stub lines)
    
    for (line_idx, line) in asm_lines.iter().enumerate() {
        let line_number = (line_idx + 1) as u32;
        let trimmed = line.trim();
        
        // Handle ORG directive
        if let Some(org_addr) = parse_org_directive(trimmed) {
            current_address = org_addr;
            seen_org = true; // Mark that we've seen ORG - start mapping from here
            // CRITICAL FIX: bin_offset must start at header_offset, not org_addr
            // The binary file has header at 0x00-0xBD, code starts at header_offset
            bin_offset = header_offset as usize; // Physical offset in binary file
            last_valid_address = current_address.wrapping_add(header_offset);
            eprintln!("Found ORG directive at line {}: ${:04X} (bin_offset: 0x{:X})", line_number, current_address, bin_offset);
            // Map ORG line itself
            asm_line_map.insert(line_number, AsmLineAddress {
                line_number,
                address: last_valid_address,
                instruction: trimmed.to_string(),
            });
            continue;
        }
        
        // Skip all lines before the first ORG (boot stub, headers, early EQUs)
        // These lines are part of the cartridge header/boot and don't have valid runtime addresses
        if !seen_org {
            continue;
        }
        
        // Handle labels (e.g., "VECTREX_PRINT_TEXT:")
        if trimmed.ends_with(':') {
            let label = &trimmed[..trimmed.len()-1];
            if let Some(&symbol_addr) = symbol_table.get(label) {
                // Symbol found in table: use it as authoritative source
                current_address = symbol_addr.wrapping_sub(header_offset);
                bin_offset = symbol_addr as usize;
            }
            // Labels are NOT added to the map - they don't represent executable code
            // The next instruction after the label will have the address
            continue;
        }
        
        // Skip comment lines - they don't have addresses
        if trimmed.starts_with(';') {
            continue;
        }
        
        // Handle instructions - use REAL disassembly from binary
        if trimmed.chars().next().map_or(false, |c| c.is_uppercase()) || trimmed.starts_with('\t') {
            // Use RUNTIME address (with header offset) for PDB - this matches what debugger expects
            // The debugger/emulator reports PC using runtime addresses (logical + header_offset)
            let runtime_address = current_address.wrapping_add(header_offset);
            last_valid_address = runtime_address;
            
            // Calculate actual instruction size by reading from binary
            let instruction_size = if bin_offset < bin_data.len() {
                disassemble_instruction_size(&bin_data, bin_offset)
            } else {
                estimate_instruction_size(trimmed) // Fallback if beyond binary
            };
            
            // DEBUG: Log first 20 instructions to see progression
            if line_number >= 430 && line_number <= 436 {
                eprintln!("  ASM line {}: current_addr=0x{:04X} runtime=0x{:04X} bin_offset=0x{:04X} size={} | {}", 
                    line_number, current_address, runtime_address, bin_offset, instruction_size, trimmed);
            }
            
            // Map this line to RUNTIME address (what debugger/emulator uses)
            asm_line_map.insert(line_number, AsmLineAddress {
                line_number,
                address: runtime_address,
                instruction: trimmed.to_string(),
            });
            
            current_address = current_address.wrapping_add(instruction_size);
            bin_offset += instruction_size as usize;
            continue; // Already mapped, continue to next line
        }
        
        // Skip comments and empty lines - they should NOT be in the map
        // Breakpoints can only be set on executable code (labels and instructions)
    }
    
    eprintln!("✓ Mapped {} ASM lines (labels + instructions only, comments ignored)", asm_line_map.len());
    
    // Update debug info with address mappings
    for (_line_num, line_addr) in asm_line_map {
        // Add to asmFunctions if it's a function start (label ending with :)
        // Classify labels into functions vs data
        if line_addr.instruction.ends_with(':') {
            let function_name = &line_addr.instruction[..line_addr.instruction.len()-1];
            
            // Skip data labels (typically start with underscore or are all caps constants)
            let is_data_label = function_name.starts_with('_') || 
                                function_name.starts_with("CONST_") ||
                                function_name.starts_with("ARRAY_") ||
                                function_name.starts_with("STRING_") ||
                                function_name == "START" ||
                                function_name == "MAIN" ||
                                function_name == "LOOP_BODY";
            
            if !is_data_label {
                // Extract ASM filename from the path
                let asm_filename = asm_path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("main.asm")
                    .to_string();
                    
                debug_info.add_asm_function(
                    function_name.to_string(),
                    asm_filename,
                    line_addr.line_number as usize,
                    (line_addr.line_number + 10) as usize, // Estimate end line (will be improved)
                    "native"
                );
                eprintln!("Added ASM function '{}' at line {} → ${:04X}", 
                    function_name, line_addr.line_number, line_addr.address);
            }
        }
        
        // Also add to address mapping
        debug_info.add_asm_address(line_addr.line_number as usize, line_addr.address);
    }
    
    eprintln!("✓ Phase 6.5 SUCCESS: ASM address mapping complete");
    Ok(())
}

/// Calculate the header offset by finding where code execution starts
/// The Vectrex header includes copyright string, title, and metadata.
/// Code starts at the first executable label (typically "START")
/// 
/// DEPRECATED: This function is no longer used. Header offset is now obtained
/// directly from the binary's START symbol in the symbol table.
/// Keeping for reference only.
#[allow(dead_code)]
fn calculate_header_offset_deprecated(asm_lines: &[&str]) -> u16 {
    // This function made ESTIMATIONS of instruction sizes which were often wrong.
    // The correct approach is to use the START symbol from the binary's symbol table.
    // See Phase 6 in main.rs where binary_symbol_table is used.
    0
}

/// Disassemble one instruction from binary and return its size
fn disassemble_instruction_size(bin_data: &[u8], offset: usize) -> u16 {
    if offset >= bin_data.len() {
        return 1;
    }
    
    let opcode = bin_data[offset];
    let next_byte = if offset + 1 < bin_data.len() {
        Some(bin_data[offset + 1])
    } else {
        None
    };
    
    // Use the REAL opcode table from m6809_opcodes module (accurate sizes)
    m6809_opcodes::get_instruction_size(opcode, next_byte)
}

/// Get instruction size based on M6809 opcode
/// Simplified version - covers most common cases
/// Parse ORG directive and return the address
fn parse_org_directive(line: &str) -> Option<u16> {
    let trimmed = line.trim().to_uppercase();
    if trimmed.starts_with("ORG") {
        // Parse "ORG $8000" or "ORG 8000" format
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let addr_str = parts[1];
            if let Some(hex_str) = addr_str.strip_prefix('$') {
                return u16::from_str_radix(hex_str, 16).ok();
            } else if let Ok(decimal) = addr_str.parse::<u16>() {
                return Some(decimal);
            }
        }
    }
    None
}

/// Estimate instruction size based on mnemonic (simplified heuristic)
fn estimate_instruction_size(instruction: &str) -> u16 {
    let trimmed = instruction.trim().to_uppercase();
    
    // Common patterns for size estimation
    if trimmed.contains('#') {
        // Immediate mode - typically 2-3 bytes
        if trimmed.contains("LD") && trimmed.contains('#') {
            return if trimmed.contains("LDD") || trimmed.contains("LDX") || trimmed.contains("LDY") { 3 } else { 2 };
        }
        return 2; // Most immediate instructions are 2 bytes
    }
    
    if trimmed.starts_with("JSR") || trimmed.starts_with("JMP") {
        return 3; // JSR/JMP absolute are 3 bytes
    }
    
    if trimmed.starts_with("BSR") || trimmed.starts_with("BR") {
        return 2; // Branch instructions are 2 bytes
    }
    
    if trimmed.starts_with("RTS") || trimmed.starts_with("RTI") {
        return 1; // Return instructions are 1 byte
    }
    
    // Default assumption for unknown instructions
    2
}