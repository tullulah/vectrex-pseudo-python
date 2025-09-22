//! Test que demuestra la solución al problema de delay loops

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_skip_delay_loops() {
    println!("🔍 Test con skip de delay loops");
    
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
    
    // Habilitar auto_drain
    cpu.integrator_auto_drain = true;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Ejecutar con skip inteligente de delay loops
    let mut step_count = 0;
    let max_steps = 50000;
    let mut delay_skips = 0;
    
    while step_count < max_steps {
        // Detectar el delay loop específico en F4EB-F4EF
        if cpu.pc == 0xF4EB {
            // Skip del delay loop: poner B=0 para que el loop termine inmediatamente
            cpu.b = 0;
            delay_skips += 1;
            if delay_skips % 100 == 0 {
                println!("Delay skip #{}: PC=0x{:04X}, B ahora={}", delay_skips, cpu.pc, cpu.b);
            }
        }
        
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar segmentos cada 5k pasos
        if step_count % 5000 == 0 {
            let current_segments = cpu.integrator.segments.len();
            println!("Step {}: PC=0x{:04X}, Segments={}, Frames={}, Delays skipped={}",
                     step_count, cpu.pc, current_segments, cpu.frame_count, delay_skips);
            
            if current_segments > 0 {
                println!("✅ ¡Segmentos encontrados!");
                
                // Mostrar algunos segmentos
                for (i, segment) in cpu.integrator.segments.iter().take(3).enumerate() {
                    println!("  Segment {}: ({:6}, {:6}) -> ({:6}, {:6}), intensity={}",
                             i, segment.x0, segment.y0, segment.x1, segment.y1, segment.intensity);
                }
                break;
            }
        }
    }
    
    println!("\n--- Resultado final ---");
    println!("Steps totales: {}", step_count);
    println!("Delay loops skipped: {}", delay_skips);
    println!("Segments finales: {}", cpu.integrator.segments.len());
    println!("Frames: {}", cpu.frame_count);
    println!("PC final: 0x{:04X}", cpu.pc);
    
    if cpu.integrator.segments.len() > 0 {
        println!("✅ SOLUCIÓN CONFIRMADA: Skipping delay loops permite generar segmentos");
        
        // Mostrar todos los segmentos encontrados
        println!("\nTodos los segmentos:");
        for (i, segment) in cpu.integrator.segments.iter().enumerate() {
            println!("  [{}] ({:6}, {:6}) -> ({:6}, {:6}), I={}, F={}",
                     i, segment.x0, segment.y0, segment.x1, segment.y1, 
                     segment.intensity, segment.frame);
        }
    } else {
        println!("❌ Aún no se generaron segmentos - necesario más trabajo");
    }
}