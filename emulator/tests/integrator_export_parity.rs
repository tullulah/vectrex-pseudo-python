use vectrex_emulator::CPU;

// This test exercises that the draining JSON export and the shared-memory copy
// (segments_c_copy) produce the same count and matching first element fields.
// It does NOT use the wasm bindings (which require wasm32 target), but validates
// the underlying transformation path used by them.
#[test]
fn integrator_json_vs_c_copy_parity() {
    let mut cpu = CPU::default();
    cpu.integrator.set_merge(false); // simplify: avoid merging changing counts mid-way
    cpu.integrator.instant_move(0.0, 0.0);
    cpu.integrator.set_velocity(1.0, 0.5);
    cpu.integrator.set_intensity(42);
    cpu.integrator.beam_on();

    // Generate a few segments by changing velocity and ticking
    cpu.integrator.tick(4, 0); // segment 0: 0->(4,2)
    cpu.integrator.set_velocity(0.0, 1.0);
    cpu.integrator.tick(3, 0); // segment 1
    cpu.integrator.set_velocity(-2.0, 0.0);
    cpu.integrator.tick(2, 0); // segment 2

    let original_count = cpu.integrator.segments.len();
    assert!(original_count >= 3, "expected at least 3 segments, got {original_count}");

    // Obtain C copy (non-draining) then JSON (draining) and compare lengths
    let copy = cpu.integrator.segments_c_copy();
    assert_eq!(copy.len(), original_count, "C copy length mismatch");

    // Now drain
    let drained = cpu.integrator.take_segments();
    assert_eq!(drained.len(), original_count, "Drained length mismatch");

    // Compare first element detailed fields
    let s_ref = &drained[0];
    let s_c = &copy[0];
    assert_eq!(s_ref.x0, s_c.x0, "x0 mismatch");
    assert_eq!(s_ref.y0, s_c.y0, "y0 mismatch");
    assert_eq!(s_ref.x1, s_c.x1, "x1 mismatch");
    assert_eq!(s_ref.y1, s_c.y1, "y1 mismatch");
    assert_eq!(s_ref.intensity, s_c.intensity, "intensity mismatch");
    assert_eq!(s_ref.frame, s_c.frame, "frame mismatch");

    // After drain, internal segments should be empty
    assert_eq!(cpu.integrator.segments.len(), 0, "segments not emptied after drain");
}
