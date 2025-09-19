//! Tests for RAM execution detector instrumentation.
//!
//! Política: no fabricar BIOS sintética. Requiere bios real presente en ruta documentada.
//! Los tests aquí se enfocan en comportamientos estructurales: se registra pseudo entrada BIOS
//! y el detector puede dispararse en un escenario inducido.

use vectrex_emulator::cpu6809::CPU;

// Helper: load real BIOS from canonical path (fail hard if missing to avoid sintética accidental).
fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin"; // mantener sincronizado con copilot-instructions
    let data = std::fs::read(path).expect("BIOS real requerida para test (ver copilot-instructions.md)");
    assert_eq!(data.len(), 8192, "BIOS size inesperado (esperado 8K)");
    // Map a 8K BIOS at E000-FFFF
    for (i, b) in data.iter().enumerate() { let addr = 0xE000 + i as u16; cpu.mem[addr as usize] = *b; cpu.bus.mem[addr as usize] = *b; }
    cpu.bios_present = true;
}

#[test]
fn pseudo_initial_bios_call_recorded() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    // Reset con BIOS debe registrar una única entrada (pseudo) en bios_calls
    cpu.reset();
    assert!(!cpu.bios_calls.is_empty(), "Se esperaba bios_calls no vacío tras reset con BIOS");
    let first_entry = &cpu.bios_calls[0];
    // Formato esperado: algo que incluya la dirección hexadecimal inicial (ej: "F192 WAIT_RECAL").
    // Extrae primeros 4 chars hex para validar rango.
    if first_entry.len() >= 4 {
        if let Ok(addr) = u16::from_str_radix(&first_entry[..4], 16) {
            assert!(addr >= 0xE000, "Primera bios_call fuera de ventana BIOS: {:04X}", addr);
        } else {
            panic!("Formato inesperado en primera bios_call: {}", first_entry);
        }
    } else {
        panic!("Entrada bios_call demasiado corta: {}", first_entry);
    }
}

// Este test induce artificialmente ejecución en RAM copiando un pequeño bucle en 0xC800 y
// forzando PC allí. No se fabrica BIOS: sólo se salta después de reset.
#[test]
fn ram_exec_detector_triggers() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    // Programa un bucle muy corto en RAM (0xC800): INC $C800; BRA -2
    // Opcode INC direct = 0x0C + direct operand; pero necesitamos DP apuntando a 0xC8.
    // Simpler: usar NOP (0x12?) 6809 no; mejor usar un par de bytes que formen un bucle estable:
    // Usaremos: 0x20 FE => BRA -2 (loop) precedido de un byte legal como NOP extendido: 0x12 no es definido.
    // Estrategia: usar solo BRA -2 repetido (dos bytes) y apuntar PC a 0xC801 para que siempre ejecute BRA.
    cpu.mem[0xC800] = 0x12; // byte relleno (tratado como NOP ilegal -> NOP)
    cpu.mem[0xC801] = 0x20; // BRA
    cpu.mem[0xC802] = 0xFE; // -2
    cpu.bus.mem[0xC800] = cpu.mem[0xC800];
    cpu.bus.mem[0xC801] = cpu.mem[0xC801];
    cpu.bus.mem[0xC802] = cpu.mem[0xC802];
    cpu.pc = 0xC801; // Forzar ejecución en RAM inmediatamente

    for _ in 0..600 { cpu.step(); }
    assert!(cpu.ram_exec.triggered, "Detector debió dispararse (count={})", cpu.ram_exec.count);
    assert!(cpu.ram_exec.snapshot.is_some(), "Snapshot esperado");
    let snap = cpu.ram_exec.snapshot.as_ref().unwrap();
    assert!(snap.iterations >= 512, "Iteraciones esperadas >=512 (got {})", snap.iterations);
    assert!(snap.first_pc >= 0xC800 && snap.first_pc <= 0xCFFF, "first_pc fuera de rango");
    assert!(snap.last_pc  >= 0xC800 && snap.last_pc  <= 0xCFFF, "last_pc fuera de rango");
    assert!(!snap.recent_pcs.is_empty(), "recent_pcs vacío");
}
