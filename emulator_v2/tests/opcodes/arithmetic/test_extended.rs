// Test file for B register arithmetic and logical operations

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_extended_addressing_modes() {
    // Test ORAB extended (0xFA)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0x0F;

    // Setup extended addressing instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xFA); // ORAB extended opcode
    cpu.memory_bus_mut().write(0xC801, 0xC9); // address high byte
    cpu.memory_bus_mut().write(0xC802, 0x00); // address low byte
    cpu.memory_bus_mut().write(0xC900, 0xF0); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0xFF); // 0x0F | 0xF0 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ANDB extended (0xF4)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().b = 0xFF;

    // Setup extended addressing instruction in RAM area
    cpu.memory_bus_mut().write(0xC800, 0xF4); // ANDB extended opcode
    cpu.memory_bus_mut().write(0xC801, 0xC9); // address high byte
    cpu.memory_bus_mut().write(0xC802, 0x10); // address low byte
    cpu.memory_bus_mut().write(0xC910, 0x0F); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    assert_eq!(cpu.registers().b, 0x0F); // 0xFF & 0x0F = 0x0F
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
}
