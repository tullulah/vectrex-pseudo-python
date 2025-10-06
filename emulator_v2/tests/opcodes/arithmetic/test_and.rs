// Tests para opcodes AND y EOR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x84, 0xC4, 0x88, 0xC8

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_and_eor_flag_comprehensive() {
    // Test flag behavior with various AND/EOR combinations
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Test 1: AND negative result
    cpu.registers_mut().a = 0xFF;
    cpu.memory_bus_mut().write(0xC800, 0x84); // ANDA immediate
    cpu.memory_bus_mut().write(0xC801, 0x80); // AND with 0x80
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x80, "A should be 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");

    // Test 2: EOR zero result
    cpu.registers_mut().b = 0x33;
    cpu.memory_bus_mut().write(0xC810, 0xC8); // EORB immediate
    cpu.memory_bus_mut().write(0xC811, 0x33); // EOR with same value
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().b, 0x00, "B should be 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}
