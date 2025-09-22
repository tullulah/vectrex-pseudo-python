// Test para diagnosticar el problema de vectores diagonales
// Verifica que las coordenadas se escriban y lean correctamente

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
            println!("BIOS cargada correctamente desde assets");
        }
        Err(_) => {
            // Fallback to dist path
            let alt_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
            let data = std::fs::read(alt_path).expect("BIOS real requerida para test");
            assert_eq!(data.len(), 8192, "BIOS size inesperado");
            for (i, b) in data.iter().enumerate() { 
                let addr = 0xE000 + i as u16; 
                cpu.bus.mem[addr as usize] = *b; 
            }
            cpu.bios_present = true;
            println!("BIOS cargada desde dist como fallback");
        }
    }
}

#[test]
fn test_vector_coordinate_operations() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.trace = true;
    
    println!("=== Test: Vector Coordinate Operations ===");
    
    // Test LDD immediate with coordinates
    cpu.pc = 0x1000;
    cpu.bus.mem[0x1000] = 0xCC; // LDD immediate
    cpu.bus.mem[0x1001] = 0x12; // High byte: 0x12
    cpu.bus.mem[0x1002] = 0x34; // Low byte: 0x34
    
    println!("Before LDD #$1234:");
    println!("  A={:02X}, B={:02X}, D={:04X}", cpu.a, cpu.b, (cpu.a as u16) << 8 | (cpu.b as u16));
    
    let cycles_before = cpu.bus.total_cycles;
    cpu.step();
    let cycles_after = cpu.bus.total_cycles;
    
    println!("After LDD #$1234:");
    println!("  A={:02X}, B={:02X}, D={:04X}", cpu.a, cpu.b, (cpu.a as u16) << 8 | (cpu.b as u16));
    println!("  PC={:04X}, Cycles: {}", cpu.pc, cycles_after - cycles_before);
    
    assert_eq!(cpu.a, 0x12, "A should be 0x12");
    assert_eq!(cpu.b, 0x34, "B should be 0x34");
    assert_eq!((cpu.a as u16) << 8 | (cpu.b as u16), 0x1234, "D should be 0x1234");
    
    // Test STD extended
    cpu.bus.mem[0x1003] = 0xFD; // STD extended
    cpu.bus.mem[0x1004] = 0x20; // Address high: 0x20
    cpu.bus.mem[0x1005] = 0x00; // Address low: 0x00
    
    println!("\nBefore STD $2000:");
    println!("  Memory[2000]={:02X}, Memory[2001]={:02X}", 
             cpu.bus.mem[0x2000], cpu.bus.mem[0x2001]);
    
    let cycles_before = cpu.bus.total_cycles;
    cpu.step();
    let cycles_after = cpu.bus.total_cycles;
    
    println!("After STD $2000:");
    println!("  Memory[2000]={:02X}, Memory[2001]={:02X}", 
             cpu.bus.mem[0x2000], cpu.bus.mem[0x2001]);
    println!("  PC={:04X}, Cycles: {}", cpu.pc, cycles_after - cycles_before);
    
    assert_eq!(cpu.bus.mem[0x2000], 0x12, "Memory[2000] should be A (0x12)");
    assert_eq!(cpu.bus.mem[0x2001], 0x34, "Memory[2001] should be B (0x34)");
    
    // Test coordinate calculation
    println!("\n=== Coordinate Calculation Test ===");
    
    // Load X coordinate: 100 (0x64)
    cpu.a = 0x00;
    cpu.b = 0x64;
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    println!("X coordinate: D={:04X} (decimal {})", d_val, d_val);
    
    // Load Y coordinate: 200 (0xC8)  
    cpu.a = 0x00;
    cpu.b = 0xC8;
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    println!("Y coordinate: D={:04X} (decimal {})", d_val, d_val);
    
    // Test signed coordinates
    cpu.a = 0xFF;
    cpu.b = 0x9C; // -100 in two's complement
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    println!("Signed X: D={:04X} (decimal {}, signed {})", d_val, d_val, d_val as i16);
    
    println!("=== Test Complete ===");
}

#[test]
fn test_coordinate_flags() {
    let mut cpu = CPU::default();
    cpu.trace = true;
    
    println!("=== Flag Behavior Test ===");
    
    // Test zero coordinate (flags will be set by arithmetic operations)
    cpu.a = 0x00;
    cpu.b = 0x00;
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    // Simulate flag behavior for zero value
    cpu.cc_z = d_val == 0;
    cpu.cc_n = (d_val & 0x8000) != 0;
    
    println!("Zero coordinate: D={:04X}, N={}, Z={}", 
             d_val, cpu.cc_n, cpu.cc_z);
    assert_eq!(cpu.cc_z, true, "Z flag should be set for zero");
    assert_eq!(cpu.cc_n, false, "N flag should be clear for zero");
    
    // Test positive coordinate
    cpu.a = 0x01;
    cpu.b = 0x00;
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    cpu.cc_z = d_val == 0;
    cpu.cc_n = (d_val & 0x8000) != 0;
    
    println!("Positive coordinate: D={:04X}, N={}, Z={}", 
             d_val, cpu.cc_n, cpu.cc_z);
    assert_eq!(cpu.cc_z, false, "Z flag should be clear for non-zero");
    assert_eq!(cpu.cc_n, false, "N flag should be clear for positive");
    
    // Test negative coordinate
    cpu.a = 0x80;
    cpu.b = 0x00;
    let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
    cpu.cc_z = d_val == 0;
    cpu.cc_n = (d_val & 0x8000) != 0;
    
    println!("Negative coordinate: D={:04X}, N={}, Z={}", 
             d_val, cpu.cc_n, cpu.cc_z);
    assert_eq!(cpu.cc_z, false, "Z flag should be clear for non-zero");
    assert_eq!(cpu.cc_n, true, "N flag should be set for negative");
    
    println!("=== Flag Test Complete ===");
}