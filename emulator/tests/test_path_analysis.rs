use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn create_cpu() -> CPU {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path)
        .expect("No se pudo cargar la BIOS. Verificar ruta.");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y resetear
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.reset();
    
    cpu
}

#[test]
fn test_path_analysis_rust_vs_js() {
    let mut cpu = create_cpu();
    let max_instructions = 50_000;
    let mut instruction_count = 0;
    let _transitions: Vec<(u16, u16)> = Vec::new();
    let _last_pc = cpu.pc;
    
    println!("=== ANÁLISIS DE RUTAS: Rust vs JavaScript ===");
    println!("PC inicial: {:04X}", cpu.pc);
    
    // Puntos clave para monitorear
    let key_points = [
        0xF19E, // Loop BITA/BEQ (que JS ejecuta 23,769 veces)
        0xF4EB, // Loop DECB/BNE (que JS ejecuta 6,012 veces)
        0xF373, // Print_Str (meta)
        0xF194, // Punto previo a F19E
        0xF4E9, // Punto previo a F4EB
    ];
    
    let mut point_counts = std::collections::HashMap::new();
    for &point in &key_points {
        point_counts.insert(point, 0);
    }
    
    loop {
        let pc_before = cpu.pc;
        
        // Ejecutar instrucción
        cpu.step();
        instruction_count += 1;
        
        let pc_after = cpu.pc;
        
        // Detectar transiciones importantes
        if pc_before != pc_after {
            // Registrar visitas a puntos clave
            if key_points.contains(&pc_after) {
                *point_counts.get_mut(&pc_after).unwrap() += 1;
                
                // Logs especiales para puntos críticos
                match pc_after {
                    0xF19E => {
                        if point_counts[&0xF19E] <= 3 || point_counts[&0xF19E] % 5000 == 0 {
                            println!("📍 F19E visita #{}: desde {:04X} (BITA loop)", 
                                     point_counts[&0xF19E], pc_before);
                        }
                    },
                    0xF4EB => {
                        println!("🚨 F4EB visita #{}: desde {:04X} (DECB loop) - B={:02X}", 
                                 point_counts[&0xF4EB], pc_before, cpu.b);
                    },
                    0xF373 => {
                        println!("🎯 F373 alcanzado: Print_Str en instrucción {}", instruction_count);
                        break;
                    },
                    _ => {}
                }
            }
            
            // Detectar transiciones especiales que JavaScript reporta
            match (pc_before, pc_after) {
                (0xF4E9, 0xF4EB) => {
                    println!("🔄 Transición F4E9->F4EB (como JavaScript reporta)");
                },
                (0xF1A0, 0xF19E) => {
                    if point_counts[&0xF19E] <= 3 || point_counts[&0xF19E] % 5000 == 0 {
                        println!("🔄 Transición F1A0->F19E (loop BITA)");
                    }
                },
                _ => {}
            }
            
            // last_pc = pc_before; // Eliminado
        }
        
        // Límites de seguridad
        if instruction_count >= max_instructions {
            println!("⚠️ Límite de instrucciones alcanzado");
            break;
        }
    }
    
    println!("\n=== RESUMEN DE VISITAS ===");
    for (&pc, &count) in &point_counts {
        match pc {
            0xF19E => println!("F19E (BITA loop): {} veces", count),
            0xF4EB => println!("F4EB (DECB loop): {} veces", count),
            0xF373 => println!("F373 (Print_Str): {} veces", count),
            0xF194 => println!("F194 (pre-F19E): {} veces", count),
            0xF4E9 => println!("F4E9 (pre-F4EB): {} veces", count),
            _ => println!("{:04X}: {} veces", pc, count),
        }
    }
    
    println!("\n=== COMPARACIÓN CON JAVASCRIPT ===");
    println!("JavaScript reportó:");
    println!("  F4EB: 6,012 veces");
    println!("  F19E: 23,769 veces");
    println!("Rust encontró:");
    println!("  F4EB: {} veces", point_counts[&0xF4EB]);
    println!("  F19E: {} veces", point_counts[&0xF19E]);
    
    if point_counts[&0xF4EB] == 0 {
        println!("🔍 RUST NUNCA ENTRA A F4EB - Diferencia clave detectada!");
    }
    
    println!("Total instrucciones: {}", instruction_count);
}