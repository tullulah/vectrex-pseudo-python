//! Test de integración BIOS ampliado: verifica estabilidad de DP a lo largo de una secuencia
//! real en BIOS donde aparecen instrucciones inmediatas antes de un TFR A,DP y confirma que
//! sólo el TFR produce la mutación. Refuerza política de "no side effects sintéticos".
//!
//! Ruta BIOS absoluta según instrucciones persistentes.

use vectrex_emulator::CPU;

fn load_real_bios() -> Vec<u8> {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    std::fs::read(path).expect("No se pudo leer bios.bin real")
}

#[test]
fn dp_only_changes_on_explicit_tfr_sequence() {
    let bios = load_real_bios();
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    assert!(cpu.bios_present);

    // Secuencia: llamamos a rutina en F1AF (como en test previo) pero avanzamos algunos bytes extra
    // para asegurar que ninguna instrucción intermedia muta DP salvo el TFR.
    cpu.pc = 0x0300;
    cpu.test_write8(0x0300, 0xBD); // JSR ext
    cpu.test_write8(0x0301, 0xF1); cpu.test_write8(0x0302, 0xAF); // destino
    cpu.test_write8(0x0303, 0x12); // NOP tras retorno

    let dp_start = cpu.dp; assert_eq!(dp_start, 0xD0, "DP reset inicial esperado D0");

    // JSR
    assert!(cpu.step());
    assert_eq!(cpu.pc, 0xF1AF);
    assert_eq!(cpu.dp, dp_start, "DP no cambia en JSR");

    // LDA #$C8 (en F1AF) -> no cambia DP
    assert!(cpu.step());
    assert_eq!(cpu.dp, dp_start, "DP permanece antes de TFR");

    // TFR A,DP -> cambia
    assert!(cpu.step());
    assert_eq!(cpu.dp, 0xC8, "DP muta exactamente en TFR A,DP");

    // RTS
    assert!(cpu.step());
    assert_eq!(cpu.pc, 0x0303);
    assert_eq!(cpu.dp, 0xC8);

    // NOP
    assert!(cpu.step());
    assert_eq!(cpu.dp, 0xC8);
}
