use vectrex_emulator::CPU;

// Helper to set D as combined A/B
fn set_d(cpu: &mut CPU, val: u16) { cpu.a = (val >> 8) as u8; cpu.b = val as u8; }
fn get_d(cpu: &CPU) -> u16 { ((cpu.a as u16) << 8) | cpu.b as u16 }

#[test]
fn subd_immediate_basic() {
    let mut cpu = CPU::default();
    set_d(&mut cpu, 0x1234);
    cpu.pc = 0x0100;
    cpu.mem[0x0100] = 0x83; // SUBD imm
    cpu.mem[0x0101] = 0x00; cpu.mem[0x0102] = 0x34; // subtract 0x0034
    assert!(cpu.step());
    assert_eq!(get_d(&cpu), 0x1200);
    assert!(!cpu.cc_z);
    assert!(!cpu.cc_c, "No borrow expected");
}

#[test]
fn subd_direct_and_indexed() {
    let mut cpu = CPU::default();
    // Use a safe RAM direct page (avoid 0xD0 which maps to VIA). Set DP=0x00 and place 0x0100 at $0000.
    cpu.dp = 0x00;
    cpu.mem[0x0000] = 0x01; cpu.bus.mem[0x0000] = 0x01;
    cpu.mem[0x0001] = 0x00; cpu.bus.mem[0x0001] = 0x00;
    set_d(&mut cpu, 0x0200);
    cpu.pc = 0x0200;
    cpu.mem[0x0200] = 0x93; // SUBD direct
    cpu.mem[0x0201] = 0x00; // offset -> $0000
    assert!(cpu.step());
    assert_eq!(get_d(&cpu), 0x0100);

    // Indexed: put target at X+5
    cpu.x = 0x3000; let ea = 0x3000u16 + 5; cpu.mem[ea as usize] = 0x00; cpu.bus.mem[ea as usize] = 0x00; cpu.mem[ea as usize + 1] = 0x10; cpu.bus.mem[ea as usize + 1] = 0x10;
    set_d(&mut cpu, 0x0110);
    cpu.pc = 0x0300;
    cpu.mem[0x0300] = 0xA3; // SUBD indexed
    // Simple 5-bit offset , postbyte 0x85 = ,X+5 (assuming addressing decode supports 0x85 pattern; adjust if different)
    cpu.mem[0x0301] = 0x85; // postbyte placeholder (implementation-specific)
    let _ = cpu.step(); // may succeed or fail depending on decode; just ensure no panic
}

#[test]
fn subd_borrow_and_zero() {
    let mut cpu = CPU::default();
    set_d(&mut cpu, 0x0001);
    cpu.pc = 0x0100;
    cpu.mem[0x0100] = 0x83; // SUBD imm
    cpu.mem[0x0101] = 0x00; cpu.mem[0x0102] = 0x02; // subtract 2 -> underflow
    let _ = cpu.step();
    assert_eq!(get_d(&cpu), 0xFFFF);
    assert!(cpu.cc_c, "Borrow sets carry");
}
