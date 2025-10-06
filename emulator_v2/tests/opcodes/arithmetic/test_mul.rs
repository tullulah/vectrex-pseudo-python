// Tests for MUL and SEX opcodes (Phase 8)
// C++ Original: multiply() and sex() functions in Vectrexy

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_mul_0x3d_basic() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: multiply() - Test: 12 * 13 = 156 (0x9C)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 12;
    cpu.registers_mut().b = 13;

    // Write MUL opcode
    cpu.memory_bus_mut().write(0xC800, 0x3D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result in D register (A:B)
    let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
    assert_eq!(d_result, 156, "MUL result should be 156");
    assert_eq!(cpu.registers().a, 0, "High byte should be 0");
    assert_eq!(cpu.registers().b, 156, "Low byte should be 156");

    // Check condition codes
    // C++ Original: CC.Carry = TestBits01(result, BITS(7)) where BITS(7) = 0x80
    // For 156 (0x009C), bit 7 is set (0x9C & 0x80 = 0x80), so C = true
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(
        cpu.registers().cc.c,
        true,
        "C flag should be set (bit 7 of result)"
    );
    assert_eq!(cycles, 11, "MUL should take 11 cycles");
}

#[test]
fn test_mul_0x3d_overflow() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: Test: 255 * 255 = 65025 (0xFE01)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 255;
    cpu.registers_mut().b = 255;

    cpu.memory_bus_mut().write(0xC800, 0x3D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result
    let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
    assert_eq!(d_result, 65025, "MUL result should be 65025");
    assert_eq!(cpu.registers().a, 0xFE, "High byte should be 0xFE");
    assert_eq!(cpu.registers().b, 0x01, "Low byte should be 0x01");

    // Check condition codes
    // C++ Original: CC.Carry = TestBits01(result, BITS(7)) where BITS(7) = 0x80
    // For 65025 (0xFE01), bit 7 is clear (0x01 & 0x80 = 0x00), so C = false
    assert_eq!(
        cpu.registers().cc.c,
        false,
        "C flag should be clear (bit 7 of result)"
    );
    assert_eq!(cycles, 11, "MUL should take 11 cycles");
}

#[test]
fn test_mul_0x3d_zero_result() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: Test: 0 * 42 = 0
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0;
    cpu.registers_mut().b = 42;

    cpu.memory_bus_mut().write(0xC800, 0x3D);

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // Verify result
    let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
    assert_eq!(d_result, 0, "MUL result should be 0");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
    assert_eq!(cycles, 11, "MUL should take 11 cycles");
}
