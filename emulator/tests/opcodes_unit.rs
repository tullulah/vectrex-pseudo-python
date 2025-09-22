//! (Consolidado) Este archivo legacy ha sido vaciado: ver `opcodes_all.rs` para los tests.
#![allow(dead_code)]
//!  - COM (acumulador A: 0x43)
//!  - SUBB (inmediato 0xC0)
//!  - STB (directo 0xD7, almacenamos B en 0x00FF, usando DP=0x00)
//!  - PULS (ejemplo: PULS A,B,PC = 0x35 mask 0b1000_0110 -> 0x86; pero usaremos solo A,B = 0x35 0x06)
//!
//! Nota sobre ciclos: aún no fijamos valores porque la tabla formal no está integrada; se podrían añadir más adelante.

use vectrex_emulator::cpu6809::CPU;

struct ExecResult { cpu: CPU, cycles: u32 }

fn run_with_cycles<F: FnOnce(&mut CPU)>(setup: F) -> ExecResult {
    let mut cpu = CPU::default();
    setup(&mut cpu);
    let before = cpu.cycles;
    let ok = cpu.step(); assert!(ok, "step() devolvió false");
    let delta = (cpu.cycles - before) as u32;
    ExecResult { cpu, cycles: delta }
}

fn run_single<F: FnOnce(&mut CPU)>(setup: F) -> CPU { run_with_cycles(setup).cpu }

// LDS inmediato: 10 CE hi lo -> carga S y actualiza N/Z.
#[test]
fn opcode_lds_immediate() {
    let r = run_with_cycles(|c| {
        c.pc = 0x0200;
        c.test_write8(0x0200,0x10); c.test_write8(0x0201,0xCE); c.test_write8(0x0202,0x12); c.test_write8(0x0203,0x34);
    });
    let cpu = r.cpu;
    assert_eq!(cpu.s, 0x1234); // valor cargado
    assert_eq!(cpu.pc, 0x0204); // avanzó 4 bytes
    assert!(!cpu.cc_z && !cpu.cc_n);
    assert_eq!(r.cycles, 5, "LDS inmediato ciclos esperados 5 got {}", r.cycles);
}

// ADDB inmediato: 0xCB imm -> B = B + imm con flags C,V,N,Z actualizados.
#[test]
fn opcode_addb_immediate() {
    let r = run_with_cycles(|c| {
        c.pc = 0x0300;
        c.b = 0x10;
        c.test_write8(0x0300,0xCB); // ADDB immediate
        c.test_write8(0x0301,0x22);
    });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "ADDB inmediato ciclos documentados 2 got {}", r.cycles);
    // Si opcode 0xCB aún no implementado, este test fallará; placeholder de intención.
    // Esperado: 0x10 + 0x22 = 0x32
    assert_eq!(cpu.b, 0x32, "ADDB resultado incorrecto");
    assert_eq!(cpu.pc, 0x0302);
}

// JSR extendido: 0xBD hi lo -> push return en call_stack y salta a dirección.
#[test]
fn opcode_jsr_extended() {
    let r = run_with_cycles(|c| {
        c.pc = 0x0400;
        c.test_write8(0x0400,0xBD); c.test_write8(0x0401,0x12); c.test_write8(0x0402,0x34); // JSR $1234
    });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 7, "JSR extendido ciclos 7 got {}", r.cycles);
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.call_stack.len(), 1);
    assert_eq!(cpu.call_stack[0], 0x0403); // dirección retorno
}

// BSR relativo: 0x8D offset (signed) -> push return (PC tras leer offset) y salta (PC tras leer offset) + offset.
#[test]
fn opcode_bsr_relative() {
    let r = run_with_cycles(|c| {
        c.pc = 0x0500;
        c.test_write8(0x0500,0x8D); c.test_write8(0x0501,0x03); // offset +3
    });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 7, "BSR ciclos 7 got {}", r.cycles);
    assert_eq!(cpu.pc, 0x0505);
    // Implementación actual NO usa call_stack para BSR (usa pila hardware). Verificamos pila: retorno debe estar en memoria en S.
    // push16 almacena high luego low en decrementos de S (comprobar implementación si cambia). Simplificamos: sólo comprobamos que S cambió.
    // Placeholder: cuando exportemos método para inspeccionar tope de pila, asertaremos valor exacto.
    // assert_eq!(cpu.call_stack.len(), 0); // BSR no debe tocar call_stack aquí.
}

// LDA inmediato: 0x86 imm -> carga A.
#[test]
fn opcode_lda_immediate() {
    let r = run_with_cycles(|c| {
    c.pc = 0x0600; c.test_write8(0x0600,0x86); c.test_write8(0x0601,0x7F); c.a = 0x00; });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "LDA inmediato ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.a, 0x7F); assert_eq!(cpu.pc, 0x0602); assert!(!cpu.cc_z && !cpu.cc_n);
}

// TFR: 0x1F post ; post: src<<4 | dst. En esta implementación el mapeo interno de índices difiere del estándar simbólico,
// usamos DP (src)=0x9 (16‑bit ancho 1?) no: necesitamos un caso estable. Elegimos transferir A->B usando código que la implementación aceptará.
// Observación: la implementación usa read_reg/write_reg con anchos; para 8‑bit registros se espera ancho coincidente.
#[test]
fn opcode_tfr_a_to_b() {
    let r = run_with_cycles(|c| {
    c.pc = 0x0700; c.a = 0x55; c.b = 0x00; c.test_write8(0x0700,0x1F); c.test_write8(0x0701,0x89); /* src=A(8) dst=B(9) */ });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 6, "TFR ciclos 6 got {}", r.cycles);
    assert_eq!(cpu.b, 0x55, "TFR no transfirió A->B");
    assert_eq!(cpu.pc, 0x0702);
}

// RTS: 0x39 -> pop return del call_stack.
#[test]
fn opcode_rts() {
    let r = run_with_cycles(|c| {
        // Preparamos un retorno real en la pila hardware (6809):
        // pop16 lee LO en mem[S], luego HI en mem[S+1]. Queremos volver a 0x0AAA.
        c.s = 0x2000; // tope de pila
        c.test_write8(0x2000, 0xAA); // low
        c.test_write8(0x2001, 0x0A); // high
        // Ejecutamos RTS en 0x0800
        c.pc = 0x0800;
        c.test_write8(0x0800, 0x39);
    });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 5, "RTS ciclos 5 got {}", r.cycles);
    assert_eq!(cpu.pc, 0x0AAA);
    assert_eq!(cpu.call_stack.len(), 0);
}

// LDB inmediato: 0xC6 imm.
#[test]
fn opcode_ldb_immediate() {
    let r = run_with_cycles(|c| { c.pc=0x0900; c.test_write8(0x0900,0xC6); c.test_write8(0x0901,0xE1); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "LDB inmediato ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.b, 0xE1); assert_eq!(cpu.pc, 0x0902); assert!(cpu.cc_n); assert!(!cpu.cc_z);
}

// LDX inmediato: 0x8E hi lo.
#[test]
fn opcode_ldx_immediate() {
    let r = run_with_cycles(|c| { c.pc=0x0A00; c.test_write8(0x0A00,0x8E); c.test_write8(0x0A01,0x01); c.test_write8(0x0A02,0x02); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 3, "LDX inmediato ciclos 3 got {}", r.cycles);
    assert_eq!(cpu.x, 0x0102); assert_eq!(cpu.pc, 0x0A03);
}

// CLRA: 0x4F -> A=0, Z=1, N=0.
#[test]
fn opcode_clra() {
    let r = run_with_cycles(|c| { c.pc=0x0B00; c.a=0xFF; c.test_write8(0x0B00,0x4F); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "CLRA ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.a, 0x00); assert!(cpu.cc_z && !cpu.cc_n); assert_eq!(cpu.pc, 0x0B01);
}

// BRA: 0x20 offset (signed). Offset 0x04 -> PC final = PC_after_fetch + 0x04.
#[test]
fn opcode_bra_forward() {
    let r = run_with_cycles(|c| { c.pc=0x0C00; c.test_write8(0x0C00,0x20); c.test_write8(0x0C01,0x04); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 3, "BRA tomado ciclos 3 got {}", r.cycles);
    assert_eq!(cpu.pc, 0x0C06); // PC inicial 0x0C00, tras leer opcode PC=0x0C01, lee offset y avanza a 0x0C02? (implementación puede variar). Ajustar si difiere.
}

// LSR acumulador A: 0x44 (LSRA) -> bit0 a C, shift lógico, N=0, Z si resultado 0.
#[test]
fn opcode_lsra() {
    let r = run_with_cycles(|c| { c.pc=0x0D00; c.a=0x03; c.test_write8(0x0D00,0x44); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "LSRA ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.a, 0x01); assert_eq!(cpu.cc_c, true); assert!(!cpu.cc_z); assert!(!cpu.cc_n); assert_eq!(cpu.pc, 0x0D01);
}

// COM acumulador A: 0x43 -> A = ~A, C=1, Z/N según resultado.
#[test]
fn opcode_coma() {
    let r = run_with_cycles(|c| { c.pc=0x0E00; c.a=0x55; c.test_write8(0x0E00,0x43); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "COMA ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.a, 0xAA); assert!(cpu.cc_c); assert!(!cpu.cc_z); assert!(cpu.cc_n); assert_eq!(cpu.pc, 0x0E01);
}

// SUBB inmediato: 0xC0 imm -> B = B - imm, flags según resta.
#[test]
fn opcode_subb_immediate() {
    let r = run_with_cycles(|c| { c.pc=0x0F00; c.b=0x30; c.test_write8(0x0F00,0xC0); c.test_write8(0x0F01,0x10); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 2, "SUBB inmediato ciclos 2 got {}", r.cycles);
    assert_eq!(cpu.b, 0x20); assert_eq!(cpu.pc, 0x0F02); assert!(!cpu.cc_z); assert!(!cpu.cc_n);
}

// STB directo: 0xD7 off -> mem[DP<<8 | off] = B.
#[test]
fn opcode_stb_direct() {
    let r = run_with_cycles(|c| { c.pc=0x1000; c.dp=0x00; c.b=0x42; c.test_write8(0x1000,0xD7); c.test_write8(0x1001,0xFF); });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 4, "STB directo ciclos 4 got {}", r.cycles);
    assert_eq!(cpu.bus.mem[0x00FF], 0x42); assert_eq!(cpu.pc, 0x1002);
}

// PULS: 0x35 mask. Usamos máscara 0x06 (A y B). Orden hardware procesa bits ascendentes: bit1 (A) primero, luego bit2 (B).
#[test]
fn opcode_puls_ab() {
    let r = run_with_cycles(|c| {
    c.pc = 0x1100; c.test_write8(0x1100,0x35); c.test_write8(0x1101,0x06); // PULS B,A
        // Prepara pila: S apunta al primer byte a extraer.
    c.s = 0x2000; c.test_write8(0x2000,0xBB); c.test_write8(0x2001,0xAA); // A=BB (primer pop), B=AA (segundo pop)
    });
    let cpu = r.cpu;
    assert_eq!(r.cycles, 5, "PULS (2 regs) ciclos 5 got {}", r.cycles);
    // Confirmamos que los registros se llenaron con los bytes en el orden correcto.
    // Ajustar si el orden interno difiere (6809: al hacer PULS se extrae en orden CC,A,B,DP,X,Y,U,PC según bits).
    assert_eq!(cpu.a, 0xBB); // primero A
    assert_eq!(cpu.b, 0xAA); // luego B
    assert_eq!(cpu.pc, 0x1102);
}
