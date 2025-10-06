// C++ Original: Tests for CMPB opcodes (0xC1, 0xD1, 0xE1, 0xF1)
// Port 1:1 from Vectrexy test patterns

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_cmpb_immediate_0xC1() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB #$50 with B=$50 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x50;

    // Write CMPB #$50 instruction
    cpu.memory_bus_mut().write(0xC800, 0xC1); // CMPB immediate
    cpu.memory_bus_mut().write(0xC801, 0x50); // Compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().b, 0x50); // B unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced
    assert_eq!(cycles, 2); // 2 cycles for immediate mode
}

#[test]
fn test_cmpb_immediate_greater() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB #$30 with B=$50 (B > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x50;

    // Write CMPB #$30 instruction
    cpu.memory_bus_mut().write(0xC800, 0xC1); // CMPB immediate
    cpu.memory_bus_mut().write(0xC801, 0x30); // Compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: B > operand, result positive
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().b, 0x50); // B unchanged
    assert_eq!(cycles, 2); // 2 cycles for immediate mode
}

#[test]
fn test_cmpb_immediate_less() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB #$70 with B=$50 (B < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x50;

    // Write CMPB #$70 instruction
    cpu.memory_bus_mut().write(0xC800, 0xC1); // CMPB immediate
    cpu.memory_bus_mut().write(0xC801, 0x70); // Compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: B < operand, borrow needed (0x50 - 0x70 = 0xE0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set (result is negative)
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set (borrow needed)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().b, 0x50); // B unchanged
    assert_eq!(cycles, 2); // 2 cycles for immediate mode
}

#[test]
fn test_cmpb_direct_0xD1() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB $20 with B=$40, memory[$C820]=$40 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x40;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8

    // Write test data to memory
    cpu.memory_bus_mut().write(0xC820, 0x40); // Memory value to compare

    // Write CMPB direct instruction
    cpu.memory_bus_mut().write(0xC800, 0xD1); // CMPB direct
    cpu.memory_bus_mut().write(0xC801, 0x20); // Direct page address

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().b, 0x40); // B unchanged
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced
    assert_eq!(cycles, 4); // 4 cycles for direct mode
}

#[test]
fn test_cmpb_extended_0xF1() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB $C820 with B=$80, memory[$C820]=$80 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x80;

    // Write test data to memory
    cpu.memory_bus_mut().write(0xC820, 0x80); // Memory value to compare

    // Write CMPB extended instruction
    cpu.memory_bus_mut().write(0xC800, 0xF1); // CMPB extended
    cpu.memory_bus_mut().write(0xC801, 0xC8); // High byte of address
    cpu.memory_bus_mut().write(0xC802, 0x20); // Low byte of address

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x00 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().b, 0x80); // B unchanged
    assert_eq!(cpu.registers().pc, 0xC803); // PC advanced
    assert_eq!(cycles, 5); // 5 cycles for extended mode
}

#[test]
fn test_cmpb_indexed_0xE1() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB ,X with B=$60, X=$C850, memory[$C850]=$60 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x60;
    cpu.registers_mut().x = 0xC850;

    // Write test data to memory
    cpu.memory_bus_mut().write(0xC850, 0x60); // Memory value to compare

    // Write CMPB indexed instruction
    cpu.memory_bus_mut().write(0xC800, 0xE1); // CMPB indexed
    cpu.memory_bus_mut().write(0xC801, 0x84); // ,X addressing mode

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().b, 0x60); // B unchanged
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced
    assert_eq!(cycles, 4); // 4 cycles for indexed mode
}

#[test]
fn test_cmpb_overflow_case() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPB with overflow condition (0x80 - 0x01 = 0x7F, overflow occurs: -128 - 1 = -129, wraps to +127)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().b = 0x80; // -128 in signed

    // Write CMPB #$01 instruction
    cpu.memory_bus_mut().write(0xC800, 0xC1); // CMPB immediate
    cpu.memory_bus_mut().write(0xC801, 0x01); // Compare value

    cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags - subtract 0x80 - 0x01 = 0x7F (overflow occurs: -128 - 1 = -129, wraps to +127)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7F)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true); // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().b, 0x80); // B unchanged
}
