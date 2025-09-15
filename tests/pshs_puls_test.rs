use vectrex_emulator::CPU;

#[test]
fn pshs_puls_roundtrip() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100;
    cpu.a = 0x12; cpu.b = 0x34; cpu.dp = 0xD0; cpu.x = 0x1111; cpu.y = 0x2222; cpu.u = 0x3333;
    // Program:
    // 0100: PSHS mask (push CC,A,B,DP,X) -> mask bits: CC=1,A=2,B=4,DP=8,X=16 -> 0x1F
    // 0102: LDA #$99 (modify A)
    // 0104: LDB #$55 (modify B)
    // 0106: PULS same mask (restore) then RTS (simulate with pulling PC too by including PC bit 0x80)
    // We'll include PC bit so PC is restored to 0x0108 after pull (simulate stack return). Put a placeholder target at 0x0200 to detect mismatch.

    // Adjust mask to include PC (0x80) and X (0x10) -> 0x9F for PULS; PSHS uses same mask.

    cpu.mem[0x0100] = 0x34; cpu.bus.mem[0x0100] = 0x34; // PSHS
    cpu.mem[0x0101] = 0x9F; cpu.bus.mem[0x0101] = 0x9F;
    cpu.mem[0x0102] = 0x86; cpu.bus.mem[0x0102] = 0x86; // LDA #
    cpu.mem[0x0103] = 0x99; cpu.bus.mem[0x0103] = 0x99;
    cpu.mem[0x0104] = 0xC6; cpu.bus.mem[0x0104] = 0xC6; // LDB #
    cpu.mem[0x0105] = 0x55; cpu.bus.mem[0x0105] = 0x55;
    cpu.mem[0x0106] = 0x35; cpu.bus.mem[0x0106] = 0x35; // PULS
    cpu.mem[0x0107] = 0x9F; cpu.bus.mem[0x0107] = 0x9F;
    // After pull with PC bit, execution continues at restored PC; ensure saved PC equals address right after PSHS (0x0102)

    // Step PSHS
    cpu.step();
    // Step LDA #$99
    cpu.step();
    // Step LDB #$55
    cpu.step();
    // Step PULS (restores registers including PC, A, B, X, DP, CC)
    cpu.step();

    assert_eq!(cpu.a, 0x12, "A should be restored from stack");
    assert_eq!(cpu.b, 0x34, "B should be restored from stack");
    assert_eq!(cpu.x, 0x1111, "X should be restored");
    assert_eq!(cpu.dp, 0xD0, "DP restored");
    // PC restored to 0x0102 (next instruction after PSHS fetch) due to push order; verify.
    assert_eq!(cpu.pc, 0x0102, "PC should restore to address after PSHS");
}
