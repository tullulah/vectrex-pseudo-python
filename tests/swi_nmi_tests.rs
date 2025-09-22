use vectrex_emulator::CPU;

// NOTE: If crate path differs, we may need to use crate::emulator or pseudo_python::core etc.
// Placeholder test scaffolding; will be updated after verifying module paths.

#[test]
fn swi_full_frame_restores_registers() {
    let mut cpu = CPU::default();
    // Install SWI vector at FFF8/FFF9 pointing to 0x0200
    cpu.bus.mem[0xFFF8] = 0x00; cpu.bus.mem[0xFFF9] = 0x02;
    cpu.bus.mem[0xFFF8] = 0x00; cpu.bus.mem[0xFFF9] = 0x02;
    // Handler at 0x0200: CLRA ; RTI
    cpu.bus.mem[0x0200] = 0x4F; // CLRA
    cpu.bus.mem[0x0201] = 0x3B; // RTI
    cpu.bus.mem[0x0200] = 0x4F; cpu.bus.mem[0x0201] = 0x3B;

    cpu.pc = 0x0100;
    // Place SWI opcode at 0x0100
    cpu.bus.mem[0x0100] = 0x3F; cpu.bus.mem[0x0100] = 0x3F;

    cpu.a = 0x12; cpu.b = 0x34; cpu.dp = 0xD0; cpu.x = 0x2222; cpu.y = 0x3333; cpu.u = 0x4444;

    // Execute SWI
    cpu.step(); // fetch & execute SWI -> jump to handler
    assert_eq!(cpu.pc, 0x0200, "SWI did not vector correctly");

    // Execute handler CLRA
    cpu.step();
    // Execute RTI
    cpu.step();

    // After RTI, A restored to original (0x12) because full frame stacked
    assert_eq!(cpu.a, 0x12, "A should be restored from full frame, not 0");
    assert_eq!(cpu.b, 0x34);
    assert_eq!(cpu.x, 0x2222);
    assert_eq!(cpu.y, 0x3333);
    assert_eq!(cpu.u, 0x4444);
}

#[test]
fn swi2_vectors_correctly() {
    let mut cpu = CPU::default();
    // SWI2 vector FFF2/FFF3 -> 0x0300
    cpu.bus.mem[0xFFF2] = 0x00; cpu.bus.mem[0xFFF3] = 0x03; cpu.bus.mem[0xFFF2] = 0x00; cpu.bus.mem[0xFFF3] = 0x03;
    cpu.bus.mem[0x0300] = 0x3B; cpu.bus.mem[0x0300] = 0x3B; // RTI only

    cpu.pc = 0x0100;
    // Encode SWI2 sequence (prefix 0x10 then 0x3F based on our temporary mapping)
    cpu.bus.mem[0x0100] = 0x10; cpu.bus.mem[0x0100] = 0x10;
    cpu.bus.mem[0x0101] = 0x3F; cpu.bus.mem[0x0101] = 0x3F;

    cpu.step(); // execute prefix and service
    assert_eq!(cpu.pc, 0x0300, "SWI2 should vector to 0x0300");
    cpu.step(); // RTI
}

#[test]
fn swi3_vectors_correctly() {
    let mut cpu = CPU::default();
    // SWI3 vector FFF0/FFF1 -> 0x0310
    cpu.bus.mem[0xFFF0] = 0x10; cpu.bus.mem[0xFFF1] = 0x03; cpu.bus.mem[0xFFF0] = 0x10; cpu.bus.mem[0xFFF1] = 0x03;
    cpu.bus.mem[0x0310] = 0x3B; cpu.bus.mem[0x0310] = 0x3B; // RTI

    cpu.pc = 0x0100;
    // Correct 6809 encoding for SWI3 is prefix 0x11 then 0x3F
    cpu.bus.mem[0x0100] = 0x11; cpu.bus.mem[0x0100] = 0x11;
    cpu.bus.mem[0x0101] = 0x3F; cpu.bus.mem[0x0101] = 0x3F;

    cpu.step();
    assert_eq!(cpu.pc, 0x0310, "SWI3 should vector to 0x0310");
    cpu.step(); // RTI
}

#[test]
fn nmi_full_frame_masks_irqs() {
    let mut cpu = CPU::default();
    // NMI vector FFFA/FFFB -> 0x0400
    cpu.bus.mem[0xFFFA] = 0x00; cpu.bus.mem[0xFFFB] = 0x04; cpu.bus.mem[0xFFFA] = 0x00; cpu.bus.mem[0xFFFB] = 0x04;
    cpu.bus.mem[0x0400] = 0x3B; cpu.bus.mem[0x0400] = 0x3B; // RTI only

    cpu.pc = 0x0100;
    cpu.nmi_pending = true;

    cpu.step(); // should service NMI before fetching opcode at 0x0100
    assert_eq!(cpu.pc, 0x0400, "NMI did not vector correctly");
    assert!(cpu.cc_i, "I flag should be set after NMI");
    cpu.step(); // RTI
}
