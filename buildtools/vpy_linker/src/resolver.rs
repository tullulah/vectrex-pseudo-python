// Symbol Resolver
//
// Builds global symbol table and resolves all references.
// 4-step algorithm: collect → verify → assign → apply
//
// Ported from core/src/linker/resolver.rs

use crate::object::{VectrexObject, RelocationType};
use crate::error::{LinkerError, LinkerResult};
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
    /// Step 1: Collect all exports from all object files
    ///
    /// Builds global symbol table with all exported symbols.
    /// Detects duplicate definitions (error if same symbol in multiple objects).
    pub fn collect_symbols(objects: &[VectrexObject]) -> LinkerResult<GlobalSymbolTable> {
        let mut global = GlobalSymbolTable { 
            symbols: HashMap::new() 
        };
        
        for (obj_idx, obj) in objects.iter().enumerate() {
            for symbol in &obj.symbols.exports {
                // Check for duplicate definitions
                if global.symbols.contains_key(&symbol.name) {
                    let existing = &global.symbols[&symbol.name];
                    return Err(LinkerError::DuplicateSymbol {
                        symbol: symbol.name.clone(),
                        first: existing.source_file.clone(),
                        second: obj.header.source_file.clone(),
                    });
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
    
    /// Step 2: Verify all imports have matching exports
    ///
    /// Checks that every imported symbol has a corresponding export.
    /// Returns error with list of all undefined symbols.
    pub fn verify_imports(
        objects: &[VectrexObject], 
        global: &GlobalSymbolTable
    ) -> LinkerResult<()> {
        let mut undefined = Vec::new();
        
        for obj in objects {
            for import in obj.symbols.imports.iter() {
                if !global.symbols.contains_key(&import.name) {
                    undefined.push((
                        import.name.clone(),
                        obj.header.source_file.clone(),
                    ));
                }
            }
        }
        
        if !undefined.is_empty() {
            return Err(LinkerError::UndefinedSymbols { symbols: undefined });
        }
        
        Ok(())
    }
    
    /// Step 3: Assign addresses to all sections and update symbol table
    ///
    /// Walks through all sections sequentially, assigns base addresses.
    /// Updates symbol table with final addresses (section_base + symbol.offset).
    /// Returns map of (object_index, section_index) -> base_address for relocation phase.
    pub fn assign_addresses(
        objects: &[VectrexObject],
        global: &mut GlobalSymbolTable,
        base_address: u16,
    ) -> LinkerResult<HashMap<(usize, usize), u16>> {
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
                            let symbol_address = current_address.wrapping_add(symbol.offset);
                            
                            // Update global table
                            if let Some(global_sym) = global.symbols.get_mut(&symbol.name) {
                                global_sym.address = symbol_address;
                            }
                        }
                    }
                }
                
                // Advance address for next section
                current_address = current_address.wrapping_add(section.size() as u16);
                
                // Apply alignment if needed
                let alignment = section.alignment as u16;
                if alignment > 1 {
                    current_address = ((current_address + alignment - 1) / alignment) * alignment;
                }
            }
        }
        
        Ok(section_bases)
    }
    
    /// Step 4: Apply relocations using resolved symbols
    ///
    /// Patches code/data sections with actual addresses from symbol table.
    /// Modifies objects in-place (mutates section.data).
    /// Supports 7 relocation types: Absolute16, Relative8/16, Direct, High8/Low8, CrossBank.
    pub fn apply_relocations(
        objects: &mut [VectrexObject],
        global: &GlobalSymbolTable,
        section_bases: &HashMap<(usize, usize), u16>,
    ) -> LinkerResult<()> {
        for (obj_idx, obj) in objects.iter_mut().enumerate() {
            // Clone relocations to avoid borrow conflict
            let relocations = obj.relocations.clone();
            
            for reloc in relocations {
                // Lookup symbol address
                let symbol = global.symbols.get(&reloc.symbol)
                    .ok_or_else(|| LinkerError::SymbolNotFound { 
                        symbol: reloc.symbol.clone() 
                    })?;
                
                // Get target section and its base address
                let section_idx = reloc.section;
                let section = obj.sections.get_mut(section_idx)
                    .ok_or_else(|| LinkerError::InvalidSection { 
                        index: section_idx 
                    })?;
                
                let section_base = section_bases.get(&(obj_idx, section_idx))
                    .ok_or_else(|| LinkerError::InvalidSection { 
                        index: section_idx 
                    })?;
                
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
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
                        }
                    }
                    RelocationType::Relative8 => {
                        // Calculate PC-relative offset (signed 8-bit)
                        let pc_address = section_base + reloc.offset + 2;  // PC after instruction
                        let relative_offset = (target_address as i32 - pc_address as i32) as i8;
                        
                        if offset < section.data.len() {
                            section.data[offset] = relative_offset as u8;
                        } else {
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
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
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
                        }
                    }
                    RelocationType::Direct => {
                        // Direct page addressing (low 8 bits only)
                        if offset < section.data.len() {
                            section.data[offset] = (target_address & 0xFF) as u8;
                        } else {
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
                        }
                    }
                    RelocationType::High8 => {
                        // High byte of address
                        if offset < section.data.len() {
                            section.data[offset] = (target_address >> 8) as u8;
                        } else {
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
                        }
                    }
                    RelocationType::Low8 => {
                        // Low byte of address
                        if offset < section.data.len() {
                            section.data[offset] = (target_address & 0xFF) as u8;
                        } else {
                            return Err(LinkerError::RelocationOutOfBounds { 
                                offset, 
                                size: section.data.len() 
                            });
                        }
                    }
                    RelocationType::CrossBank => {
                        return Err(LinkerError::CrossBankNotImplemented { 
                            symbol: reloc.symbol.clone() 
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{
        VectrexObject, Section, SectionType, Symbol, SymbolScope, SymbolType,
    };

    fn create_test_object(name: &str, export_name: &str) -> VectrexObject {
        let mut obj = VectrexObject::new(name.to_string());
        
        // Add text section with some code
        obj.sections.push(Section {
            name: ".text".to_string(),
            section_type: SectionType::Text,
            bank_hint: None,
            alignment: 1,
            data: vec![0x12, 0x34, 0x56, 0x78],  // Mock code
        });
        
        // Add exported symbol
        obj.symbols.exports.push(Symbol {
            name: export_name.to_string(),
            section: Some(0),  // In first section
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        obj
    }

    #[test]
    fn test_collect_symbols() {
        let obj1 = create_test_object("file1.vpy", "func_a");
        let obj2 = create_test_object("file2.vpy", "func_b");
        
        let global = SymbolResolver::collect_symbols(&[obj1, obj2]).unwrap();
        
        assert_eq!(global.symbols.len(), 2);
        assert!(global.symbols.contains_key("func_a"));
        assert!(global.symbols.contains_key("func_b"));
    }

    #[test]
    fn test_duplicate_symbol_error() {
        let obj1 = create_test_object("file1.vpy", "duplicate");
        let obj2 = create_test_object("file2.vpy", "duplicate");
        
        let result = SymbolResolver::collect_symbols(&[obj1, obj2]);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            LinkerError::DuplicateSymbol { symbol, .. } => {
                assert_eq!(symbol, "duplicate");
            }
            _ => panic!("Expected DuplicateSymbol error"),
        }
    }

    #[test]
    fn test_verify_imports_success() {
        let mut obj1 = create_test_object("file1.vpy", "func_a");
        let obj2 = create_test_object("file2.vpy", "func_b");
        
        // obj1 imports func_b (which obj2 exports)
        obj1.symbols.imports.push(Symbol {
            name: "func_b".to_string(),
            section: None,
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        let global = SymbolResolver::collect_symbols(&[obj1.clone(), obj2.clone()]).unwrap();
        let result = SymbolResolver::verify_imports(&[obj1, obj2], &global);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_imports_undefined() {
        let mut obj1 = create_test_object("file1.vpy", "func_a");
        
        // Import non-existent symbol
        obj1.symbols.imports.push(Symbol {
            name: "nonexistent".to_string(),
            section: None,
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        let global = SymbolResolver::collect_symbols(&[obj1.clone()]).unwrap();
        let result = SymbolResolver::verify_imports(&[obj1], &global);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            LinkerError::UndefinedSymbols { symbols } => {
                assert_eq!(symbols.len(), 1);
                assert_eq!(symbols[0].0, "nonexistent");
            }
            _ => panic!("Expected UndefinedSymbols error"),
        }
    }

    #[test]
    fn test_assign_addresses() {
        let obj1 = create_test_object("file1.vpy", "func_a");
        let obj2 = create_test_object("file2.vpy", "func_b");
        
        let mut global = SymbolResolver::collect_symbols(&[obj1.clone(), obj2.clone()]).unwrap();
        let section_bases = SymbolResolver::assign_addresses(
            &[obj1, obj2], 
            &mut global, 
            0x4000  // Base address
        ).unwrap();
        
        // Check section bases were assigned
        assert_eq!(section_bases.len(), 2);
        assert_eq!(section_bases[&(0, 0)], 0x4000);  // First object, first section
        assert_eq!(section_bases[&(1, 0)], 0x4004);  // Second object (after 4 bytes)
        
        // Check symbols got addresses
        let func_a = global.symbols.get("func_a").unwrap();
        assert_eq!(func_a.address, 0x4000);
        
        let func_b = global.symbols.get("func_b").unwrap();
        assert_eq!(func_b.address, 0x4004);
    }
}
