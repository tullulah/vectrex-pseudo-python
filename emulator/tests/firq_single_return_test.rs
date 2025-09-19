use vectrex_emulator::CPU;

// Verifica que un FIRQ simple (sin IRQ activo) retorna al PC correcto y limpia shadow stack.
#[test]
fn firq_single_return() {
    let mut cpu = CPU::default();
    // FIRQ vector estándar: high en 0xFFF6, low en 0xFFF7 (big-endian). Handler 0x0620
    cpu.bus.mem[0xFFF6] = 0x06; // high
    cpu.bus.mem[0xFFF7] = 0x20; // low

    // Mainline PC
    cpu.pc = 0x0123;
    cpu.bus.mem[0x0123] = 0x12; cpu.mem[0x0123] = 0x12; // NOP

    // Handler FIRQ: LDA #$42 ; RTI
    cpu.bus.mem[0x0620] = 0x86; cpu.mem[0x0620] = 0x86; // LDA imm
    cpu.bus.mem[0x0621] = 0x42; cpu.mem[0x0621] = 0x42;
    cpu.bus.mem[0x0622] = 0x3B; cpu.mem[0x0622] = 0x3B; // RTI

    let depth0 = cpu.shadow_stack.len();

    cpu.firq_pending = true;
    cpu.test_force_firq();
    assert_eq!(cpu.pc, 0x0620, "FIRQ no vectorizó correctamente");
    assert_eq!(cpu.shadow_stack.len(), depth0 + 1, "Shadow stack no incrementó tras FIRQ");
    let ret = cpu.shadow_stack.last().unwrap().ret;
    assert_eq!(ret, 0x0123, "Ret capturado incorrecto");

    cpu.step(); // LDA (inmediato)
    cpu.step(); // RTI

    assert_eq!(cpu.pc, 0x0123, "RTI FIRQ no regresó al mainline");
    assert_eq!(cpu.shadow_stack.len(), depth0, "Shadow stack no se limpió tras RTI FIRQ");
    assert_eq!(cpu.a, 0x42, "Valor A no persistió tras retorno");
}
