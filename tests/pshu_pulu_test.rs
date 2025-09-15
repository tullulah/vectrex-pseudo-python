use vectrex_emulator::CPU;

#[test]
fn pshu_pulu_roundtrip() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100; cpu.u = 0x0200; // set user stack pointer start
    cpu.a = 0x11; cpu.b = 0x22; cpu.dp = 0xD0; cpu.x = 0x1234; cpu.y = 0x5678;

    // Program:
    // 0100: PSHU mask (push CC,A,B,DP,X,PC) -> bits 0x1F + PC bit 0x80 = 0x9F
    // 0102: LDA #$99 (modify A)
    // 0104: PULU same mask (restore) ; afterwards A should be 0x11 and PC restored to 0x0102

    cpu.mem[0x0100] = 0x36; cpu.bus.mem[0x0100] = 0x36; // PSHU
    cpu.mem[0x0101] = 0x9F; cpu.bus.mem[0x0101] = 0x9F;
    cpu.mem[0x0102] = 0x86; cpu.bus.mem[0x0102] = 0x86; // LDA #
    cpu.mem[0x0103] = 0x99; cpu.bus.mem[0x0103] = 0x99;
    cpu.mem[0x0104] = 0x37; cpu.bus.mem[0x0104] = 0x37; // PULU
    cpu.mem[0x0105] = 0x9F; cpu.bus.mem[0x0105] = 0x9F;

    cpu.step(); // PSHU
    let u_after_push = cpu.u;
    assert!(u_after_push < 0x0200, "U should move downward after pushes");
    cpu.step(); // LDA modify A
    assert_eq!(cpu.a, 0x99);
    cpu.step(); // PULU

    assert_eq!(cpu.a, 0x11, "A restored");
    assert_eq!(cpu.pc, 0x0102, "PC restored to after PSHU");
    assert_eq!(cpu.dp, 0xD0, "DP restored");
    assert_eq!(cpu.x, 0x1234, "X restored");
}
