use vectrex_emulator::{CPU};

// Helper to manually drive integrator without executing real opcodes: we directly manipulate
// velocity/intensity/beam flags through the public integrator and call advance via advance_cycles.
// advance_cycles is private, so we piggy-back on tick by simulating a NOP costing some cycles:
// We'll hack by directly calling integrator.tick which is public through CPU.integrator.

#[test]
fn single_segment_generated() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(true);
    cpu.integrator.instant_move(0.0, 0.0);
    cpu.integrator.set_velocity(1.0, 0.0);
    cpu.integrator.set_intensity(10);
    cpu.integrator.beam_on();
    cpu.integrator.tick(5, 0); // simulate 5 cycles in frame 0
    assert_eq!(cpu.integrator.segments.len(), 1, "Expected one segment");
    let s = &cpu.integrator.segments[0];
    assert_eq!((s.x0, s.y0, s.x1, s.y1, s.intensity), (0.0, 0.0, 5.0, 0.0, 10));
}

#[test]
fn collinear_merge() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(true);
    cpu.integrator.instant_move(0.0, 0.0);
    cpu.integrator.set_velocity(1.0, 0.0); cpu.integrator.set_intensity(5); cpu.integrator.beam_on();
    cpu.integrator.tick(3, 0); // segment 0->3
    cpu.integrator.tick(2, 0); // should merge to 0->5
    assert_eq!(cpu.integrator.segments.len(), 1, "Segments should merge when collinear");
    let s = &cpu.integrator.segments[0];
    assert_eq!(s.x0, 0.0); assert_eq!(s.x1, 5.0); assert_eq!(s.intensity, 5);
}

#[test]
fn no_merge_when_disabled() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(false);
    cpu.integrator.instant_move(0.0, 0.0);
    cpu.integrator.set_velocity(1.0, 0.0); cpu.integrator.set_intensity(8); cpu.integrator.beam_on();
    cpu.integrator.tick(2, 0); cpu.integrator.tick(2, 0);
    assert_eq!(cpu.integrator.segments.len(), 2, "Merging disabled should keep two segments");
}

#[test]
fn beam_off_no_segment() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(true);
    cpu.integrator.instant_move(1.0, 1.0);
    cpu.integrator.set_velocity(2.0, -1.0); cpu.integrator.set_intensity(9); cpu.integrator.beam_off();
    cpu.integrator.tick(4, 1); // blank move
    assert_eq!(cpu.integrator.segments.len(), 0, "Blanked beam should not create segment");
    // Position should have updated
    let (vx, vy) = cpu.integrator.velocity(); assert_eq!((vx, vy), (2.0, -1.0));
}

#[test]
fn zero_intensity_no_segment() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(true);
    cpu.integrator.instant_move(0.0, 0.0);
    cpu.integrator.set_velocity(1.0, 1.0); cpu.integrator.set_intensity(0); cpu.integrator.beam_on();
    cpu.integrator.tick(3, 2);
    assert_eq!(cpu.integrator.segments.len(), 0, "Zero intensity should suppress segment emission");
}
