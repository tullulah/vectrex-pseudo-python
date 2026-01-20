// Symbol Resolver
//
// Builds global symbol table and resolves all references.

use crate::linker::object::{VectrexObject, Symbol};
use std::collections::HashMap;

/// Global symbol table after resolution
#[derive(Debug, Clone)]
pub struct GlobalSymbolTable {
    pub symbols: HashMap<String, ResolvedSymbol>,
}

/// A resolved symbol with final address assignment
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub address: u16,              // Final address after linking
    pub section: String,           // Which section it belongs to
    pub source_file: String,       // Which .vo file it came from
    pub object_index: usize,       // Index in the objects array
}

pub struct SymbolResolver;

impl SymbolResolver {
    /// Collect all exports from all object files
    pub fn collect_symbols(objects: &[VectrexObject]) -> Result<GlobalSymbolTable, String> {
        let mut global = GlobalSymbolTable { 
            symbols: HashMap::new() 
        };
        
        for (obj_idx, obj) in objects.iter().enumerate() {
            for symbol in &obj.symbols.exports {
                // Check for duplicate definitions
                if global.symbols.contains_key(&symbol.name) {
                    let existing = &global.symbols[&symbol.name];
                    return Err(format!(
                        "Duplicate symbol '{}' defined in:\n  - {} (first definition)\n  - {} (duplicate)",
                        symbol.name, 
                        existing.source_file,
                        obj.header.source_file
                    ));
                }
                
                // Add to global table (address not yet assigned)
                global.symbols.insert(symbol.name.clone(), ResolvedSymbol {
                    name: symbol.name.clone(),
                    address: 0,  // Will be assigned in assign_addresses()
                    section: format!("section_{}", symbol.section.unwrap_or(0)),
                    source_file: obj.header.source_file.clone(),
                    object_index: obj_idx,
                });
            }
        }
        
        Ok(global)
    }
    
    /// Verify all imports have matching exports
    pub fn verify_imports(
        objects: &[VectrexObject], 
        global: &GlobalSymbolTable
    ) -> Result<(), String> {
        let mut errors = Vec::new();
        
        for obj in objects {
            for import in obj.symbols.imports.iter() {
                if !global.symbols.contains_key(&import.name) {
                    errors.push(format!(
                        "  - Undefined reference to '{}' in {}",
                        import.name,
                        obj.header.source_file
                    ));
                }
            }
        }
        
        if !errors.is_empty() {
            return Err(format!(
                "Undefined symbols:\n{}",
                errors.join("\n")
            ));
        }
        
        Ok(())
    }
    
    /// Assign addresses to all sections and update symbol table
    /// Returns map of (object_index, section_index) -> base_address
    pub fn assign_addresses(
        objects: &[VectrexObject],
        global: &mut GlobalSymbolTable,
        base_address: u16,
    ) -> Result<HashMap<(usize, usize), u16>, String> {
        let mut section_bases: HashMap<(usize, usize), u16> = HashMap::new();
        let mut current_address = base_address;
        
        for (obj_idx, obj) in objects.iter().enumerate() {
            for (section_idx, section) in obj.sections.iter().enumerate() {
                // Assign section base address
                section_bases.insert((obj_idx, section_idx), current_address);
                
                // Update symbols in this section
                for symbol in &obj.symbols.exports {
                    // Check if this symbol belongs to this section
                    if let Some(sym_section_idx) = symbol.section {
                        if sym_section_idx == section_idx {
                            let symbol_address = current_address + symbol.offset;
                            
                            // Update global table
                            if let Some(global_sym) = global.symbols.get_mut(&symbol.name) {
                                global_sym.address = symbol_address;
                            }
                        }
                    }
                }
                
                // Advance address for next section
                current_address = current_address.wrapping_add(section.data.len() as u16);
                
                // Apply alignment if needed
                let alignment = section.alignment as u16;
                if alignment > 1 {
                    current_address = ((current_address + alignment - 1) / alignment) * alignment;
                }
            }
        }
        
        Ok(section_bases)
    }
    
    /// Apply relocations using resolved symbols
    pub fn apply_relocations(
        objects: &mut [VectrexObject],
        global: &GlobalSymbolTable,
        section_bases: &HashMap<(usize, usize), u16>,
    ) -> Result<(), String> {
        use crate::linker::object::RelocationType;
        
        for (obj_idx, obj) in objects.iter_mut().enumerate() {
            for reloc in obj.relocations.clone() {  // Clone to avoid borrow conflict
                // Lookup symbol address
                let symbol = global.symbols.get(&reloc.symbol)
                    .ok_or_else(|| format!("Symbol '{}' not found during relocation", reloc.symbol))?;
                
                // Get target section and its base address
                let section_idx = reloc.section;
                let section = obj.sections.get_mut(section_idx)
                    .ok_or_else(|| format!("Section index {} out of bounds", section_idx))?;
                
                let section_base = section_bases.get(&(obj_idx, section_idx))
                    .ok_or_else(|| format!("No base address for section {} in object {}", section_idx, obj_idx))?;
                
                // Calculate target address with addend
                let target_address = (symbol.address as i32 + reloc.addend) as u16;
                let offset = reloc.offset as usize;
                
                // Apply relocation based on type
                match reloc.reloc_type {
                    RelocationType::Absolute16 => {
                        // Patch 2 bytes with absolute address (big-endian M6809)
                        if offset + 1 < section.data.len() {
                            section.data[offset] = (target_address >> 8) as u8;
                            section.data[offset + 1] = (target_address & 0xFF) as u8;
                        } else {
                            return Err(format!("Relocation offset {} + 2 exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::Relative8 => {
                        // Calculate PC-relative offset (signed 8-bit)
                        let pc_address = section_base + reloc.offset + 2;  // PC after instruction
                        let relative_offset = (target_address as i32 - pc_address as i32) as i8;
                        
                        if offset < section.data.len() {
                            section.data[offset] = relative_offset as u8;
                        } else {
                            return Err(format!("Relocation offset {} exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::Relative16 => {
                        // Calculate PC-relative offset (signed 16-bit)
                        let pc_address = section_base + reloc.offset + 3;  // PC after LBRA
                        let relative_offset = (target_address as i32 - pc_address as i32) as i16;
                        
                        if offset + 1 < section.data.len() {
                            section.data[offset] = (relative_offset >> 8) as u8;
                            section.data[offset + 1] = (relative_offset & 0xFF) as u8;
                        } else {
                            return Err(format!("Relocation offset {} + 2 exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::Direct => {
                        // Direct page addressing (low 8 bits only)
                        if offset < section.data.len() {
                            section.data[offset] = (target_address & 0xFF) as u8;
                        } else {
                            return Err(format!("Relocation offset {} exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::High8 => {
                        // High byte of address
                        if offset < section.data.len() {
                            section.data[offset] = (target_address >> 8) as u8;
                        } else {
                            return Err(format!("Relocation offset {} exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::Low8 => {
                        // Low byte of address
                        if offset < section.data.len() {
                            section.data[offset] = (target_address & 0xFF) as u8;
                        } else {
                            return Err(format!("Relocation offset {} exceeds section size {}", 
                                offset, section.data.len()));
                        }
                    }
                    RelocationType::CrossBank => {
                        return Err(format!("CrossBank relocation not yet implemented for symbol '{}'", reloc.symbol));
                    }
                }
            }
        }
        
        Ok(())
    }
}

// Legacy compatibility wrapper (deprecated - use SymbolResolver methods directly)
#[deprecated(note = "Use SymbolResolver::collect_symbols() instead")]
pub struct LegacySymbolResolver {
    global_symbols: HashMap<String, LegacyResolvedSymbol>,
}

#[derive(Debug, Clone)]
struct LegacyResolvedSymbol {
    pub name: String,
    pub final_address: u16,
    pub bank: u8,
    pub object_index: usize,
    pub section_index: usize,
}

#[allow(deprecated)]
impl LegacySymbolResolver {
    pub fn new() -> Self {
        Self {
            global_symbols: HashMap::new(),
        }
    }

    pub fn build_global_table(&mut self, objects: &[VectrexObject]) -> Result<(), String> {
        // Phase 1: Collect all exported symbols
        for (obj_idx, obj) in objects.iter().enumerate() {
            for symbol in &obj.symbols.exports {
                if self.global_symbols.contains_key(&symbol.name) {
                    return Err(format!("Duplicate symbol: {}", symbol.name));
                }
                
                // Placeholder - will be filled during address calculation
                self.global_symbols.insert(
                    symbol.name.clone(),
                    LegacyResolvedSymbol {
                        name: symbol.name.clone(),
                        final_address: 0,
                        bank: 0,
                        object_index: obj_idx,
                        section_index: symbol.section.unwrap_or(0),
                    },
                );
            }
        }
        
        // Phase 2: Check all imports can be resolved
        for obj in objects {
            for import in obj.symbols.imports.iter() {
                if !self.global_symbols.contains_key(&import.name) {
                    return Err(format!(
                        "Undefined symbol: {} (required by {})",
                        import.name, obj.header.source_file
                    ));
                }
            }
        }
        
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&LegacyResolvedSymbol> {
        self.global_symbols.get(name)
    }

    pub fn update_address(&mut self, name: &str, address: u16, bank: u8) {
        if let Some(symbol) = self.global_symbols.get_mut(name) {
            symbol.final_address = address;
            symbol.bank = bank;
        }
    }
}

// ========== TESTS ==========

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linker::object::{ObjectHeader, TargetArch, ObjectFlags, Section, SectionType, SymbolTable, DebugInfo, Relocation, SymbolScope, SymbolType};
    
    fn create_test_object(source_file: &str, exports: Vec<(&str, usize, u16)>) -> VectrexObject {
        let mut symbol_table = SymbolTable::default();
        
        for (name, section_idx, offset) in exports {
            symbol_table.exports.push(Symbol {
                name: name.to_string(),
                section: Some(section_idx),
                offset,
                scope: SymbolScope::Global,
                symbol_type: SymbolType::Function,
            });
        }
        
        VectrexObject {
            header: ObjectHeader {
                magic: *b"VObj",
                version: 1,
                target: TargetArch::M6809,
                flags: ObjectFlags {
                    position_independent: false,
                    contains_bank_hints: false,
                },
                source_file: source_file.to_string(),
            },
            sections: vec![
                Section {
                    name: ".text".to_string(),
                    section_type: SectionType::Text,
                    bank_hint: None,
                    alignment: 1,
                    data: vec![0; 100], // 100 bytes
                }
            ],
            symbols: symbol_table,
            relocations: vec![],
            debug_info: DebugInfo::default(),
        }
    }
    
    fn create_test_object_with_import(source_file: &str, import_name: &str) -> VectrexObject {
        let mut obj = create_test_object(source_file, vec![]);
        obj.symbols.imports.push(Symbol {
            name: import_name.to_string(),
            section: None,
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        obj
    }
    
    #[test]
    fn test_collect_symbols() {
        let obj1 = create_test_object("main.vo", vec![("MAIN", 0, 0)]);
        let obj2 = create_test_object("lib.vo", vec![("HELPER", 0, 0)]);
        
        let global = SymbolResolver::collect_symbols(&[obj1, obj2]).unwrap();
        
        assert_eq!(global.symbols.len(), 2);
        assert!(global.symbols.contains_key("MAIN"));
        assert!(global.symbols.contains_key("HELPER"));
        assert_eq!(global.symbols["MAIN"].source_file, "main.vo");
        assert_eq!(global.symbols["HELPER"].source_file, "lib.vo");
    }
    
    #[test]
    fn test_duplicate_symbol_detection() {
        let obj1 = create_test_object("main.vo", vec![("MAIN", 0, 0)]);
        let obj2 = create_test_object("lib.vo", vec![("MAIN", 0, 0)]);  // Duplicate!
        
        let result = SymbolResolver::collect_symbols(&[obj1, obj2]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Duplicate symbol 'MAIN'"));
        assert!(err.contains("main.vo"));
        assert!(err.contains("lib.vo"));
    }
    
    #[test]
    fn test_undefined_import_detection() {
        let obj = create_test_object_with_import("main.vo", "UNDEFINED_FUNC");
        let global = GlobalSymbolTable { symbols: HashMap::new() };
        
        let result = SymbolResolver::verify_imports(&[obj], &global);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Undefined reference to 'UNDEFINED_FUNC'"));
        assert!(err.contains("main.vo"));
    }
    
    #[test]
    fn test_valid_import_resolution() {
        let obj1 = create_test_object("main.vo", vec![("MAIN", 0, 0)]);
        let obj2 = create_test_object_with_import("lib.vo", "MAIN");
        
        let global = SymbolResolver::collect_symbols(&[obj1.clone(), obj2.clone()]).unwrap();
        let result = SymbolResolver::verify_imports(&[obj1, obj2], &global);
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_assign_addresses() {
        let obj1 = create_test_object("main.vo", vec![("MAIN", 0, 10)]);
        let obj2 = create_test_object("lib.vo", vec![("HELPER", 0, 20)]);
        
        let mut global = SymbolResolver::collect_symbols(&[obj1.clone(), obj2.clone()]).unwrap();
        let section_bases = SymbolResolver::assign_addresses(
            &[obj1, obj2],
            &mut global,
            0xC880
        ).unwrap();
        
        // Verify section bases
        assert_eq!(section_bases[&(0, 0)], 0xC880);  // obj1, section 0
        assert_eq!(section_bases[&(1, 0)], 0xC8E4);  // obj2, section 0 (0xC880 + 100)
        
        // Verify symbol addresses
        assert_eq!(global.symbols["MAIN"].address, 0xC880 + 10);    // base + offset
        assert_eq!(global.symbols["HELPER"].address, 0xC8E4 + 20);  // base + offset
    }
    
    #[test]
    fn test_apply_relocations() {
        use crate::linker::object::RelocationType;
        
        // Create object with MAIN symbol and a call to HELPER
        let mut obj1 = create_test_object("main.vo", vec![("MAIN", 0, 0)]);
        // Add placeholder JSR instruction: $BD $00 $00 (JSR abs)
        obj1.sections[0].data[10] = 0xBD;  // JSR opcode
        obj1.sections[0].data[11] = 0x00;  // Placeholder high byte
        obj1.sections[0].data[12] = 0x00;  // Placeholder low byte
        
        // Add relocation for JSR target
        obj1.relocations.push(Relocation {
            section: 0,
            offset: 11,  // Offset to address bytes
            reloc_type: RelocationType::Absolute16,
            symbol: "HELPER".to_string(),
            addend: 0,
        });
        
        // Create object with HELPER symbol
        let obj2 = create_test_object("lib.vo", vec![("HELPER", 0, 0)]);
        
        let mut objects = vec![obj1, obj2];
        
        // Resolve symbols and assign addresses
        let mut global = SymbolResolver::collect_symbols(&objects).unwrap();
        let section_bases = SymbolResolver::assign_addresses(
            &objects,
            &mut global,
            0xC880
        ).unwrap();
        
        // Apply relocations
        SymbolResolver::apply_relocations(&mut objects, &global, &section_bases).unwrap();
        
        // Verify JSR target was patched correctly
        // HELPER is at 0xC8E4 (obj2 section 0 base)
        let patched_high = objects[0].sections[0].data[11];
        let patched_low = objects[0].sections[0].data[12];
        let patched_address = ((patched_high as u16) << 8) | (patched_low as u16);
        
        assert_eq!(patched_address, 0xC8E4, "JSR target should be patched to HELPER address");
    }
}