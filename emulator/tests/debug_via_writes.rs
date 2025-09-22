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
fn debug_via_writes() {
    println!("=== VIA Writes Debug ===");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Monitor VIA writes during BIOS initialization
    let mut via_writes = Vec::new();
    
    // Run until F19E loop, tracking VIA writes
    let mut step_count = 0;
    while cpu.pc != 0xF19E && step_count < 2000 {
        let old_last_write = cpu.bus.last_via_write;
        cpu.step();
        
        // Check for new VIA write
        if let Some((addr, val)) = cpu.bus.last_via_write {
            if old_last_write != Some((addr, val)) {
                via_writes.push((step_count, cpu.pc, addr, val));
                
                // Decode VIA register name
                let reg_name = match addr & 0x0F {
                    0x00 => "ORB/IRB",
                    0x01 => "ORA/IRA",
                    0x02 => "DDRB",
                    0x03 => "DDRA",
                    0x04 => "T1C-L",
                    0x05 => "T1C-H",
                    0x06 => "T1L-L",
                    0x07 => "T1L-H",
                    0x08 => "T2C-L",
                    0x09 => "T2C-H",
                    0x0A => "SR",
                    0x0B => "ACR",
                    0x0C => "PCR",
                    0x0D => "IFR",
                    0x0E => "IER",
                    0x0F => "ORA/IRA_NH",
                    _ => "UNKNOWN"
                };
                
                println!("Step {}: PC={:04X} wrote VIA ${:04X} ({}) = ${:02X}", 
                         step_count, cpu.pc, addr, reg_name, val);
                
                // Special attention to IER writes
                if (addr & 0x0F) == 0x0E {
                    if val & 0x80 != 0 {
                        println!("  -> IER ENABLE: {:02X} (T2={}, T1={}, CB1={}, CB2={}, SR={}, CA2={}, CA1={})",
                                 val & 0x7F,
                                 (val & 0x20) != 0,
                                 (val & 0x40) != 0,
                                 (val & 0x10) != 0,
                                 (val & 0x08) != 0,
                                 (val & 0x04) != 0,
                                 (val & 0x02) != 0,
                                 (val & 0x01) != 0);
                    } else {
                        println!("  -> IER DISABLE: {:02X}", val & 0x7F);
                    }
                }
                
                // Special attention to Timer writes
                if (addr & 0x0F) == 0x09 {
                    println!("  -> Timer2 high byte written - timer started/restarted");
                }
            }
        }
        
        step_count += 1;
    }
    
    if cpu.pc == 0xF19E {
        println!("\nReached F19E loop at step {}", step_count);
    } else {
        println!("\nDid not reach F19E loop after {} steps, PC={:04X}", step_count, cpu.pc);
        return;
    }
    
    // Summary of VIA writes
    println!("\n=== VIA Writes Summary ({} total) ===", via_writes.len());
    for (step, pc, addr, val) in &via_writes {
        let reg_name = match addr & 0x0F {
            0x0E => "IER",
            0x09 => "T2C-H",
            0x08 => "T2C-L",
            0x05 => "T1C-H",
            0x04 => "T1C-L",
            0x0B => "ACR",
            _ => "OTHER"
        };
        if reg_name == "IER" || reg_name.starts_with("T") {
            println!("  Step {}: PC={:04X} {} = ${:02X}", step, pc, reg_name, val);
        }
    }
    
    // Check final IER state
    let final_ier = cpu.bus.via.read(0x0E);
    println!("\nFinal IER state: ${:02X}", final_ier);
    println!("  - Master enable: {}", (final_ier & 0x80) != 0);
    println!("  - Timer2 enable: {}", (final_ier & 0x20) != 0);
    
    if (final_ier & 0x20) == 0 {
        println!("\n*** PROBLEM: Timer2 interrupts are NOT enabled in IER! ***");
        println!("*** The BIOS expects Timer2 interrupt but IER5=0 ***");
    }
}