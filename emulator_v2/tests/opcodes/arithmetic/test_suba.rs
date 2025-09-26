// Test suite for SUBA (Subtract from A) opcode 0x80
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpSUB for A register

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    Cpu6809::new(memory_bus)
}

#[test]
fn test_suba_immediate_basic() {
    // Test SUBA #$0F - Subtract immediate from A
    // C++ Original: OpSUB<0, 0x80>(A); reg = SubtractImpl(reg, ReadOperandValue8<addrMode>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x20; // Initial A = 32
    
    // Setup: SUBA #$0F instruction at 0xC800 (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x80); // SUBA immediate opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // Subtract 15
    cpu.registers_mut().pc = RAM_START;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x11, "A should be 0x20 - 0x0F = 0x11");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (no overflow)");
    assert_eq!(cpu.registers().pc, RAM_START + 2, "PC should advance by 2");
    assert_eq!(cycles, 2, "SUBA immediate should take 2 cycles");
}

#[test]
fn test_suba_immediate_borrow() {
    // Test SUBA with borrow (underflow)
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x05; // Initial A = 5
    
    // Setup: SUBA #$10 instruction (subtract 16 from 5)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x80); // SUBA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x10); // Subtract 16
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xF5, "A should be 0x05 - 0x10 = 0xF5 (wrapped)");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result is negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, true, "C flag inverted by subtract implementation");
}

#[test]
fn test_suba_immediate_zero_result() {
    // Test SUBA resulting in zero
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x42; // Initial A = 66
    
    // Setup: SUBA #$42 instruction (subtract same value)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x80); // SUBA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x42); // Subtract same value
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x42 - 0x42 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

// ========== DIRECT ADDRESSING MODE TESTS (0x90) ==========

#[test]
fn test_suba_direct_0x90_basic() {
    // C++ Original: OpSUB<1, 0x90>(A); reg = SubtractImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x50, [0xC810]=0x20
    cpu.registers_mut().a = 0x50;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8xx (in RAM range)
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC810, 0x20); // Data in RAM
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x90); // SUBA direct
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x10); // Direct address offset
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x50 - 0x20 = 0x30
    assert_eq!(cpu.registers().a, 0x30);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

#[test]
fn test_suba_direct_0x90_zero_result() {
    // C++ Original: OpSUB<1, 0x90>(A); reg = SubtractImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x42, [0xC820]=0x42 (same value)
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC820, 0x42);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x90);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x20);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x42 - 0x42 = 0x00
    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z); // Zero flag set
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
}

#[test]
fn test_suba_direct_0x90_borrow() {
    // C++ Original: OpSUB<1, 0x90>(A); reg = SubtractImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x30, [0xC830]=0x50 (borrow case)
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC830, 0x50);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x90);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x30);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x30 - 0x50 = 0xE0 (with borrow)
    assert_eq!(cpu.registers().a, 0xE0);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(cpu.registers().cc.n); // Negative
    assert!(cpu.registers().cc.c); // Borrow occurred
    assert!(!cpu.registers().cc.v); // No overflow
}