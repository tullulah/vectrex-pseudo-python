//! Test para diagnosticar qué llamadas BIOS se están haciendo y por qué no generan vectores

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_bios_calls_detail() {
    println!("🔍 Diagnóstico detallado de llamadas BIOS");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    // Habilitar auto_drain y trace
    cpu.integrator_auto_drain = true;
    cpu.trace = true;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Ejecutar algunos pasos y capturar las primeras llamadas BIOS
    let mut step_count = 0;
    let max_initial_steps = 1000;
    let mut last_bios_calls = 0;
    
    println!("\n--- Primeras llamadas BIOS ---");
    
    while step_count < max_initial_steps {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar si hay nuevas llamadas BIOS
        if cpu.jsr_log_len > last_bios_calls {
            for i in last_bios_calls..cpu.jsr_log_len {
                if i < cpu.jsr_log.len() {
                    let call_addr = cpu.jsr_log[i];
                    println!("BIOS Call {}: 0x{:04X}", i + 1, call_addr);
                }
            }
            last_bios_calls = cpu.jsr_log_len;
        }
        
        // Parar cuando tengamos suficientes llamadas para análisis
        if cpu.jsr_log_len >= 20 {
            break;
        }
    }
    
    println!("\n--- Estado después de {} pasos ---", step_count);
    println!("PC: 0x{:04X}", cpu.pc);
    println!("Total BIOS calls: {}", cpu.jsr_log_len);
    println!("Frames: {}", cpu.frame_count);
    println!("Segments: {}", cpu.integrator.segments.len());
    
    // Ahora ejecutar muchos más pasos para llegar a las rutinas de dibujo
    println!("\n--- Ejecutando hasta rutinas de dibujo ---");
    
    let max_total_steps = 50000;
    let mut last_segment_check = cpu.integrator.segments.len();
    
    while step_count < max_total_steps {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar segmentos cada 5k pasos
        if step_count % 5000 == 0 {
            let current_segments = cpu.integrator.segments.len();
            println!("Step {}: PC=0x{:04X}, Segments={} (+{}), Frames={}", 
                     step_count, cpu.pc, current_segments, 
                     current_segments - last_segment_check, cpu.frame_count);
            last_segment_check = current_segments;
            
            if current_segments > 0 {
                println!("✅ ¡Segmentos encontrados!");
                break;
            }
        }
    }
    
    println!("\n--- Resultado final ---");
    println!("Steps totales: {}", step_count);
    println!("Segments finales: {}", cpu.integrator.segments.len());
    println!("PC final: 0x{:04X}", cpu.pc);
    
    if cpu.integrator.segments.is_empty() {
        println!("❌ No se generaron segmentos después de {} pasos", step_count);
        
        // Mostrar las últimas llamadas BIOS para debug
        println!("\nÚltimas llamadas BIOS:");
        let start_idx = if cpu.jsr_log_len >= 10 { cpu.jsr_log_len - 10 } else { 0 };
        for i in start_idx..cpu.jsr_log_len {
            if i < cpu.jsr_log.len() {
                println!("  Call {}: 0x{:04X}", i + 1, cpu.jsr_log[i]);
            }
        }
    } else {
        println!("✅ Segmentos generados exitosamente");
    }
}