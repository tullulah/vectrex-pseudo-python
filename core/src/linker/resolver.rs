// Symbol Resolver
//
// Builds global symbol table and resolves all references.

use crate::linker::object::{Symbol, SymbolScope, VectrexObject};
use std::collections::HashMap;

pub struct SymbolResolver {
    global_symbols: HashMap<String, ResolvedSymbol>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub final_address: u16,
    pub bank: u8,
    pub object_index: usize,
    pub section_index: usize,
}

impl SymbolResolver {
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
                    ResolvedSymbol {
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
            for symbol in &obj.symbols.imports {
                if !self.global_symbols.contains_key(&symbol.name) {
                    return Err(format!(
                        "Undefined symbol: {} (required by {})",
                        symbol.name, obj.header.source_file
                    ));
                }
            }
        }
        
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ResolvedSymbol> {
        self.global_symbols.get(name)
    }

    pub fn update_address(&mut self, name: &str, address: u16, bank: u8) {
        if let Some(symbol) = self.global_symbols.get_mut(name) {
            symbol.final_address = address;
            symbol.bank = bank;
        }
    }
}
