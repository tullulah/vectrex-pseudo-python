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
fn debug_timer_analysis() {
    println!("=== Timer Analysis Debug ===");
    
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
    
    // Analyze all timer registers
    let t1c_l = cpu.bus.via.read(0x04);
    let t1c_h = cpu.bus.via.read(0x05);
    let t1l_l = cpu.bus.via.read(0x06);
    let t1l_h = cpu.bus.via.read(0x07);
    let t2c_l = cpu.bus.via.read(0x08);
    let t2c_h = cpu.bus.via.read(0x09);
    let acr = cpu.bus.via.read(0x0B);
    let ifr = cpu.bus.via.read(0x0D);
    let ier = cpu.bus.via.read(0x0E);
    
    let t1c = (t1c_h as u16) << 8 | t1c_l as u16;
    let t1l = (t1l_h as u16) << 8 | t1l_l as u16;
    let t2c = (t2c_h as u16) << 8 | t2c_l as u16;
    
    println!("\nTimer register analysis:");
    println!("T1C (Timer1 Counter): {:04X} ({} decimal) [regs 0x04,0x05 = {:02X},{:02X}]", t1c, t1c, t1c_l, t1c_h);
    println!("T1L (Timer1 Latch):   {:04X} ({} decimal) [regs 0x06,0x07 = {:02X},{:02X}]", t1l, t1l, t1l_l, t1l_h);
    println!("T2C (Timer2 Counter): {:04X} ({} decimal) [regs 0x08,0x09 = {:02X},{:02X}]", t2c, t2c, t2c_l, t2c_h);
    println!("ACR (Aux Control):    {:02X} [reg 0x0B]", acr);
    println!("  - Timer1 continuous: {}", (acr & 0x40) != 0);
    println!("  - Timer1 PB7 output: {}", (acr & 0x80) != 0);
    println!("IFR (Interrupt Flag): {:02X} [reg 0x0D]", ifr);
    println!("  - IFR7 (IRQ): {}", (ifr & 0x80) != 0);
    println!("  - IFR6 (T1):  {}", (ifr & 0x40) != 0);
    println!("  - IFR5 (T2):  {}", (ifr & 0x20) != 0);
    println!("  - IFR4 (CB1): {}", (ifr & 0x10) != 0);
    println!("  - IFR3 (CB2): {}", (ifr & 0x08) != 0);
    println!("  - IFR2 (SR):  {}", (ifr & 0x04) != 0);
    println!("  - IFR1 (CA2): {}", (ifr & 0x02) != 0);
    println!("  - IFR0 (CA1): {}", (ifr & 0x01) != 0);
    println!("IER (Interrupt Enable): {:02X} [reg 0x0E]", ier);
    println!("  - IER7 (Master): {}", (ier & 0x80) != 0);
    println!("  - IER6 (T1):     {}", (ier & 0x40) != 0);
    println!("  - IER5 (T2):     {}", (ier & 0x20) != 0);
    println!("  - IER4 (CB1):    {}", (ier & 0x10) != 0);
    println!("  - IER3 (CB2):    {}", (ier & 0x08) != 0);
    println!("  - IER2 (SR):     {}", (ier & 0x04) != 0);
    println!("  - IER1 (CA2):    {}", (ier & 0x02) != 0);
    println!("  - IER0 (CA1):    {}", (ier & 0x01) != 0);
    
    // Check which timer(s) are actually active
    println!("\nActive timers:");
    if t1c > 0 {
        println!("  - Timer1 is ACTIVE (counting down from {:04X})", t1c);
    } else {
        println!("  - Timer1 is STOPPED");
    }
    if t2c > 0 {
        println!("  - Timer2 is ACTIVE (counting down from {:04X})", t2c);
    } else {
        println!("  - Timer2 is STOPPED");
    }
    
    // Analyze what the BIOS is waiting for
    println!("\nBIOS wait analysis:");
    if ifr & 0x40 != 0 {
        println!("  - IFR6 (Timer1) is SET - Timer1 has expired");
    }
    if ifr & 0x20 != 0 {
        println!("  - IFR5 (Timer2) is SET - Timer2 has expired");
    }
    if ifr & 0x20 == 0 && t2c > 0 {
        println!("  - Timer2 is running but hasn't expired yet");
        println!("  - BIOS is waiting for Timer2 to expire and set IFR5");
    }
    if ifr & 0x40 != 0 && ifr & 0x20 == 0 {
        println!("  - Timer1 has expired but Timer2 hasn't");
        println!("  - BIOS may be confused about which timer to wait for");
    }
    
    // Test DP register value for the F19E loop
    println!("\nF19E loop context:");
    println!("PC: {:04X}, DP: {:02X}", cpu.pc, cpu.dp);
    println!("Testing address DP*256 + 0x0D = {:04X}", (cpu.dp as u16) * 256 + 0x0D);
    if (cpu.dp as u16) * 256 + 0x0D == 0xD00D {
        println!("  - DP is correctly set for VIA access (0xD00D)");
    } else {
        println!("  - DP may be incorrectly set - should be 0xD0 for VIA access");
    }
}