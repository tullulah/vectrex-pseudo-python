use vectrex_emulator::CPU;

// Baseline IRQ behavior test (pre-refactor):
// - Arms Timer1 to generate IRQ
// - Enters WAI
// - IRQ vectors to handler which immediately RTS
// - Verifies key registers (A,B,DP,X,U) unchanged since handler did not modify them
// NOTE: This locks current semantics (RTS used instead of RTI). After refactor this test will
// be replaced or adjusted to use RTI and verify proper stack pop.
#[test]
fn irq_baseline_registers_unchanged_after_simple_handler() {
    let mut cpu = CPU::default();
    // Set initial register state
    cpu.a = 0x12; cpu.b = 0x04; cpu.dp = 0xD0; cpu.x = 0x1234; cpu.u = 0x5678;
    let start_s = cpu.s;
    // Program start at 0x0000: arm timer and WAI
    // Use existing instruction subset:
    //  LDA #$00 (high byte)
    //  LDB #$20 (low byte for timer; large enough so WAI executes before underflow)
    //  STB $D004 (T1 low)
    //  STA $D005 (T1 high -> load 0x0004)
    //  LDA #$C0 (enable T1 IRQ)
    //  STA $D00E
    //  WAI
    let prog = [
        0x86,0x00,      // LDA #00
    0xC6,0x20,      // LDB #20
        0xD7,0x04,      // STB $D004
        0x97,0x05,      // STA $D005
        0x86,0xC0,      // LDA #C0 (IER set + bit6)
        0x97,0x0E,      // STA $D00E
        0x3E            // WAI
    ];
    for (i,b) in prog.iter().enumerate(){ cpu.mem[i]=*b; cpu.bus.mem[i]=*b; }
    // IRQ vector (FFF6/FFF7) -> 0x0200
    cpu.mem[0xFFF6]=0x00; cpu.mem[0xFFF7]=0x02; cpu.bus.mem[0xFFF6]=0x00; cpu.bus.mem[0xFFF7]=0x02;
    // Handler at 0x0200: RTI only (proper full frame restore)
    cpu.mem[0x0200]=0x3B; cpu.bus.mem[0x0200]=0x3B;
    cpu.pc = 0x0000;
    // Ensure IRQs unmasked prior to running (I flag false)
    cpu.cc_i = false;
    // Run until WAI
    for _ in 0..20 { if cpu.wai_halt { break } cpu.step(); }
    assert!(cpu.wai_halt, "Did not enter WAI");
    // Spin until timer hits zero and IRQ taken
    for _ in 0..32 { if !cpu.wai_halt { break } cpu.step(); }
    assert!(!cpu.wai_halt, "IRQ did not release WAI");
    if cpu.pc == 0x0200 {
        let sp_during_handler = cpu.s; // after push
        let expected_delta = 12; // full IRQ frame bytes
        assert_eq!(start_s - sp_during_handler, expected_delta as u16, "IRQ frame depth mismatch");
        cpu.step(); // RTI
    }
    assert_eq!(cpu.pc, prog.len() as u16, "Return PC unexpected: {:04X}", cpu.pc);
    // After RTI stack pointer restored
    assert_eq!(cpu.s, start_s, "Stack pointer not restored after RTI");
    // Registers unchanged (B holds programmed timer low byte 0x20)
    assert_eq!(cpu.a, 0xC0, "A changed unexpectedly (expected last loaded value for IER)");
    assert_eq!(cpu.b, 0x20, "B changed unexpectedly");
    assert_eq!(cpu.dp, 0xD0, "DP changed unexpectedly");
    assert_eq!(cpu.x, 0x1234, "X changed unexpectedly");
    assert_eq!(cpu.u, 0x5678, "U changed unexpectedly");
    // Final state validated by earlier assertion
}
