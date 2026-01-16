// Bank Layout Integration
//
// Connects symbol resolver with bank allocator to produce multibank ROM.
// Handles assignment of sections to banks and cross-bank relocation.

use crate::object::VectrexObject;
use crate::resolver::{GlobalSymbolTable, SymbolResolver, ResolvedSymbol};
use crate::error::{LinkerError, LinkerResult};
use std::collections::HashMap;

/// Bank configuration for Vectrex cartridge
#[derive(Debug, Clone)]
pub struct BankConfig {
    pub num_banks: usize,
    pub bank_size: usize,        // Bytes per bank
    pub fixed_bank_id: u8,       // Bank 31 (always visible at $4000-$7FFF)
    pub switchable_base: u16,    // $0000 (switchable window)
    pub fixed_base: u16,         // $4000 (fixed bank)
}

impl BankConfig {
    /// Standard Vectrex multibank (512KB, 32 banks × 16KB)
    pub fn vectrex_512kb() -> Self {
        BankConfig {
            num_banks: 32,
            bank_size: 16384,  // 16KB per bank
            fixed_bank_id: 31,
            switchable_base: 0x0000,
            fixed_base: 0x4000,
        }
    }

    /// Single bank (32KB)
    pub fn single_bank() -> Self {
        BankConfig {
            num_banks: 1,
            bank_size: 32768,  // 32KB single bank
            fixed_bank_id: 0,
            switchable_base: 0x0000,
            fixed_base: 0x0000,
        }
    }
}

/// Section assignment to specific bank
#[derive(Debug, Clone)]
pub struct SectionAssignment {
    pub object_index: usize,
    pub section_index: usize,
    pub bank_id: u8,
    pub offset: u16,  // Offset within bank
}

/// Multibank ROM layout after linking
#[derive(Debug, Clone)]
pub struct MultibankLayout {
    pub banks: Vec<BankData>,
    pub symbol_table: GlobalSymbolTable,
    pub config: BankConfig,
}

/// Data for a single bank
#[derive(Debug, Clone)]
pub struct BankData {
    pub bank_id: u8,
    pub data: Vec<u8>,
    pub symbols: Vec<ResolvedSymbol>,  // Symbols in this bank
}

impl MultibankLayout {
    /// Create layout by assigning sections to banks
    pub fn from_objects(
        mut objects: Vec<VectrexObject>,
        config: BankConfig,
    ) -> LinkerResult<Self> {
        // Step 1: Collect symbols
        let mut global = SymbolResolver::collect_symbols(&objects)?;
        
        // Step 2: Verify imports
        SymbolResolver::verify_imports(&objects, &global)?;
        
        // Step 3: Assign sections to banks (simplified for now - sequential)
        let assignments = Self::assign_sections_to_banks(&objects, &config)?;
        
        // Step 4: Assign addresses within each bank
        let section_bases = Self::assign_addresses_multibank(
            &objects,
            &mut global,
            &assignments,
            &config,
        )?;
        
        // Step 5: Apply relocations
        SymbolResolver::apply_relocations(&mut objects, &global, &section_bases)?;
        
        // Step 6: Build bank data
        let banks = Self::build_banks(objects, assignments, &config)?;
        
        Ok(MultibankLayout {
            banks,
            symbol_table: global,
            config,
        })
    }

    /// Assign sections to banks (simplified: sequential allocation)
    fn assign_sections_to_banks(
        objects: &[VectrexObject],
        config: &BankConfig,
    ) -> LinkerResult<Vec<SectionAssignment>> {
        let mut assignments = Vec::new();
        let mut current_bank = 0u8;
        let mut current_offset = 0u16;
        
        for (obj_idx, obj) in objects.iter().enumerate() {
            for (section_idx, section) in obj.sections.iter().enumerate() {
                let section_size = section.size() as u16;
                
                // Check if section fits in current bank
                if current_offset + section_size > config.bank_size as u16 {
                    // Move to next bank
                    current_bank += 1;
                    current_offset = 0;
                    
                    if current_bank as usize >= config.num_banks {
                        return Err(LinkerError::BinaryTooLarge {
                            size: 0,  // TODO: calculate actual size
                            limit: config.num_banks * config.bank_size,
                        });
                    }
                }
                
                assignments.push(SectionAssignment {
                    object_index: obj_idx,
                    section_index: section_idx,
                    bank_id: current_bank,
                    offset: current_offset,
                });
                
                current_offset += section_size;
                
                // Apply alignment
                let alignment = section.alignment as u16;
                if alignment > 1 {
                    current_offset = ((current_offset + alignment - 1) / alignment) * alignment;
                }
            }
        }
        
        Ok(assignments)
    }

    /// Assign addresses considering bank layout
    fn assign_addresses_multibank(
        objects: &[VectrexObject],
        global: &mut GlobalSymbolTable,
        assignments: &[SectionAssignment],
        config: &BankConfig,
    ) -> LinkerResult<HashMap<(usize, usize), u16>> {
        let mut section_bases: HashMap<(usize, usize), u16> = HashMap::new();
        
        for assignment in assignments {
            let obj = &objects[assignment.object_index];
            let _section = &obj.sections[assignment.section_index];
            
            // Calculate address: bank base + offset
            let base_address = if assignment.bank_id == config.fixed_bank_id {
                config.fixed_base + assignment.offset
            } else {
                config.switchable_base + assignment.offset
            };
            
            section_bases.insert(
                (assignment.object_index, assignment.section_index),
                base_address,
            );
            
            // Update symbols in this section
            for symbol in &obj.symbols.exports {
                if let Some(sym_section_idx) = symbol.section {
                    if sym_section_idx == assignment.section_index {
                        let symbol_address = base_address.wrapping_add(symbol.offset);
                        
                        if let Some(global_sym) = global.symbols.get_mut(&symbol.name) {
                            global_sym.address = symbol_address;
                        }
                    }
                }
            }
        }
        
        Ok(section_bases)
    }

    /// Build final bank data from linked objects
    fn build_banks(
        objects: Vec<VectrexObject>,
        assignments: Vec<SectionAssignment>,
        config: &BankConfig,
    ) -> LinkerResult<Vec<BankData>> {
        let mut banks: Vec<BankData> = (0..config.num_banks)
            .map(|i| BankData {
                bank_id: i as u8,
                data: vec![0xFF; config.bank_size],  // Initialize with 0xFF (unprogrammed)
                symbols: Vec::new(),
            })
            .collect();
        
        // Copy section data to banks
        for assignment in assignments {
            let obj = &objects[assignment.object_index];
            let section = &obj.sections[assignment.section_index];
            
            let bank = &mut banks[assignment.bank_id as usize];
            let start = assignment.offset as usize;
            let end = start + section.data.len();
            
            if end > config.bank_size {
                return Err(LinkerError::BinaryTooLarge {
                    size: end,
                    limit: config.bank_size,
                });
            }
            
            bank.data[start..end].copy_from_slice(&section.data);
            
            // Add symbols from this section
            for symbol in &obj.symbols.exports {
                if symbol.section == Some(assignment.section_index) {
                    bank.symbols.push(ResolvedSymbol {
                        name: symbol.name.clone(),
                        address: assignment.offset + symbol.offset,
                        section: section.name.clone(),
                        source_file: obj.header.source_file.clone(),
                        object_index: assignment.object_index,
                    });
                }
            }
        }
        
        Ok(banks)
    }

    /// Write multibank ROM to files
    pub fn write_banks(&self, base_path: &std::path::Path) -> LinkerResult<()> {
        use std::io::Write;
        
        for bank in &self.banks {
            let filename = format!("bank_{:02}.bin", bank.bank_id);
            let path = base_path.join(filename);
            
            let mut file = std::fs::File::create(&path)
                .map_err(|e| LinkerError::Error(format!("Failed to create bank file: {}", e)))?;
            
            file.write_all(&bank.data)
                .map_err(|e| LinkerError::Error(format!("Failed to write bank data: {}", e)))?;
        }
        
        Ok(())
    }

    /// Write single merged binary (for single-bank or testing)
    pub fn write_merged(&self, path: &std::path::Path) -> LinkerResult<()> {
        use std::io::Write;
        
        let mut merged = Vec::new();
        for bank in &self.banks {
            merged.extend_from_slice(&bank.data);
        }
        
        let mut file = std::fs::File::create(path)
            .map_err(|e| LinkerError::Error(format!("Failed to create file: {}", e)))?;
        
        file.write_all(&merged)
            .map_err(|e| LinkerError::Error(format!("Failed to write data: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{Symbol, SymbolScope, SymbolType, Section, SectionType};

    fn create_test_object(name: &str, code_size: usize) -> VectrexObject {
        let mut obj = VectrexObject::new(name.to_string());
        
        obj.sections.push(Section {
            name: ".text".to_string(),
            section_type: SectionType::Text,
            bank_hint: None,
            alignment: 1,
            data: vec![0x12; code_size],  // Mock code
        });
        
        obj.symbols.exports.push(Symbol {
            name: format!("{}_func", name),
            section: Some(0),
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        obj
    }

    #[test]
    fn test_single_bank_layout() {
        let obj = create_test_object("test", 100);
        let config = BankConfig::single_bank();
        
        let layout = MultibankLayout::from_objects(vec![obj], config).unwrap();
        
        assert_eq!(layout.banks.len(), 1);
        assert_eq!(layout.banks[0].bank_id, 0);
        assert_eq!(layout.banks[0].symbols.len(), 1);
    }

    #[test]
    fn test_multibank_assignment() {
        let obj1 = create_test_object("module1", 8000);
        let obj2 = create_test_object("module2", 8000);
        let obj3 = create_test_object("module3", 8000);
        
        let config = BankConfig::vectrex_512kb();
        
        let layout = MultibankLayout::from_objects(vec![obj1, obj2, obj3], config).unwrap();
        
        // Should use multiple banks (8KB each, 16KB limit)
        assert!(layout.banks.iter().filter(|b| !b.data.iter().all(|&x| x == 0xFF)).count() >= 2);
    }

    #[test]
    fn test_section_assignment_overflow() {
        let config = BankConfig::vectrex_512kb();
        
        // Create objects that fit in multiple banks (10KB each, 16KB limit)
        let objects: Vec<_> = (0..20)
            .map(|i| create_test_object(&format!("module{}", i), 10000))
            .collect();
        
        let assignments = MultibankLayout::assign_sections_to_banks(&objects, &config).unwrap();
        
        // Verify no bank overflow
        let mut bank_usage: HashMap<u8, usize> = HashMap::new();
        for assignment in &assignments {
            *bank_usage.entry(assignment.bank_id).or_insert(0) += 10000;
        }
        
        for (_, size) in bank_usage {
            assert!(size <= config.bank_size);
        }
    }
}
