use vectrex_emulator::CPU;

#[test]
fn nop_advances_pc_and_cycles() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100;
    cpu.bus.mem[0x0100] = 0x12; cpu.bus.mem[0x0100] = 0x12; // NOP
    let cycles_before = cpu.cycles;
    cpu.step();
    assert_eq!(cpu.pc, 0x0101, "NOP should advance PC by 1");
    assert!(cpu.cycles > cycles_before, "Cycles should increment after NOP");
    // Registers unchanged
    assert_eq!(cpu.a, 0); assert_eq!(cpu.b, 0);
}
