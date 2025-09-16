use vectrex_emulator::integrator::Integrator;

#[test]
fn splits_long_segments() {
    let mut integ = Integrator::new();
    integ.set_max_segment_length(50.0);
    integ.set_merge(false); // ensure we can observe splitting without re-merge
    integ.beam_on();
    integ.set_intensity(100);
    // velocity large enough to exceed the max length over 10 cycles
    integ.set_velocity(30.0, 0.0); // 30 * 20 = 600 which should split with max 50
    integ.tick(20, 0);
    let segs = integ.segments_slice();
    assert!(segs.len() > 1, "Expected splitting, got {} segments", segs.len());
    // Each individual piece should not exceed 60 length (a little slack for float rounding)
    for s in segs { let dx = s.x1 - s.x0; assert!(dx.abs() <= 60.0, "segment too long: {}", dx.abs()); }
}

#[test]
fn blank_slews_recorded_when_enabled() {
    let mut integ = Integrator::new();
    integ.set_record_blank_slews(true);
    integ.set_velocity(10.0, 0.0);
    // beam is off, should record blank segment
    integ.tick(5, 0);
    assert_eq!(integ.segments_slice().len(), 1, "Should record one blank slew segment");
    assert_eq!(integ.segments_slice()[0].intensity, 0);
}

#[test]
fn origin_reset_changes_origin() {
    let mut integ = Integrator::new();
    integ.instant_move(0.0, 0.0);
    integ.reset_origin();
    integ.instant_move(100.0, 50.0);
    integ.reset_origin();
    let (ox, oy) = integ.origin();
    assert_eq!(ox, 100.0);
    assert_eq!(oy, 50.0);
}
