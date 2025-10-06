// Tests for MUL and SEX opcodes (Phase 8)
// C++ Original: multiply() and sex() functions in Vectrexy

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_sex_0x1d_negative() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: SEX with negative B (bit 7 = 1)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x80; // Negative value

    cpu.memory_bus_mut().write(0xC800, 0x1D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result - A should be 0xFF for negative B
    assert_eq!(cpu.registers().a, 0xFF, "A should be 0xFF for negative B");
    assert_eq!(cpu.registers().b, 0x80, "B should remain unchanged");

    // Check condition codes
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
}

#[test]
fn test_sex_0x1d_positive() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: SEX with positive B (bit 7 = 0)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x42; // Positive value

    cpu.memory_bus_mut().write(0xC800, 0x1D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result - A should be 0x00 for positive B
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x00 for positive B");
    assert_eq!(cpu.registers().b, 0x42, "B should remain unchanged");

    // Check condition codes
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
}

#[test]
fn test_sex_0x1d_zero() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: SEX with B = 0
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x00;

    cpu.memory_bus_mut().write(0xC800, 0x1D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x00 for zero B");
    assert_eq!(cpu.registers().b, 0x00, "B should remain 0x00");

    // Check condition codes
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
}
