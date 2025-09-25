// C++ Original: Tests for CMPX opcodes (0x8C, 0x9C, 0xAC, 0xBC)
// Port 1:1 from Vectrexy test patterns for 16-bit X register comparison

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
fn test_cmpx_immediate_0x8C() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX #$1234 with X=$1234 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x1234;
    
    // Write CMPX #$1234 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8C); // CMPX immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC802, 0x34); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().x, 0x1234);   // X unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes)
    assert_eq!(cycles, 4);                   // 4 cycles for immediate mode (16-bit)
}

#[test]
fn test_cmpx_immediate_greater() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX #$1000 with X=$1234 (X > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x1234;
    
    // Write CMPX #$1000 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8C); // CMPX immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x10); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: X > operand, result positive (0x1234 - 0x1000 = 0x0234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().x, 0x1234);   // X unchanged
    assert_eq!(cycles, 4);                   // 4 cycles for immediate mode
}

#[test]
fn test_cmpx_immediate_less() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX #$2000 with X=$1234 (X < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x1234;
    
    // Write CMPX #$2000 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8C); // CMPX immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x20); // High byte of compare value
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00); // Low byte of compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: X < operand, borrow needed (0x1234 - 0x2000 = 0xF234)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (result is negative)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow needed)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().x, 0x1234);   // X unchanged
    assert_eq!(cycles, 4);                   // 4 cycles for immediate mode
}

#[test]
fn test_cmpx_direct_0x9C() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX $20 with X=$5678, memory[$C820]=$5678 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x56); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0x78); // Low byte
    
    // Write CMPX direct instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x9C); // CMPX direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x20); // Direct page address
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().x, 0x5678);   // X unchanged
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 5);                   // 5 cycles for direct mode
}

#[test]
fn test_cmpx_extended_0xBC() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX $C820 with X=$9ABC, memory[$C820]=$9ABC (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x9ABC;
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC820, 0x9A); // High byte
    cpu.memory_bus().borrow_mut().write(0xC821, 0xBC); // Low byte
    
    // Write CMPX extended instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xBC); // CMPX extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8); // High byte of address
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // Low byte of address
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x0000 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().x, 0x9ABC);   // X unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced
    assert_eq!(cycles, 6);                   // 6 cycles for extended mode
}

#[test]
fn test_cmpx_indexed_0xAC() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX ,Y with X=$DEF0, Y=$C850, memory[$C850]=$DEF0 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0xDEF0;
    cpu.registers_mut().y = 0xC850;
    
    // Write test data to memory (16-bit value in big-endian)
    cpu.memory_bus().borrow_mut().write(0xC850, 0xDE); // High byte
    cpu.memory_bus().borrow_mut().write(0xC851, 0xF0); // Low byte
    
    // Write CMPX indexed instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xAC); // CMPX indexed
    cpu.memory_bus().borrow_mut().write(0xC801, 0xA4); // ,Y addressing mode
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().x, 0xDEF0);   // X unchanged
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 5);                   // 5 cycles for indexed mode
}

#[test]
fn test_cmpx_overflow_case() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPX with overflow condition (0x8000 - 0x0001 = 0x7FFF, overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x8000; // -32768 in signed 16-bit
    
    // Write CMPX #$0001 instruction  
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8C); // CMPX immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x00); // High byte
    cpu.memory_bus().borrow_mut().write(0xC802, 0x01); // Low byte
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags - subtract 0x8000 - 0x0001 = 0x7FFF (overflow occurs: -32768 - 1 = -32769, wraps to +32767)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7FFF)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true);  // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().x, 0x8000);   // X unchanged
}