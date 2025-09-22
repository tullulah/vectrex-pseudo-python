use vectrex_emulator::emulator::Emulator;
use std::fs;

#[test]
fn debug_reset_vector() {
    let mut emulator = Emulator::new();
    
    // ¡CRÍTICO! Cargar BIOS primero, como hace el sistema real
    let bios_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to read BIOS file");
    emulator.load_bios(&bios_data);
    
    println!("🔍 DEBUG RESET VECTOR");
    println!("🔧 PC antes de reset: 0x{:04X}", emulator.cpu.pc);
    println!("🔧 BIOS presente: {}", emulator.cpu.bios_present);
    println!("🔧 BIOS size: {} bytes", bios_data.len());
    
    // Verificar bytes del vector de reset ANTES del reset
    let reset_hi = emulator.cpu.test_read8(0xFFFC);
    let reset_lo = emulator.cpu.test_read8(0xFFFD);
    let reset_vector = ((reset_hi as u16) << 8) | (reset_lo as u16);
    
    println!("📋 ANTES DEL RESET:");
    println!("   Vector bytes: 0xFFFC=0x{:02X}, 0xFFFD=0x{:02X}", reset_hi, reset_lo);
    println!("   Vector calculado: 0x{:04X}", reset_vector);
    
    // Verificar algunos bytes de BIOS para ver si está cargada
    let mut bios_sample = String::new();
    for addr in 0xF000u16..0xF010u16 {
        let byte = emulator.cpu.test_read8(addr);
        bios_sample.push_str(&format!(" {:02X}", byte));
    }
    println!("   BIOS sample (0xF000-0xF00F):{}", bios_sample);
    
    // Llamar a reset y ver qué pasa
    println!("\n🔄 LLAMANDO A RESET...");
    emulator.reset();
    
    println!("\n📋 DESPUÉS DEL RESET:");
    println!("   PC resultado: 0x{:04X}", emulator.cpu.pc);
    println!("   BIOS presente: {}", emulator.cpu.bios_present);
    
    // Verificar vector nuevamente después del reset
    let reset_hi_after = emulator.cpu.test_read8(0xFFFC);
    let reset_lo_after = emulator.cpu.test_read8(0xFFFD);
    let reset_vector_after = ((reset_hi_after as u16) << 8) | (reset_lo_after as u16);
    
    println!("   Vector bytes después: 0xFFFC=0x{:02X}, 0xFFFD=0x{:02X}", reset_hi_after, reset_lo_after);
    println!("   Vector calculado después: 0x{:04X}", reset_vector_after);
    
    // Verificar que la primera instrucción en PC es válida
    let first_opcode = emulator.cpu.test_read8(emulator.cpu.pc);
    println!("   Primera instrucción: 0x{:02X}", first_opcode);
    
    // El test debe fallar si PC no está en el rango BIOS
    if emulator.cpu.pc < 0xE000 {
        println!("❌ FALLA: PC debería estar en rango BIOS (>= 0xE000) después del reset");
        println!("   PC actual: 0x{:04X}", emulator.cpu.pc);
        println!("   Esto significa que no está ejecutando BIOS primero");
        
        assert!(false, "Reset debería configurar PC en rango BIOS");
    } else {
        println!("✅ ÉXITO: PC está en rango BIOS después del reset");
        println!("   Ahora debería ejecutar inicialización BIOS antes de saltar a cartucho");
    }
}