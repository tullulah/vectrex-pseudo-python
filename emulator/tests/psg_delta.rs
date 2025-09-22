//! Test delta PCM export para AyPsg.
//! Verifica:
//! 1. Delta vacío cuando no avanza el tiempo.
//! 2. Delta menor que snapshot completo tras pocos ticks.
//! 3. Overflow: si generamos >= tamaño ring entre lecturas, se marca overflow y longitud coincide con snapshot completa.

use vectrex_emulator::psg_ay::AyPsg;

#[test]
fn psg_delta_basic() {
    let mut psg = AyPsg::new(1_500_000, 50_000, 12); // ring = 4096 muestras aprox
    // Sin avance
    let d0 = psg.prepare_delta_export();
    assert_eq!(d0, 0, "delta inicial debe ser 0");
    assert_eq!(psg.delta_overflow(), false);
    // Generar unas pocas muestras
    for _ in 0..200 { psg.tick(64); } // produce algunas decenas de samples
    let d1 = psg.prepare_delta_export();
    assert!(d1 > 0 && d1 < 4096, "delta razonable < ring size");
    assert_eq!(psg.delta_overflow(), false, "no debe overflow en delta parcial");
    let ptr = psg.delta_ptr();
    assert!(!ptr.is_null(), "puntero delta no nulo cuando len>0");
    // Nueva llamada sin avance
    let d2 = psg.prepare_delta_export();
    assert_eq!(d2, 0, "sin nuevas muestras delta=0");
}

#[test]
fn psg_delta_overflow() {
    let mut psg = AyPsg::new(1_500_000, 50_000, 10); // ring = 1024
    // Generar > ring muestras antes de primer delta
    // cycles_per_sample ~ 30 => para 1500 muestras ~ 45k cycles aproximado
    for _ in 0..60_000 { psg.tick(1); }
    let d = psg.prepare_delta_export();
    assert_eq!(d, 1024, "overflow debe devolver snapshot completo");
    assert_eq!(psg.delta_overflow(), true, "flag overflow debe estar activo");
}
