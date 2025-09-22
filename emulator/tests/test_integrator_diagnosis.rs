//! Test específico para diagnosticar por qué no se generan segmentos

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_integrator_segments_diagnosis() {
    println!("🔍 Diagnóstico del integrador y segmentos");
    
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
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    println!("Auto drain inicial: {}", cpu.integrator_auto_drain);
    
    // Habilitar auto_drain que parece ser necesario
    cpu.integrator_auto_drain = true;
    println!("Auto drain después: {}", cpu.integrator_auto_drain);
    
    // Ejecutar hasta que se generen algunos segmentos o hasta límite
    let mut total_segments = 0;
    let max_steps = 100000; // Más pasos para llegar a rutinas de dibujo
    let mut step_count = 0;
    
    while step_count < max_steps {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar segmentos cada 10k pasos
        if step_count % 10000 == 0 {
            let segments = &cpu.integrator.segments;
            let frame_count = cpu.frame_count;
            let bios_calls = cpu.jsr_log_len;
            
            println!("🎮 Step {}: PC=0x{:04X}, Frames={}, Segments={}, BIOS_calls={}",
                     step_count, cpu.pc, frame_count, segments.len(), bios_calls);
            
            total_segments += segments.len();
            
            // Si encontramos segmentos, mostrar detalles
            if !segments.is_empty() {
                println!("✅ ¡Segmentos encontrados! Total: {}", segments.len());
                for (i, segment) in segments.iter().take(5).enumerate() {
                    println!("  Segment {}: start=({:6}, {:6}), end=({:6}, {:6}), intensity={}",
                             i, segment.x0, segment.y0, 
                             segment.x1, segment.y1, segment.intensity);
                }
                break;
            }
        }
    }
    
    // Verificar estado final del integrador
    println!("\n📊 Estado final del integrador:");
    println!("Total segments encontrados: {}", total_segments);
    println!("Auto drain: {}", cpu.integrator_auto_drain);
    
    // Obtener un último drain de segmentos
    let final_segments = &cpu.integrator.segments;
    println!("Final segments: {}", final_segments.len());
    
    if total_segments > 0 || !final_segments.is_empty() {
        println!("✅ Segmentos generados exitosamente");
    } else {
        println!("❌ No se generaron segmentos");
        
        // Información adicional para debug
        println!("\nInfo adicional:");
        println!("Frames totales: {}", cpu.frame_count);
        println!("BIOS calls: {}", cpu.jsr_log_len);
        println!("PC final: 0x{:04X}", cpu.pc);
    }
}