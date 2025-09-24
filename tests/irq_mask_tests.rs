use vectrex_emulator::CPU;

// Test that ORCC can set I (mask IRQ) and ANDCC can clear it, controlling IRQ servicing.
#[test]
fn irq_mask_prevents_and_then_allows_irq() {
    let mut cpu = CPU::default();
    // Install IRQ vector at FFF6/FFF7 -> 0x0200
    cpu.bus.mem[0xFFF6] = 0x00; cpu.bus.mem[0xFFF7] = 0x02;
    cpu.bus.mem[0xFFF6] = 0x00; cpu.bus.mem[0xFFF7] = 0x02;
    // IRQ handler at 0x0200: CLRA ; RTI
    cpu.bus.mem[0x0200] = 0x4F; cpu.bus.mem[0x0200] = 0x4F;
    cpu.bus.mem[0x0201] = 0x3B; cpu.bus.mem[0x0201] = 0x3B;

    // Program layout:
    // 0100: ORCC #$10 (set I)
    // 0102: (simulate IRQ pending before executing next NOP) -> should NOT service due to I=1
    // 0102: NOP (we'll use 0x12 which is otherwise unused/unimplemented -> expect unimpl false stop after executing masking logic?)
    // Instead, use a harmless implemented op: LDA #$01 (0x86 0x01)
    // 0104: ANDCC #$EF (clear I bit since EFHINZVC & 0x10 cleared)
    // 0106: (IRQ pending again) -> should service now (PC jumps to 0x0200)
    // 0106: NOP placeholder if not serviced (won't execute)

    cpu.pc = 0x0100;
    cpu.bus.mem[0x0100] = 0x1A; cpu.bus.mem[0x0100] = 0x1A; // ORCC
    cpu.bus.mem[0x0101] = 0x10; cpu.bus.mem[0x0101] = 0x10; // set I
    cpu.bus.mem[0x0102] = 0x86; cpu.bus.mem[0x0102] = 0x86; // LDA #$01
    cpu.bus.mem[0x0103] = 0x01; cpu.bus.mem[0x0103] = 0x01;
    cpu.bus.mem[0x0104] = 0x1C; cpu.bus.mem[0x0104] = 0x1C; // ANDCC
    cpu.bus.mem[0x0105] = 0xEF; cpu.bus.mem[0x0105] = 0xEF; // clear I bit (mask retains others)
    cpu.bus.mem[0x0106] = 0x86; cpu.bus.mem[0x0106] = 0x86; // LDA #$02 (should not run until after IRQ service)
    cpu.bus.mem[0x0107] = 0x02; cpu.bus.mem[0x0107] = 0x02;

    // Step ORCC -> I set
    cpu.step();
    assert!(cpu.cc_i, "I flag should be set by ORCC");

    // Set an IRQ pending (simulate VIA line asserted). We directly set irq_pending flag.
    cpu.irq_pending = true;

    // Execute LDA #$01; IRQ should NOT service because I=1
    cpu.step();
    assert_eq!(cpu.a, 0x01, "LDA should execute normally with IRQ masked");
    assert_eq!(cpu.pc, 0x0104, "PC should advance to ANDCC instruction");
    assert!(cpu.cc_i, "I flag should still be set (IRQ masked)");

    // Execute ANDCC #$EF to clear I
    cpu.step();
    assert!(!cpu.cc_i, "I flag should be cleared by ANDCC");

    // IRQ still pending -> should now service before executing next opcode
    cpu.step();
    assert_eq!(cpu.pc, 0x0200, "IRQ should vector now that I cleared");
    // Execute handler (CLRA + RTI)
    cpu.step(); // CLRA
    cpu.step(); // RTI

    // After RTI we return to 0x0106 (next instruction after ANDCC) and execute LDA #$02
    assert_eq!(cpu.pc, 0x0106, "Return address should be next instruction after ANDCC");
    cpu.step();
    assert_eq!(cpu.a, 0x02, "Post-IRQ instruction executed");
}
