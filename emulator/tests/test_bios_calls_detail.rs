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
    
    // 🔧 CAMBIO CRUCIAL: Desactivar auto_drain para permitir acumulación de segmentos
    cpu.integrator_auto_drain = false;
    cpu.trace = true;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Ejecutar algunos pasos y capturar las primeras llamadas BIOS
    let mut step_count = 0;
    let max_initial_steps = 1000;
    let mut last_bios_calls = 0;
    let mut print_str_calls = 0;
    
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
                    
                    // Detectar llamadas a Print_Str
                    if call_addr == 0xF373 || call_addr == 0xF378 || call_addr == 0xF37A {
                        print_str_calls += 1;
                        println!("🎯 BIOS Call {}: 0x{:04X} (Print_Str variant {})", i + 1, call_addr, print_str_calls);
                        
                        // Intentar capturar el texto desde la memoria
                        let text_ptr = cpu.x; // X suele apuntar al texto
                        let mut text = String::new();
                        for offset in 0..32 { // Leer hasta 32 chars
                            let byte = cpu.bus.read8(text_ptr + offset);
                            if byte == 0x80 || byte == 0x00 { break; } // Terminador
                            if byte >= 0x20 && byte < 0x7F {
                                text.push(byte as char);
                            } else {
                                text.push_str(&format!("\\x{:02X}", byte));
                            }
                        }
                        println!("   📄 Texto: \"{}\"", text);
                    } else {
                        println!("BIOS Call {}: 0x{:04X}", i + 1, call_addr);
                    }
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
    
    // Buscar llamadas específicas a Print_Str y capturar texto
    println!("\n--- Ejecutando hasta rutinas de dibujo y texto ---");
    
    let max_total_steps = 2500000; // Delay real de Vectrex antes del copyright
    let mut last_segment_check = cpu.integrator.segments.len();
    let mut button_simulated = false;

    while step_count < max_total_steps {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Simular entrada de botón después de algunos frames para continuar (solo una vez)
        if cpu.frame_count == 50 && !button_simulated {
            println!("⚡ Simulando presión de botón en frame 50");
            // Simular botón presionado (bit 0)
            cpu.input_state.buttons = 0x01;
            button_simulated = true;
        }        // Verificar segmentos cada 100k pasos (menos spam)
        if step_count % 100000 == 0 {
            let current_segments = cpu.integrator.segments.len();
            let segment_diff = if current_segments >= last_segment_check { 
                current_segments - last_segment_check 
            } else { 
                0 
            };
            println!("Step {}: PC=0x{:04X}, Segments={} (+{}), Frames={}, Print_Str calls={}", 
                     step_count, cpu.pc, current_segments, 
                     segment_diff, cpu.frame_count, print_str_calls);
            last_segment_check = current_segments;
            
            // Si estamos en el bucle de delay conocido, mostrar registro B
            if cpu.pc >= 0xF4EB && cpu.pc <= 0xF4EF {
                println!("  🔄 DELAY LOOP detectado: B=0x{:02X}, A=0x{:02X}", cpu.b, cpu.a);
            }
            
            // Continuar hasta encontrar texto o límite
            if print_str_calls > 0 && current_segments > 10 {
                println!("✅ ¡Texto y segmentos encontrados!");
                break;
            }
        }
    }
    
    println!("\n--- Resultado final ---");
    println!("Steps totales: {}", step_count);
    println!("Segments finales: {}", cpu.integrator.segments.len());
    println!("Print_Str calls: {}", print_str_calls);
    println!("PC final: 0x{:04X}", cpu.pc);
    
    if print_str_calls == 0 {
        println!("❌ No se encontraron llamadas a Print_Str después de {} pasos", step_count);
    } else {
        println!("✅ Se encontraron {} llamadas a Print_Str", print_str_calls);
    }
    
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