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
fn debug_f19e_instructions() {
    println!("=== F19E Loop Instructions Debug ===");
    
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
    
    // Now let's carefully trace what the F19E loop is doing by watching PC and memory reads
    println!("\nTracing F19E loop instructions...");
    
    let mut loop_iterations = 0;
    let max_iterations = 20; // Just trace a few iterations to see the pattern
    
    while loop_iterations < max_iterations {
        if cpu.pc == 0xF19E {
            println!("\n--- Loop iteration {} ---", loop_iterations + 1);
            loop_iterations += 1;
        }
        
        let pre_pc = cpu.pc;
        let pre_cycles = cpu.cycles;
        
        // Decode the instruction at current PC to see what it's doing
        let opcode = cpu.mem[cpu.pc as usize];
        let decoded = match opcode {
            0x0D => "TST ($nnnn)",
            0x27 => "BEQ",
            0x2A => "BPL", 
            0x8D => "BSR",
            0x39 => "RTS",
            0x86 => "LDA #$nn",
            0x96 => "LDA $nn",
            0xB6 => "LDA $nnnn",
            0x4F => "CLRA",
            0x5F => "CLRB",
            _ => "OTHER"
        };
        
        cpu.step();
        
        let post_pc = cpu.pc;
        let post_cycles = cpu.cycles;
        let cycles_spent = post_cycles - pre_cycles;
        
        // Show the instruction and where it went
        println!("  {:04X}: {:02X} ({}) -> {:04X} ({} cycles)", 
                 pre_pc, opcode, decoded, post_pc, cycles_spent);
        
        // If we're not in the immediate F19E area anymore, we might have escaped
        if post_pc < 0xF190 || post_pc > 0xF1B0 {
            println!("*** Escaped F19E loop area! Now at PC={:04X} ***", post_pc);
            break;
        }
    }
    
    // Final state
    let final_t2 = cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8);
    let final_ifr = cpu.bus.via.read(0x0D);
    
    println!("\nFinal state after tracing:");
    println!("PC: {:04X}", cpu.pc);
    println!("Timer2: {:04X}", final_t2);
    println!("IFR: {:02X} (bit5={})", final_ifr, (final_ifr & 0x20) != 0);
}