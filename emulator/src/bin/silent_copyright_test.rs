use vectrex_emulator::cpu6809::CPU;
use std::fs;
use std::time::Instant;

fn main() {
    println!("=== TEST SILENCIOSO COPYRIGHT ===");
    println!("🚀 Iniciando test sin trazas...");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    let start_time = Instant::now();
    let mut step_count = 0;
    let max_steps = 10_000_000; // 10M pasos - debería ser suficiente
    
    // SOLO reportar hitos importantes
    let mut f50a_reached = false;
    let mut reset0ref_count = 0;
    let mut minestorm_reached = false;
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Solo detectar hitos críticos
        match pc_before {
            0xF50A => {
                if !f50a_reached {
                    f50a_reached = true;
                    let elapsed = start_time.elapsed();
                    println!("🎉 F50A (fin copyright) alcanzado en paso {} - {:?}", step_count, elapsed);
                }
            }
            0xF354 => {
                reset0ref_count += 1;
                if reset0ref_count <= 3 {
                    let elapsed = start_time.elapsed();
                    println!("🔄 Reset0Ref #{} en paso {} - {:?}", reset0ref_count, step_count, elapsed);
                }
            }
            0xE000..=0xE7FF => {
                if !minestorm_reached {
                    minestorm_reached = true;
                    let elapsed = start_time.elapsed();
                    println!("🎮 MINESTORM alcanzado en {:04X} paso {} - {:?}", pc_before, step_count, elapsed);
                    break;
                }
            }
            _ => {}
        }
        
        // Progress silencioso cada millón
        if step_count % 1_000_000 == 0 {
            let elapsed = start_time.elapsed();
            println!("📊 {}M pasos - {:?} - PC={:04X}", step_count / 1_000_000, elapsed, cpu.pc);
        }
    }
    
    let total_time = start_time.elapsed();
    
    println!("\n=== RESULTADOS FINALES ===");
    println!("⏱️  Tiempo total: {:?}", total_time);
    println!("📊 Pasos totales: {}", step_count);
    println!("⚡ Velocidad: {:.0} pasos/segundo", step_count as f64 / total_time.as_secs_f64());
    println!("🎯 F50A alcanzado: {}", f50a_reached);
    println!("🔄 Reset0Ref count: {}", reset0ref_count);
    println!("🎮 Minestorm alcanzado: {}", minestorm_reached);
    println!("📍 PC final: {:04X}", cpu.pc);
    
    if step_count >= max_steps {
        println!("⏰ Timeout después de {} pasos", max_steps);
    }
    
    // Estimación de hardware real
    let hw_freq = 1_500_000.0; // ~1.5MHz 6809
    let hw_time = step_count as f64 / hw_freq;
    println!("🔧 Tiempo equivalente hardware real: {:.2} segundos", hw_time);
}