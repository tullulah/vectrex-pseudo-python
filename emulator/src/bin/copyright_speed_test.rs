use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== SPEED TEST COPYRIGHT (SIN TRAZAS) ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    println!("🚀 Ejecutando a velocidad completa...");
    
    let mut step_count = 0;
    let max_steps = 25_000_000; // ~15-20 segundos a 1.5MHz para dar tiempo suficiente al copyright timeout
    
    let mut last_report = 0;
    let report_interval = 2_500_000; // Reportar cada 2.5M pasos (cada ~1.7 segundos)
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Solo reportar puntos clave, NO trazas detalladas
        match pc_before {
            0xF50A => {
                println!("🎉 LLEGÓ A F50A (Timer1 config) en paso {}", step_count);
                println!("✅ COPYRIGHT TERMINÓ - PROGRESANDO AL JUEGO");
                break;
            }
            0xF354 => {
                println!("🔄 Reset0Ref en paso {}", step_count);
            }
            0xF084 => {
                println!("📦 Chequeo cartucho en paso {}", step_count);
            }
            0xE000 => {
                println!("🎮 MINESTORM START en paso {}", step_count);
                break;
            }
            _ => {}
        }
        
        // Reporte de progreso cada 1M pasos
        if step_count - last_report >= report_interval {
            println!("📊 Progreso: {} M pasos, PC={:04X}", step_count / 1_000_000, cpu.pc);
            last_report = step_count;
        }
    }
    
    if step_count >= max_steps {
        println!("⏰ Timeout después de {} pasos", max_steps);
        println!("📊 PC final: {:04X}", cpu.pc);
        
        // Si estamos en copyright, esto confirma que es perpetuo
        if cpu.pc >= 0xF4EB && cpu.pc <= 0xF500 {
            println!("❌ CONFIRMADO: Copyright es perpetuo - nunca sale");
        }
    }
    
    println!("📊 Pasos totales ejecutados: {}", step_count);
}