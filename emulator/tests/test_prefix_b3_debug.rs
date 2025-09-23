use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_prefix_b3_debug() {
    println!("=== TEST ANÁLISIS PREFIJOS 0xB3 ===");
    
    // Test 1: SUBD extended directo (0xB3)
    let mut cpu1 = CPU::default();
    cpu1.test_write8(0xC800, 0xB3);  // SUBD extended
    cpu1.test_write8(0xC801, 0xC8);  // addr alto
    cpu1.test_write8(0xC802, 0x2C);  // addr bajo
    cpu1.test_write8(0xC82C, 0x01);  // valor hi
    cpu1.test_write8(0xC82D, 0x23);  // valor lo = 0x0123
    cpu1.pc = 0xC800;
    cpu1.a = 0x05;
    cpu1.b = 0xFF;  // D = 0x05FF
    
    let d_inicial_1 = ((cpu1.a as u16) << 8) | (cpu1.b as u16);
    let x_inicial_1 = cpu1.x;
    
    println!("\n🔍 Test 1: SUBD directo (0xB3)");
    println!("   PC: 0x{:04X}", cpu1.pc);
    println!("   Opcode en PC: 0x{:02X}", cpu1.test_read8(cpu1.pc));
    println!("   D inicial: 0x{:04X}", d_inicial_1);
    println!("   X inicial: 0x{:04X}", x_inicial_1);
    
    let step1 = cpu1.step();
    let d_final_1 = ((cpu1.a as u16) << 8) | (cpu1.b as u16);
    let x_final_1 = cpu1.x;
    
    println!("   Step exitoso: {}", step1);
    println!("   D final: 0x{:04X} (cambio: {})", d_final_1, d_final_1 != d_inicial_1);
    println!("   X final: 0x{:04X} (cambio: {})", x_final_1, x_final_1 != x_inicial_1);
    
    // Test 2: CMPD extended con prefijo (0x10 0xB3)
    let mut cpu2 = CPU::default();
    cpu2.test_write8(0xC800, 0x10);  // Prefijo grupo 1
    cpu2.test_write8(0xC801, 0xB3);  // CMPD extended
    cpu2.test_write8(0xC802, 0xC8);  // addr alto
    cpu2.test_write8(0xC803, 0x2C);  // addr bajo
    cpu2.test_write8(0xC82C, 0x01);  // valor hi
    cpu2.test_write8(0xC82D, 0x23);  // valor lo = 0x0123
    cpu2.pc = 0xC800;
    cpu2.a = 0x05;
    cpu2.b = 0xFF;  // D = 0x05FF
    
    let d_inicial_2 = ((cpu2.a as u16) << 8) | (cpu2.b as u16);
    let x_inicial_2 = cpu2.x;
    
    println!("\n🔍 Test 2: CMPD con prefijo (0x10 0xB3)");
    println!("   PC: 0x{:04X}", cpu2.pc);
    println!("   Opcode en PC: 0x{:02X}", cpu2.test_read8(cpu2.pc));
    println!("   Opcode+1 en PC+1: 0x{:02X}", cpu2.test_read8(cpu2.pc + 1));
    println!("   D inicial: 0x{:04X}", d_inicial_2);
    println!("   X inicial: 0x{:04X}", x_inicial_2);
    
    let step2 = cpu2.step();
    let d_final_2 = ((cpu2.a as u16) << 8) | (cpu2.b as u16);
    let x_final_2 = cpu2.x;
    
    println!("   Step exitoso: {}", step2);
    println!("   D final: 0x{:04X} (cambio: {})", d_final_2, d_final_2 != d_inicial_2);
    println!("   X final: 0x{:04X} (cambio: {})", x_final_2, x_final_2 != x_inicial_2);
    
    // Test 3: CMPU extended con prefijo (0x11 0xB3)
    let mut cpu3 = CPU::default();
    cpu3.test_write8(0xC800, 0x11);  // Prefijo grupo 2
    cpu3.test_write8(0xC801, 0xB3);  // CMPU extended
    cpu3.test_write8(0xC802, 0xC8);  // addr alto
    cpu3.test_write8(0xC803, 0x2C);  // addr bajo  
    cpu3.test_write8(0xC82C, 0x12);  // valor hi para comparación U
    cpu3.test_write8(0xC82D, 0x34);  // valor lo = 0x1234
    cpu3.pc = 0xC800;
    cpu3.a = 0x05;
    cpu3.b = 0xFF;  // D = 0x05FF
    cpu3.u = 0x1234; // U para comparación
    
    let d_inicial_3 = ((cpu3.a as u16) << 8) | (cpu3.b as u16);
    let x_inicial_3 = cpu3.x;
    let u_inicial_3 = cpu3.u;
    
    println!("\n🔍 Test 3: CMPU con prefijo (0x11 0xB3)");
    println!("   PC: 0x{:04X}", cpu3.pc);
    println!("   Opcode en PC: 0x{:02X}", cpu3.test_read8(cpu3.pc));
    println!("   Opcode+1 en PC+1: 0x{:02X}", cpu3.test_read8(cpu3.pc + 1));
    println!("   D inicial: 0x{:04X}", d_inicial_3);
    println!("   X inicial: 0x{:04X}", x_inicial_3);
    println!("   U inicial: 0x{:04X}", u_inicial_3);
    
    let step3 = cpu3.step();
    let d_final_3 = ((cpu3.a as u16) << 8) | (cpu3.b as u16);
    let x_final_3 = cpu3.x;
    let u_final_3 = cpu3.u;
    
    println!("   Step exitoso: {}", step3);
    println!("   D final: 0x{:04X} (cambio: {})", d_final_3, d_final_3 != d_inicial_3);
    println!("   X final: 0x{:04X} (cambio: {})", x_final_3, x_final_3 != x_inicial_3);
    println!("   U final: 0x{:04X} (cambio: {})", u_final_3, u_final_3 != u_inicial_3);
    
    println!("\n📊 RESUMEN:");
    println!("   Test 1 (0xB3): D cambió: {}, X cambió: {}", 
             d_final_1 != d_inicial_1, x_final_1 != x_inicial_1);
    println!("   Test 2 (0x10 0xB3): D cambió: {}, X cambió: {}", 
             d_final_2 != d_inicial_2, x_final_2 != x_inicial_2);
    println!("   Test 3 (0x11 0xB3): D cambió: {}, X cambió: {}, U cambió: {}", 
             d_final_3 != d_inicial_3, x_final_3 != x_inicial_3, u_final_3 != u_inicial_3);
             
    // Verificación adicional del problema central
    if x_final_1 != x_inicial_1 {
        println!("\n⚠️  PROBLEMA ENCONTRADO: SUBD (0xB3) modificó X cuando debe modificar D");
        println!("    X cambió de 0x{:04X} → 0x{:04X}", x_inicial_1, x_final_1);
    }
    
    if d_final_1 == d_inicial_1 && x_final_1 != x_inicial_1 {
        println!("\n🎯 CONFIRMADO: 0xB3 afecta X en lugar de D (interceptado por prefijo 0x11?)");
    }
}