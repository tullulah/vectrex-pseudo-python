#[cfg(test)]
mod bios_early_trace {
    use vectrex_emulator::CPU;

    #[test]
    fn trace_early_bios_execution() {
        let mut cpu = CPU::default();
        
        // Load BIOS
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
        let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
        cpu.load_bios(&bios_data);
        
        // Reset CPU and trace first 50 instructions
        cpu.reset();
        cpu.trace = true;
        
        println!("Early BIOS execution trace:");
        println!("Initial state: PC=${:04X} S=${:04X}", cpu.pc, cpu.s);
        
        for step in 0..200 {
            let pc_before = cpu.pc;
            let opcode = cpu.bus.read8(cpu.pc);
            
            cpu.step();
            
            println!("Step {}: PC=${:04X} op=${:02X} -> PC=${:04X} S=${:04X}", 
                     step, pc_before, opcode, cpu.pc, cpu.s);
                     
            // Check if we reach any VIA writes
            if let Some((addr, val)) = cpu.bus.last_via_write {
                println!("  -> VIA WRITE: ${:04X} = ${:02X}", addr, val);
            }
            
            // Stop if we hit any special conditions
            if cpu.pc < 0xE000 {
                println!("  -> PC left BIOS ROM region, now in ${:04X}", cpu.pc);
                break;
            }
            
            if cpu.pc == pc_before {
                println!("  -> PC stuck at ${:04X} (infinite loop)", cpu.pc);
                break;
            }
        }
        
        // Check final state
        println!("\nFinal state:");
        println!("PC=${:04X} S=${:04X}", cpu.pc, cpu.s);
        println!("IFR=${:02X} IER=${:02X}", cpu.bus.via_ifr(), cpu.bus.via_ier());
        
        // Look at what's at current PC
        if cpu.pc >= 0xE000 {
            println!("Code at current PC:");
            for i in 0..10 {
                let addr = cpu.pc.wrapping_add(i);
                let byte = cpu.bus.read8(addr);
                println!("  ${:04X}: ${:02X}", addr, byte);
            }
        }
    }
}