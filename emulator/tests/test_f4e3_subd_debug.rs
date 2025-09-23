use vectrex_emulator::emulator::Emulator;
use std::path::Path;

#[test]
fn test_f4e3_subd_debug() {
    println!("=== DEBUG F4E3 SUBD ESPECÍFICO ===");
    
    // Crear emulador y cargar BIOS
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let mut emu = Emulator::new();
    let bios_data = std::fs::read(bios_path).expect("No se pudo leer BIOS");
    emu.load_bios(&bios_data);
    
    // Configurar CPU en estado similar al problema
    emu.cpu.pc = 0xF4E3;
    emu.cpu.x = 0xF9D4;  // Valor antes de la corrupción
    emu.cpu.a = 0x0F;
    emu.cpu.b = 0xDC;
    
    println!("🔧 Estado inicial:");
    println!("   PC: 0x{:04X}", emu.cpu.pc);
    println!("   X: 0x{:04X}", emu.cpu.x);
    let d_val = ((emu.cpu.a as u16) << 8) | emu.cpu.b as u16;
    println!("   D: 0x{:04X} (A=0x{:02X}, B=0x{:02X})", d_val, emu.cpu.a, emu.cpu.b);
    
    // Leer opcode directamente del bus
    let actual_opcode = emu.cpu.bus.read8(0xF4E3);
    println!("   Opcode en 0xF4E3: 0x{:02X}", actual_opcode);
    
    // Leer siguientes bytes también
    let byte1 = emu.cpu.bus.read8(0xF4E4);
    let byte2 = emu.cpu.bus.read8(0xF4E5);
    println!("   Bytes siguientes: 0x{:02X} 0x{:02X}", byte1, byte2);
    
    // Verificar si esto es realmente SUBD extended
    if actual_opcode == 0xB3 {
        let addr = ((byte1 as u16) << 8) | byte2 as u16;
        let data_hi = emu.cpu.bus.read8(addr);
        let data_lo = emu.cpu.bus.read8(addr.wrapping_add(1));
        let data = ((data_hi as u16) << 8) | data_lo as u16;
        println!("   SUBD extended de 0x{:04X} = 0x{:04X}", addr, data);
        
        let d_before = ((emu.cpu.a as u16) << 8) | emu.cpu.b as u16;
        let expected_result = d_before.wrapping_sub(data);
        println!("   Cálculo esperado: 0x{:04X} - 0x{:04X} = 0x{:04X}", d_before, data, expected_result);
    }
    
    println!("\n🚀 Ejecutando step...");
    
    // Ejecutar step y capturar resultado
    let success = emu.cpu.step();
    
    println!("\n📊 Resultado:");
    println!("   PC final: 0x{:04X}", emu.cpu.pc);
    println!("   X final: 0x{:04X}", emu.cpu.x);
    let d_final = ((emu.cpu.a as u16) << 8) | emu.cpu.b as u16;
    println!("   D final: 0x{:04X} (A=0x{:02X}, B=0x{:02X})", d_final, emu.cpu.a, emu.cpu.b);
    println!("   Step exitoso: {}", success);
    
    // ¿X cambió incorrectamente?
    if emu.cpu.x != 0xF9D4 {
        println!("⚠️  X fue modificado incorrectamente!");
        println!("   X esperado: 0xF9D4");
        println!("   X actual: 0x{:04X}", emu.cpu.x);
        println!("   Δ: {:+}", (emu.cpu.x as i32) - (0xF9D4_i32));
    } else {
        println!("✅ X no fue modificado (correcto para SUBD)");
    }
    
    // Verificar si PC avanzó correctamente (0xF4E3 + 3 = 0xF4E6)
    if emu.cpu.pc == 0xF4E6 {
        println!("✅ PC avanzó correctamente para SUBD extended");
    } else {
        println!("⚠️  PC no avanzó como esperado para SUBD extended");
        println!("   PC esperado: 0xF4E6");
        println!("   PC actual: 0x{:04X}", emu.cpu.pc);
    }
}