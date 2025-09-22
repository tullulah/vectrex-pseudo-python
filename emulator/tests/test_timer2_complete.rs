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
fn test_timer2_wait_complete() {
    println!("=== Complete Timer2 Wait Test ===");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Run until F19E loop
    let mut step_count = 0;
    while cpu.pc != 0xF19E && step_count < 2000 {
        cpu.step();
        step_count += 1;
    }
    
    if cpu.pc != 0xF19E {
        println!("Failed to reach F19E loop after {} steps, PC={:04X}", step_count, cpu.pc);
        return;
    }
    
    println!("Reached F19E loop at step {}", step_count);
    
    // Get initial Timer2 value
    let initial_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    println!("Initial Timer2: {:04X} ({} decimal)", initial_t2, initial_t2);
    
    // Run exactly enough cycles to let Timer2 expire
    // We'll track progress and break when Timer2 expires OR we escape the loop
    let mut total_cycles = 0u32;
    let max_cycles = initial_t2 as u32 + 100; // Timer2 + some buffer
    
    println!("Running up to {} cycles to let Timer2 expire...", max_cycles);
    
    
    let mut report_counter = 0;
    
    while total_cycles < max_cycles && step_count < 100000 {
        let pre_cycles = cpu.cycles;
        let pre_pc = cpu.pc;
        let pre_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let pre_ifr = cpu.bus.via.read(0x0D);
        
        cpu.step();
        
        let post_cycles = cpu.cycles;
        let post_pc = cpu.pc;
        let post_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let post_ifr = cpu.bus.via.read(0x0D);
        
        let step_cycles = (post_cycles - pre_cycles) as u32;
        total_cycles += step_cycles;
        step_count += 1;
        
        // Report progress every 5000 cycles or when important things happen
        if total_cycles / 5000 > report_counter || 
           post_t2 == 0 || 
           post_ifr != pre_ifr ||
           (post_pc != 0xF19E && post_pc != 0xF1A0) {
            
            println!("Step {}: cycles={}, PC {:04X}->{:04X}, T2 {:04X}->{:04X}, IFR {:02X}->{:02X}", 
                     step_count, total_cycles, pre_pc, post_pc, pre_t2, post_t2, pre_ifr, post_ifr);
            
            if post_t2 == 0 && pre_t2 > 0 {
                println!("*** Timer2 EXPIRED! ***");
            }
            
            if post_ifr != pre_ifr {
                println!("*** IFR CHANGED! New IFR={:02X} ***", post_ifr);
                if post_ifr & 0x20 != 0 {
                    println!("*** IFR5 (Timer2) NOW SET! ***");
                }
            }
            
            if post_pc != 0xF19E && post_pc != 0xF1A0 {
                println!("*** ESCAPED F19E LOOP! Now at PC={:04X} ***", post_pc);
                break;
            }
            
            report_counter = total_cycles / 5000;
        }
        
        // Safety: if Timer2 has been expired for a while but loop continues, break
        if post_t2 == 0 && total_cycles > initial_t2 as u32 + 1000 {
            println!("*** Timer2 expired {} cycles ago but still in loop - potential bug! ***", 
                     total_cycles - initial_t2 as u32);
            break;
        }
    }
    
    // Final report
    let final_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let final_ifr = cpu.bus.via.read(0x0D);
    let final_ier = cpu.bus.via.read(0x0E);
    
    println!("\n=== FINAL STATE ===");
    println!("Steps: {}, Total cycles: {}", step_count, total_cycles);
    println!("PC: {:04X}", cpu.pc);
    println!("Timer2: {:04X} (started at {:04X})", final_t2, initial_t2);
    println!("IFR: {:02X} (bit5={}, bit7={})", final_ifr, (final_ifr & 0x20) != 0, (final_ifr & 0x80) != 0);
    println!("IER: {:02X} (bit5={}, bit7={})", final_ier, (final_ier & 0x20) != 0, (final_ier & 0x80) != 0);
    
    if cpu.pc != 0xF19E && cpu.pc != 0xF1A0 {
        println!("SUCCESS: Escaped F19E loop!");
    } else if final_t2 == 0 {
        println!("ISSUE: Timer2 expired but still in loop - check IFR/IER logic");
    } else {
        println!("TIMEOUT: Ran out of cycles before Timer2 expired");
    }
}