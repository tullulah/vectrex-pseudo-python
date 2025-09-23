// Test para detectar bucles enganchados después del fix Timer2
use vectrex_emulator::emulator::Emulator;
use std::collections::HashMap;

#[test]
fn test_detect_stuck_loops_post_timer2() {
    println!("=== DETECCIÓN DE BUCLES ENGANCHADOS POST-TIMER2 ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    let mut pc_histogram: HashMap<u16, u32> = HashMap::new();
    let mut last_progress_step = 0;
    let max_steps = 100_000; // Límite de seguridad
    
    println!("🔍 Ejecutando {} steps y detectando bucles...", max_steps);
    
    for step in 0..max_steps {
        let pc = emulator.cpu.pc;
        
        // Contar frecuencia de cada PC
        *pc_histogram.entry(pc).or_insert(0) += 1;
        
        // Detectar si estamos en un bucle (mismo PC ejecutado muchas veces)
        if let Some(&count) = pc_histogram.get(&pc) {
            if count > 1000 {
                println!("🚨 BUCLE DETECTADO en step {}: PC=0x{:04X} ejecutado {} veces", step, pc, count);
                
                // Mostrar contexto del bucle
                println!("📍 Estado del CPU:");
                println!("   PC: 0x{:04X}", emulator.cpu.pc);
                println!("   A: 0x{:02X}, B: 0x{:02X}", emulator.cpu.a, emulator.cpu.b);
                println!("   X: 0x{:04X}, Y: 0x{:04X}", emulator.cpu.x, emulator.cpu.y);
                println!("   Flags: Z={} N={} C={} V={}", emulator.cpu.cc_z, emulator.cpu.cc_n, emulator.cpu.cc_c, emulator.cpu.cc_v);
                
                // Estado VIA
                println!("🎯 Estado VIA:");
                println!("   IFR: 0x{:02X}", emulator.cpu.bus.via_ifr());
                println!("   IER: 0x{:02X}", emulator.cpu.bus.via_ier());
                
                // Mostrar los PCs más frecuentes
                println!("📊 Top 10 PCs más ejecutados:");
                let mut sorted_pcs: Vec<_> = pc_histogram.iter().collect();
                sorted_pcs.sort_by(|a, b| b.1.cmp(a.1));
                for (i, (pc, count)) in sorted_pcs.iter().take(10).enumerate() {
                    println!("   {}. PC=0x{:04X}: {} veces", i+1, pc, count);
                }
                
                panic!("Bucle infinito detectado en PC=0x{:04X}", pc);
            }
        }
        
        // Mostrar progreso cada 10k steps
        if step % 10_000 == 0 && step > 0 {
            println!("⏱️  Step {}: PC=0x{:04X}", step, pc);
            last_progress_step = step;
        }
        
        // Detectar progreso hacia Minestorm (asumiendo que está en ROM alta)
        if pc >= 0x0000 && pc < 0x8000 {
            println!("🎯 POTENCIAL SALTO A MINESTORM detectado en step {}: PC=0x{:04X}", step, pc);
            break;
        }
        
        emulator.step();
    }
    
    println!("✅ Test completado. Último step: {}", last_progress_step);
}