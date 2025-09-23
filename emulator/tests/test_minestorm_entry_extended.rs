use vectrex_emulator::emulator::Emulator;

#[test]
fn test_minestorm_entry_extended_comparison() {
    println!("=== COMPARACIÓN RUST vs JSVecx - ENTRADA A MINESTORM (EXTENDIDO) ===");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("No se pudo cargar BIOS");
    
    // Crear emulador Rust
    let mut system = Emulator::new();
    system.load_bios(&bios_data);
    
    println!("🦀 RUST EMULATOR:");
    println!("   Ejecutando hasta 500 millones de ciclos buscando entrada a Minestorm...");
    
    let max_steps = 500_000_000;
    let mut minestorm_entries = Vec::new();
    let mut print_str_calls = Vec::new();
    let mut final_step = 0;
    
    // Direcciones importantes de Minestorm en BIOS (confirmado en bios.asm línea 127)
    let minestorm_addresses = vec![
        0xE000, // Minestorm start address (confirmed in bios.asm line 127)
        0xE100, 0xE200, 0xE300, 0xE400, 0xE500, 0xE600, 0xE700, 0xE800, 0xE900, 0xEA00,
        0xEB00, 0xEC00, 0xED00, 0xEE00, 0xEF00, // Minestorm address range
    ];
    
    for step in 0..max_steps {
        final_step = step;
        let pc = system.cpu.pc;
        
        // Detectar entrada a Minestorm (0xE000-0xEFFF)
        if minestorm_addresses.contains(&(pc & 0xFF00)) && step > 100000 {
            if minestorm_entries.len() < 10 {
                minestorm_entries.push((step, pc));
                println!("🎯 ¡ENTRADA A MINESTORM! PC=0x{:04X} en step {}", pc, step);
            }
        }
        
        // Detectar llamadas a Print_Str (0xF373)
        if pc == 0xF373 {
            if print_str_calls.len() < 5 {
                print_str_calls.push((step, pc));
                println!("📝 Print_Str detectado en step {}", step);
            }
        }
        
        // Detectar si llegamos a 0x0000 (cartucho externo)
        if pc == 0x0000 && step > 1000 {
            println!("❌ RUST llegó a 0x0000 (cartucho externo) en step {}", step);
            break;
        }
        
        // Detectar entrada específica a Minestorm (0xE000)
        if pc == 0xE000 && step > 1000 {
            println!("🎮 ¡MINESTORM INICIADO! PC=0xE000 en step {}", step);
            // Ejecutar algunas instrucciones más para confirmar
            for i in 0..10 {
                system.step();
                let new_pc = system.cpu.pc;
                println!("   Step {}: PC=0x{:04X}", step + i + 1, new_pc);
            }
            final_step = step + 10;
            break;
        }
        
        // Progress reporting cada 25M pasos
        if step % 25_000_000 == 0 && step > 0 {
            println!("   Progress: {} millones de steps, PC actual: 0x{:04X}", step / 1_000_000, pc);
        }
        
        system.step();
        
        // Si encontramos evidencia clara de Minestorm, podemos parar antes
        if minestorm_entries.len() >= 5 && print_str_calls.len() >= 2 {
            println!("✅ RUST: Evidencia suficiente de actividad Minestorm encontrada");
            break;
        }
    }
    
    println!("📊 RESULTADOS RUST:");
    println!("   Entradas a Minestorm (0xE000-0xEFFF): {}", minestorm_entries.len());
    for (step, pc) in &minestorm_entries {
        println!("     Step {}: PC=0x{:04X}", step, pc);
    }
    println!("   Llamadas Print_Str: {}", print_str_calls.len());
    for (step, pc) in &print_str_calls {
        println!("     Step {}: PC=0x{:04X}", step, pc);
    }
    
    println!();
    println!("📝 INSTRUCCIONES PARA JAVASCRIPT:");
    println!("1. Ejecutar: node test_minestorm_entry_js.js");
    println!("2. Comparar timing de aparición de rutinas Minestorm");
    println!("3. Verificar si JavaScript también llega a 0xE000");
    
    println!();
    println!("🔬 ANÁLISIS TIMING VIA/CPU:");
    println!("   VIA status: Análisis disponible desde CPU");
    println!("   Total steps ejecutados: {} (máximo 500M)", final_step);
}