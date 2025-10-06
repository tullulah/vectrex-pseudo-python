// Test file for B register arithmetic and logical operations

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_orb_all_addressing_modes() {
    // Test ORAB with immediate mode (0xCA)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0xAA;

    // Setup instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    cpu.memory_bus_mut().write(0xC801, 0x55); // operand
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0xFF); // 0xAA | 0x55 = 0xFF
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.v, false); // Overflow always clear for OR

    // Test ORAB with direct mode (0xDA)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0xF0;
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM page

    // Setup instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xDA); // ORAB direct opcode
    cpu.memory_bus_mut().write(0xC801, 0x10); // direct address offset
    cpu.memory_bus_mut().write(0xC810, 0x0F); // data at 0xC800 + 0x10
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0xFF); // 0xF0 | 0x0F = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ORAB with zero result
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0x00;

    // Setup instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    cpu.memory_bus_mut().write(0xC801, 0x00); // operand
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set
}
