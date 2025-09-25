// C++ Original: Tests for CMPY opcodes (0x10 0x8C, 0x10 0x9C, 0x10 0xAC, 0x10 0xBC)
// Port 1:1 from Vectrexy test patterns for 16-bit Y register comparison (Page 1 prefix)

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn setup_cpu_with_memory() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));

    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());

    Cpu6809::new(memory_bus)
}

#[test]
fn test_cmpy_immediate_0x10_8C() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY #$1234 with Y=$1234 (equal) - Page 1 prefixed instruction
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x1234;

    // Write CMPY #$1234 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x8C); // CMPY immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x12); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x34); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().y, 0x1234);   // Y unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC804);  // PC advanced (4 bytes: prefix + opcode + 2 data)
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode (16-bit prefixed)
}

#[test]
fn test_cmpy_immediate_greater() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY #$1000 with Y=$1234 (Y > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x1234;

    // Write CMPY #$1000 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x8C); // CMPY immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x10); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x00); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Y > operand, result positive (0x1234 - 0x1000 = 0x0234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().y, 0x1234);   // Y unchanged
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode
}

#[test]
fn test_cmpy_immediate_less() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY #$2000 with Y=$1234 (Y < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x1234;

    // Write CMPY #$2000 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x8C); // CMPY immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x00); // Low byte of compare value

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Y < operand, borrow needed (0x1234 - 0x2000 = 0xF234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (result is negative)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow needed)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().y, 0x1234);   // Y unchanged
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode
}

#[test]
fn test_cmpy_direct_0x10_9C() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY $20 with Y=$5678, memory[$C820]=$5678 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x5678;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x56); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0x78); // Low byte

    // Write CMPY direct instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x9C); // CMPY direct
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // Direct page address

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().y, 0x5678);   // Y unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes: prefix + opcode + address)
    assert_eq!(cycles, 6);                   // 6 cycles for direct mode
}

#[test]
fn test_cmpy_extended_0x10_BC() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY $C820 with Y=$9ABC, memory[$C820]=$9ABC (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x9ABC;

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x9A); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0xBC); // Low byte

    // Write CMPY extended instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0xBC); // CMPY extended
    cpu.memory_bus().borrow_mut().write(0xC802, 0xC8); // High byte of address
    cpu.memory_bus().borrow_mut().write(0xC803, 0x20); // Low byte of address

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x0000 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().y, 0x9ABC);   // Y unchanged
    assert_eq!(cpu.registers().pc, 0xC804);  // PC advanced (4 bytes: prefix + opcode + 2 address)
    assert_eq!(cycles, 7);                   // 7 cycles for extended mode
}

#[test]
fn test_cmpy_indexed_0x10_AC() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY ,X with Y=$DEF0, X=$C850, memory[$C850]=$DEF0 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0xDEF0;
    cpu.registers_mut().x = 0xC850;

    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC850, 0xDE); // High byte
    cpu.memory_bus().borrow_mut().write(0xC851, 0xF0); // Low byte

    // Write CMPY indexed instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0xAC); // CMPY indexed
    cpu.memory_bus().borrow_mut().write(0xC802, 0x84); // ,X addressing mode

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().y, 0xDEF0);   // Y unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes: prefix + opcode + postbyte)
    assert_eq!(cycles, 6);                   // 6 cycles for indexed mode
}

#[test]
fn test_cmpy_overflow_case() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPY with overflow condition (0x8000 - 0x0001 = 0x7FFF, overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x8000; // -32768 in signed 16-bit

    // Write CMPY #$0001 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x8C); // CMPY immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00); // High byte
    cpu.memory_bus().borrow_mut().write(0xC803, 0x01); // Low byte

    cpu.execute_instruction(false, false);

    // C++ Original: Verify flags - subtract 0x8000 - 0x0001 = 0x7FFF (overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7FFF)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true);  // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().y, 0x8000);   // Y unchanged
}

#[test]
fn test_cmpy_boundary_values() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: Test boundary case with maximum unsigned value
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0xFFFF; // Maximum unsigned 16-bit value

    // Write CMPY #$FFFE instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x8C); // CMPY immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0xFF); // High byte
    cpu.memory_bus().borrow_mut().write(0xC803, 0xFE); // Low byte

    cpu.execute_instruction(false, false);

    // C++ Original: Verify comparison (0xFFFF - 0xFFFE = 0x0001, positive result)
    assert_eq!(cpu.registers().cc.z, false); // Not equal
    assert_eq!(cpu.registers().cc.n, false); // Positive result
    assert_eq!(cpu.registers().cc.c, false); // No borrow
    assert_eq!(cpu.registers().cc.v, false); // No overflow
    assert_eq!(cpu.registers().y, 0xFFFF);   // Y unchanged
}