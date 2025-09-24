use vectrex_emulator::CPU;

#[test]
fn tfr_16bit_x_to_y() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100; cpu.x = 0x1234; cpu.y = 0xAAAA;
    // 0100: TFR X,Y -> opcode 0x1F postbyte src=X(1)<<4 | dst=Y(2) -> 0x12
    cpu.bus.mem[0x0100] = 0x1F; cpu.bus.mem[0x0100] = 0x1F;
    cpu.bus.mem[0x0101] = 0x12; cpu.bus.mem[0x0101] = 0x12;
    cpu.step();
    assert_eq!(cpu.y, 0x1234);
    assert_eq!(cpu.x, 0x1234);
}

#[test]
fn exg_8bit_a_b() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100; cpu.a = 0x55; cpu.b = 0xAA;
    // EXG A,B -> opcode 0x1E postbyte src=A(8)<<4 | dst=B(9) -> 0x89
    cpu.bus.mem[0x0100] = 0x1E; cpu.bus.mem[0x0100] = 0x1E;
    cpu.bus.mem[0x0101] = 0x89; cpu.bus.mem[0x0101] = 0x89;
    cpu.step();
    assert_eq!(cpu.a, 0xAA);
    assert_eq!(cpu.b, 0x55);
}

#[test]
fn exg_invalid_mixed_width_rejected() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100; cpu.a = 0x11; cpu.x = 0x2222;
    // Attempt EXG A,X (8-bit vs 16-bit) -> src=A(8), dst=X(1) => post 0x81
    cpu.bus.mem[0x0100] = 0x1E; cpu.bus.mem[0x0100] = 0x1E;
    cpu.bus.mem[0x0101] = 0x81; cpu.bus.mem[0x0101] = 0x81;
    let ok = cpu.step();
    assert!(!ok, "Mixed width EXG should return false / unimplemented");
}
