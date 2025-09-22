//! Test para ejecutar emulación durante 15 segundos reales y encontrar el copyright

use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn test_real_time_execution() {
    println!("🕒 Ejecutando emulación durante 15 segundos reales");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.integrator_auto_drain = false; // Para acumular segmentos
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    println!("Iniciando ejecución en tiempo real...");
    
    // Timer de 15 segundos reales
    let start_time = Instant::now();
    let target_duration = Duration::from_secs(15);
    
    let mut step_count = 0;
    let mut last_report_time = start_time;
    let mut print_str_calls = 0;
    let mut last_bios_calls = 0;
    
    // Buscar llamadas Print_Str específicas
    let print_str_addresses = [0xF373, 0xF378, 0xF37A];
    
    while start_time.elapsed() < target_duration {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar nuevas llamadas BIOS y buscar Print_Str
        if cpu.jsr_log_len > last_bios_calls {
            for i in last_bios_calls..cpu.jsr_log_len {
                if i < cpu.jsr_log.len() {
                    let call_addr = cpu.jsr_log[i];
                    
                    // Detectar llamadas Print_Str
                    if print_str_addresses.contains(&call_addr) {
                        print_str_calls += 1;
                        
                        // Extraer texto desde X register (apunta a la cadena)
                        let text_ptr = cpu.x;
                        let mut text = String::new();
                        for offset in 0..32 { // Leer hasta 32 chars
                            let char_addr = text_ptr.wrapping_add(offset);
                            let byte = cpu.bus.read8(char_addr);
                            if byte == 0x80 || byte == 0x00 { break; } // Terminadores típicos
                            if byte >= 0x20 && byte <= 0x7E { // ASCII imprimible
                                text.push(byte as char);
                            } else {
                                text.push('?');
                            }
                        }
                        
                        println!("🎯 PRINT_STR ENCONTRADO! Call {}: 0x{:04X}", i + 1, call_addr);
                        println!("   📄 Texto: \"{}\"", text);
                        println!("   📍 X register (ptr): 0x{:04X}", text_ptr);
                    }
                }
            }
            last_bios_calls = cpu.jsr_log_len;
        }
        
        // Reporte cada 2 segundos
        if last_report_time.elapsed() >= Duration::from_secs(2) {
            let elapsed = start_time.elapsed();
            let segments = cpu.integrator.segments.len();
            
            println!("⏱️  {:2.1}s: {} steps, PC=0x{:04X}, Segments={}, Frames={}, Print_Str={}", 
                     elapsed.as_secs_f32(), step_count, cpu.pc, segments, 
                     cpu.frame_count, print_str_calls);
            
            // Si estamos en el bucle de delay, mostrar estado
            if cpu.pc >= 0xF4EB && cpu.pc <= 0xF4EF {
                println!("     🔄 En delay loop: B=0x{:02X}", cpu.b);
            }
            
            last_report_time = Instant::now();
        }
        
        // Si encontramos Print_Str y tenemos segmentos, podemos parar antes
        if print_str_calls > 0 && cpu.integrator.segments.len() > 5 {
            println!("✅ ¡Copyright encontrado! Parando antes de los 15 segundos");
            break;
        }
    }
    
    let final_elapsed = start_time.elapsed();
    
    println!("\n=== RESULTADO FINAL ===");
    println!("Tiempo real ejecutado: {:.2} segundos", final_elapsed.as_secs_f32());
    println!("Steps totales: {}", step_count);
    println!("Steps per segundo: {:.0}", step_count as f32 / final_elapsed.as_secs_f32());
    println!("PC final: 0x{:04X}", cpu.pc);
    println!("Frames: {}", cpu.frame_count);
    println!("Segments finales: {}", cpu.integrator.segments.len());
    println!("Print_Str calls: {}", print_str_calls);
    
    if print_str_calls > 0 {
        println!("✅ ¡COPYRIGHT ENCONTRADO! La BIOS llegó a las rutinas de texto");
    } else {
        println!("❌ No se encontraron llamadas Print_Str en {} segundos", final_elapsed.as_secs());
    }
    
    // Mostrar las últimas llamadas BIOS para debug
    if cpu.jsr_log_len > 0 {
        println!("\nÚltimas 10 llamadas BIOS:");
        let start_idx = if cpu.jsr_log_len >= 10 { cpu.jsr_log_len - 10 } else { 0 };
        for i in start_idx..cpu.jsr_log_len {
            if i < cpu.jsr_log.len() {
                println!("  Call {}: 0x{:04X}", i + 1, cpu.jsr_log[i]);
            }
        }
    }
}