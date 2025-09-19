//! Verifica que el primer BSR ejecutado decremente correctamente S (push16 del return address)
//! sin depender de contexto previo. Aísla un posible bug reportado donde el primer BSR no
//! reducía S.

use vectrex_emulator::CPU;

#[test]
fn first_bsr_decrements_s_and_stores_return() {
    let mut cpu = CPU::default();
    // Configuración: PC en 0x0800, pila S en 0x900 (valor arbitrario alto en RAM).
    cpu.pc = 0x0800;
    cpu.s  = 0x0900;
    // Instrucción BSR con offset +3 -> retorno debería ser 0x0802 (PC tras opcode+offset byte) y salto a 0x0802+3=0x0805.
    cpu.test_write8(0x0800, 0x8D); // BSR
    cpu.test_write8(0x0801, 0x03); // offset

    let s_before = cpu.s;
    let ok = cpu.step(); assert!(ok, "step BSR");
    // Después del push16, S debe decrementar 2 (dos push8). Orden de push16 implementado: high primero, luego low.
    // push8: S--, write => tras high: S = s_before-1, tras low: S = s_before-2. Mem:
    // [s_before-1] = high, [s_before-2] = low? (ver implementación: high push -> S=s_before-1 escribe high allí; low push -> S=s_before-2 escribe low)
    assert_eq!(cpu.s, s_before - 2, "S debe decrementar en 2 tras primer BSR (push16)");

    let expected_return = 0x0802u16; // dirección siguiente a bytes del BSR
    let high_addr = s_before - 1; // primera escritura (high)
    let low_addr  = s_before - 2; // segunda escritura (low)
    let high_mem = cpu.mem[high_addr as usize];
    let low_mem  = cpu.mem[low_addr as usize];
    assert_eq!(high_mem, (expected_return >> 8) as u8, "Byte alto return addr incorrecto (posición high_addr)");
    assert_eq!(low_mem, (expected_return & 0xFF) as u8, "Byte bajo return addr incorrecto (posición low_addr)");

    assert_eq!(cpu.pc, expected_return + 3, "PC destino BSR incorrecto");
}
