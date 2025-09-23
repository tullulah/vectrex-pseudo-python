use vectrex_emulator::*;

#[test]
fn test_timer2_wait_recal_debug() {
    let bios_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Cannot read BIOS file");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios_data);
    
    println!("=== DEBUGGING TIMER2 IN WAIT_RECAL ===");
    
    // Ejecutar hasta justo antes de Set_Refresh (F1A2)
    let mut step = 0;
    loop {
        cpu.step();
        step += 1;
        
        // Detectar cuando llegamos a Set_Refresh
        if cpu.pc == 0xF1A2 {
            println!("✓ Reached Set_Refresh at step {}", step);
            break;
        }
        
        if step > 10000 {
            println!("⚠️ Didn't reach Set_Refresh in 10k steps, stopping at PC={:04X}", cpu.pc);
            break;
        }
    }
    
    // Mostrar estado antes de Set_Refresh
    println!("\n--- Estado antes de Set_Refresh ---");
    println!("PC: {:04X}", cpu.pc);
    println!("IER: {:02X}", cpu.bus.via_ier());
    println!("IFR: {:02X}", cpu.bus.via_ifr());
    println!("$C83D (refresh lo): {:02X}", cpu.bus.read8(0xC83D));
    println!("$C83E (refresh hi): {:02X}", cpu.bus.read8(0xC83E));
    
    // Ejecutar Set_Refresh paso a paso
    println!("\n--- Ejecutando Set_Refresh paso a paso ---");
    for i in 0..10 {
        let prev_pc = cpu.pc;
        let opcode = cpu.bus.read8(cpu.pc);
        cpu.step();
        println!("Step {}: {:04X} -> {:04X} (opcode {:02X})", i+1, prev_pc, cpu.pc, opcode);
        
        // Mostrar estado VIA después de cada instrucción
        println!("  IFR: {:02X}", cpu.bus.via_ifr());
        
        // Si llegamos al loop problemático F19E
        if cpu.pc == 0xF19E {
            println!("✓ Reached problematic loop F19E");
            break;
        }
        
        // Si salimos de Set_Refresh
        if cpu.pc < 0xF1A2 || cpu.pc > 0xF1B0 {
            println!("✓ Exited Set_Refresh region");
            break;
        }
    }
    
    // Ahora simular algunos ciclos de Timer2 para ver si funciona
    println!("\n--- Simulando Timer2 comportamiento ---");
    let initial_ifr = cpu.bus.via_ifr();
    
    println!("Inicial: IFR={:02X}", initial_ifr);
    
    // Simular Timer2 countdown
    for cycle in 1..=20 {
        cpu.bus.via.tick(1);
        let current_ifr = cpu.bus.via_ifr();
        
        if current_ifr != initial_ifr || cycle % 5 == 0 {
            println!("Cycle {}: IFR={:02X}", cycle, current_ifr);
        }
        
        // Si Timer2 expiró (IFR bit 5 set)
        if (current_ifr & 0x20) != 0 {
            println!("🎯 Timer2 expired! IFR bit 5 set at cycle {}", cycle);
            break;
        }
    }
    
    // Test del loop Wait_Recal
    println!("\n--- Testing Wait_Recal loop behavior ---");
    if cpu.pc == 0xF19E {
        let mut loop_cycles = 0;
        loop {
            cpu.step();
            loop_cycles += 1;
            
            // Check si seguimos en el loop
            if cpu.pc == 0xF19E {
                if loop_cycles % 10 == 0 {
                    println!("Loop cycle {}: Still at F19E, IFR={:02X}", loop_cycles, cpu.bus.via_ifr());
                }
            } else {
                println!("✓ Exited loop after {} cycles, now at PC={:04X}", loop_cycles, cpu.pc);
                break;
            }
            
            if loop_cycles > 100 {
                println!("⚠️ Still in loop after {} cycles, stopping", loop_cycles);
                break;
            }
        }
    }
    
    println!("\n=== Timer2 Wait_Recal Debug Complete ===");
}