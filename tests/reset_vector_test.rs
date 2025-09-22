use vectrex_emulator::CPU;

#[test]
fn reset_uses_vector() {
    let mut cpu = CPU::default();
    // Install reset vector bytes at FFFC/FFFD -> 0x1234
    cpu.bus.mem[0xFFFC] = 0x34; cpu.bus.mem[0xFFFD] = 0x12;
    cpu.bus.mem[0xFFFC] = 0x34; cpu.bus.mem[0xFFFD] = 0x12;
    cpu.reset();
    assert_eq!(cpu.pc, 0x1234, "PC should load from reset vector");
    // Flags should be cleared
    assert!(!cpu.cc_i && !cpu.cc_e && !cpu.cc_f, "Condition code flags should be cleared on reset");
}
