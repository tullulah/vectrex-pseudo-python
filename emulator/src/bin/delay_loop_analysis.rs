use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== ANÁLISIS DELAY LOOP F4EB ===");
    
    let bios_path = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    println!("🎯 Analizando delay loop F4EB específicamente...");
    
    let mut step_count = 0;
    let max_steps = 2_000_000; // Suficiente para TODA la secuencia de copyright
    
    let mut in_delay_loop = false;
    let mut delay_starts = Vec::new();
    let mut delay_ends = Vec::new();
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        let b_before = cpu.b; // Capturar valor de B antes de ejecutar
        cpu.step();
        step_count += 1;
        
        // Detectar entrada al delay loop F4EB
        if !in_delay_loop && pc_before == 0xF4EB {
            in_delay_loop = true;
            delay_starts.push((step_count, b_before));
            println!("🔍 ENTRADA delay loop F4EB: paso {}, B={} ({:02X})", step_count, b_before, b_before);
        }
        
        // Dentro del delay loop, monitorear B
        if in_delay_loop && pc_before == 0xF4EB {
            // Solo mostrar cada 50 decrementos para no saturar
            if b_before % 50 == 0 || b_before < 10 {
                println!("  📍 F4EB: B={} ({:02X}) paso {}", b_before, b_before, step_count);
            }
            
            // Detectar cuando B llega a 0
            if b_before == 0 {
                println!("  🎯 F4EB: B=0 detectado en paso {}", step_count);
            }
        }
        
        // Detectar salida del delay loop
        if in_delay_loop && pc_before != 0xF4EB && pc_before != 0xF4ED {
            in_delay_loop = false;
            delay_ends.push((step_count, pc_before));
            println!("🚪 SALIDA delay loop: paso {}, nuevo PC={:04X}", step_count, pc_before);
        }
        
        // Detectar puntos clave después del copyright
        match pc_before {
            0xF50A => {
                println!("🎉 LLEGÓ A F50A (fin copyright): paso {}", step_count);
                println!("✅ COPYRIGHT TERMINÓ CORRECTAMENTE");
                break;
            }
            0xF354 => println!("🔄 Reset0Ref llamado en paso {}", step_count),
            _ => {}
        }
        
        // Progress cada 20k pasos
        if step_count % 20000 == 0 {
            println!("📊 Paso {}: PC={:04X} B={:02X}", step_count, cpu.pc, cpu.b);
        }
    }
    
    if step_count >= max_steps {
        println!("⏰ Timeout después de {} pasos", max_steps);
    }
    
    println!("\n=== RESUMEN DELAY LOOPS ===");
    println!("📊 Delay loops iniciados: {}", delay_starts.len());
    println!("📊 Delay loops terminados: {}", delay_ends.len());
    
    if delay_starts.len() > 0 {
        println!("🔍 Primeros 5 delay loops:");
        for (i, &(paso, b_val)) in delay_starts.iter().take(5).enumerate() {
            println!("  {}. Inicio: paso {}, B={}", i+1, paso, b_val);
        }
    }
    
    if delay_ends.len() > 0 {
        println!("🚪 Primeros 5 finales:");
        for (i, &(paso, pc)) in delay_ends.iter().take(5).enumerate() {
            println!("  {}. Final: paso {}, PC={:04X}", i+1, paso, pc);
        }
    }
    
    // Estado final
    println!("\n=== ESTADO FINAL ===");
    println!("📊 PC final: {:04X}", cpu.pc);
    println!("📊 B final: {:02X}", cpu.b);
    println!("📊 Pasos totales: {}", step_count);
}