use vectrex_emulator::Emulator;
use std::path::Path;

#[test]
fn test_bios_minestorm_jump_analysis() {
    let mut emulator = Emulator::new();
    
    // Cargar BIOS real
    let bios_path = Path::new(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin");
    if !bios_path.exists() {
        panic!("BIOS no encontrada en: {:?}", bios_path);
    }
    
    let bios_data = std::fs::read(bios_path).unwrap();
    let success = emulator.load_bios(&bios_data);
    if !success {
        panic!("No se pudo cargar la BIOS");
    }
    
    let debug_initial = emulator.debug_state();
    println!("=== ANÁLISIS DEL SALTO A MINE STORM ===");
    println!("BIOS cargada, PC inicial: 0x{:04X}", debug_initial.cpu_pc);
    
    // NO cargar ningún cartucho - dejar que la BIOS detecte que no hay cartucho
    // y debe saltar a Mine Storm interno
    
    let mut last_pc = debug_initial.cpu_pc;
    let mut pc_changes = 0;
    let mut step_count = 0;
    let mut significant_jumps = Vec::new();
    
    // Ejecutar por un tiempo limitado y buscar saltos significativos
    for step in 0..50000 {
        let debug_before = emulator.debug_state();
        let pc_before = debug_before.cpu_pc;
        
        emulator.step();
        step_count += 1;
        
        let debug_after = emulator.debug_state();
        let pc_after = debug_after.cpu_pc;
        
        // Registrar cambios de PC significativos
        if pc_after != last_pc {
            pc_changes += 1;
            last_pc = pc_after;
            
            // Si el salto es muy grande, puede ser el salto a Mine Storm
            let pc_diff = if pc_after > pc_before { 
                pc_after - pc_before 
            } else { 
                pc_before - pc_after 
            };
            
            if pc_diff > 0x1000 || pc_after < 0xE000 {
                significant_jumps.push((step, pc_before, pc_after, pc_diff));
                println!("Step {}: SALTO SIGNIFICATIVO 0x{:04X} → 0x{:04X} (diff: 0x{:04X})", 
                        step, pc_before, pc_after, pc_diff);
                        
                // Si salimos del rango de BIOS, ¡encontramos el salto!
                if pc_after < 0xE000 {
                    println!("¡ÉXITO! PC salió de BIOS en step {}: 0x{:04X}", step, pc_after);
                    println!("Rango de Mine Storm típico: 0x0000-0x7FFF");
                    break;
                }
            }
        }
        
        // Log periódico para ver progreso
        if step % 10000 == 0 && step > 0 {
            println!("Step {}: PC actual = 0x{:04X}, cambios de PC hasta ahora = {}", 
                    step, pc_after, pc_changes);
        }
        
        // Detectar bucles infinitos
        if step > 5000 {
            // Si llevamos muchos pasos y el PC no ha cambiado significativamente
            let debug_current = emulator.debug_state();
            if (debug_current.cpu_pc >= 0xF400 && debug_current.cpu_pc <= 0xF500) && 
               significant_jumps.is_empty() {
                println!("Posible bucle infinito detectado - PC se mantiene en rango 0xF4xx");
                println!("PC actual: 0x{:04X}", debug_current.cpu_pc);
                break;
            }
        }
    }
    
    let final_debug = emulator.debug_state();
    println!("\n=== RESUMEN FINAL ===");
    println!("Steps ejecutados: {}", step_count);
    println!("PC final: 0x{:04X}", final_debug.cpu_pc);
    println!("Cambios de PC totales: {}", pc_changes);
    println!("Saltos significativos encontrados: {}", significant_jumps.len());
    
    // Analizar si estamos todavía en BIOS
    if final_debug.cpu_pc >= 0xE000 {
        println!("❌ PROBLEMA: Todavía en BIOS después de {} pasos", step_count);
        println!("   La BIOS no está saltando a Mine Storm automáticamente");
        println!("   Esto explica por qué ves vectores diagonales (de la BIOS)");
    } else {
        println!("✅ ÉXITO: PC salió de BIOS a dirección 0x{:04X}", final_debug.cpu_pc);
        println!("   Probablemente en Mine Storm ahora");
    }
    
    if significant_jumps.is_empty() {
        println!("🔍 ANÁLISIS: No se detectaron saltos significativos");
        println!("   Esto sugiere que la BIOS está en un bucle de espera o demo");
        println!("   Posibles causas:");
        println!("   1. La BIOS espera una condición específica");
        println!("   2. La detección de cartucho no funciona correctamente");
        println!("   3. Falta inicialización de hardware específico");
    }
    
    println!("\n=== DIAGNÓSTICO ===");
    println!("Frames generados: {}", final_debug.total_frames);
    println!("Total de ciclos: {}", final_debug.total_cycles);
    println!("BIOS frame: {}", final_debug.bios_frame);
}