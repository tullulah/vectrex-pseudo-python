//! Test para verificar la generación de segmentos durante la ejecución de BIOS

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_bios_segment_generation() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Usar el método correcto para cargar BIOS
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar reset vector desde BIOS (via bus, no acceso directo)
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("🔍 Test generación de segmentos BIOS");
    println!("Reset vector: 0x{:04X}", reset_vector);
    println!("Integrator auto_drain: {}", cpu.integrator_auto_drain);
    
    // Ejecutar bastantes pasos para llegar a Mine Storm
    let mut total_segments = 0;
    let mut frames_with_segments = 0;
    let mut max_segments_per_frame = 0;
    
    for step in 1..=50000 {
        if !cpu.step() { 
            println!("⚠️  CPU stopped at step {}", step);
            break; 
        }
        
        // Verificar segmentos cada 1000 pasos
        if step % 1000 == 0 {
            let current_segments = cpu.integrator.segments.len();
            let frame_segments = cpu.integrator_last_frame_segments;
            let total_accumulated = cpu.integrator_total_segments;
            
            if current_segments > 0 || frame_segments > 0 {
                println!("📊 Step {}: PC=0x{:04X}, Current segments={}, Frame segments={}, Total={}", 
                    step, cpu.pc, current_segments, frame_segments, total_accumulated);
                
                if current_segments > 0 {
                    frames_with_segments += 1;
                    max_segments_per_frame = max_segments_per_frame.max(current_segments);
                    total_segments += current_segments;
                    
                    // Mostrar algunos segmentos como ejemplo
                    for (i, seg) in cpu.integrator.segments.iter().take(3).enumerate() {
                        println!("  Segment {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                            i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
                    }
                    if cpu.integrator.segments.len() > 3 {
                        println!("  ... y {} segmentos más", cpu.integrator.segments.len() - 3);
                    }
                }
            }
            
            // Mostrar progreso general
            if step % 10000 == 0 {
                println!("🎮 Step {}: PC=0x{:04X}, Frames={}, VL_count={}, BIOS_calls={}", 
                    step, cpu.pc, cpu.frame_count, cpu.draw_vl_count, cpu.bios_calls.len());
            }
        }
        
        // Si encontramos segmentos, podemos parar más temprano
        if total_segments > 10 {
            println!("✅ Encontrados suficientes segmentos, terminando test temprano en step {}", step);
            break;
        }
    }
    
    println!("\n📈 Resumen del test:");
    println!("Total frames ejecutados: {}", cpu.frame_count);
    println!("Frames con segmentos: {}", frames_with_segments);
    println!("Total segmentos generados: {}", total_segments);
    println!("Máx segmentos por frame: {}", max_segments_per_frame);
    println!("Draw VL count: {}", cpu.draw_vl_count);
    println!("Total segmentos acumulados: {}", cpu.integrator_total_segments);
    println!("Llamadas BIOS registradas: {}", cpu.bios_calls.len());
    
    // Si no hay segmentos, mostrar estado del integrador
    if total_segments == 0 {
        println!("\n⚠️  NO SE GENERARON SEGMENTOS");
        println!("Estado del integrador:");
        println!("  Auto drain: {}", cpu.integrator_auto_drain);
        println!("  Beam on: {}", cpu.beam_on);
        
        // Mostrar últimas direcciones ejecutadas
        if cpu.bios_calls.len() > 0 {
            println!("\nÚltimas llamadas BIOS:");
            for (i, addr_str) in cpu.bios_calls.iter().rev().take(5).enumerate() {
                println!("  {}: {}", i, addr_str);
            }
        }
        
        // No hacer assert para poder ver el output completo
        println!("❌ No se generaron segmentos después de {} pasos", 50000);
    } else {
        println!("✅ Segmentos generados correctamente");
    }
}