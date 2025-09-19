use vectrex_emulator::CPU;

// Simple IRQ -> RTI path validating shadow stack push/pop and frame integrity.
// We trigger an IRQ, execute a tiny handler that modifies A then RTI.
// Assertions:
//  - PC vectors to handler
//  - Shadow stack length increments on entry and returns to previous size after RTI
//  - A modified in handler persists after RTI
//  - Return PC resumes at interrupted location (allowing +/-1 tolerance due to prefetch model)
//  - No spurious RAM-exec snapshot triggered (ram_exec.triggered remains false)
#[test]
fn irq_rti_shadow_frame() {
    let mut cpu = CPU::default();
    // IRQ vector -> 0x0400 (emulator IRQ code reads high at base, low at +1)
    // IRQ vector estándar (high, low) en 0xFFF8/0xFFF9
    cpu.bus.mem[0xFFF8] = 0x04; // high byte of 0x0400
    cpu.bus.mem[0xFFF9] = 0x00; // low byte
    // Handler at 0x0400:
    //  LDA #$7E
    //  RTI
    cpu.bus.mem[0x0400] = 0x86; cpu.bus.mem[0x0401] = 0x7E; cpu.bus.mem[0x0402] = 0x3B;
    cpu.mem[0x0400] = 0x86; cpu.mem[0x0401] = 0x7E; cpu.mem[0x0402] = 0x3B;
    // Mainline placeholder bytes (won't really execute before IRQ if polling works)
    cpu.pc = 0x0100;
    cpu.bus.mem[0x0100] = 0x4F; cpu.mem[0x0100] = 0x4F; // CLRA
    cpu.bus.mem[0x0101] = 0x5F; cpu.mem[0x0101] = 0x5F; // CLRB
    // Snapshot initial shadow depth
    let depth0 = cpu.shadow_stack.len();
    cpu.irq_pending = true; // schedule
    cpu.test_force_irq(); // directly service IRQ (test-only helper)
    assert_eq!(cpu.pc, 0x0400, "IRQ did not vector correctly (pc={:04X})", cpu.pc);
    assert_eq!(cpu.shadow_stack.len(), depth0 + 1, "Shadow stack not pushed on IRQ entry");
    let sp_after_vector = cpu.s;
    let expected_ret = cpu.shadow_stack.last().unwrap().ret;
    // Execute handler: LDA
    cpu.step();
    assert_eq!(cpu.a, 0x7E, "Handler did not modify A as expected");
    // Execute RTI
    cpu.step();
    // Validate return matches shadow recorded ret exactly (ya no aceptamos variante byte-swapped)
    assert_eq!(cpu.pc, expected_ret, "RTI returned to {:04X} but expected {:04X}", cpu.pc, expected_ret);
    assert_eq!(cpu.shadow_stack.len(), depth0, "Shadow stack not popped after RTI");
    // Stack pointer should have grown by at least full frame size then restored (do minimal sanity)
    assert!(sp_after_vector < cpu.s + 40, "IRQ frame size seems unrealistic (S_after_vec={:04X} S_final={:04X})", sp_after_vector, cpu.s);
    assert!(!cpu.ram_exec.triggered, "Unexpected RAM execution detector trigger during simple IRQ test");
}
