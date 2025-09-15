use vectrex_emulator::CPU;

#[test]
fn opcode_metrics_collect() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100;
    // Program: LDA #$01 (0x86), NOP (0x12), UNIMPL (0x02), NOP (0x12)
    cpu.mem[0x0100] = 0x86; cpu.bus.mem[0x0100] = 0x86; // implemented
    cpu.mem[0x0101] = 0x01; cpu.bus.mem[0x0101] = 0x01; // operand
    cpu.mem[0x0102] = 0x12; cpu.bus.mem[0x0102] = 0x12; // NOP
    cpu.mem[0x0103] = 0x02; cpu.bus.mem[0x0103] = 0x02; // likely unimplemented
    cpu.mem[0x0104] = 0x12; cpu.bus.mem[0x0104] = 0x12; // not reached

    assert!(cpu.step(), "LDA should execute");
    assert!(cpu.step(), "NOP should execute");
    let ok = cpu.step();
    assert!(!ok, "Unimplemented opcode should return false");

    let snapshot = cpu.metrics_snapshot();
    assert_eq!(snapshot.total, 3, "Three opcodes fetched");
    assert_eq!(snapshot.counts[0x86], 1, "LDA counted once");
    assert_eq!(snapshot.counts[0x12], 1, "NOP counted once (second NOP not executed)");
    assert_eq!(snapshot.unimplemented, 1, "One unimplemented execution");
    assert!(snapshot.unique_unimplemented.contains(&0x02), "Opcode 0x02 should be in unique list");
}
