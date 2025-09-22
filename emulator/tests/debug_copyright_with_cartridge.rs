use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::{Duration, Instant};

#[test]
fn debug_copyright_with_cartridge() {
    println!("🔍 Investigando copyright con cartucho cargado (hello.bin)");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    println!("📁 BIOS cargada: {} bytes", bios_data.len());
    
    // Cargar cartucho
    let cart_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\examples\hello.bin";
    let cart_data = fs::read(cart_path).expect("Failed to load cartridge");
    println!("🎮 Cartucho cargado: {} bytes", cart_data.len());
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y cartucho
    cpu.bus.load_bios_image(&bios_data);
    cpu.load_bin(&cart_data, 0x0000); // Cargar cartucho en 0x0000
    cpu.bios_present = true;
    cpu.integrator_auto_drain = false;
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Verificar copyright en el header del cartucho
    println!("\n📄 Análisis del header del cartucho:");
    println!("Header típico Vectrex en 0x0000-0x007F:");
    for addr in 0x0000..0x0080 {
        let val = cpu.bus.mem[addr];
        if val != 0 {
            let chr = if val >= 0x20 && val <= 0x7E { 
                format!("'{}'", val as char) 
            } else { 
                "   ".to_string() 
            };
            println!("  [{:04X}]: 0x{:02X} {}", addr, val, chr);
        }
    }
    
    // Buscar strings que parezcan copyright en el cartucho
    println!("\n🔎 Buscando strings en el cartucho:");
    for start_addr in (0x0000..0x1000).step_by(1) {
        let mut potential_string = String::new();
        
        for i in 0..64 {
            let addr = start_addr + i;
            if addr >= 0x1000 { break; }
            
            let byte = cpu.bus.mem[addr];
            if byte == 0 { break; }
            if byte < 0x20 || byte > 0x7E { break; }
            
            potential_string.push(byte as char);
        }
        
        if potential_string.len() >= 8 {
            println!("  String en 0x{:04X}: \"{}\"", start_addr, potential_string);
        }
    }
    
    println!("\n🚀 Iniciando emulación con cartucho...");
    
    let max_steps = 15_000;
    let target_duration = Duration::from_secs(5);
    let start_time = Instant::now();
    
    for step in 0..max_steps {
        if start_time.elapsed() >= target_duration {
            println!("⏰ Timeout alcanzado después de {} steps", step);
            break;
        }
        
        cpu.step();
        
        // Monitorear cambios en el área de texto
        if step % 1000 == 0 {
            println!("Step {}: PC=0x{:04X}, Area texto 0xC800: 0x{:02X}", 
                step, cpu.pc, cpu.bus.mem[0xC800]);
        }
        
        // Detectar Print_Str calls
        if cpu.pc == 0xF373 || cpu.pc == 0xF378 || cpu.pc == 0xF37A {
            println!("\n🎯 Print_Str encontrado en step {}: 0x{:04X}", step, cpu.pc);
            println!("   X register: 0x{:04X}", cpu.x);
            
            // Leer texto desde X register
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
            
            // Comprobar si ahora hay vectores
            println!("\n🎨 Verificando vectores generados:");
            let segments_before = cpu.integrator.segments.len();
            
            // Continuar 2000 steps más para capturar vectores
            for extra in 0..2000 {
                cpu.step();
                if (extra + 1) % 500 == 0 {
                    let segments_now = cpu.integrator.segments.len();
                    println!("   Extra step {}: Segments = {} (+{})", 
                        extra + 1, segments_now, segments_now - segments_before);
                }
            }
            
            let final_segments = cpu.integrator.segments.len();
            let new_segments = final_segments - segments_before;
            
            if new_segments > 0 {
                println!("✅ Print_Str generó {} nuevos segmentos", new_segments);
                
                // Mostrar los últimos segmentos
                println!("\nÚltimos 5 segmentos generados:");
                let start_idx = if final_segments >= 5 { final_segments - 5 } else { 0 };
                for i in start_idx..final_segments {
                    if i < cpu.integrator.segments.len() {
                        let seg = &cpu.integrator.segments[i];
                        println!("  Segment {}: ({:.1}, {:.1}) -> ({:.1}, {:.1}) intensidad={}", 
                                 i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
                    }
                }
            } else {
                println!("❌ Print_Str NO generó nuevos segmentos");
            }
            
            break;
        }
    }
    
    println!("\n✅ Debug de copyright con cartucho completo");
}