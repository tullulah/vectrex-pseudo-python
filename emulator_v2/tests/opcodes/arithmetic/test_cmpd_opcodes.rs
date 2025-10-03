// C++ Original: Tests for CMPD opcodes (0x10 0x83, 0x10 0x93, 0x10 0xA3, 0x10 0xB3)
// Port 1:1 from Vectrexy test patterns for 16-bit D register comparison (Page 1 prefix)

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
fn test_cmpd_immediate_0x10_83() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD #$1234 with D=$1234 (equal) - prefixed instruction
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x1234);
    
    // Write CMPD #$1234 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // CMPD immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x12); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x34); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().d(), 0x1234); // D unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC804);  // PC advanced (4 bytes: prefix + opcode + 2 data)
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode (16-bit prefixed)
}

#[test]
fn test_cmpd_immediate_greater() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD #$1000 with D=$1234 (D > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x1234);
    
    // Write CMPD #$1000 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // CMPD immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x10); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x00); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: D > operand, result positive (0x1234 - 0x1000 = 0x0234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().d(), 0x1234); // D unchanged
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode
}

#[test]
fn test_cmpd_immediate_less() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD #$2000 with D=$1234 (D < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x1234);
    
    // Write CMPD #$2000 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // CMPD immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC803, 0x00); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: D < operand, borrow needed (0x1234 - 0x2000 = 0xF234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (result is negative)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow needed)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().d(), 0x1234); // D unchanged
    assert_eq!(cycles, 5);                   // 5 cycles for immediate mode
}

#[test]
fn test_cmpd_direct_0x10_93() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD $20 with D=$5678, memory[$C820]=$5678 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x5678);
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x56); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0x78); // Low byte
    
    // Write CMPD direct instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x93); // CMPD direct
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // Direct page address
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().d(), 0x5678); // D unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes: prefix + opcode + address)
    assert_eq!(cycles, 6);                   // 6 cycles for direct mode
}

#[test]
fn test_cmpd_extended_0x10_B3() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD $C820 with D=$9ABC, memory[$C820]=$9ABC (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x9ABC);
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x9A); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0xBC); // Low byte
    
    // Write CMPD extended instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0xB3); // CMPD extended
    cpu.memory_bus().borrow_mut().write(0xC802, 0xC8); // High byte of address
    cpu.memory_bus().borrow_mut().write(0xC803, 0x20); // Low byte of address
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x0000 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().d(), 0x9ABC); // D unchanged
    assert_eq!(cpu.registers().pc, 0xC804);  // PC advanced (4 bytes: prefix + opcode + 2 address)
    assert_eq!(cycles, 7);                   // 7 cycles for extended mode
}

#[test]
fn test_cmpd_indexed_0x10_A3() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD ,X with D=$DEF0, X=$C850, memory[$C850]=$DEF0 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0xDEF0);
    cpu.registers_mut().x = 0xC850;
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC850, 0xDE); // High byte
    cpu.memory_bus().borrow_mut().write(0xC851, 0xF0); // Low byte
    
    // Write CMPD indexed instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0xA3); // CMPD indexed
    cpu.memory_bus().borrow_mut().write(0xC802, 0x84); // ,X addressing mode
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().d(), 0xDEF0); // D unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes: prefix + opcode + postbyte)
    assert_eq!(cycles, 6);                   // 6 cycles for indexed mode
}

#[test]
fn test_cmpd_overflow_case() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPD with overflow condition (0x8000 - 0x0001 = 0x7FFF, overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().set_d(0x8000); // -32768 in signed 16-bit
    
    // Write CMPD #$0001 instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // CMPD immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00); // High byte
    cpu.memory_bus().borrow_mut().write(0xC803, 0x01); // Low byte
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags - subtract 0x8000 - 0x0001 = 0x7FFF (overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7FFF)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true);  // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().d(), 0x8000); // D unchanged
}

#[test]
fn test_cmpd_a_b_separate_registers() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: Test that D register properly combines A:B for comparison
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0xAB; // High byte
    cpu.registers_mut().b = 0xCD; // Low byte
    // D should be 0xABCD
    
    // Write CMPD #$ABCD instruction (prefixed with 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // CMPD immediate
    cpu.memory_bus().borrow_mut().write(0xC802, 0xAB); // High byte
    cpu.memory_bus().borrow_mut().write(0xC803, 0xCD); // Low byte
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: Verify D register combination and comparison
    assert_eq!(cpu.registers().d(), 0xABCD); // D properly combines A:B
    assert_eq!(cpu.registers().cc.z, true);  // Equal comparison
    assert_eq!(cpu.registers().cc.n, false); // Positive result
    assert_eq!(cpu.registers().cc.c, false); // No borrow
    assert_eq!(cpu.registers().a, 0xAB);     // A unchanged
    assert_eq!(cpu.registers().b, 0xCD);     // B unchanged
}