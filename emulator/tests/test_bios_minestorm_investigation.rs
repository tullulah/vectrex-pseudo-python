use std::fs;
use vectrex_emulator::Emulator;

#[test]
fn test_bios_minestorm_investigation() {
    println!("🕵️ INVESTIGACIÓN: ¿Por qué BIOS no salta a Mine Storm?");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to read BIOS file");
    println!("📁 BIOS cargada: {} bytes", bios_data.len());
    
    // Crear emulador y cargar BIOS
    let mut emulator = Emulator::new();
    let result = emulator.load_bios(&bios_data);
    assert!(result, "BIOS debe cargar correctamente");
    
    println!("🔧 PC inicial: 0x{:04X}", emulator.cpu.pc);
    
    // Ejecutar por más tiempo y monitorear diferentes aspectos
    let max_steps = 200_000; // Más pasos para dar tiempo
    let mut key_events = Vec::new();
    let mut timer_events = Vec::new();
    let mut last_pc_pattern = Vec::new();
    
    for step in 0..max_steps {
        let pc_before = emulator.cpu.pc;
        let timer1_before = emulator.cpu.timer1_counter;
        let frame_count_before = emulator.cpu.frame_count;
        
        emulator.step();
        
        let pc_after = emulator.cpu.pc;
        let timer1_after = emulator.cpu.timer1_counter;
        let frame_count_after = emulator.cpu.frame_count;
        
        // Detectar cambios importantes
        if frame_count_after != frame_count_before {
            key_events.push(format!("Step {}: FRAME_COUNT cambió de {} a {}", 
                           step, frame_count_before, frame_count_after));
        }
        
        // Timer1 expiry
        if timer1_before > 0 && timer1_after == 0 {
            timer_events.push(format!("Step {}: Timer1 expiró en PC=0x{:04X}", step, pc_before));
        }
        
        // Salto fuera de BIOS
        if pc_before >= 0xE000 && pc_after < 0xE000 {
            println!("🎯 ¡SALIÓ DE BIOS A MINE STORM!");
            println!("   Step: {}", step);
            println!("   PC antes: 0x{:04X}", pc_before);
            println!("   PC después: 0x{:04X}", pc_after);
            println!("   Frame count: {}", frame_count_after);
            break;
        }
        
        // Trackear últimos PCs para detectar loops
        last_pc_pattern.push(pc_after);
        if last_pc_pattern.len() > 10 {
            last_pc_pattern.remove(0);
        }
        
        // Logs periódicos
        if step % 50_000 == 0 {
            println!("📊 Step {}: PC=0x{:04X}, Timer1={}, Frame={}", 
                     step, pc_after, timer1_after, frame_count_after);
            
            // Mostrar patrón de PCs recientes
            println!("   Últimos PCs: {:04X?}", 
                     last_pc_pattern.iter().map(|pc| format!("{:04X}", pc)).collect::<Vec<_>>());
        }
        
        // Detección de loop infinito
        if step > 10_000 && last_pc_pattern.len() == 10 {
            let unique_pcs: std::collections::HashSet<_> = last_pc_pattern.iter().collect();
            if unique_pcs.len() <= 3 {
                println!("⚠️  Detectado posible loop infinito en step {}", step);
                println!("   PCs repetidos: {:04X?}", 
                         last_pc_pattern.iter().map(|pc| format!("{:04X}", pc)).collect::<Vec<_>>());
                
                // Si hay timer events, tal vez está esperando algo
                if !timer_events.is_empty() {
                    println!("🕐 Timer events detectados:");
                    for event in &timer_events {
                        println!("   {}", event);
                    }
                }
                
                break;
            }
        }
    }
    
    println!("\n📋 RESUMEN:");
    println!("   PC final: 0x{:04X}", emulator.cpu.pc);
    println!("   Frame count final: {}", emulator.cpu.frame_count);
    println!("   Timer1 final: {}", emulator.cpu.timer1_counter);
    
    println!("\n🔑 Key events: {}", key_events.len());
    for event in key_events {
        println!("   {}", event);
    }
    
    println!("\n🕐 Timer events: {}", timer_events.len());
    for event in timer_events {
        println!("   {}", event);
    }
    
    // Verificar si la BIOS está esperando input del usuario
    println!("\n🎮 HIPÓTESIS: ¿BIOS esperando input de usuario?");
    println!("   En Vectrex real, tal vez se necesita presionar botón para ir a Mine Storm");
}