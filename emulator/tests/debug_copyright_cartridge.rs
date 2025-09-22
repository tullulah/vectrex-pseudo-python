use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn debug_copyright_cartridge() {
    println!("🔍 Investigando copyright de cartucho vs BIOS");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    println!("📁 BIOS cargada: {} bytes", bios_data.len());
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.integrator_auto_drain = false;
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    println!("🚀 Iniciando emulación sin cartucho...");
    
    let max_steps = 15_000;
    let target_duration = Duration::from_secs(3);
    let start_time = Instant::now();
    
    // Rastrear la secuencia antes de Print_Str
    let mut pre_print_str_trace = Vec::new();
    let mut found_print_str = false;
    
    for step in 0..max_steps {
        if start_time.elapsed() >= target_duration {
            println!("⏰ Timeout alcanzado después de {} steps", step);
            break;
        }
        
        let pc_before = cpu.pc;
        cpu.step();
        let pc_after = cpu.pc;
        
        // Rastrear las últimas 20 instrucciones antes de Print_Str
        if !found_print_str {
            pre_print_str_trace.push((step, pc_before, pc_after));
            if pre_print_str_trace.len() > 20 {
                pre_print_str_trace.remove(0);
            }
        }
        
        // Detectar escrituras en el rango de texto
        if step % 1000 == 0 {
            println!("Step {}: PC=0x{:04X}, Area texto 0xC800: 0x{:02X}", 
                step, cpu.pc, cpu.bus.mem[0xC800]);
        }
        
        // Detectar Print_Str calls
        if cpu.pc == 0xF373 || cpu.pc == 0xF378 || cpu.pc == 0xF37A {
            found_print_str = true;
            println!("\n🎯 Print_Str encontrado en step {}: 0x{:04X}", step, cpu.pc);
            println!("   X register: 0x{:04X}", cpu.x);
            
            // Mostrar las últimas instrucciones que llevaron a Print_Str
            println!("\n📋 Últimas 10 instrucciones antes de Print_Str:");
            for (i, (s, pc_from, pc_to)) in pre_print_str_trace.iter().rev().take(10).enumerate() {
                println!("  -{}: Step {} PC: 0x{:04X} -> 0x{:04X}", 
                    10-i, s, pc_from, pc_to);
            }
            
            // Investigar el área donde se supone que está el copyright
            println!("\n📄 Análisis del área de copyright:");
            
            // Buscar en rangos típicos de copyright de cartridge
            let copyright_ranges = [
                (0x0000, 0x0020, "Inicio cartucho"),
                (0x0020, 0x0080, "Header cartucho"), 
                (0xC800, 0xC820, "Buffer texto actual"),
                (0xCBE0, 0xCC00, "Area stack/buffer"),
            ];
            
            for (start, end, desc) in copyright_ranges {
                println!("  {}: 0x{:04X}-0x{:04X}", desc, start, end-1);
                for addr in start..end {
                    let val = cpu.bus.mem[addr];
                    if val != 0 {
                        let chr = if val >= 0x20 && val <= 0x7E { 
                            format!("'{}'", val as char) 
                        } else { 
                            "   ".to_string() 
                        };
                        println!("    [{:04X}]: 0x{:02X} {}", addr, val, chr);
                    }
                }
            }
            
            // Buscar strings que parezcan copyright en toda la RAM
            println!("\n🔎 Buscando strings tipo copyright en RAM...");
            for start_addr in (0x0000..0xC000).step_by(1) {
                let mut potential_string = String::new();
                let mut has_copyright_chars = false;
                
                for i in 0..32 {
                    let addr = start_addr + i;
                    if addr >= 0xC000 { break; }
                    
                    let byte = cpu.bus.mem[addr];
                    if byte == 0 { break; }
                    if byte < 0x20 || byte > 0x7E { break; }
                    
                    let ch = byte as char;
                    potential_string.push(ch);
                    
                    // Buscar caracteres típicos de copyright
                    if ch == '©' || ch == '(' || ch == ')' || 
                       potential_string.to_uppercase().contains("GCE") ||
                       potential_string.to_uppercase().contains("COPYRIGHT") {
                        has_copyright_chars = true;
                    }
                }
                
                if potential_string.len() >= 6 && has_copyright_chars {
                    println!("  Posible copyright en 0x{:04X}: \"{}\"", start_addr, potential_string);
                }
            }
            
            break;
        }
    }
    
    if !found_print_str {
        println!("❌ No se encontró Print_Str en el tiempo límite");
        println!("🔍 Estado final:");
        println!("   PC: 0x{:04X}", cpu.pc);
        println!("   Área 0xC800: 0x{:02X}", cpu.bus.mem[0xC800]);
    }
    
    println!("\n✅ Debug de copyright completo");
}