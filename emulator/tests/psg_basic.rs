use vectrex_emulator::{Bus};

#[test]
fn psg_generates_samples() {
    let mut bus = Bus::default();
    // Escribir periodo canal A muy bajo para forzar toggles rápidos.
    // Reg 0 (fine), 1 (coarse lower 4 bits)
    bus.psg.write_reg(0, 0x01); // fine = 1
    bus.psg.write_reg(1, 0x00); // coarse = 0 => periodo = 1 -> se normaliza a 1
    // Habilitar tono canal A, deshabilitar ruido canal A en mixer (reg7 bits 0= tone A disable, 3= noise A disable)
    // Queremos tone_enabled => bit0 = 0, noise_enabled => podemos dejar ruido habilitado también.
    bus.psg.write_reg(7, 0b0011_1110 & !0b0000_0001); // asegurar bit0=0
    // Amplitud canal A (reg8) = 0x0F (máx, sin envolvente)
    bus.psg.write_reg(8, 0x0F);

    // Avanzar suficientes ciclos para generar muestras.
    for _ in 0..10 { bus.tick(256); }

    assert!(bus.psg.metric_samples > 0, "No samples generated");
    assert!(bus.psg.metric_tone_toggles > 0, "Tone channel did not toggle");

    // Preparar export y verificar longitud > 0
    let prepared = bus.psg.prepare_export();
    assert!(prepared > 0, "Prepared export length zero");
    let ptr = bus.psg.export_ptr();
    assert!(!ptr.is_null(), "Export pointer is null");
}
