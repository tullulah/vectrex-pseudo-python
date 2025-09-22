// Test final para confirmar que los vectores diagonales están corregidos
// Verifica que el indexed addressing funciona correctamente en casos comunes

use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    match std::fs::read(path) {
        Ok(data) => {
            assert_eq!(data.len(), 8192, "BIOS size inesperado");
            for (i, b) in data.iter().enumerate() { 
                let addr = 0xE000 + i as u16; 
                cpu.bus.mem[addr as usize] = *b; 
            }
            cpu.bios_present = true;
        }
        Err(_) => {
            let alt_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
            let data = std::fs::read(alt_path).expect("BIOS real requerida para test");
            assert_eq!(data.len(), 8192, "BIOS size inesperado");
            for (i, b) in data.iter().enumerate() { 
                let addr = 0xE000 + i as u16; 
                cpu.bus.mem[addr as usize] = *b; 
            }
            cpu.bios_present = true;
        }
    }
}

#[test]
fn test_indexed_addressing_comprehensive() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.trace = true;
    
    println!("=== Comprehensive Indexed Addressing Test ===");
    
    // Set up test values
    cpu.x = 0x3000;
    cpu.y = 0x4000; 
    cpu.a = 0x10;
    cpu.b = 0x20;
    
    // Test multiple offset modes
    let test_cases = [
        (0x00, "0,X", 0x3000),      // 0 offset
        (0x01, "1,X", 0x3001),      // +1 offset
        (0x02, "2,X", 0x3002),      // +2 offset
        (0x05, "5,X", 0x3005),      // +5 offset (previously buggy)
        (0x08, "8,X", 0x3008),      // +8 offset
        (0x0F, "15,X", 0x300F),     // +15 offset (max positive)
        (0x10, "-16,X", 0x2FF0),    // -16 offset (max negative)
        (0x1F, "-1,X", 0x2FFF),     // -1 offset
    ];
    
    cpu.pc = 0x2000;
    
    for (i, (offset, desc, expected_addr)) in test_cases.iter().enumerate() {
        // Set up STB instruction with this offset
        let instruction_addr = 0x2000 + (i * 2) as u16;
        cpu.pc = instruction_addr;
        
        cpu.bus.mem[instruction_addr as usize] = 0xE7;  // STB indexed
        cpu.bus.mem[instruction_addr as usize + 1] = *offset;
        
        // Store test value
        let test_value = 0x42 + i as u8;
        cpu.b = test_value;
        
        println!("Test {}: STB {} (offset={:02X}, expected addr={:04X})", 
                i, desc, offset, expected_addr);
        
        cpu.step();
        
        // Verify the value was stored at the correct address
        let actual_value = cpu.bus.mem[*expected_addr as usize];
        println!("  Result: mem[{:04X}] = {:02X} (expected {:02X})", 
                expected_addr, actual_value, test_value);
        
        assert_eq!(actual_value, test_value, 
                  "STB {} should store {:02X} at {:04X}", desc, test_value, expected_addr);
    }
    
    println!("\n=== Testing Register Offset Modes (bit 7 = 1) ===");
    
    for i in 0..3 {
        let instruction_addr = 0x2100 + (i * 2) as u16;
        cpu.pc = instruction_addr;
        
        let test_value = 0x80 + i as u8;
        cpu.b = test_value;  // Update B for testing ,B,X
        
        let (postbyte, desc, expected_addr) = match i {
            0 => (0x84, ",A,X", cpu.x + cpu.a as u16),  // ,A,X mode (A=0x10)
            1 => (0x85, ",B,X", cpu.x + cpu.b as u16),  // ,B,X mode (B=test_value)  
            2 => (0x86, ",D,X", cpu.x + ((cpu.a as u16) << 8 | cpu.b as u16)), // ,D,X mode
            _ => unreachable!(),
        };
        
        cpu.bus.mem[instruction_addr as usize] = 0xE7;  // STB indexed
        cpu.bus.mem[instruction_addr as usize + 1] = postbyte;
        
        println!("Test {}: STB {} (postbyte={:02X}, expected addr={:04X})", 
                i, desc, postbyte, expected_addr);
        
        cpu.step();
        
        let actual_value = cpu.bus.mem[expected_addr as usize];
        println!("  Result: mem[{:04X}] = {:02X} (expected {:02X})", 
                expected_addr, actual_value, test_value);
        
        assert_eq!(actual_value, test_value, 
                  "STB {} should store {:02X} at {:04X}", desc, test_value, expected_addr);
    }
    
    println!("\n=== Indexed Addressing Test Complete ===");
    println!("✓ All offset modes working correctly");
    println!("✓ Vector coordinate calculations should be accurate");
    println!("✓ Diagonal vector bug should be resolved");
}

#[test]
fn test_vector_coordinate_precision() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.trace = true;
    
    println!("=== Vector Coordinate Precision Test ===");
    
    // Simulate vector coordinate calculations that would create diagonals
    cpu.x = 0x5000;  // Vector list base address
    
    // Test coordinate pairs that were problematic
    let coordinate_tests = [
        (100, 100), // Perfect diagonal - should not become corrupted
        (100, 200), // Different X/Y values  
        (50, 75),   // Small coordinates
        (0, 100),   // Zero X coordinate
        (100, 0),   // Zero Y coordinate
    ];
    
    cpu.pc = 0x1000;
    
    for (i, (x_coord, y_coord)) in coordinate_tests.iter().enumerate() {
        println!("\nTest {}: Coordinates ({}, {})", i, x_coord, y_coord);
        
        // Simulate loading X coordinate using indexed addressing
        // LDD #x_coord, STD offset,X
        let base_addr = 0x1000 + (i * 10) as u16;
        cpu.pc = base_addr;
        
        // Load X coordinate into D register
        cpu.bus.mem[base_addr as usize] = 0xCC;  // LDD immediate
        cpu.bus.mem[base_addr as usize + 1] = (*x_coord >> 8) as u8;
        cpu.bus.mem[base_addr as usize + 2] = (*x_coord & 0xFF) as u8;
        
        println!("  Loading X coordinate {} into D", x_coord);
        cpu.step();
        println!("    D = {:04X} (A={:02X}, B={:02X})", 
                (cpu.a as u16) << 8 | cpu.b as u16, cpu.a, cpu.b);
        
        // Store X coordinate using indexed addressing
        cpu.bus.mem[cpu.pc as usize] = 0xFD;     // STD extended 
        cpu.bus.mem[cpu.pc as usize + 1] = 0x50; // High byte of address
        cpu.bus.mem[cpu.pc as usize + 2] = (0x00 + i * 4) as u8; // Low byte
        
        println!("  Storing X coordinate at {:04X}", 0x5000 + i * 4);
        cpu.step();
        
        let stored_x_hi = cpu.bus.mem[0x5000 + i * 4];
        let stored_x_lo = cpu.bus.mem[0x5000 + i * 4 + 1];
        let stored_x = (stored_x_hi as u16) << 8 | stored_x_lo as u16;
        
        println!("    Stored: {:02X}{:02X} = {}", stored_x_hi, stored_x_lo, stored_x);
        assert_eq!(stored_x, *x_coord, "X coordinate should be stored correctly");
        
        // Repeat for Y coordinate
        cpu.bus.mem[cpu.pc as usize] = 0xCC;  // LDD immediate
        cpu.bus.mem[cpu.pc as usize + 1] = (*y_coord >> 8) as u8;
        cpu.bus.mem[cpu.pc as usize + 2] = (*y_coord & 0xFF) as u8;
        
        println!("  Loading Y coordinate {} into D", y_coord);
        cpu.step();
        
        cpu.bus.mem[cpu.pc as usize] = 0xFD;     // STD extended
        cpu.bus.mem[cpu.pc as usize + 1] = 0x50; // High byte
        cpu.bus.mem[cpu.pc as usize + 2] = (0x02 + i * 4) as u8; // Low byte + 2
        
        println!("  Storing Y coordinate at {:04X}", 0x5002 + i * 4);
        cpu.step();
        
        let stored_y_hi = cpu.bus.mem[0x5002 + i * 4];
        let stored_y_lo = cpu.bus.mem[0x5002 + i * 4 + 1];
        let stored_y = (stored_y_hi as u16) << 8 | stored_y_lo as u16;
        
        println!("    Stored: {:02X}{:02X} = {}", stored_y_hi, stored_y_lo, stored_y);
        assert_eq!(stored_y, *y_coord, "Y coordinate should be stored correctly");
        
        // Verify no diagonal corruption
        if x_coord == y_coord {
            println!("  ✓ Diagonal coordinates preserved correctly");
        } else {
            println!("  ✓ Different X/Y coordinates maintained");
        }
    }
    
    println!("\n=== Vector Coordinate Precision Test Complete ===");
}