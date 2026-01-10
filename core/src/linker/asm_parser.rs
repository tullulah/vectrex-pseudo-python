// ASM Parser for .vo object generation
// Extracts sections, symbols, and relocations from ASM with section markers

use super::object::{Section, SectionType, Symbol, SymbolScope, SymbolTable, SymbolType, Relocation, RelocationType};
use crate::backend::asm_to_binary::assemble_m6809;
use std::collections::HashMap;

/// Parsed section with ASM code
#[derive(Debug, Clone)]
struct ParsedSection {
    name: String,
    section_type: SectionType,
    flags: String,
    lines: Vec<String>,
}

/// Extract sections from ASM with section markers
/// 
/// Input: ASM text with .section markers
/// Output: Vec<Section> with binary data (assembled)
pub fn extract_sections(asm: &str) -> Result<Vec<Section>, String> {
    let parsed = parse_section_markers(asm)?;
    
    // Convert parsed sections to Section structs with binary data
    let mut sections = Vec::new();
    for ps in parsed {
        // Phase 3.1: Just extract sections (no binary)
        // Phase 3.2: Add binary assembly (see extract_sections_with_binary)
        let section = Section {
            name: ps.name.clone(),
            section_type: ps.section_type,
            bank_hint: None, // Linker decides
            alignment: 1, // Default alignment
            data: Vec::new(), // Empty for now
        };
        sections.push(section);
    }
    
    Ok(sections)
}

/// Extract sections from ASM with section markers AND assemble to binary
/// 
/// Input: ASM text with .section markers
/// Output: Vec<Section> with assembled binary data
pub fn extract_sections_with_binary(asm: &str, base_org: u16) -> Result<Vec<Section>, String> {
    let parsed = parse_section_markers(asm)?;
    
    // Convert parsed sections to Section structs with binary data
    let mut sections = Vec::new();
    let mut current_org = base_org;
    
    for ps in parsed {
        // BSS sections have no data (uninitialized)
        if ps.section_type == SectionType::Bss {
            // For BSS, estimate size from EQU directives
            let size = estimate_bss_size(&ps.lines);
            let section = Section {
                name: ps.name.clone(),
                section_type: ps.section_type,
                bank_hint: None,
                alignment: size as u16, // Store size in alignment field for BSS
                data: Vec::new(), // BSS has no data in file
            };
            sections.push(section);
            continue;
        }
        
        // Assemble section ASM to binary
        let section_asm = ps.lines.join("\n");
        let (binary, _line_map, _symbols) = assemble_m6809(&section_asm, current_org)
            .map_err(|e| format!("Failed to assemble section {}: {}", ps.name, e))?;
        
        let section = Section {
            name: ps.name.clone(),
            section_type: ps.section_type,
            bank_hint: None,
            alignment: 1,
            data: binary.clone(),
        };
        
        // Update org for next section
        current_org = current_org.wrapping_add(binary.len() as u16);
        
        sections.push(section);
    }
    
    Ok(sections)
}

/// Parse .section markers and group ASM lines by section
fn parse_section_markers(asm: &str) -> Result<Vec<ParsedSection>, String> {
    let mut sections = Vec::new();
    let mut current_section: Option<ParsedSection> = None;
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Check for .section marker
        if trimmed.starts_with(".section ") {
            // Save previous section if exists
            if let Some(section) = current_section.take() {
                sections.push(section);
            }
            
            // Parse: .section NAME, "FLAGS", @TYPE
            let parts: Vec<&str> = trimmed.splitn(4, ' ').collect();
            if parts.len() < 4 {
                return Err(format!("Invalid .section directive: {}", trimmed));
            }
            
            let name = parts[1].trim_end_matches(',').to_string();
            let flags = parts[2].trim_matches(|c| c == '"' || c == ',').to_string();
            let type_str = parts[3].trim_start_matches('@');
            
            let section_type = match type_str {
                "progbits" => {
                    // Determine if it's Text or ReadOnly based on flags
                    if flags.contains('x') {
                        SectionType::Text
                    } else {
                        SectionType::ReadOnly
                    }
                },
                "nobits" => SectionType::Bss,
                _ => return Err(format!("Unknown section type: {}", type_str)),
            };
            
            current_section = Some(ParsedSection {
                name,
                section_type,
                flags,
                lines: Vec::new(),
            });
        } else if let Some(ref mut section) = current_section {
            // Add line to current section
            section.lines.push(line.to_string());
        }
    }
    
    // Save last section
    if let Some(section) = current_section {
        sections.push(section);
    }
    
    Ok(sections)
}

/// Parse section flags string ("ax", "a", "aw")
fn parse_flags(flags_str: &str) -> u32 {
    let mut flags = 0u32;
    
    if flags_str.contains('a') {
        flags |= 0x01; // SHF_ALLOC - Section is allocated in memory
    }
    if flags_str.contains('w') {
        flags |= 0x02; // SHF_WRITE - Section is writable
    }
    if flags_str.contains('x') {
        flags |= 0x04; // SHF_EXECINSTR - Section is executable
    }
    
    flags
}

/// Build symbol table from sections
/// 
/// Scans all sections for labels (exports) and external references (imports)
pub fn build_symbol_table(sections: &[Section], asm: &str) -> Result<SymbolTable, String> {
    let mut exports = Vec::new();
    let mut imports = Vec::new();
    let mut seen_exports = HashMap::new();
    
    let parsed = parse_section_markers(asm)?;
    
    for (section_idx, ps) in parsed.iter().enumerate() {
        let mut offset = 0u32;
        
        for line in &ps.lines {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }
            
            // Check for label definition (ends with :)
            if let Some(label_end) = trimmed.find(':') {
                let label = trimmed[..label_end].trim().to_string();
                
                // Skip local labels (start with .)
                if !label.starts_with('.') {
                    if seen_exports.contains_key(&label) {
                        return Err(format!("Duplicate symbol definition: {}", label));
                    }
                    
                    exports.push(Symbol {
                        name: label.clone(),
                        section: Some(section_idx),
                        offset: offset as u16,
                        scope: SymbolScope::Global,
                        symbol_type: SymbolType::Function, // TODO: Distinguish functions from variables
                    });
                    
                    seen_exports.insert(label, section_idx);
                }
            }
            
            // Check for external references (JSR, LDD #)
            // TODO: More sophisticated parsing
            if trimmed.contains("JSR ") || trimmed.contains("LDD #") {
                // Extract symbol name (simplified)
                // This needs more robust parsing in production
            }
            
            // Estimate size (very rough - real assembler needed)
            offset += estimate_instruction_size(trimmed);
        }
    }
    
    Ok(SymbolTable { exports, imports })
}

/// Estimate instruction size (very rough approximation)
/// Real implementation needs full M6809 assembler
fn estimate_instruction_size(line: &str) -> u32 {
    let trimmed = line.trim();
    
    // Skip directives and labels
    if trimmed.starts_with('.') || trimmed.ends_with(':') || trimmed.starts_with(';') {
        return 0;
    }
    
    // FCB/FDB/FCC directives
    if trimmed.starts_with("FCB ") {
        return 1;
    }
    if trimmed.starts_with("FDB ") {
        return 2;
    }
    if trimmed.starts_with("FCC ") {
        // Count characters in string
        if let Some(start) = trimmed.find('"') {
            if let Some(end) = trimmed[start+1..].find('"') {
                return (end + 1) as u32;
            }
        }
        return 0;
    }
    
    // Instructions - average 2-3 bytes
    // This is a VERY rough estimate
    if trimmed.contains("JSR ") || trimmed.contains("LDD #") {
        return 3;
    }
    
    2 // Default estimate
}

/// Estimate BSS section size from EQU directives
fn estimate_bss_size(lines: &[String]) -> usize {
    let mut total_size = 0;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Look for EQU directives with size comments
        // Example: VAR_X EQU $C880+0 ; (2 bytes)
        if trimmed.contains("EQU") && trimmed.contains("bytes") {
            // Extract size from comment
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed[start..].find(')') {
                    let size_str = &trimmed[start+1..start+end];
                    if let Some(num_str) = size_str.split_whitespace().next() {
                        if let Ok(size) = num_str.parse::<usize>() {
                            total_size += size;
                        }
                    }
                }
            }
        }
    }
    
    // If no size found, estimate based on number of EQU directives
    if total_size == 0 {
        let equ_count = lines.iter().filter(|l| l.contains("EQU")).count();
        total_size = equ_count * 2; // Assume 2 bytes per variable
    }
    
    total_size
}

/// Collect relocations from sections
/// 
/// Scans for references to external symbols that need patching during linking
pub fn collect_relocations(
    sections: &[Section],
    symbols: &SymbolTable,
    asm: &str
) -> Result<Vec<Relocation>, String> {
    let mut relocations = Vec::new();
    let parsed = parse_section_markers(asm)?;
    
    // Build a set of exported symbol names for quick lookup
    let mut exported_names = std::collections::HashSet::new();
    for sym in &symbols.exports {
        exported_names.insert(sym.name.as_str());
    }
    
    // Scan each section for references
    for (section_idx, ps) in parsed.iter().enumerate() {
        let mut offset = 0u16;
        
        for line in &ps.lines {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }
            
            // Skip label definitions
            if trimmed.ends_with(':') {
                continue;
            }
            
            // JSR instruction - Absolute16 relocation
            if let Some(target) = extract_jsr_target(trimmed) {
                if !exported_names.contains(target.as_str()) {
                    relocations.push(Relocation {
                        section: section_idx,
                        offset,
                        reloc_type: RelocationType::Absolute16,
                        symbol: target,
                        addend: 0,
                    });
                }
            }
            
            // LDD #label, LDX #label - Absolute16 relocation
            if let Some(target) = extract_immediate_target(trimmed) {
                if !exported_names.contains(target.as_str()) {
                    relocations.push(Relocation {
                        section: section_idx,
                        offset,
                        reloc_type: RelocationType::Absolute16,
                        symbol: target,
                        addend: 0,
                    });
                }
            }
            
            // BRA, BEQ, BNE - Relative8 relocation
            if let Some(target) = extract_branch_target(trimmed) {
                if !exported_names.contains(target.as_str()) {
                    relocations.push(Relocation {
                        section: section_idx,
                        offset,
                        reloc_type: RelocationType::Relative8,
                        symbol: target,
                        addend: 0,
                    });
                }
            }
            
            // LBRA, LBEQ - Relative16 relocation
            if let Some(target) = extract_long_branch_target(trimmed) {
                if !exported_names.contains(target.as_str()) {
                    relocations.push(Relocation {
                        section: section_idx,
                        offset,
                        reloc_type: RelocationType::Relative16,
                        symbol: target,
                        addend: 0,
                    });
                }
            }
            
            // Update offset (rough estimate)
            offset = offset.wrapping_add(estimate_instruction_size(trimmed) as u16);
        }
    }
    
    Ok(relocations)
}

/// Extract target symbol from JSR instruction
fn extract_jsr_target(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("JSR ") {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            // Remove trailing comments
            let target = parts[1].split(';').next().unwrap().trim();
            return Some(target.to_string());
        }
    }
    None
}

/// Extract target symbol from immediate addressing (LDD #label, LDX #label)
fn extract_immediate_target(line: &str) -> Option<String> {
    let trimmed = line.trim();
    
    // LDD #symbol, LDX #symbol, LDA #symbol, etc.
    if (trimmed.starts_with("LDD #") || 
        trimmed.starts_with("LDX #") || 
        trimmed.starts_with("LDU #") ||
        trimmed.starts_with("LDS #")) && 
       !trimmed.contains("$") && // Not a hex literal
       !trimmed.chars().nth(5).map(|c| c.is_digit(10)).unwrap_or(false) { // Not a decimal literal
        
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let target = parts[1].trim_start_matches('#').split(';').next().unwrap().trim();
            return Some(target.to_string());
        }
    }
    None
}

/// Extract target symbol from short branch (BRA, BEQ, BNE, etc.)
fn extract_branch_target(line: &str) -> Option<String> {
    let trimmed = line.trim();
    
    let branch_ops = ["BRA ", "BEQ ", "BNE ", "BCC ", "BCS ", 
                      "BPL ", "BMI ", "BVC ", "BVS ", "BGE ", 
                      "BGT ", "BLE ", "BLT ", "BHI ", "BLS "];
    
    for op in &branch_ops {
        if trimmed.starts_with(op) {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let target = parts[1].split(';').next().unwrap().trim();
                // Skip numeric offsets (e.g., BRA -5)
                if !target.starts_with('-') && !target.starts_with('+') && 
                   !target.chars().next().map(|c| c.is_digit(10)).unwrap_or(false) {
                    return Some(target.to_string());
                }
            }
        }
    }
    None
}

/// Extract target symbol from long branch (LBRA, LBEQ, etc.)
fn extract_long_branch_target(line: &str) -> Option<String> {
    let trimmed = line.trim();
    
    let long_branch_ops = ["LBRA ", "LBEQ ", "LBNE ", "LBCC ", "LBCS ", 
                          "LBPL ", "LBMI ", "LBVC ", "LBVS "];
    
    for op in &long_branch_ops {
        if trimmed.starts_with(op) {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let target = parts[1].split(';').next().unwrap().trim();
                return Some(target.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_section_markers() {
        let asm = r#"
; Header
.section .text.header, "ax", @progbits
    FCC "g GCE 1982"
    FCB $80

.section .text.main, "ax", @progbits
MAIN:
    JSR Wait_Recal
    RTS

.section .rodata, "a", @progbits
STR_0:
    FCC "HELLO"
    FCB $80
"#;
        
        let sections = parse_section_markers(asm).unwrap();
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].name, ".text.header");
        assert_eq!(sections[1].name, ".text.main");
        assert_eq!(sections[2].name, ".rodata");
    }
    
    #[test]
    fn test_parse_flags() {
        assert_eq!(parse_flags("ax"), 0x01 | 0x04); // allocatable + executable
        assert_eq!(parse_flags("a"), 0x01); // allocatable only
        assert_eq!(parse_flags("aw"), 0x01 | 0x02); // allocatable + writable
    }
    
    #[test]
    fn test_build_symbol_table() {
        let asm = r#"
.section .text.main, "ax", @progbits
MAIN:
    JSR Wait_Recal
    RTS

LOOP_BODY:
    LDD #100
    RTS
"#;
        
        let sections = extract_sections(asm).unwrap();
        let symbols = build_symbol_table(&sections, asm).unwrap();
        
        assert_eq!(symbols.exports.len(), 2);
        assert!(symbols.exports.iter().any(|s| s.name == "MAIN"));
        assert!(symbols.exports.iter().any(|s| s.name == "LOOP_BODY"));
    }
    
    #[test]
    fn test_extract_sections_with_binary() {
        let asm = r#"
.section .text.main, "ax", @progbits
START:
    LDA #$80
    STA $C880
    RTS
"#;
        
        let sections = extract_sections_with_binary(asm, 0x0000).unwrap();
        
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].name, ".text.main");
        assert_eq!(sections[0].section_type, SectionType::Text);
        
        // Binary should contain assembled instructions (not empty)
        assert!(sections[0].data.len() > 0, "Section data should be assembled");
        
        // LDA #$80 = 86 80 (2 bytes)
        // STA $C880 = B7 C8 80 (3 bytes)  
        // RTS = 39 (1 byte)
        // Total: 6 bytes expected
        assert_eq!(sections[0].data.len(), 6, "Expected 6 bytes of assembled code");
    }
    
    #[test]
    fn test_collect_relocations() {
        let asm = r#"
.section .text.main, "ax", @progbits
MAIN:
    JSR Wait_Recal
    LDX #STR_0
    BRA LOOP_BODY
    RTS

INTERNAL_FUNC:
    RTS
"#;
        
        let sections = extract_sections(asm).unwrap();
        let symbols = build_symbol_table(&sections, asm).unwrap();
        let relocations = collect_relocations(&sections, &symbols, asm).unwrap();
        
        // Should find 3 relocations (Wait_Recal, STR_0, LOOP_BODY)
        // INTERNAL_FUNC is exported so no relocation needed
        assert_eq!(relocations.len(), 3, "Expected 3 relocations for external symbols");
        
        // Check JSR Wait_Recal
        assert!(relocations.iter().any(|r| r.symbol == "Wait_Recal" && 
                matches!(r.reloc_type, RelocationType::Absolute16)));
        
        // Check LDX #STR_0
        assert!(relocations.iter().any(|r| r.symbol == "STR_0" && 
                matches!(r.reloc_type, RelocationType::Absolute16)));
        
        // Check BRA LOOP_BODY
        assert!(relocations.iter().any(|r| r.symbol == "LOOP_BODY" && 
                matches!(r.reloc_type, RelocationType::Relative8)));
    }
}
