use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let data = std::fs::read(path).expect("BIOS real requerida para test");
    assert_eq!(data.len(), 8192, "BIOS size inesperado");
    for (i, b) in data.iter().enumerate() { 
        let addr = 0xE000 + i as u16; 
        cpu.bus.mem[addr as usize] = *b; 
        cpu.bus.mem[addr as usize] = *b; 
    }
    cpu.bios_present = true;
}

#[test]
fn analyze_f19e_loop() {
    println!("=== Analyzing F19E Loop ===");
    
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Run until we reach the F19E loop
    for step in 0..5000 {
        cpu.step();
        if cpu.pc == 0xF19E || cpu.pc == 0xF1A0 {
            println!("Reached F19E loop at step {}", step);
            break;
        }
    }
    
    println!("CPU state at F19E loop:");
    println!("PC: {:04X}, DP: {:02X}", cpu.pc, cpu.dp);
    
    // Analyze what the loop is testing
    let test_addr = ((cpu.dp as u16) << 8) | 0x0D;
    let test_value = cpu.bus.mem[test_addr as usize];
    
    println!("Testing address: {:04X} (DP={:02X}, offset=0D)", test_addr, cpu.dp);
    println!("Current value at {:04X}: {:02X}", test_addr, test_value);
    
    // Let's see what's in the surrounding memory area
    println!("\nMemory around test address:");
    for offset in 0x00..0x20 {
        let addr = ((cpu.dp as u16) << 8) | offset;
        let value = cpu.bus.mem[addr as usize];
        println!("  {:04X}: {:02X}", addr, value);
    }
    
    // Try to understand what should change this value
    // Look for any IRQ/timer activity that might modify this
    println!("\nVIA Timer state:");
    println!("T1: {:04X}", cpu.bus.via.read(0x04) as u16 | ((cpu.bus.via.read(0x05) as u16) << 8));
    println!("T2: {:04X}", cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8));
    println!("IER: {:02X}", cpu.bus.via.read(0x0E));
    println!("IFR: {:02X}", cpu.bus.via.read(0x0D));
    
    // Check if any interrupts are enabled/pending
    println!("\nCPU interrupt state:");
    println!("CC_I (interrupt mask): {}", cpu.cc_i);
    println!("IRQ pending: {}", cpu.irq_pending);
    println!("NMI pending: {}", cpu.nmi_pending);
}