// ASM Parser for .vo object generation
// Extracts sections, symbols, and relocations from ASM with section markers

use super::object::{Section, SectionType, Symbol, SymbolScope, SymbolTable, SymbolType, Relocation, RelocationType};
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
        // For now, we'll keep the ASM as-is (binary assembly comes later)
        // Phase 3.1: Just extract sections
        // Phase 3.2: Add binary assembly
        let section = Section {
            name: ps.name.clone(),
            section_type: ps.section_type,
            bank_hint: None, // Linker decides
            alignment: 1, // Default alignment
            data: Vec::new(), // TODO: Assemble ASM to binary
        };
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

/// Collect relocations from sections
/// 
/// Scans for references to external symbols that need patching during linking
pub fn collect_relocations(
    sections: &[Section],
    symbols: &SymbolTable,
    asm: &str
) -> Result<Vec<Relocation>, String> {
    let mut relocations = Vec::new();
    
    // TODO: Parse ASM for JSR, BRA, LDD # instructions
    // and create Relocation entries
    
    Ok(relocations)
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
}
