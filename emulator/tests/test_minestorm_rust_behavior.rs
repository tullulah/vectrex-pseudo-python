use std::fs;
use vectrex_emulator::Emulator;

#[test]
fn test_minestorm_rust_vs_expected() {
    println!("🎮 MINE STORM RUST EMULATOR TEST");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to read BIOS file");
    println!("📁 BIOS cargada: {} bytes", bios_data.len());
    
    // Crear emulador y cargar BIOS (esto hace reset automático ahora)
    let mut emulator = Emulator::new();
    let result = emulator.load_bios(&bios_data);
    assert!(result, "BIOS debe cargar correctamente");
    
    println!("🔧 PC después de cargar BIOS: 0x{:04X}", emulator.cpu.pc);
    assert!(emulator.cpu.pc >= 0xE000, "PC debe estar en BIOS");
    
    // Cargar Mine Storm (el cartucho integrado en la BIOS)
    // Mine Storm está incluido en la BIOS, no necesitamos cargar cartucho externo
    
    println!("\n🚀 EJECUTANDO EMULADOR...");
    println!("🔍 Buscando cuándo sale de BIOS y va a Mine Storm...");
    
    let max_steps = 50_000; // Límite para encontrar el jump
    let mut bios_exit_found = false;
    let mut steps_to_exit = 0;
    
    for step in 0..max_steps {
        let pc_before = emulator.cpu.pc;
        
        // Ejecutar un paso
        emulator.step();
        
        let pc_after = emulator.cpu.pc;
        
        // Detectar si salió de BIOS (PC < 0xE000)
        if pc_before >= 0xE000 && pc_after < 0xE000 {
            bios_exit_found = true;
            steps_to_exit = step + 1;
            println!("🎯 SALIÓ DE BIOS!");
            println!("   Step: {}", steps_to_exit);
            println!("   Cycles: {}", emulator.cpu.cycles);
            println!("   PC antes: 0x{:04X}", pc_before);
            println!("   PC después: 0x{:04X}", pc_after);
            println!("   Timer1: counter={}, enabled={}", 
                     emulator.cpu.timer1_counter, emulator.cpu.timer1_enabled);
            break;
        }
        
        // Log periódico
        if step % 10_000 == 0 {
            println!("📊 Step {}: PC=0x{:04X}, cycles={}, timer1={}", 
                     step, emulator.cpu.pc, emulator.cpu.cycles, emulator.cpu.timer1_counter);
        }
    }
    
    if bios_exit_found {
        println!("\n✅ RUST: Sale de BIOS después de {} steps", steps_to_exit);
        println!("🎮 Ahora debería estar ejecutando Mine Storm");
        
        // Ejecutar unos pasos más para ver qué hace Mine Storm
        println!("\n🔍 PRIMEROS PASOS EN MINE STORM:");
        for i in 0..10 {
            let pc = emulator.cpu.pc;
            emulator.step();
            println!("   Step {}: PC=0x{:04X}", i, pc);
        }
    } else {
        println!("\n❌ RUST: NO salió de BIOS en {} steps", max_steps);
        println!("   PC final: 0x{:04X}", emulator.cpu.pc);
        println!("   Cycles: {}", emulator.cpu.cycles);
    }
}