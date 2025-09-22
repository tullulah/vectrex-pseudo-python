use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_skip_bios_delays() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Usar el método correcto para cargar BIOS
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar reset vector desde BIOS
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;

    println!("🚀 Iniciando test con skip de delays de BIOS");
    
    let mut max_steps = 50000;
    let mut step_count = 0;
    let mut last_pc = 0;
    let mut pc_count = 0;
    let mut last_segment_count = 0;
    
    // Detectar loops de delay conocidos
    let delay_loops = vec![
        (0xF4EB, 0xF4EF), // Loop principal de delay: LDA #$81; NOP; DECB; BNE
        (0xF33D, 0xF33F), // Loop BITB $D00D; BEQ
        (0xF341, 0xF342), // Loop DECA; BNE
        (0xF57A, 0xF57B), // Loop DECB; BPL  
    ];
    
    while step_count < max_steps {
        let pc = cpu.pc;
        
        // Detectar si estamos en un loop de delay
        let mut in_delay_loop = false;
        for &(start, end) in &delay_loops {
            if pc >= start && pc <= end {
                in_delay_loop = true;
                
                // Verificar si realmente es un loop (mismo PC repetido)
                if pc == last_pc {
                    pc_count += 1;
                    if pc_count > 10 { // Después de 10 iteraciones, saltar
                        println!("🔧 SKIP DELAY: PC=0x{:04X}, saltando loop de delay", pc);
                        
                        // Simular que el loop terminó
                        match pc {
                            0xF4EB..=0xF4EF => {
                                // Setear B = 0 para terminar el loop DECB; BNE
                                cpu.b = 0;
                                cpu.step(); // Ejecutar la instrucción BNE que ahora fallará
                            },
                            0xF33D..=0xF33F => {
                                // Simular que el bit está listo
                                cpu.step(); // Continuar
                            },
                            0xF341..=0xF342 => {
                                // Setear A = 0 para terminar el loop DECA; BNE
                                cpu.a = 0;
                                cpu.step();
                            },
                            0xF57A..=0xF57B => {
                                // Setear B = 0x80 para hacer BPL falso
                                cpu.b = 0x80;
                                cpu.step();
                            },
                            _ => {
                                cpu.step();
                            }
                        }
                        pc_count = 0;
                        step_count += 1;
                        continue;
                    }
                } else {
                    pc_count = 0;
                }
                break;
            }
        }
        
        if !in_delay_loop {
            pc_count = 0;
        }
        
        last_pc = pc;
        cpu.step();
        step_count += 1;
        
        // Verificar progreso de segmentos cada 5000 pasos
        if step_count % 5000 == 0 {
            let segments = &cpu.integrator.segments;
            let frames = cpu.cycle_frame;
            
            println!("📊 Step {}: PC=0x{:04X}, Segments={}, Frames={}", 
                     step_count, pc, segments.len(), frames);
            
            // Si tenemos segmentos nuevos, mostrar info
            if segments.len() > last_segment_count {
                for (i, segment) in segments.iter().enumerate() {
                    if i >= last_segment_count {
                        println!("  📐 Nuevo segmento {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                                i, segment.x0, segment.y0, 
                                segment.x1, segment.y1, segment.intensity);
                    }
                }
                last_segment_count = segments.len();
            }
            
            // Si tenemos suficientes segmentos, el test es exitoso
            if segments.len() >= 10 {
                println!("✅ ¡Suficientes segmentos generados! Terminando test temprano");
                break;
            }
        }
        
        // Timeout de seguridad
        if step_count >= max_steps {
            println!("⏰ Timeout alcanzado en {} pasos", max_steps);
            break;
        }
    }
    
    let final_segments = &cpu.integrator.segments;
    let final_frames = cpu.cycle_frame;
    let final_pc = cpu.pc;
    
    println!("\n📈 Resumen final:");
    println!("Total pasos: {}", step_count);
    println!("PC final: 0x{:04X}", final_pc);
    println!("Frames: {}", final_frames);
    println!("Segmentos totales: {}", final_segments.len());
    
    for (i, segment) in final_segments.iter().enumerate() {
        println!("  Segmento {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                i, segment.x0, segment.y0, 
                segment.x1, segment.y1, segment.intensity);
    }
    
    // El test pasa si generamos múltiples segmentos
    assert!(final_segments.len() > 1, 
            "Debería generar múltiples segmentos, pero solo generó {}", 
            final_segments.len());
    
    println!("✅ Test completado exitosamente - Skip de delays funciona");
}