//! Tests for LDA (Load A Register) opcodes
//! Port-1:1 tests based on Vectrexy CPU implementation
//! 
//! C++ Original: Tests should verify exact behavior of OpLD<0, opCode>(A)
//! from vectrexy/libs/emulator/src/Cpu.cpp

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_lda_immediate_0x86() {
    // C++ Original: LDA #immediate - opcode 0x86
    // Test loading immediate value into A register
    let mut cpu = create_test_cpu();
    
    // Set up memory: 0x86 0x42 (LDA #$42) - place in RAM area (0xC800+)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x86); // LDA #immediate
    memory_bus.borrow_mut().write(0xC801, 0x42); // immediate value
    
    // Set PC to start of instruction  
    cpu.registers_mut().pc = 0xC800;
    
    // Execute one instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0;
    assert_eq!(cpu.registers().a, 0x42, "A register should contain loaded value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 2, "LDA #immediate should take 2 cycles"); // From CpuOpCodes.h
    
    // Condition codes
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
    assert_eq!(cpu.registers().cc.v, false, "Overflow flag should be clear (always for LD)");
}

#[test]
fn test_lda_immediate_zero() {
    // Test LDA with zero value to verify Zero flag setting
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0x86); // LDA #immediate
    memory_bus.borrow_mut().write(0xC811, 0x00); // zero value
    
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_lda_immediate_negative() {
    // Test LDA with negative value (bit 7 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0x86); // LDA #immediate
    memory_bus.borrow_mut().write(0xC821, 0x80); // negative value
    
    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 7 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_lda_direct_0x96() {
    // C++ Original: LDA direct - opcode 0x96
    // Test loading from direct page memory
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDA $20 - place in RAM
    memory_bus.borrow_mut().write(0xC830, 0x96); // LDA direct
    memory_bus.borrow_mut().write(0xC831, 0x20); // direct page address (low byte)
    
    // Set up target memory location (DP = 0xC8, so address = $C820) - must be in RAM range
    memory_bus.borrow_mut().write(0xC820, 0x55); // value to load
    
    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55, "A should contain value from direct page memory");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance past instruction");
    assert_eq!(cycles, 4, "LDA direct should take 4 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag check");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_lda_extended_0xb6() {
    // C++ Original: LDA extended - opcode 0xB6  
    // Test loading from 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDA $C850 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xB6); // LDA extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x50); // low byte of address - target $C850
    
    // Set up target memory location in RAM
    memory_bus.borrow_mut().write(0xC850, 0xAA); // value to load
    
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xAA, "A should contain value from extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 5, "LDA extended should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xAA");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

// TODO: Add test for LDA indexed (0xA6) - requires implementing indexed addressing mode helpers
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add boundary condition tests (direct page boundaries, etc.)