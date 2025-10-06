// C++ Original: Tests for CMPU opcodes (0x11 0x83, 0x11 0x93, 0x11 0xA3, 0x11 0xB3)
// Port 1:1 from Vectrexy test patterns for 16-bit U register comparison (Page 2 prefix)

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_cmpu_immediate_0x11_83() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU #$1234 with U=$1234 (equal) - Page 2 prefixed instruction
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x1234;

    // Write CMPU #$1234 instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x83); // CMPU immediate
    cpu.memory_bus_mut().write(0xC802, 0x12); // High byte of compare value
    cpu.memory_bus_mut().write(0xC803, 0x34); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().u, 0x1234); // U unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC804); // PC advanced (4 bytes: prefix + opcode + 2 data)
    assert_eq!(cycles, 5); // 5 cycles for immediate mode (16-bit prefixed)
}

#[test]
fn test_cmpu_immediate_greater() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU #$1000 with U=$1234 (U > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x1234;

    // Write CMPU #$1000 instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x83); // CMPU immediate
    cpu.memory_bus_mut().write(0xC802, 0x10); // High byte of compare value
    cpu.memory_bus_mut().write(0xC803, 0x00); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: U > operand, result positive (0x1234 - 0x1000 = 0x0234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().u, 0x1234); // U unchanged
    assert_eq!(cycles, 5); // 5 cycles for immediate mode
}

#[test]
fn test_cmpu_immediate_less() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU #$2000 with U=$1234 (U < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x1234;

    // Write CMPU #$2000 instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x83); // CMPU immediate
    cpu.memory_bus_mut().write(0xC802, 0x20); // High byte of compare value
    cpu.memory_bus_mut().write(0xC803, 0x00); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: U < operand, borrow needed (0x1234 - 0x2000 = 0xF234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set (result is negative)
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set (borrow needed)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().u, 0x1234); // U unchanged
    assert_eq!(cycles, 5); // 5 cycles for immediate mode
}

#[test]
fn test_cmpu_direct_0x11_93() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU $20 with U=$5678, memory[$C820]=$5678 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x5678;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus_mut().write(0xC820, 0x56); // High byte
    cpu.memory_bus_mut().write(0xC821, 0x78); // Low byte

    // Write CMPU direct instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x93); // CMPU direct
    cpu.memory_bus_mut().write(0xC802, 0x20); // Direct page address

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().u, 0x5678); // U unchanged
    assert_eq!(cpu.registers().pc, 0xC803); // PC advanced (3 bytes: prefix + opcode + address)
    assert_eq!(cycles, 6); // 6 cycles for direct mode
}

#[test]
fn test_cmpu_extended_0x11_B3() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU $C820 with U=$9ABC, memory[$C820]=$9ABC (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x9ABC;

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus_mut().write(0xC820, 0x9A); // High byte
    cpu.memory_bus_mut().write(0xC821, 0xBC); // Low byte

    // Write CMPU extended instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0xB3); // CMPU extended
    cpu.memory_bus_mut().write(0xC802, 0xC8); // High byte of address
    cpu.memory_bus_mut().write(0xC803, 0x20); // Low byte of address

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x0000 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().u, 0x9ABC); // U unchanged
    assert_eq!(cpu.registers().pc, 0xC804); // PC advanced (4 bytes: prefix + opcode + 2 address)
    assert_eq!(cycles, 7); // 7 cycles for extended mode
}

#[test]
fn test_cmpu_indexed_0x11_A3() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU ,X with U=$DEF0, X=$C850, memory[$C850]=$DEF0 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0xDEF0;
    cpu.registers_mut().x = 0xC850;

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus_mut().write(0xC850, 0xDE); // High byte
    cpu.memory_bus_mut().write(0xC851, 0xF0); // Low byte

    // Write CMPU indexed instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0xA3); // CMPU indexed
    cpu.memory_bus_mut().write(0xC802, 0x84); // ,X addressing mode

    let cycles = cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().u, 0xDEF0); // U unchanged
    assert_eq!(cpu.registers().pc, 0xC803); // PC advanced (3 bytes: prefix + opcode + postbyte)
    assert_eq!(cycles, 6); // 6 cycles for indexed mode
}

#[test]
fn test_cmpu_overflow_case() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: CMPU with overflow condition (0x8000 - 0x0001 = 0x7FFF, overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x8000; // -32768 in signed 16-bit

    // Write CMPU #$0001 instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x83); // CMPU immediate
    cpu.memory_bus_mut().write(0xC802, 0x00); // High byte
    cpu.memory_bus_mut().write(0xC803, 0x01); // Low byte

    cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify flags - subtract 0x8000 - 0x0001 = 0x7FFF (overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7FFF)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true); // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().u, 0x8000); // U unchanged
}

#[test]
fn test_cmpu_stack_pointer_values() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: Test typical stack pointer values (U is user stack pointer)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0xCFFF; // Typical user stack pointer value

    // Write CMPU #$D000 instruction (prefixed with 0x11)
    cpu.memory_bus_mut().write(0xC800, 0x11); // Page 2 prefix
    cpu.memory_bus_mut().write(0xC801, 0x83); // CMPU immediate
    cpu.memory_bus_mut().write(0xC802, 0xD0); // High byte
    cpu.memory_bus_mut().write(0xC803, 0x00); // Low byte

    cpu.execute_instruction(false, false).unwrap();

    // C++ Original: Verify comparison (0xCFFF < 0xD000, borrow needed)
    assert_eq!(cpu.registers().cc.z, false); // Not equal
    assert_eq!(cpu.registers().cc.n, true); // Negative result
    assert_eq!(cpu.registers().cc.c, true); // Borrow needed
    assert_eq!(cpu.registers().cc.v, false); // No overflow
    assert_eq!(cpu.registers().u, 0xCFFF); // U unchanged
}
