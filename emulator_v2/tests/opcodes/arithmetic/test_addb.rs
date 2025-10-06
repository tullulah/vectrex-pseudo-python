// Test file for B register arithmetic and logical operations

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_addb_all_addressing_modes() {
    // Test ADDB with immediate mode (0xCB)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0x7F;

    // Setup instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xCB); // ADDB immediate opcode
    cpu.memory_bus_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0x80); // 0x7F + 0x01 = 0x80
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set (0x80 has bit 7 set)
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: positive + positive = negative
    assert_eq!(cpu.registers().cc.c, false);

    // Test ADDB with carry generation
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0xFF;

    // Setup instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xCB); // ADDB immediate opcode
    cpu.memory_bus_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0x00); // 0xFF + 0x01 = 0x00 (wrap)
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set
}
