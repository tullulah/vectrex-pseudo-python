//! Test para investigar específicamente qué pasa con los vectores de Print_Str

use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn debug_print_str_vectors() {
    println!("🔍 Investigando vectores generados por Print_Str");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.integrator_auto_drain = false; // Para acumular segmentos
    cpu.integrator.set_record_blank_slews(true); // Registrar también slews en blanco
    cpu.integrator.set_merge(false); // No merge para ver vectores individuales
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Ejecutar hasta encontrar Print_Str
    let start_time = Instant::now();
    let target_duration = Duration::from_secs(5); // Solo 5 segundos para llegar rápido
    
    let mut step_count = 0;
    let mut last_bios_calls = 0;
    let mut print_str_found = false;
    let mut segments_before_print_str = 0;
    
    let print_str_addresses = [0xF373, 0xF378, 0xF37A];
    
    while start_time.elapsed() < target_duration && !print_str_found {
        let success = cpu.step();
        if !success {
            println!("❌ CPU se detuvo en step {}", step_count + 1);
            break;
        }
        
        step_count += 1;
        
        // Verificar nuevas llamadas BIOS
        if cpu.jsr_log_len > last_bios_calls {
            for i in last_bios_calls..cpu.jsr_log_len {
                if i < cpu.jsr_log.len() {
                    let call_addr = cpu.jsr_log[i];
                    
                    if print_str_addresses.contains(&call_addr) {
                        segments_before_print_str = cpu.integrator.segments.len();
                        
                        println!("🎯 Print_Str encontrado en step {}: 0x{:04X}", step_count, call_addr);
                        println!("   Segmentos antes: {}", segments_before_print_str);
                        println!("   X register: 0x{:04X}", cpu.x);
                        println!("   A register: 0x{:02X}", cpu.a);
                        println!("   B register: 0x{:02X}", cpu.b);
                        println!("   X register: 0x{:04X}", cpu.x);
                        println!("   U register: 0x{:04X}", cpu.u);
                        
                        // Leer el texto desde U (no X!) - Print_Str_d recibe el puntero en U
                        let text_ptr = cpu.u;
                        let mut text = String::new();
                        println!("   Leyendo texto desde U=0x{:04X}:", text_ptr);
                        for offset in 0..16 {
                            let char_addr = text_ptr.wrapping_add(offset);
                            let byte = cpu.bus.read8(char_addr);
                            print!("     [{:04X}]: 0x{:02X}", char_addr, byte);
                            if byte == 0x80 || byte == 0x00 {
                                println!(" (terminador)");
                                break;
                            }
                            if byte >= 0x20 && byte <= 0x7E {
                                text.push(byte as char);
                                println!(" '{}'", byte as char);
                            } else {
                                text.push('?');
                                println!(" (no ASCII)");
                            }
                        }
                        
                        println!("   📄 Texto extraído: \"{}\"", text);
                        print_str_found = true;
                        break;
                    }
                }
            }
            last_bios_calls = cpu.jsr_log_len;
        }
    }
    
    if print_str_found {
        println!("   📋 Estado DDR inicial después de Print_Str:");
        println!("      DDR A (0xD003) = 0x{:02X}", cpu.ddr_a);
        println!("      DDR B (0xD002) = 0x{:02X}", cpu.ddr_b);
        println!("      Port A = 0x{:02X}", cpu.port_a_value);
        println!("      Port B = 0x{:02X}", cpu.port_b_value);
        
        println!("\n🕒 Ejecutando 20000 steps más después de Print_Str para capturar vectores...");
        
        let mut last_pc = 0;
        let mut via_write_count = cpu.via_writes.len();
        for extra_step in 0..20000 {
            let success = cpu.step();
            if !success {
                println!("❌ CPU se detuvo en extra step {}", extra_step);
                break;
            }
            
            step_count += 1;
            
            // Contar escrituras al VIA (indican generación de vectores)
            let current_via_writes = cpu.via_writes.len();
            if current_via_writes > via_write_count {
                // Analizar las nuevas escrituras VIA
                for write_idx in via_write_count..current_via_writes {
                    if write_idx < cpu.via_writes.len() {
                        let via_write = &cpu.via_writes[write_idx];
                        let address = via_write.addr;
                        let value = via_write.val;
                        
                        // Identificar tipos de escritura VIA
                        let via_type = match address {
                            0xD000 => "PORTB (X DAC)",  // Port B controla X DAC
                            0xD001 => "PORTA (Y DAC)",  // Port A controla Y DAC 
                            0xD002 => "DDRA",
                            0xD003 => "DDRB",
                            0xD004 => "T1C_L",
                            0xD005 => "T1C_H",
                            0xD006 => "T1L_L", 
                            0xD007 => "T1L_H",
                            0xD008 => "T2C_L",
                            0xD009 => "T2C_H",
                            0xD00A => "SR",
                            0xD00B => "ACR",
                            0xD00C => "PCR",
                            0xD00D => "IFR",
                            0xD00E => "IER",
                            0xD00F => "PORTA_NH",
                            _ => "OTHER",
                        };
                        
                        // Solo reportar DAC y otros registros importantes
                        if address == 0xD000 || address == 0xD001 || address == 0xD002 || address == 0xD003 || address == 0xD00A || extra_step % 500 == 0 {
                            let ddr_info = match address {
                                0xD002 => format!(" DDR_B=0x{:02X}", cpu.ddr_b),
                                0xD003 => format!(" DDR_A=0x{:02X}", cpu.ddr_a),
                                _ => String::new(),
                            };
                            println!("   Step {}: VIA write 0x{:04X} <- 0x{:02X} ({}){}", 
                                extra_step, address, value, via_type, ddr_info);
                        }
                    }
                }
                via_write_count = current_via_writes;
            }
            
            // Detectar llamadas a Reset0Ref (que borra vectores)
            if cpu.pc == 0xF354 {
                let current_segments = cpu.integrator.segments.len();
                println!("   🚫 Reset0Ref detectado en step {}, segmentos antes: {}", extra_step, current_segments);
            }
            if extra_step % 1000 == 0 {
                let current_segments = cpu.integrator.segments.len();
                let pc = cpu.pc;
                println!("   Extra step {}: Segments = {} (+{}), PC = 0x{:04X}, VIA writes = {}", 
                         extra_step, current_segments, 
                         current_segments.saturating_sub(segments_before_print_str), pc, current_via_writes);
                
                // Detectar si el PC está estancado (posible loop infinito)
                if extra_step > 0 && pc == last_pc && extra_step % 5000 == 0 {
                    println!("   ⚠️  PC sin cambios en 0x{:04X} - posible loop infinito o delay", pc);
                }
                last_pc = pc;
            }
        }
        
        let final_segments = cpu.integrator.segments.len();
        let new_segments = final_segments.saturating_sub(segments_before_print_str);
        
        println!("\n=== ANÁLISIS DE VECTORES ===");
        println!("Segmentos antes de Print_Str: {}", segments_before_print_str);
        println!("Segmentos después: {}", final_segments);
        println!("Nuevos segmentos generados: {}", new_segments);
        
        if new_segments > 0 {
            println!("✅ Print_Str SÍ generó {} nuevos segmentos", new_segments);
            
            // Analizar los últimos segmentos
            if final_segments > 0 {
                println!("\nÚltimos 3 segmentos generados:");
                let start_idx = if final_segments >= 3 { final_segments - 3 } else { 0 };
                for i in start_idx..final_segments {
                    if i < cpu.integrator.segments.len() {
                        let seg = &cpu.integrator.segments[i];
                        println!("  Segment {}: ({}, {}) -> ({}, {}) intensidad={}", 
                                 i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
                    }
                }
            }
        } else {
            println!("❌ Print_Str NO generó nuevos segmentos");
            
            // Verificar si hay actividad en el integrator
            println!("\nDiagnóstico del integrator:");
            println!("  Auto drain: {}", cpu.integrator_auto_drain);
            let velocity = cpu.integrator.velocity();
            let origin = cpu.integrator.origin();
            println!("  Velocity: ({}, {})", velocity.0, velocity.1);
            println!("  Origin: ({}, {})", origin.0, origin.1);
        }
        
    } else {
        println!("❌ No se encontró Print_Str en {} segundos", target_duration.as_secs());
    }
    
    println!("\nSteps totales: {}", step_count);
    println!("PC final: 0x{:04X}", cpu.pc);
}