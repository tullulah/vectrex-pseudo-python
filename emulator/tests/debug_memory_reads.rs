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
fn test_memory_reads_around_timer_expiry() {
    std::env::set_var("VIA_TRACE", "1");
    std::env::set_var("IRQ_TRACE", "1");
    std::env::set_var("DIRECT_TRACE", "1");
    std::env::set_var("MEM_TRACE", "1"); // Si existe
    
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
    
    // Fast forward to timer near expiry
    let mut timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let mut cycles = 0;
    
    while timer2 > 10 {  // Stop earlier
        cpu.step();
        timer2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
        cycles += 1;
        if cycles > 30000 {
            panic!("Timer2 never got close to expiry");
        }
    }
    
    println!("Timer2 near expiry: {:04X} ({} decimal), total cycles: {}", timer2, timer2, cycles);
    
    // Now implement our own memory read tracing
    println!("\nDetailed instruction tracing...");
    
    let mut detailed_steps = 0;
    let mut timer_has_expired = false;
    
    loop {
        let old_pc = cpu.pc;
        // ¡CUIDADO! NO leer T2C-H porque limpia IFR5
        // Solo leer T2C-L para el valor bajo, y asumir que el alto es 0 después de expirar
        let old_timer2_low = cpu.bus.via.read(0x08);
        let old_ifr = cpu.bus.via.read(0x0D);
        
        // Print instruction info
        let opcode = cpu.mem[old_pc as usize];
        let operand = if old_pc < 0xFFFF { cpu.mem[(old_pc + 1) as usize] } else { 0 };
        println!("Step {}: PC {:04X}, opcode {:02X}, operand {:02X}, T2L {:02X}, IFR {:02X}, DP {:02X}", 
                 detailed_steps, old_pc, opcode, operand, old_timer2_low, old_ifr, cpu.dp);
        
        // Check if Timer2 is about to expire (solo el byte bajo)
        if old_timer2_low <= 3 && old_timer2_low > 0 {
            println!("*** Timer2 critical: T2L={} ***", old_timer2_low);
        }
        
        // Take a single step
        cpu.step();
        detailed_steps += 1;
        
        let new_pc = cpu.pc;
        let new_timer2_low = cpu.bus.via.read(0x08);  // Solo T2C-L
        let new_ifr = cpu.bus.via.read(0x0D);
        
        // Check if Timer2 expired this step (detectar por el cambio en IFR5)
        if (old_ifr & 0x20) == 0 && (new_ifr & 0x20) != 0 {
            println!("*** Timer2 EXPIRED! Step {}: PC {:04X} → {:04X} (IFR5 set) ***", detailed_steps, old_pc, new_pc);
            timer_has_expired = true;
        }
        
        // Check if IFR5 was set or cleared
        if (old_ifr & 0x20) == 0 && (new_ifr & 0x20) != 0 {
            println!("*** IFR5 SET! Step {}: {:02X} → {:02X} ***", detailed_steps, old_ifr, new_ifr);
        }
        if (old_ifr & 0x20) != 0 && (new_ifr & 0x20) == 0 {
            println!("*** IFR5 CLEARED! Step {}: {:02X} → {:02X} ***", detailed_steps, old_ifr, new_ifr);
            if timer_has_expired {
                println!("Timer expired and IFR5 cleared - this is the bug!");
                break;
            }
        }
        
        if detailed_steps > 30 {
            println!("Timer2L: {:02X}, IFR: {:02X}, PC: {:04X}", new_timer2_low, new_ifr, new_pc);
            break;
        }
    }
    
    println!("\nTest completed");
}