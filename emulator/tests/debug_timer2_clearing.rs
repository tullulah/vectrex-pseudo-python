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
fn debug_timer2_when_clearing() {
    println!("=== Timer2 Clearing Debug ===");
    
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
    
    // Run until Timer2 is close to expiring, then trace every single memory read
    let mut total_cycles = 0u32;
    let near_expiry_cycles = initial_t2 as u32 - 100; // Stop when Timer2 has ~100 cycles left
    
    println!("Running {} cycles to get Timer2 close to expiration...", near_expiry_cycles);
    
    // Fast forward to near expiry
    while total_cycles < near_expiry_cycles && step_count < 100000 {
        let pre_cycles = cpu.cycles;
        cpu.step();
        let post_cycles = cpu.cycles;
        total_cycles += (post_cycles - pre_cycles) as u32;
        step_count += 1;
    }
    
    let near_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    println!("Timer2 near expiry: {:04X} ({} decimal), total cycles: {}", near_t2, near_t2, total_cycles);
    
    // Now trace very carefully until Timer2 expires and observe exactly when IFR5 gets cleared
    println!("\nDetailed tracing until Timer2 expiration...");
    
    let mut detailed_steps = 0;
    while detailed_steps < 200 { // Max 200 more steps
        let pre_pc = cpu.pc;
        let pre_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let pre_ifr = cpu.bus.via.read(0x0D);
        let pre_cycles = cpu.cycles;
        
        // Check if Timer2 just expired
        if pre_t2 == 0 && detailed_steps > 0 {
            println!("Timer2 has expired, taking a few more steps to observe IFR behavior...");
        }
        
        cpu.step();
        
        let post_pc = cpu.pc;
        let post_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let post_ifr = cpu.bus.via.read(0x0D);
        let post_cycles = cpu.cycles;
        
        let step_cycles = (post_cycles - pre_cycles) as u32;
        total_cycles += step_cycles;
        detailed_steps += 1;
        
        // Report when Timer2 expires or IFR changes
        if pre_t2 > 0 && post_t2 == 0 {
            println!("*** Step {}: Timer2 EXPIRED! {} -> 0 ***", detailed_steps, pre_t2);
        }
        
        if pre_ifr != post_ifr {
            println!("*** Step {}: IFR CHANGED! {:02X} -> {:02X} ***", detailed_steps, pre_ifr, post_ifr);
            println!("    PC: {:04X} -> {:04X}, T2: {:04X} -> {:04X}", pre_pc, post_pc, pre_t2, post_t2);
        }
        
        // Show last few steps before expiration and first few after
        if pre_t2 <= 3 || post_t2 <= 3 || (pre_ifr & 0x20) != 0 || (post_ifr & 0x20) != 0 {
            println!("Step {}: PC {:04X}->{:04X}, T2 {:04X}->{:04X}, IFR {:02X}->{:02X}, cycles +{}", 
                     detailed_steps, pre_pc, post_pc, pre_t2, post_t2, pre_ifr, post_ifr, step_cycles);
        }
        
        // Stop after Timer2 has been expired for a while
        if post_t2 == 0 && detailed_steps > 150 {
            println!("Timer2 expired long ago, stopping trace");
            break;
        }
        
        // Escape if we leave the F19E area  
        if post_pc < 0xF190 || post_pc > 0xF1B0 {
            println!("*** ESCAPED F19E AREA! Now at PC={:04X} ***", post_pc);
            break;
        }
    }
    
    // Final report
    let final_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let final_ifr = cpu.bus.via.read(0x0D);
    
    println!("\nFinal state after detailed tracing:");
    println!("PC: {:04X}", cpu.pc);
    println!("Timer2: {:04X}", final_t2);
    println!("IFR: {:02X} (bit5={})", final_ifr, (final_ifr & 0x20) != 0);
}