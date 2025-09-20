//! Barridos adicionales:
//! 1. Prefijos extendidos 0x10 / 0x11: verificar que todos los sub‑opcodes válidos ejecutan (ok && avanzan ciclos).
//! 2. VIA register sweep básico: lecturas 0xD000..0xD00F no paniquean y bit7 IFR coherente.

use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11};
use vectrex_emulator::ILLEGAL_BASE_OPCODES;
use vectrex_emulator::cycle_table::{CYCLES_PREFIX10, CYCLES_PREFIX11, INVALID};

#[test]
fn opcode_extended_full_sweep_unimplemented() {
    let mut cpu = CPU::default();
    // Recalcular cobertura para poblar bitmap y lista extendida
    let (_impld, _unimpl, _list) = cpu.recompute_opcode_coverage();
    assert!(cpu.extended_unimplemented_list().is_empty(), "Quedan sub-opcodes extendidos sin implementar: {:?}", cpu.extended_unimplemented_list());
    // Ejecución real de cada sub-opcode válido (prefijos 0x10 y 0x11)
    for &(prefix, table, valid) in &[(0x10u8, CYCLES_PREFIX10.as_ref(), VALID_PREFIX10 as &[u8]), (0x11u8, CYCLES_PREFIX11.as_ref(), VALID_PREFIX11 as &[u8])] {
        for &sub in valid {
            let expect_valid = table[sub as usize] != INVALID;
            cpu.pc = 0x0800;
            cpu.test_write8(0x0800, prefix);
            cpu.test_write8(0x0801, sub);
            cpu.test_write8(0x0802, 0x00);
            cpu.test_write8(0x0803, 0x00);
            let before_c = cpu.cycles;
            let ok = cpu.step();
            let delta = cpu.cycles - before_c;
            if expect_valid {
                assert!(ok, "Prefijo {:02X} sub {:02X} devolvió ok=false", prefix, sub);
                assert!(delta > 0, "Prefijo {:02X} sub {:02X} no avanzó ciclos", prefix, sub);
            }
        }
    }
}

#[test]
fn via_register_sweep_basic() {
    let mut cpu = CPU::default();
    // Antes de interactuar con VIA: vigilar que no haya huecos de implementación (primarios ni extendidos)
    let (_impld0, _cnt0, primary_missing0) = cpu.recompute_opcode_coverage();
    let expected_illegals: std::collections::BTreeSet<u8> = ILLEGAL_BASE_OPCODES.iter().cloned().collect();
    let got_primary: std::collections::BTreeSet<u8> = primary_missing0.iter().cloned().collect();
    assert_eq!(got_primary, expected_illegals, "Huecos primarios antes de VIA no coinciden con ilegales. got={:?} expected={:?}", got_primary, expected_illegals);
    assert!(cpu.extended_unimplemented_list().is_empty(), "Huecos extendidos antes de VIA: {:?}", cpu.extended_unimplemented_list());
    // Asegurar memoria limpia en ventana VIA
    for ofs in 0xD000..=0xD00F { let _ = cpu.test_read8(ofs); } // lecturas iniciales
    // IFR coherencia: bit7 == (ifr & 0x7F != 0)
    let ifr1 = cpu.test_read8(0xD00D);
    assert_eq!(((ifr1 >> 7) & 1) != 0, (ifr1 & 0x7F) != 0, "IFR bit7 inconsistente al inicio (0x{:02X})", ifr1);
    // Escribir eco en ORB / ORA / ACR / PCR (considerados seguros) con mismo valor leído
    let orb = cpu.test_read8(0xD000); cpu.test_write8(0xD000, orb);
    let ora = cpu.test_read8(0xD001); cpu.test_write8(0xD001, ora);
    let acr = cpu.test_read8(0xD00B); cpu.test_write8(0xD00B, acr);
    let pcr = cpu.test_read8(0xD00C); cpu.test_write8(0xD00C, pcr);
    // Ejecutar unas NOPs (0x12) para permitir que timers (si configurados) avancen sin efectos
    for i in 0..8 { cpu.pc = 0x0900 + i; cpu.test_write8(0x0900 + i, 0x12); let _ = cpu.step(); }
    let ifr2 = cpu.test_read8(0xD00D);
    assert_eq!(((ifr2 >> 7) & 1) != 0, (ifr2 & 0x7F) != 0, "IFR bit7 inconsistente tras pasos (0x{:02X})", ifr2);
    // Tras interacción con VIA: volver a vigilar cobertura por si algún refactor futuro modifica tablas dinámicamente
    let (_impld1, _cnt1, primary_missing1) = cpu.recompute_opcode_coverage();
    let got_after: std::collections::BTreeSet<u8> = primary_missing1.iter().cloned().collect();
    assert_eq!(got_after, expected_illegals, "Huecos primarios tras VIA cambiaron. got={:?} expected={:?}", got_after, expected_illegals);
    assert!(cpu.extended_unimplemented_list().is_empty(), "Huecos extendidos tras VIA: {:?}", cpu.extended_unimplemented_list());
}
