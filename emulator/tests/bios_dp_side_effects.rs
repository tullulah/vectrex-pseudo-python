//! Test de integración BIOS: mutación de DP en llamada a DP_to_C8.
//! Objetivo: documentar que el cambio D0->C8 proviene de la rutina BIOS (y del side effect en record_bios_call),
//! no de una instrucción LDA inmediata posterior.
//!
//! Política BIOS: usa BIOS real (ruta absoluta indicada en instrucciones persistentes).

use vectrex_emulator::CPU;

fn load_real_bios() -> Vec<u8> {
    // Ruta oficial (no sintetizar):
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    std::fs::read(path).expect("No se pudo leer bios.bin real; asegúrate de que existe la ruta oficial")
}

#[test]
fn dp_transition_via_bios_dp_to_c8() {
    let bios = load_real_bios();
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    assert!(cpu.bios_present, "BIOS debe marcarse presente tras load_bios");
    // Programa en 0x0200: JSR $F1AF (DP_to_C8), luego NOP (0x12) para observar DP tras retorno.
    cpu.pc = 0x0200;
    // Escribir mediante helper para reflejar también en bus (read8 ahora delega al bus)
    cpu.test_write8(0x0200, 0xBD); // JSR extended
    cpu.test_write8(0x0201, 0xF1); cpu.test_write8(0x0202, 0xAF); // target F1AF
    cpu.test_write8(0x0203, 0x12); // NOP
    let dp_initial = cpu.dp; assert_eq!(dp_initial, 0xD0, "Estado inicial DP esperado D0 por reset por defecto");

    // Paso 1: ejecutar JSR; en modo estricto (único ahora) NO debe mutar DP todavía.
    // Asegura bandera bios_present (defensivo en caso de futuros refactors):
    cpu.bios_present = true; 
    let ok = cpu.step(); assert!(ok, "JSR step");
    assert_eq!(cpu.pc, 0xF1AF, "PC debe saltar a rutina BIOS");
    assert_eq!(cpu.dp, 0xD0, "DP NO debe cambiar todavía (sin side effects sintéticos)");

    // Paso 2: ejecutar LDA #$C8 (primera instrucción rutina) — aún no cambia DP.
    let ok2 = cpu.step(); assert!(ok2);
    assert_eq!(cpu.pc, 0xF1B1, "PC tras LDA immediate (2 bytes)");
    assert_eq!(cpu.dp, 0xD0, "DP permanece D0 tras LDA inmediata");

    // Paso 3: ejecutar TFR A,DP (aquí recién debe cambiar a C8)
    let ok3 = cpu.step(); assert!(ok3);
    assert_eq!(cpu.dp, 0xC8, "DP cambia a C8 únicamente al ejecutar TFR A,DP");

    // Paso 4: ejecutar RTS y volver a 0x0203
    let ok4 = cpu.step(); assert!(ok4);
    assert_eq!(cpu.pc, 0x0203, "Retorno correcto tras JSR BIOS");
    assert_eq!(cpu.dp, 0xC8, "DP estable tras retorno");

    // Paso 5: NOP no debe alterar DP
    let ok5 = cpu.step(); assert!(ok5);
    assert_eq!(cpu.dp, 0xC8, "DP permanece C8 tras instrucción no relacionada");
}
