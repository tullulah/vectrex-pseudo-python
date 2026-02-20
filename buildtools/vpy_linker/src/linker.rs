//! Main linker algorithm - combines objects and applies relocations

use crate::object::VectrexObject;
use crate::resolver::SymbolResolver;
use crate::error::LinkerResult;
use crate::{LinkedBinary, LinkedSection};

/// Link a list of object files into a single-bank binary.
///
/// Runs the canonical 4-step algorithm:
///   1. collect_symbols  – build global symbol table, detect duplicates
///   2. verify_imports   – all referenced symbols must be defined
///   3. assign_addresses – compute final address of each section/symbol
///   4. apply_relocations – patch raw bytes with resolved addresses
///
/// For multibank output use `MultibankLayout::from_objects()` instead.
pub fn link(mut objects: Vec<VectrexObject>, base_address: u16) -> LinkerResult<LinkedBinary> {
    // Step 1
    let mut global = SymbolResolver::collect_symbols(&objects)?;

    // Step 2
    SymbolResolver::verify_imports(&objects, &global)?;

    // Step 3
    let section_bases = SymbolResolver::assign_addresses(&objects, &mut global, base_address)?;

    // Step 4
    SymbolResolver::apply_relocations(&mut objects, &global, &section_bases)?;

    // Concatenate sections in (object, section) order to build output binary
    let mut binary: Vec<u8> = Vec::new();
    let mut sections: Vec<LinkedSection> = Vec::new();

    for (obj_idx, obj) in objects.iter().enumerate() {
        for (section_idx, section) in obj.sections.iter().enumerate() {
            let start = *section_bases
                .get(&(obj_idx, section_idx))
                .unwrap_or(&base_address) as u32;

            sections.push(LinkedSection {
                name: section.name.clone(),
                start,
                size: section.size(),
            });

            binary.extend_from_slice(&section.data);
        }
    }

    // Build name → address map from the resolved global symbol table
    let symbols = global
        .symbols
        .into_iter()
        .map(|(name, sym)| (name, sym.address as u32))
        .collect();

    Ok(LinkedBinary {
        binary,
        symbols,
        sections,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{Section, SectionType, Symbol, SymbolScope, SymbolType};

    fn make_object(source: &str, export: &str, code: Vec<u8>) -> VectrexObject {
        let mut obj = VectrexObject::new(source.to_string());
        obj.sections.push(Section {
            name: ".text".to_string(),
            section_type: SectionType::Text,
            bank_hint: None,
            alignment: 1,
            data: code,
        });
        obj.symbols.exports.push(Symbol {
            name: export.to_string(),
            section: Some(0),
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        obj
    }

    #[test]
    fn test_link_single_object() {
        let obj = make_object("main.vpy", "START", vec![0x12, 0x39]);
        let result = link(vec![obj], 0x0000).unwrap();
        assert_eq!(result.binary, vec![0x12, 0x39]);
        assert_eq!(result.symbols["START"], 0x0000);
    }

    #[test]
    fn test_link_two_objects() {
        let obj1 = make_object("a.vpy", "func_a", vec![0x01, 0x02]);
        let obj2 = make_object("b.vpy", "func_b", vec![0x03, 0x04]);
        let result = link(vec![obj1, obj2], 0x4000).unwrap();
        assert_eq!(result.binary, vec![0x01, 0x02, 0x03, 0x04]);
        assert_eq!(result.symbols["func_a"], 0x4000);
        assert_eq!(result.symbols["func_b"], 0x4002);
    }

    #[test]
    fn test_link_duplicate_symbol_error() {
        let obj1 = make_object("a.vpy", "dup", vec![0x01]);
        let obj2 = make_object("b.vpy", "dup", vec![0x02]);
        assert!(link(vec![obj1, obj2], 0x0000).is_err());
    }
}
