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
fn test_andb_indexed_0xe4() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: B = 0xFF, X = 0xC850, memory at [X] = 0x0F
    cpu.registers_mut().b = 0xFF;
    cpu.registers_mut().x = RAM_START + 0x50;
    cpu.memory_bus_mut().write(RAM_START + 0x50, 0x0F);

    // Write opcode: ANDB indexed [,X]
    cpu.memory_bus_mut().write(RAM_START, 0xE4);
    cpu.memory_bus_mut().write(RAM_START + 1, 0x84); // Postbyte: X register, no offset

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    // Verify result: B = 0xFF AND 0x0F = 0x0F
    assert_eq!(cpu.registers().b, 0x0F, "B should be 0xFF AND 0x0F = 0x0F");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(
        cpu.registers().cc.v,
        false,
        "Overflow flag should always be clear for AND"
    );
}

#[test]
fn test_andb_zero_result() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: B = 0x0F, memory = 0xF0 (no common bits)
    cpu.registers_mut().b = 0x0F;
    cpu.registers_mut().x = RAM_START + 0x50;
    cpu.memory_bus_mut().write(RAM_START + 0x50, 0xF0);

    cpu.memory_bus_mut().write(RAM_START, 0xE4);
    cpu.memory_bus_mut().write(RAM_START + 1, 0x84);

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().b, 0x00, "B should be zero");
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}
