//! Test directo de rutinas de vectores BIOS

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_direct_vector_routines() {
    println!("🔍 Test directo de rutinas de vectores BIOS");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Habilitar auto_drain y trace
    cpu.integrator_auto_drain = false; // NO auto drain para acumular segmentos
    cpu.trace = true;
    
    // Configurar stack pointer y DP manualmente
    cpu.s = 0xCBEA;
    cpu.dp = 0xC8;
    
    println!("=== TEST 1: Rutina Moveto_d_7F (0xF2FC) ===");
    
    // Preparar registros para Moveto_d_7F
    cpu.pc = 0xF2FC; // Moveto_d_7F
    cpu.a = 0x40;    // Coordenada X (ejemplo)
    cpu.b = 0x30;    // Coordenada Y (ejemplo)
    
    println!("PC: 0x{:04X}, A: 0x{:02X}, B: 0x{:02X}", cpu.pc, cpu.a, cpu.b);
    
    // Ejecutar rutina Moveto_d_7F
    let mut steps = 0;
    let max_routine_steps = 100;
    
    while steps < max_routine_steps && cpu.pc >= 0xF000 {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", steps + 1);
            break;
        }
        steps += 1;
        
        // Si volvemos al loop principal o salimos de BIOS, terminar
        if cpu.pc < 0xF000 || cpu.pc == 0xF4EB {
            break;
        }
    }
    
    println!("Moveto_d_7F ejecutado en {} pasos", steps);
    println!("Segments después de Moveto: {}", cpu.integrator.segments.len());
    
    if !cpu.integrator.segments.is_empty() {
        for (i, segment) in cpu.integrator.segments.iter().enumerate() {
            println!("  [{}] ({:6}, {:6}) -> ({:6}, {:6}), I={}",
                     i, segment.x0, segment.y0, segment.x1, segment.y1, segment.intensity);
        }
    }
    
    println!("\n=== TEST 2: Configurar DACs manualmente ===");
    
    // Reset CPU pero mantener configuración
    cpu.pc = 0xF000;
    cpu.integrator.segments.clear();
    
    // Escribir directamente en los DACs para simular movimiento de vector
    println!("Escribiendo en DACs X e Y...");
    
    // Escribir coordenada X en DAC (Port A del VIA)
    cpu.bus.write8(0xD001, 0x80); // X = centro (128)
    
    // Escribir coordenada Y en DAC (Port B del VIA) 
    cpu.bus.write8(0xD000, 0x80); // Y = centro (128)
    
    // Simular algunos movimientos
    for i in 0..5 {
        let x = 0x80 + (i * 10) as u8;
        let y = 0x80 + (i * 15) as u8;
        
        cpu.bus.write8(0xD001, x); // X DAC
        cpu.bus.write8(0xD000, y); // Y DAC
        
        // Simular algunos ciclos ejecutando NOPs
        for _ in 0..10 {
            cpu.step(); // Ejecutar algunos pasos para que el integrador procese
        }
        
        println!("Movimiento {}: X={}, Y={}, Segments={}", 
                 i + 1, x, y, cpu.integrator.segments.len());
    }
    
    println!("\n=== RESULTADO FINAL ===");
    println!("Total segments: {}", cpu.integrator.segments.len());
    
    if cpu.integrator.segments.len() > 0 {
        println!("✅ Segmentos generados!");
        for (i, segment) in cpu.integrator.segments.iter().enumerate() {
            println!("  [{}] ({:6.1}, {:6.1}) -> ({:6.1}, {:6.1}), I={}, F={}",
                     i, segment.x0, segment.y0, segment.x1, segment.y1, 
                     segment.intensity, segment.frame);
        }
    } else {
        println!("❌ No se generaron segmentos");
        
        // Debug info del integrador
        println!("Debug integrador:");
        println!("  Auto drain: {}", cpu.integrator_auto_drain);
        println!("  Current X: {}", cpu.current_x);
        println!("  Current Y: {}", cpu.current_y);
        println!("  Last intensity: {}", cpu.last_intensity);
    }
}