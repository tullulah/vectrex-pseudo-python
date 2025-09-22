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
fn debug_timer2_zero_expiration() {
    println!("=== Timer2 Zero Expiration Debug ===");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Run until F19E loop
    let mut step_count = 0;
    while cpu.pc != 0xF19E && step_count < 2000 {
        cpu.step();
        step_count += 1;
    }
    
    if cpu.pc == 0xF19E {
        println!("Reached F19E loop at step {}", step_count);
    } else {
        println!("Did not reach F19E loop after {} steps, PC={:04X}", step_count, cpu.pc);
        return;
    }
    
    // Get initial timer state
    let initial_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let initial_ifr = cpu.bus.via.read(0x0D);
    let initial_ier = cpu.bus.via.read(0x0E);
    
    println!("Initial Timer2 state: T2={:04X} ({} decimal), IFR={:02X}, IER={:02X}", 
             initial_t2, initial_t2, initial_ifr, initial_ier);
    
    // Step Timer2 down to near zero and watch what happens
    let mut steps = 0;
    let target_cycles = initial_t2 + 10; // Run enough cycles to definitely expire T2
    let mut total_cycles = 0u32;
    
    while total_cycles < target_cycles as u32 && steps < 2000 {
        let pre_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let pre_ifr = cpu.bus.via.read(0x0D);
        let pre_cycles = cpu.cycles;
        
        cpu.step();
        
        let post_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let post_ifr = cpu.bus.via.read(0x0D);
        let post_cycles = cpu.cycles;
        let step_cycles = post_cycles - pre_cycles;
        total_cycles += step_cycles as u32;
        
        // Show transition when T2 approaches zero or IFR changes
        if pre_t2 <= 10 || post_t2 == 0 || pre_ifr != post_ifr {
            println!("Step {}: cycles +{}, T2 {:04X}->{:04X}, IFR {:02X}->{:02X}", 
                     steps, step_cycles, pre_t2, post_t2, pre_ifr, post_ifr);
        }
        
        // If Timer2 reached zero, show detailed state
        if post_t2 == 0 && pre_t2 > 0 {
            println!("*** Timer2 EXPIRED! ***");
            println!("Final T2: {}, IFR: {:02X} (bit5={}), IER: {:02X}", 
                     post_t2, 
                     post_ifr, 
                     (post_ifr & 0x20) != 0,
                     cpu.bus.via.read(0x0E));
            break;
        }
        
        steps += 1;
    }
    
    println!("Final state after {} steps, {} total cycles:", steps, total_cycles);
    println!("T2: {:04X}, IFR: {:02X}, IER: {:02X}", 
             cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8),
             cpu.bus.via.read(0x0D), 
             cpu.bus.via.read(0x0E));
}