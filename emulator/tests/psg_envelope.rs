use vectrex_emulator::Bus;

// Helper: avanza ticks suficientes para generar algunas muestras y pasos de envolvente
fn run(bus: &mut Bus, cycles: u32, reps: u32){ for _ in 0..reps { bus.tick(cycles); } }

#[test]
fn envelope_attack_hold() {
    let mut bus = Bus::default();
    // Canal A usa envolvente (bit4 en reg8)
    bus.psg.write_reg(8, 0x10); // env enable, amplitude nibble ignorado
    // Periodo envolvente pequeño para avanzar rápido
    bus.psg.write_reg(11, 0x04); // low
    bus.psg.write_reg(12, 0x00); // high -> periodo=0x0004
    // Shape: Attack=1 (bit2), Continue=0 (bit3=0), Alternate=0, Hold=1 -> sube y se queda (A.. + hold one-shot)
    // bits: C A Alt Hold => 0 1 0 1 => 0b0101 = 0x5
    bus.psg.write_reg(13, 0x05);
    let initial_serial = bus.psg.export_serial();
    run(&mut bus, 64, 200); // avanzar
    assert!(bus.psg.metric_env_steps > 0, "Envelope did not step");
    // Debe haberse detenido (hold) => no más cambios posteriores tras muchas iteraciones
    let steps_after = bus.psg.metric_env_steps;
    run(&mut bus, 64, 200);
    assert_eq!(steps_after, bus.psg.metric_env_steps, "Envelope should hold, but steps increased");
    assert!(bus.psg.export_serial() >= initial_serial, "Serial should not decrement");
}

#[test]
fn log_curve_monotonic() {
    let mut bus = Bus::default();
    // Forzar tono estable con amplitud creciente y medir muestras medias
    // Usar sólo canal A, sin envolvente.
    bus.psg.write_reg(7, 0b0011_1110 & !0b0000_0001); // habilitar tone A
    let mut last_energy = 0.0f64;
    for amp in 1..15 { // 1..14
        bus.psg.write_reg(8, amp as u8); // nibble directo
        run(&mut bus, 64, 50);
        // preparar export y calcular RMS approx de ventana pequeña
        bus.psg.prepare_export();
        let len = bus.psg.export_len();
        let slice = unsafe { std::slice::from_raw_parts(bus.psg.export_ptr(), len) };
        let sample_count = 256.min(slice.len());
        let mut sum2=0.0;
        for &s in &slice[..sample_count] { let v = s as f64 / 32767.0; sum2 += v*v; }
        let rms = (sum2 / sample_count as f64).sqrt();
        assert!(rms >= last_energy * 0.95, "Log curve not roughly monotonic amp={} prev={} now={}", amp, last_energy, rms);
        last_energy = rms;
    }
}
