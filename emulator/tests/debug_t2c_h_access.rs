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
fn test_who_reads_t2c_h() {
    std::env::set_var("VIA_TRACE", "1");
    std::env::set_var("IRQ_TRACE", "1");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    let mut steps = 0;
    
    // Run until F19E loop
    loop {
        let pc = cpu.pc;
        if pc == 0xF19E {
            break;
        }
        cpu.step();
        steps += 1;
        if steps > 50000 {
            panic!("Could not reach F19E loop");
        }
    }
    
    println!("Reached F19E loop at step {}", steps);
    
    // Fast forward to timer expiry
    let mut timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let mut cycles = 0;
    
    while timer2 > 100 {
        cpu.step();
        timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        cycles += 1;
        if cycles > 30000 {
            panic!("Timer2 never got close to expiry");
        }
    }
    
    println!("Timer2 near expiry: {:04X} ({} decimal), total cycles: {}", timer2, timer2, cycles);
    
    // Now trace every bus access until we see T2C-H read
    println!("\nDetailed tracing to find T2C-H read...");
    let mut detailed_steps = 0;
    loop {
        let old_pc = cpu.pc;
        let old_timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let old_ifr = cpu.bus.via.read(0x0D);  // IFR register
        
        // Hook into bus reads by monitoring VIA directly
        // First check if Timer2 is about to expire
        if old_timer2 <= 5 && old_timer2 > 0 {
            println!("*** Timer2 about to expire: {} ***", old_timer2);
        }
        
        // Take a step
        cpu.step();
        detailed_steps += 1;
        
        let new_pc = cpu.pc;
        let new_timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        let new_ifr = cpu.bus.via.read(0x0D);  // IFR register
        
        // Check if Timer2 expired this step
        if old_timer2 > 0 && new_timer2 == 0 {
            println!("*** Timer2 EXPIRED on step {}! PC: {:04X} ***", detailed_steps, old_pc);
        }
        
        // Check if IFR5 was cleared unexpectedly
        if (old_ifr & 0x20) != 0 && (new_ifr & 0x20) == 0 {
            println!("*** IFR5 CLEARED on step {}! PC: {:04X} → {:04X} ***", detailed_steps, old_pc, new_pc);
            
            // Let's see what the last instruction was
            println!("Last instruction PC: {:04X}", old_pc);
            
            // We need to check if this instruction accessed $D009
            // This is tricky to check post-facto, but the VIA trace should show it
            break;
        }
        
        if detailed_steps > 200 {
            println!("Timer2: {:04X}, IFR: {:02X}, PC: {:04X}", new_timer2, new_ifr, new_pc);
            break;
        }
    }
}