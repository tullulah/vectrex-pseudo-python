use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn debug_text_memory() {
    println!("🔍 Investigando contenido de memoria durante arranque");
    
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
    
    println!("🚀 Iniciando emulación con monitoreo de memoria...");
    
    let max_steps = 15_000;
    let target_duration = Duration::from_secs(3);
    let start_time = Instant::now();
    
    for step in 0..max_steps {
        if start_time.elapsed() >= target_duration {
            println!("⏰ Timeout alcanzado después de {} steps", step);
            break;
        }
        
        cpu.step();
        
        // Cada 1000 steps, verificar algunas direcciones de memoria clave
        if step % 1000 == 0 {
            println!("\n=== Step {} ===", step);
            println!("PC: 0x{:04X}", cpu.pc);
            
            // Verificar memoria en rangos típicos para strings
            println!("Memoria en 0xC800-0xC810:");
            for addr in 0xC800..=0xC810 {
                let val = cpu.bus.mem[addr];
                let chr = if val >= 0x20 && val <= 0x7E { 
                    format!("'{}'", val as char) 
                } else { 
                    "   ".to_string() 
                };
                println!("  [{:04X}]: 0x{:02X} {}", addr, val, chr);
            }
            
            // También verificar otras direcciones comunes para text buffers
            println!("Memoria en 0x0040-0x0050:");
            for addr in 0x0040..=0x0050 {
                let val = cpu.bus.mem[addr];
                let chr = if val >= 0x20 && val <= 0x7E { 
                    format!("'{}'", val as char) 
                } else { 
                    "   ".to_string() 
                };
                println!("  [{:04X}]: 0x{:02X} {}", addr, val, chr);
            }
            
            println!("Memoria en 0xCBE0-0xCBF0:");
            for addr in 0xCBE0..=0xCBF0 {
                let val = cpu.bus.mem[addr];
                let chr = if val >= 0x20 && val <= 0x7E { 
                    format!("'{}'", val as char) 
                } else { 
                    "   ".to_string() 
                };
                println!("  [{:04X}]: 0x{:02X} {}", addr, val, chr);
            }
        }
        
        // Detectar Print_Str calls
        if cpu.pc == 0xF373 || cpu.pc == 0xF378 || cpu.pc == 0xF37A {
            println!("\n🎯 Print_Str encontrado en step {}: 0x{:04X}", step, cpu.pc);
            println!("   X register: 0x{:04X}", cpu.x);
            
            // Leer memoria desde X register
            let text_addr = cpu.x as usize;
            println!("   Leyendo texto desde 0x{:04X}:", text_addr);
            
            let mut text = String::new();
            for i in 0..64 {
                let addr = text_addr + i;
                if addr >= 0x10000 { break; }
                let byte = cpu.bus.mem[addr];
                println!("     [{:04X}]: 0x{:02X} {}", addr, byte, 
                    if byte >= 0x20 && byte <= 0x7E { format!("'{}'", byte as char) } else { "   ".to_string() });
                if byte == 0 { break; }
                if byte >= 0x20 && byte <= 0x7E {
                    text.push(byte as char);
                }
            }
            println!("   📄 Texto extraído: \"{}\"", text);
            
            // Continuar un poco más para ver qué pasa
            println!("   🔄 Continuando 500 steps más...");
            for extra in 0..500 {
                cpu.step();
                if (extra + 1) % 100 == 0 {
                    println!("     Extra step {}: PC=0x{:04X}", extra + 1, cpu.pc);
                }
            }
            break;
        }
    }
    
    println!("\n✅ Debug de memoria completo");
}