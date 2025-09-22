use vectrex_emulator::CPU;

// Scenario: An IRQ handler (full frame) executes, then enables FIRQ arrival midway (by clearing I early).
// We simulate by: triggering IRQ; inside handler code we execute ANDCC to clear I, then set cpu.firq_pending=true
// and ensure that when the handler executes RTI, the pending FIRQ is serviced before normal code resumes.
// Current CPU model services interrupts at step() entry; after RTI returns, next step() poll should see FIRQ.

#[test]
fn firq_after_irq_unmask() {
    let mut cpu = CPU::default();
    // IRQ vector -> 0x0200, FIRQ vector -> 0x0220
    cpu.bus.mem[0xFFF6] = 0x00; cpu.bus.mem[0xFFF7] = 0x02; cpu.bus.mem[0xFFF6] = 0x00; cpu.bus.mem[0xFFF7] = 0x02;
    cpu.bus.mem[0xFFF4] = 0x20; cpu.bus.mem[0xFFF5] = 0x02; cpu.bus.mem[0xFFF4] = 0x20; cpu.bus.mem[0xFFF5] = 0x02;

    // IRQ handler at 0x0200:
    // 0200: ANDCC #$EF  (clear I to allow new interrupts)
    // 0202: LDA #$55    (some work)
    // 0204: RTI         (end IRQ)
    cpu.bus.mem[0x0200] = 0x1C; cpu.bus.mem[0x0200] = 0x1C; // ANDCC
    cpu.bus.mem[0x0201] = 0xEF; cpu.bus.mem[0x0201] = 0xEF; // clear I
    cpu.bus.mem[0x0202] = 0x86; cpu.bus.mem[0x0202] = 0x86; // LDA #
    cpu.bus.mem[0x0203] = 0x55; cpu.bus.mem[0x0203] = 0x55;
    cpu.bus.mem[0x0204] = 0x3B; cpu.bus.mem[0x0204] = 0x3B; // RTI

    // FIRQ handler at 0x0220:
    // 0220: LDB #$AA
    // 0222: RTI (partial frame)
    cpu.bus.mem[0x0220] = 0xC6; cpu.bus.mem[0x0220] = 0xC6; // LDB #
    cpu.bus.mem[0x0221] = 0xAA; cpu.bus.mem[0x0221] = 0xAA;
    cpu.bus.mem[0x0222] = 0x3B; cpu.bus.mem[0x0222] = 0x3B; // RTI

    // Main code at 0x0100: LDA #$01; (will be pre-empted by IRQ before executing if irq_pending set early)
    cpu.pc = 0x0100;
    cpu.bus.mem[0x0100] = 0x86; cpu.bus.mem[0x0100] = 0x86; cpu.bus.mem[0x0101] = 0x01; cpu.bus.mem[0x0101] = 0x01;

    // Trigger IRQ first
    cpu.irq_pending = true;
    // step: should service IRQ immediately, vector to 0x0200
    cpu.step();
    assert_eq!(cpu.pc, 0x0200, "IRQ should vector to handler");
    assert!(cpu.cc_i, "I should be set inside IRQ");

    // Execute ANDCC (clear I) allowing new interrupts
    cpu.step();
    assert!(!cpu.cc_i, "I should be cleared by ANDCC within IRQ handler");

    // Before next instruction of handler, schedule a FIRQ
    cpu.firq_pending = true;

    // Execute LDA #$55 inside IRQ handler
    cpu.step();
    assert_eq!(cpu.a, 0x55);

    // Execute RTI from IRQ handler (returns to main, but next step should see FIRQ)
    cpu.step();
    assert!(!cpu.in_irq_handler, "IRQ handler should have ended");

    // Now FIRQ should service
    cpu.step();
    assert_eq!(cpu.pc, 0x0220, "FIRQ should vector after IRQ RTI when pending and unmasked");

    // Execute FIRQ handler LDB #$AA then RTI
    cpu.step();
    assert_eq!(cpu.b, 0xAA);
    cpu.step(); // RTI

    // Resume main code at 0x0100 (first instruction not yet executed earlier)
    cpu.step();
    assert_eq!(cpu.a, 0x01, "Mainline LDA #$01 should finally execute after nested interrupts");
}
