use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let data = std::fs::read(path).expect("BIOS real requerida para test");
    assert_eq!(data.len(), 8192, "BIOS size inesperado");
    for (i, b) in data.iter().enumerate() { 
        let addr = 0xE000 + i as u16; 
        cpu.mem[addr as usize] = *b; 
        cpu.bus.mem[addr as usize] = *b; 
    }
    cpu.bios_present = true;
}

#[test]
fn debug_timer2_expiration() {
    println!("=== Timer2 Expiration Debug ===");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Run until we reach the F19E loop
    for step in 0..5000 {
        cpu.step();
        if cpu.pc == 0xF19E {
            println!("Reached F19E loop at step {}", step);
            break;
        }
    }
    
    // Monitor Timer2 countdown and IFR changes
    println!("\nMonitoring Timer2 countdown...");
    let mut last_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let mut last_ifr = cpu.bus.via.read(0x0D);
    let ier = cpu.bus.via.read(0x0E);
    
    println!("Initial state: T2={:04X}, IFR={:02X}, IER={:02X}", last_t2, last_ifr, ier);
    
    for step in 0..1000 {
        let old_cycles = cpu.cycles;
        cpu.step();
        let cycles_spent = cpu.cycles - old_cycles;
        
        let new_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let new_ifr = cpu.bus.via.read(0x0D);
        
        // Report any changes
        if new_t2 != last_t2 || new_ifr != last_ifr {
            println!("Step {}: cycles +{}, T2 {:04X}->{:04X}, IFR {:02X}->{:02X}", 
                step, cycles_spent, last_t2, new_t2, last_ifr, new_ifr);
            
            if new_t2 == 0 && last_t2 > 0 {
                println!("*** TIMER2 EXPIRED! ***");
            }
            
            if new_ifr != last_ifr {
                println!("*** IFR CHANGED! Bits set: {:02X} ***", new_ifr);
                if new_ifr & 0x20 != 0 {
                    println!("*** IFR5 (Timer2) FLAG SET! ***");
                }
            }
            
            last_t2 = new_t2;
            last_ifr = new_ifr;
        }
        
        // Check if loop breaks
        if cpu.pc != 0xF19E && cpu.pc != 0xF1A0 {
            println!("*** LOOP BROKEN! PC now at {:04X} ***", cpu.pc);
            break;
        }
        
        // Safety exit if timer goes to 0 but loop doesn't break
        if new_t2 == 0 && step > 100 {
            println!("*** Timer2 expired but loop still running - there may be a bug ***");
            break;
        }
    }
    
    println!("\nFinal state:");
    println!("PC: {:04X}", cpu.pc);
    println!("T2: {:04X}", cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8));
    println!("IFR: {:02X}", cpu.bus.via.read(0x0D));
    println!("IER: {:02X}", cpu.bus.via.read(0x0E));
}