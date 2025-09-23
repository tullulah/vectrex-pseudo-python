// Comparación JavaScript vs Rust en el bucle F4EB
use vectrex_emulator::emulator::Emulator;

#[test]
fn test_f4eb_loop_js_vs_rust_comparison() {
    println!("=== COMPARACIÓN JS vs RUST - BUCLE F4EB ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    // Ejecutar hasta llegar a F4EB
    let mut step_count = 0;
    while emulator.cpu.pc != 0xF4EB && step_count < 10000 {
        emulator.step();
        step_count += 1;
    }
    
    if emulator.cpu.pc != 0xF4EB {
        panic!("No se alcanzó F4EB en {} steps", step_count);
    }
    
    println!("🎯 RUST EMULATOR - Estado en F4EB:");
    println!("   PC: 0x{:04X}", emulator.cpu.pc);
    println!("   DP: 0x{:02X}", emulator.cpu.dp);
    println!("   A: 0x{:02X}, B: 0x{:02X}", emulator.cpu.a, emulator.cpu.b);
    println!("   X: 0x{:04X}, Y: 0x{:04X}", emulator.cpu.x, emulator.cpu.y);
    println!("   IFR: 0x{:02X}, IER: 0x{:02X}", emulator.cpu.bus.via_ifr(), emulator.cpu.bus.via_ier());
    println!("   Steps para llegar: {}", step_count);
    
    // Verificar qué está leyendo el bucle
    let address_5a = 0xD000u16 | 0x5A; // DP=0xD0, así que <$5A = 0xD05A
    let value_at_5a = emulator.cpu.bus.read8(address_5a);
    println!("   Valor en 0x{:04X}: 0x{:02X}", address_5a, value_at_5a);
    
    // Verificar registros VIA específicos
    println!("   Registros VIA:");
    for reg in 0x00..=0x0F {
        let addr = 0xD000 + reg;
        let val = emulator.cpu.bus.read8(addr);
        println!("     0xD{:03X} (reg {:02X}): 0x{:02X}", addr, reg, val);
    }
    
    // Analizar el bucle por unas iteraciones
    println!("🔄 RUST - Comportamiento del bucle:");
    let mut iterations = 0;
    let max_iterations = 20;
    
    while iterations < max_iterations {
        let pc_before = emulator.cpu.pc;
        
        emulator.step();
        
        let pc_after = emulator.cpu.pc;
        let a_after = emulator.cpu.a;
        
        if pc_before == 0xF4EB {
            println!("   Iter {}: LDA #$81 → A=0x{:02X}", iterations, a_after);
        } else if pc_before == 0xF4ED {
            // STX <$5A - verificar que está escribiendo
            let x_val = emulator.cpu.x;
            let addr = 0xD000u16 | 0x5A;
            let mem_val = emulator.cpu.bus.read8(addr);
            println!("   Iter {}: STX <$5A → X=0x{:04X} escribió en 0x{:04X}=0x{:02X}", 
                     iterations, x_val, addr, mem_val);
        } else if pc_before == 0xF4EF {
            // BNE - verificar si salta o no
            if pc_after == 0xF4EB {
                println!("   Iter {}: BNE → SALTA a F4EB (bucle continúa)", iterations);
            } else {
                println!("   Iter {}: BNE → NO SALTA, PC=0x{:04X} (bucle roto)", iterations, pc_after);
                break;
            }
        }
        
        iterations += 1;
        
        if pc_after == 0xF4EB && iterations > 3 {
            println!("   ⚠️  RUST: Bucle confirmado después de {} iteraciones", iterations);
            break;
        }
    }
    
    println!("\n📝 INSTRUCCIONES PARA VERIFICAR EN JAVASCRIPT:");
    println!("1. Abrir emulador JavaScript en el navegador");
    println!("2. Cargar la misma BIOS");
    println!("3. Ejecutar hasta PC=0xF4EB");
    println!("4. Verificar:");
    println!("   - Estado de registros CPU (A, B, X, Y, DP)");
    println!("   - Estado VIA (IFR, IER)");
    println!("   - Valor en dirección 0xD05A");
    println!("   - Si el bucle se atasca o progresa");
    println!("5. Comparar con nuestros resultados RUST arriba");
    
    println!("\n🔧 PUNTOS CLAVE A VERIFICAR:");
    println!("   - ¿JavaScript también se atasca en F4EB?");
    println!("   - ¿JavaScript lee 0xFF desde 0xD05A?");
    println!("   - ¿El Timer2 ya expiró en JavaScript (IFR bit 5)?");
    println!("   - ¿Cuántos steps toma llegar a F4EB en JavaScript?");
    
    println!("\n💡 HIPÓTESIS:");
    println!("   Si JavaScript TAMBIÉN se atasca → Comportamiento esperado");
    println!("   Si JavaScript progresa → Bug en nuestra emulación Rust");
}