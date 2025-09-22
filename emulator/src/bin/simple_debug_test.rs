use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== TEST BIOS COMPLETO HASTA COPYRIGHT ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    println!("🚀 Iniciando ejecución hasta Wait_Recal region (Timer2 test)...");
    
    let mut step_count = 0;
    let max_steps = 50_000; // Límite para evitar loops infinitos
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Detectar eventos importantes
        match pc_before {
            0xF000 => println!("🔄 RESET vector en paso {}", step_count),
            0xF4EE => println!("📋 Init_OS completed en paso {}", step_count),
            0xF533 => println!("⚙️ Init_VIA completed en paso {}", step_count),
            0xF1A2 => println!("🕐 Set_Refresh en paso {}", step_count),
            0xF192 => println!("⏳ Wait_Recal start en paso {}", step_count),
            0xF1AF => println!("✅ Wait_Recal end (copyright check) en paso {}", step_count),
            _ => {}
        }
        
        // Mostrar progress cada 5000 pasos
        if step_count % 5000 == 0 {
            println!("🎯 Paso {}: PC={:04X}", step_count, cpu.pc);
        }
        
        // Salir si llegamos al loop TST en Wait_Recal
        if cpu.pc >= 0xF19E && cpu.pc <= 0xF1A0 {
            println!("🎯 Detectado loop TST en Wait_Recal (PC={:04X}) en paso {}", cpu.pc, step_count);
            
            // Ejecutar un número limitado de pasos más para verificar si Timer2 funciona
            let mut wait_steps = 0;
            let wait_limit = 20_000; // Límite para el wait
            
            while wait_steps < wait_limit && cpu.pc >= 0xF19E && cpu.pc <= 0xF1A0 {
                cpu.step();
                wait_steps += 1;
                step_count += 1;
                
                // Mostrar progress cada 1000 pasos durante el wait
                if wait_steps % 1000 == 0 {
                    let ifr = cpu.bus.via_ifr();
                    let ier = cpu.bus.via_ier();
                    println!("⏳ Wait paso {}: PC={:04X}, IFR={:02X}, IER={:02X}", 
                             wait_steps, cpu.pc, ifr, ier);
                }
            }
            
            if cpu.pc >= 0xF19E && cpu.pc <= 0xF1A0 {
                println!("❌ Timer2 no expiró después de {} wait steps", wait_limit);
            } else {
                println!("✅ Timer2 expiró, salió del loop en step {}", wait_steps);
            }
            
            break;
        }
    }
    
    // Mostrar estado del integrator si hay segmentos
    let segments_count = cpu.integrator.segments.len();
    println!("📊 Integrator tiene {} segmentos", segments_count);
    
    if segments_count > 0 {
        println!("📋 Primeros 5 segmentos:");
        for (i, seg) in cpu.integrator.segments.iter().take(5).enumerate() {
            println!("  {}. ({}, {}) → ({}, {}) intensidad={}", 
                     i+1, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
        }
    }
    
    if step_count >= max_steps {
        println!("❌ Test timeout después de {} pasos", max_steps);
        println!("❌ Timer2 no expiró en tiempo razonable");
        return;
    }
    
    // Verificar que Timer2 expiró
    let ifr = cpu.bus.via_ifr();
    println!("🎯 IFR final: {:02X}", ifr);
    
    if (ifr & 0x20) != 0 {
        println!("✅ Timer2 expiró correctamente (IFR bit 5 set)");
    } else {
        println!("❌ Timer2 no expiró (IFR bit 5 clear)");
    }
    
    // Mostrar estadísticas
    println!("📊 Pasos totales: {}", step_count);
    println!("📊 PC final: {:04X}", cpu.pc);
    println!("📊 Ciclos totales: {}", cpu.cycles);
    
    // Success si salimos del loop
    if cpu.pc < 0xF19E || cpu.pc > 0xF1A0 {
        println!("✅ SUCCESS: CPU salió del loop TST, Timer2 funcionando correctamente");
    } else {
        println!("❌ FAILURE: CPU todavía está en el loop TST");
    }
}