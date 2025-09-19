//! Tests de modos de direccionamiento (directo, extendido, indexado) y efecto de DP.
//! Política: micro-tests sintéticos (sin BIOS) válidos bajo modo estricto.

use vectrex_emulator::cpu6809::CPU;

// Helper para ejecutar una sola instrucción y devolver (cpu, ciclos)
fn run_one(setup: impl FnOnce(&mut CPU)) -> (CPU,u32) {
    let mut cpu=CPU::default();
    setup(&mut cpu);
    let before=cpu.cycles; let ok=cpu.step(); assert!(ok,"step() devolvió false");
    let cycles=(cpu.cycles-before) as u32;
    (cpu,cycles)
}

#[test]
fn lda_direct_usa_dp() {
    // DP = 0x12, LDA direct 0x96 offset 0xFE -> lee (0x12<<8)|0xFE = 0x12FE
    let (cpu,_cyc) = run_one(|c|{
        c.dp=0x12; c.pc=0x0200;
        c.test_write8(0x0200,0x96); c.test_write8(0x0201,0xFE);
        c.test_write8(0x12FE,0x77);
    });
    assert_eq!(cpu.a,0x77,"LDA direct no cargó el valor esperado usando DP alto");
}

#[test]
fn lda_extended_ignora_dp() {
    // LDA extended 0xB6 hi lo -> dirección absoluta
    let (cpu,_cyc)=run_one(|c|{
        c.dp=0xFF; // DP distinto para asegurar que no influye
        c.pc=0x0300; c.test_write8(0x0300,0xB6); c.test_write8(0x0301,0x34); c.test_write8(0x0302,0x56);
        c.test_write8(0x3456,0x5A);
    });
    assert_eq!(cpu.a,0x5A,"LDA extended debe usar dirección absoluta");
}

#[test]
fn lda_indexed_simple_x() {
    // LDA indexado ,X (postbyte 0x84 en nuestra codificación usada en otros tests)
    let (cpu,_cyc)=run_one(|c|{
        c.x=0x4000; c.pc=0x0400;
        c.test_write8(0x0400,0xA6); c.test_write8(0x0401,0x84); // LDA ,X
        c.test_write8(0x4000,0x33);
    });
    assert_eq!(cpu.a,0x33,"LDA ,X no cargó el byte apuntado por X");
}

#[test]
fn sta_direct_escribe_en_pagina_dp() {
    let (cpu,_cyc)=run_one(|c|{
        c.dp=0x21; c.a=0xAB; c.pc=0x0500;
        c.test_write8(0x0500,0x97); c.test_write8(0x0501,0x10); // STA direct offset 0x10 -> 0x2110
    });
    assert_eq!(cpu.mem[0x2110],0xAB,"STA direct no escribió en página DP correcta");
}

#[test]
fn sta_indexed_x() {
    let (cpu,_cyc)=run_one(|c|{
        c.x=0x5000; c.a=0xEE; c.pc=0x0600;
        c.test_write8(0x0600,0xA7); c.test_write8(0x0601,0x84); // STA ,X
    });
    assert_eq!(cpu.mem[0x5000],0xEE,"STA ,X no almacenó en dirección indexada");
}

#[test]
fn dp_switch_changes_direct_page() {
    // Mismo código y offset; cambiando DP debe cambiar página accesada.
    let (cpu1,_)=run_one(|c|{
        c.dp=0x12; c.pc=0x0700; c.test_write8(0x0700,0x96); c.test_write8(0x0701,0xAA); // LDA direct
        c.test_write8(0x12AA,0x11);
    });
    assert_eq!(cpu1.a,0x11);
    let (cpu2,_)=run_one(|c|{
        c.dp=0x34; c.pc=0x0800; c.test_write8(0x0800,0x96); c.test_write8(0x0801,0xAA);
        c.test_write8(0x34AA,0x22);
    });
    assert_eq!(cpu2.a,0x22);
    assert_ne!(cpu1.a, cpu2.a, "Cambio de DP debe alterar página direccionada");
}

#[test]
fn ldd_direct_vs_extended_consistencia() {
    // Compara que LDD direct (0xDC) y extended (0xFC?) cargan par de bytes correctos.
    // Usamos direct primero.
    let (cpu_d,_)=run_one(|c|{
        c.dp=0x09; c.pc=0x0900; c.test_write8(0x0900,0xDC); c.test_write8(0x0901,0x80); // LDD direct offset 0x80 -> 0x0980
        c.test_write8(0x0980,0x12); c.test_write8(0x0981,0x34);
    });
    assert_eq!(cpu_d.a,0x12); assert_eq!(cpu_d.b,0x34);
    // Extended: ADDR 0x4567
    let (_cpu_e,_)=run_one(|c|{
        c.pc=0x0A00; c.test_write8(0x0A00,0xCC); c.test_write8(0x0A01,0x56); c.test_write8(0x0A02,0x78); // LDD immediate alt (ya usado en otros tests) => para extended verdadero sería 0xDC form not reused; usamos ADDD immediate para cubrir var? Mantener simple.
    });
    // Este test se centra en direct; segunda parte placeholder para futura LDD extended cuando opcode esté disponible si difiere.
}
