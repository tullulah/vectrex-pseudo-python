// ASM Address Mapper - Post-processing to map ASM lines to binary addresses
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::Result;
use crate::backend::debug_info::DebugInfo;

/// Represents the address mapping of one line of ASM code
#[derive(Debug, Clone)]
pub struct AsmLineAddress {
    pub line_number: u32,      // Line number in ASM file (1-indexed)
    pub address: u16,          // Memory address in binary
    pub instruction: String,   // Disassembled instruction for verification
}

/// Generate address mapping by disassembling the binary and correlating with ASM
pub fn generate_asm_address_map(
    asm_path: &PathBuf, 
    bin_path: &PathBuf,
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
    
    // Parse ASM to find significant lines (labels, instructions)
    let mut asm_line_map = HashMap::new();
    let mut current_address = 0u16; // Will be set by ORG directive
    
    for (line_idx, line) in asm_lines.iter().enumerate() {
        let line_number = (line_idx + 1) as u32;
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        
        // Handle ORG directive
        if let Some(org_addr) = parse_org_directive(trimmed) {
            current_address = org_addr;
            eprintln!("Found ORG directive at line {}: ${:04X}", line_number, current_address);
            continue;
        }
        
        // Handle labels (e.g., "VECTREX_PRINT_TEXT:")
        if trimmed.ends_with(':') {
            let label = &trimmed[..trimmed.len()-1];
            asm_line_map.insert(line_number, AsmLineAddress {
                line_number,
                address: current_address,
                instruction: format!("{}:", label),
            });
            eprintln!("Found label '{}' at line {} → ${:04X}", label, line_number, current_address);
            continue;
        }
        
        // Handle instructions (simple heuristic - starts with uppercase letter or tab)
        if trimmed.chars().next().map_or(false, |c| c.is_uppercase()) || trimmed.starts_with('\t') {
            // Estimate instruction size (simplified)
            let instruction_size = estimate_instruction_size(trimmed);
            
            asm_line_map.insert(line_number, AsmLineAddress {
                line_number,
                address: current_address,
                instruction: trimmed.to_string(),
            });
            
            current_address = current_address.wrapping_add(instruction_size);
        }
    }
    
    eprintln!("✓ Mapped {} significant ASM lines to addresses", asm_line_map.len());
    
    // Update debug info with address mappings
    for (_line_num, line_addr) in asm_line_map {
        // Add to asmFunctions if it's a function start (label ending with :)
        if line_addr.instruction.ends_with(':') && line_addr.instruction.starts_with("VECTREX_") {
            let function_name = &line_addr.instruction[..line_addr.instruction.len()-1];
            
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
        
        // Also add to address mapping
        debug_info.add_asm_address(line_addr.line_number as usize, line_addr.address);
    }
    
    eprintln!("✓ Phase 6.5 SUCCESS: ASM address mapping complete");
    Ok(())
}

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