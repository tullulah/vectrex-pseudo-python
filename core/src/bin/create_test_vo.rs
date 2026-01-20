// Test program to create valid .vo files and test the linker
// Run with: cargo run --bin create_test_vo

use vectrex_lang::linker::*;
use vectrex_lang::linker::object::{SymbolScope, SymbolType, RelocationType};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating test .vo files...");
    
    // Create lib.vo - exports helper_function
    let mut lib = VectrexObject::new("lib.vpy".to_string());
    
    // Add .text section with simple code
    let lib_code = vec![
        0x86, 0x7F,        // LDA #127
        0xBD, 0xF2, 0xAB,  // JSR $F2AB (Intensity_a BIOS call)
        0x39,              // RTS
    ];
    
    lib.sections.push(Section {
        name: ".text.lib".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: lib_code,
    });
    
    // Add exported symbol
    lib.symbols.exports.push(Symbol {
        name: "helper_function".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    let mut lib_file = File::create("lib.vo")?;
    lib.write(&mut lib_file)?;
    println!("✓ Created lib.vo");
    println!("  Sections: {}", lib.sections.len());
    println!("  Exports: {}", lib.symbols.exports.len());
    
    // Create main.vo - imports helper_function and calls it
    let mut main_obj = VectrexObject::new("main.vpy".to_string());
    
    // Add .text section with code that calls external function
    let main_code = vec![
        0x86, 0x7F,        // LDA #127
        0xBD, 0xF2, 0xAB,  // JSR $F2AB (Intensity_a)
        0xBD, 0x00, 0x00,  // JSR $0000 (placeholder for helper_function)
        0x39,              // RTS
    ];
    
    main_obj.sections.push(Section {
        name: ".text.main".to_string(),
        section_type: SectionType::Text,
        bank_hint: None,
        alignment: 1,
        data: main_code,
    });
    
    // Add exported symbol
    main_obj.symbols.exports.push(Symbol {
        name: "main_function".to_string(),
        section: Some(0),
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Add imported symbol
    main_obj.symbols.imports.push(Symbol {
        name: "helper_function".to_string(),
        section: None,
        offset: 0,
        scope: SymbolScope::Global,
        symbol_type: SymbolType::Function,
    });
    
    // Add relocation for JSR helper_function
    // Byte layout: [LDA #127, JSR $F2AB, JSR $0000, RTS]
    //              [0-1,      2-4,       5-7,       8]
    // JSR opcode at offset 5, address placeholder at 6-7
    main_obj.relocations.push(Relocation {
        section: 0,
        offset: 6,  // Offset to first byte of address (not opcode)
        reloc_type: RelocationType::Absolute16,
        symbol: "helper_function".to_string(),
        addend: 0,
    });
    
    let mut main_file = File::create("main.vo")?;
    main_obj.write(&mut main_file)?;
    println!("✓ Created main.vo");
    println!("  Sections: {}", main_obj.sections.len());
    println!("  Exports: {}", main_obj.symbols.exports.len());
    println!("  Imports: {}", main_obj.symbols.imports.len());
    println!("  Relocations: {}", main_obj.relocations.len());
    
    println!("\nNow run:");
    println!("  cargo run --bin vectrexc -- link lib.vo main.vo -o linked.bin");
    
    Ok(())
}
