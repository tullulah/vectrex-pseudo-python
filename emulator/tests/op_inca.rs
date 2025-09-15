use vectrex_emulator::CPU;

#[test]
fn inca_basic() {
    let mut cpu = CPU::default();
    cpu.a = 0x00; cpu.pc = 0x0100; cpu.mem[0x0100] = 0x4C; // INCA
    cpu.step();
    assert_eq!(cpu.a, 0x01);
    assert!(!cpu.cc_z && !cpu.cc_n && !cpu.cc_v, "Flags incorrect after INCA from 0x00");
}

#[test]
fn inca_sets_zero() {
    let mut cpu = CPU::default();
    cpu.a = 0xFF; cpu.pc = 0x0200; cpu.mem[0x0200] = 0x4C; // INCA wraps to 0
    cpu.step();
    assert_eq!(cpu.a, 0x00);
    assert!(cpu.cc_z && !cpu.cc_n && !cpu.cc_v, "Z should be set, N/V cleared after wrap to 0");
}

#[test]
fn inca_overflow_flag() {
    let mut cpu = CPU::default();
    cpu.a = 0x7F; cpu.pc = 0x0300; cpu.mem[0x0300] = 0x4C; // 0x7F -> 0x80 sets V
    cpu.step();
    assert_eq!(cpu.a, 0x80);
    assert!(cpu.cc_n && cpu.cc_v && !cpu.cc_z, "N and V should be set after 0x7F -> 0x80, Z cleared");
}
