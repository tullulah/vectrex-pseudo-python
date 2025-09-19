// Test: early RAM execution snapshot on invalid return to RAM region
// Uses real BIOS (policy: no synthetic BIOS). Crafts a call stack entry that
// returns directly into 0xC800 window and executes RTS to trigger early snapshot.

use vectrex_emulator::cpu6809::CPU;
use std::path::Path;

fn load_real_bios(cpu: &mut CPU) {
    let bios_path = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin"; // real BIOS path (no synthetic)
    let data = std::fs::read(Path::new(bios_path)).expect("BIOS real no encontrada");
    cpu.bus.load_bios_image(&data);
    cpu.bios_present = true;
}

#[test]
fn early_snapshot_on_rts_invalid_return() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    // Ejecutaremos RTS desde RAM (no BIOS) y prepararemos la pila real con retorno 0xC900.
    // 1) Colocar opcode RTS en RAM en una dirección cualquiera.
    cpu.pc = 0xC8F0;
    cpu.test_write8(0xC8F0, 0x39);
    // 2) Preparar la pila hardware con el valor de retorno 0xC900 (LO en S, HI en S+1).
    cpu.s = 0xC8A0;
    cpu.test_write8(0xC8A0, 0x00); // LO
    cpu.test_write8(0xC8A1, 0xC9); // HI
    // 3) Opcional: reflejar la "llamada" en stacks de análisis (no requerido por el detector pero útil para coherencia).
    cpu.call_stack.push(0xC900);
    cpu.shadow_stack.push(vectrex_emulator::cpu6809::ShadowFrame{ ret:0xC900, sp_at_push: cpu.s, kind: vectrex_emulator::cpu6809::ShadowKind::JSR });
    cpu.step();
    // Verificar que snapshot temprano se generó.
    let det = &cpu.ram_exec;
    assert!(det.triggered, "Detector no marcado como triggered tras retorno inválido");
    let snap = det.snapshot.as_ref().expect("Snapshot ausente tras retorno inválido");
    assert_eq!(snap.last_pc, 0xC900, "PC de snapshot no coincide con retorno forzado");
    assert!(snap.reason.contains("RTS-invalid-return") || snap.reason.contains("shadow-"), "Reason inesperado: {}", snap.reason);
    // iterations puede ser 0 o 1 dependiendo de si contador se incrementa antes; aceptamos >=0
    assert!(snap.iterations <= 1, "Iteraciones inesperadamente altas para snapshot temprano: {}", snap.iterations);
    // Confirmar que first_pc coincide (o es <=) con last_pc
    assert!(snap.first_pc <= snap.last_pc);
}
