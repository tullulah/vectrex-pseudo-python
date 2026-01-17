// End-to-End Integration Test
//
// Tests full pipeline: Link objects → Resolve symbols → Output binary
// Simplified version focusing on linker functionality

use vpy_linker::{VectrexObject, MultibankLayout, BankConfig};
use vpy_linker::{Section, SectionType, Symbol, SymbolScope, SymbolType};
use vpy_linker::{Relocation, RelocationType};

#[test]
fn test_end_to_end_simple_link() {
    // Create a simple object file
    let mut obj = VectrexObject::new("test.vpy".to_string());
    
    // Add code section
    obj.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: vec![
            0x8E, 0x10, 0x00,  // LDX #$1000
            0x10, 0x8E, 0x20, 0x00,  // LDY #$2000
            0x39,  // RTS
        ],
    });
    
    // Add symbol for function
    obj.symbols.exports.push(Symbol {
        name: "test_func".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Link single object
    let config = BankConfig::single_bank();
    let layout = MultibankLayout::from_objects(vec![obj], config)
        .expect("Link failed");
    
    // Verify layout
    assert_eq!(layout.banks.len(), 1);
    assert_eq!(layout.banks[0].bank_id, 0);
    assert_eq!(layout.banks[0].symbols.len(), 1);
    assert_eq!(layout.banks[0].symbols[0].name, "test_func");
    
    // Verify code was copied
    let code = &layout.banks[0].data;
    assert_eq!(code[0], 0x8E);  // LDX opcode
    assert_eq!(code[1], 0x10);
    assert_eq!(code[2], 0x00);
    
    println!("✅ Simple link test passed!");
    println!("   Function at address: 0x{:04X}", layout.banks[0].symbols[0].address);
}

#[test]
fn test_end_to_end_multibank() {
    // Create multiple objects for multibank ROM
    let mut obj1 = VectrexObject::new("module1.vpy".to_string());
    obj1.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: Some(0),
        alignment: 1,
        data: vec![0x12; 10000],  // 10KB
    });
    obj1.symbols.exports.push(Symbol {
        name: "func_a".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    let mut obj2 = VectrexObject::new("module2.vpy".to_string());
    obj2.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: Some(1),
        alignment: 1,
        data: vec![0x34; 10000],  // 10KB
    });
    obj2.symbols.exports.push(Symbol {
        name: "func_b".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Link with multibank config
    let config = BankConfig::vectrex_512kb();
    let layout = MultibankLayout::from_objects(vec![obj1, obj2], config)
        .expect("Multibank link failed");
    
    // Verify multiple banks used
    let used_banks = layout.banks.iter()
        .filter(|b| b.symbols.len() > 0)
        .count();
    assert!(used_banks >= 2, "Should use at least 2 banks");
    
    // Verify symbols are in correct banks
    let all_symbols: Vec<_> = layout.banks.iter()
        .flat_map(|b| b.symbols.iter())
        .collect();
    assert_eq!(all_symbols.len(), 2);
    
    println!("✅ Multibank layout: {} banks used", used_banks);
    for (i, bank) in layout.banks.iter().enumerate() {
        if !bank.symbols.is_empty() {
            println!("   Bank {}: {} symbols", i, bank.symbols.len());
        }
    }
}

#[test]
fn test_end_to_end_with_imports() {
    // Object 1: Exports func_a
    let mut obj1 = VectrexObject::new("module1.vpy".to_string());
    obj1.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: vec![0x39],  // RTS
    });
    obj1.symbols.exports.push(Symbol {
        name: "func_a".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Object 2: Imports func_a
    let mut obj2 = VectrexObject::new("module2.vpy".to_string());
    obj2.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: vec![
            0xBD, 0x00, 0x00,  // JSR $0000 (will be relocated)
            0x39,  // RTS
        ],
    });
    obj2.symbols.exports.push(Symbol {
        name: "func_b".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    obj2.symbols.imports.push(Symbol {
        name: "func_a".to_string(),
        section: None,
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Add relocation for JSR
    obj2.relocations.push(Relocation {
        section: 0,
        offset: 1,  // Offset of address in JSR instruction
        reloc_type: RelocationType::Absolute16,
        symbol: "func_a".to_string(),
        addend: 0,
    });
    
    // Link both objects
    let config = BankConfig::single_bank();
    let layout = MultibankLayout::from_objects(vec![obj1, obj2], config)
        .expect("Link with imports failed");
    
    // Verify symbols resolved
    assert_eq!(layout.symbol_table.symbols.len(), 2);
    
    // Verify relocation was applied (JSR should now point to func_a)
    let func_a_addr = layout.symbol_table.symbols.get("func_a").unwrap().address;
    let code = &layout.banks[0].data;
    
    // After func_a (1 byte RTS), func_b starts
    // JSR is at offset after func_a
    let jsr_target = ((code[2] as u16) << 8) | (code[3] as u16);
    assert_eq!(jsr_target, func_a_addr, "Relocation should point to func_a");
    
    println!("✅ Import resolution test passed!");
    println!("   func_a at: 0x{:04X}", func_a_addr);
    println!("   JSR target: 0x{:04X}", jsr_target);
}

#[test]
fn test_end_to_end_file_output() {
    use std::fs;
    
    // Create simple object
    let mut obj = VectrexObject::new("test.vpy".to_string());
    obj.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: vec![0x12, 0x34, 0x56, 0x78],
    });
    obj.symbols.exports.push(Symbol {
        name: "test".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Link
    let config = BankConfig::single_bank();
    let layout = MultibankLayout::from_objects(vec![obj], config)
        .expect("Link failed");
    
    // Write to temp directory
    let temp_dir = std::env::temp_dir().join("vpy_linker_test");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    let output_path = temp_dir.join("test_output.bin");
    layout.write_merged(&output_path).expect("Failed to write output");
    
    // Verify file was created
    assert!(output_path.exists(), "Output file should exist");
    
    // Read back and verify
    let data = fs::read(&output_path).expect("Failed to read output");
    assert_eq!(data[0], 0x12);
    assert_eq!(data[1], 0x34);
    assert_eq!(data[2], 0x56);
    assert_eq!(data[3], 0x78);
    
    // Cleanup
    fs::remove_file(&output_path).ok();
    fs::remove_dir(&temp_dir).ok();
    
    println!("✅ File output test passed! Wrote {} bytes", data.len());
}

#[test]
fn test_end_to_end_symbol_table() {
    // Create object with multiple symbols
    let mut obj = VectrexObject::new("test.vpy".to_string());
    
    obj.sections.push(Section {
        name: ".text".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: vec![0x12; 50],
    });
    
    obj.sections.push(Section {
        name: ".data".to_string(),
        section_type: SectionType::Data,
        bank_hint: None,
        alignment: 1,
        data: vec![0x34; 20],
    });
    
    // Add symbols at different offsets
    obj.symbols.exports.push(Symbol {
        name: "func1".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    obj.symbols.exports.push(Symbol {
        name: "func2".to_string(),
        section: Some(0),
        offset: 20,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    obj.symbols.exports.push(Symbol {
        name: "data_var".to_string(),
        section: Some(1),
        offset: 5,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Variable,
    });
    
    // Link
    let config = BankConfig::single_bank();
    let layout = MultibankLayout::from_objects(vec![obj], config)
        .expect("Link failed");
    
    // Verify all symbols have correct addresses
    let sym_table = &layout.symbol_table.symbols;
    
    let func1 = sym_table.get("func1").unwrap();
    let func2 = sym_table.get("func2").unwrap();
    let data_var = sym_table.get("data_var").unwrap();
    
    assert_eq!(func1.address, 0);
    assert_eq!(func2.address, 20);
    assert_eq!(data_var.address, 50 + 5);  // After .text section + offset
    
    println!("✅ Symbol table test passed!");
    println!("   func1: 0x{:04X}", func1.address);
    println!("   func2: 0x{:04X}", func2.address);
    println!("   data_var: 0x{:04X}", data_var.address);
}
