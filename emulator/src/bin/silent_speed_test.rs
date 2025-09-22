use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== SPEED TEST REAL (COMENTANDO TRAZAS) ===");
    println!("ATENCIÓN: Este test requiere comentar manualmente las trazas en:");
    println!("- bus.rs línea 104");
    println!("- cpu6809.rs líneas 283, 295, 304, 326, 748, 754, 769, 773, 778, 847");
    println!("- via6522.rs línea 74");
    println!();
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    println!("🚀 Ejecutando a velocidad MÁXIMA...");
    
    let mut step_count = 0;
    let max_steps = 50_000_000; // 50M pasos - velocidad real
    
    let mut last_report = 0;
    let report_interval = 5_000_000; // Reportar cada 5M pasos
    
    let start_time = std::time::Instant::now();
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Solo puntos MUY específicos
        match pc_before {
            0xF50A => {
                println!("🎉 LLEGÓ A F50A (Timer1 config) en paso {} - COPYRIGHT TERMINÓ!", step_count);
                break;
            }
            0xF354 => {
                if step_count % 100000 == 0 { // Solo cada 100k para no spam
                    println!("🔄 Reset0Ref en paso {}", step_count);
                }
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
        
        // Reporte de progreso cada 5M pasos
        if step_count - last_report >= report_interval {
            let elapsed = start_time.elapsed();
            println!("📊 {} M pasos en {:.2}s, PC={:04X} (velocidad: {:.0} steps/sec)", 
                step_count / 1_000_000, 
                elapsed.as_secs_f64(),
                cpu.pc,
                step_count as f64 / elapsed.as_secs_f64()
            );
            last_report = step_count;
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("\n=== RESULTADO FINAL ===");
    println!("📊 Pasos ejecutados: {}", step_count);
    println!("⏱️ Tiempo total: {:.2}s", elapsed.as_secs_f64());
    println!("🚀 Velocidad: {:.0} steps/sec", step_count as f64 / elapsed.as_secs_f64());
    println!("📊 PC final: {:04X}", cpu.pc);
    
    if step_count >= max_steps {
        println!("⏰ Timeout - copyright aún ejecutándose");
        println!("❌ CONFIRMADO: Copyright es perpetuo a velocidad real también");
    }
}