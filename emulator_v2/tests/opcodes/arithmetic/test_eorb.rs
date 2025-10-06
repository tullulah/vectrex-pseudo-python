//! Tests for ANDB, EORB, ORAB opcodes (B register logic operations)
//! MC6809 Programming Manual opcodes:
//! - ANDB: 0xC4 (immediate), 0xD4 (direct), 0xE4 (indexed), 0xF4 (extended)
//! - EORB: 0xC8 (immediate), 0xD8 (direct), 0xE8 (indexed), 0xF8 (extended)
//! - ORAB: 0xCA (immediate), 0xDA (direct), 0xEA (indexed), 0xFA (extended)

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_eorb_extended_0xf8() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: B = 0x12, memory at 0xC900 = 0x12 (XOR with itself = 0)
    cpu.registers_mut().b = 0x12;
    cpu.memory_bus_mut().write(RAM_START + 0x100, 0x12);

    // Write opcode: EORB extended
    cpu.memory_bus_mut().write(RAM_START, 0xF8);
    cpu.memory_bus_mut().write(RAM_START + 1, 0xC9);
    cpu.memory_bus_mut().write(RAM_START + 2, 0x00);

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    // Verify result: B = 0x12 XOR 0x12 = 0x00
    assert_eq!(
        cpu.registers().b,
        0x00,
        "B should be zero (XOR with itself)"
    );
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}

#[test]
fn test_eorb_indexed_0xe8() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: B = 0xAA (10101010), memory = 0x55 (01010101)
    cpu.registers_mut().b = 0xAA;
    cpu.registers_mut().y = RAM_START + 0x50;
    cpu.memory_bus_mut().write(RAM_START + 0x50, 0x55);

    // Write opcode: EORB indexed [,Y]
    cpu.memory_bus_mut().write(RAM_START, 0xE8);
    cpu.memory_bus_mut().write(RAM_START + 1, 0xA4); // Postbyte: Y register

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    // Verify result: B = 0xAA XOR 0x55 = 0xFF
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0xAA XOR 0x55 = 0xFF");
    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set (bit 7 = 1)"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(
        cpu.registers().cc.v,
        false,
        "Overflow flag should always be clear for EOR"
    );
}
