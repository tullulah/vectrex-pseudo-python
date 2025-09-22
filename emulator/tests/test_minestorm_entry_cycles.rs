use vectrex_emulator::emulator::Emulator;

/// Test de comparación: cuántos ciclos tarda nuestro emulador vs otros 
/// en llegar al código principal de Mine Storm
#[test]
fn test_minestorm_entry_timing() {
    let mut emulator = Emulator::new();
    emulator.reset(); // ¡CRÍTICO! Esto llama cpu.reset() que configura PC desde el vector de reset
    
    println!("🎮 MINE STORM ENTRY TIMING TEST");
    println!("🔍 Buscando cuándo el emulador llega al código principal de Mine Storm...");
    println!("🔧 PC inicial después de reset: 0x{:04X}", emulator.cpu.pc);
    
    // Direcciones sospechosas donde podría estar el código principal
    // Basado en cartuchos típicos de Vectrex, Mine Storm suele estar en rango 0x0000-0x7FFF
    let target_addresses = vec![
        0x0000, // Inicio típico de cartucho
        0x0010, // Después de vectores de interrupción
        0x0100, // Posible inicio de main loop
        0x0200,
        0x0500,
        0x1000, // Otro rango común
    ];
    
    let max_cycles = 100_000; // Límite para evitar loop infinito
    let mut found_targets = Vec::new();
    
    for step in 0..max_cycles {
        let pc_before = emulator.cpu.pc;
        
        // Verificar si llegamos a alguna dirección objetivo
        for &target in &target_addresses {
            if emulator.cpu.pc == target {
                found_targets.push((target, step, emulator.cpu.cycles));
                println!("🎯 LLEGÓ A 0x{:04X} en step={} cycles={}", target, step, emulator.cpu.cycles);
            }
        }
        
        // Verificar si está ejecutando fuera del rango BIOS (indica cartucho)
        if emulator.cpu.pc < 0xE000 {
            println!("🏁 PRIMERA VEZ FUERA DE BIOS: PC=0x{:04X} en step={} cycles={}", 
                     emulator.cpu.pc, step, emulator.cpu.cycles);
            break;
        }
        
        emulator.step();
        
        // Log periódico para ver progreso
        if step % 10_000 == 0 {
            println!("📊 Step {}: PC=0x{:04X} cycles={}", step, emulator.cpu.pc, emulator.cpu.cycles);
        }
        
        // Verificar si está en loop infinito
        if step > 50_000 && emulator.cpu.pc == pc_before {
            println!("⚠️  POSIBLE LOOP INFINITO: PC=0x{:04X} en step={}", emulator.cpu.pc, step);
            break;
        }
    }
    
    println!("\n📋 RESUMEN:");
    println!("   Targets encontrados: {}", found_targets.len());
    for (addr, step, cycles) in found_targets {
        println!("   • 0x{:04X}: step={}, cycles={}", addr, step, cycles);
    }
    
    // Este test NO debe fallar - solo reporta información
    assert!(true, "Test informativo completado");
}

/// Test que monitorea específicamente si llega a código de cartucho
#[test] 
fn test_cartridge_execution_entry() {
    let mut emulator = Emulator::new();
    emulator.reset(); // ¡CRÍTICO! Configura PC desde vector de reset
    
    println!("🎮 CARTRIDGE EXECUTION TEST");
    println!("🔧 PC inicial después de reset: 0x{:04X}", emulator.cpu.pc);
    
    let mut bios_exit_found = false;
    let max_steps = 200_000;
    
    for step in 0..max_steps {
        // BIOS está en 0xE000-0xFFFF, cartucho en 0x0000-0x7FFF
        if !bios_exit_found && emulator.cpu.pc < 0xE000 {
            println!("🚀 PRIMERA EJECUCIÓN DE CARTUCHO:");
            println!("   PC: 0x{:04X}", emulator.cpu.pc);
            println!("   Step: {}", step);
            println!("   Cycles: {}", emulator.cpu.cycles);
            println!("   Timer1: counter={}, enabled={}", emulator.cpu.timer1_counter, emulator.cpu.timer1_enabled);
            
            // Verificar stack para ver cómo llegamos aquí
            println!("   Stack pointer: 0x{:04X}", emulator.cpu.s);
            let mut stack_dump = String::new();
            for i in 0..8 {
                let addr = emulator.cpu.s.wrapping_add(i);
                let val = emulator.cpu.test_read8(addr);
                stack_dump.push_str(&format!(" {:02X}", val));
            }
            println!("   Stack content:{}", stack_dump);
            
            bios_exit_found = true;
            break;
        }
        
        if step % 25_000 == 0 {
            println!("📊 Step {}: PC=0x{:04X} (BIOS) cycles={}", step, emulator.cpu.pc, emulator.cpu.cycles);
        }
        
        emulator.step();
    }
    
    if !bios_exit_found {
        println!("❌ NO llegó a ejecutar código de cartucho en {} steps", max_steps);
        println!("   PC final: 0x{:04X}", emulator.cpu.pc);
        println!("   Cycles finales: {}", emulator.cpu.cycles);
        println!("   Timer1: counter={}, enabled={}", emulator.cpu.timer1_counter, emulator.cpu.timer1_enabled);
    }
    
    // El test falla si no llegamos a cartucho - indica problema serio
    assert!(bios_exit_found, "Debería llegar a código de cartucho después de inicialización BIOS");
}