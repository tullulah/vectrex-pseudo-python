use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_subd_opcode_b3_simple() {
    println!("=== TEST OPCODE 0xB3 SIMPLE ===");
    
    let mut cpu = CPU::default();
    
    // Configurar instrucción SUBD extended en 0xC800
    cpu.test_write8(0xC800, 0xB3);  // SUBD extended
    cpu.test_write8(0xC801, 0xC8);  // addr alto
    cpu.test_write8(0xC802, 0x2C);  // addr bajo
    
    // Configurar valor en la dirección target 0xC82C
    cpu.test_write8(0xC82C, 0x01);  // hi
    cpu.test_write8(0xC82D, 0x23);  // lo = 0x0123
    
    // Configurar registros iniciales
    cpu.pc = 0xC800;
    cpu.a = 0x05;  // D = 0x05FF
    cpu.b = 0xFF;
    let d_inicial = ((cpu.a as u16) << 8) | (cpu.b as u16);
    
    println!("🔧 Configuración inicial:");
    println!("   PC: 0x{:04X}", cpu.pc);
    println!("   D: 0x{:04X} (A=0x{:02X}, B=0x{:02X})", d_inicial, cpu.a, cpu.b);
    println!("   Valor en 0xC82C: 0x{:04X}", (cpu.test_read8(0xC82C) as u16) << 8 | cpu.test_read8(0xC82D) as u16);
    
    // Verificar que la instrucción sea exactamente 0xB3
    let opcode_at_pc = cpu.test_read8(cpu.pc);
    println!("   Opcode en PC: 0x{:02X}", opcode_at_pc);
    
    println!("\n🚀 Ejecutando step...");
    let initial_pc = cpu.pc;
    let initial_cycles = cpu.cycles;
    let step_result = cpu.step();
    let final_cycles = cpu.cycles;
    
    let d_final = ((cpu.a as u16) << 8) | (cpu.b as u16);
    
    println!("\n📊 Resultado:");
    println!("   PC inicial: 0x{:04X}", initial_pc);
    println!("   PC final: 0x{:04X}", cpu.pc);
    println!("   Increment PC: {} bytes", cpu.pc - initial_pc);
    println!("   D inicial: 0x{:04X}", d_inicial);
    println!("   D final: 0x{:04X}", d_final);
    println!("   Cambio en D: {}", if d_final != d_inicial { "SÍ" } else { "NO" });
    println!("   Ciclos iniciales: {}", initial_cycles);
    println!("   Ciclos finales: {}", final_cycles);
    println!("   Ciclos usados: {}", final_cycles - initial_cycles);
    println!("   Esperado: 0x{:04X} (0x05FF - 0x0123 = 0x04DC)", 0x05FF - 0x0123);
    println!("   Step exitoso: {}", step_result);
    
    if d_final == 0x04DC {
        println!("✅ SUBD funcionó correctamente!");
    } else {
        println!("❌ SUBD NO funcionó - resultado incorrecto");
        if d_final == d_inicial {
            println!("   🔍 D no cambió en absoluto - ¿se ejecutó la instrucción?");
        }
    }
}