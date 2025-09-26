// Test suite for ANDA (AND with A) opcode 0x84
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpAND for A register

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

/// Test ANDA immediate (0x84) - AND A with immediate value
#[test]
fn test_anda_immediate_0x84() {
    // C++ Original: OpAND sets reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // Initial A = 11111111
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: ANDA #$0F instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x84); // ANDA immediate opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // AND mask 00001111
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x0F, "A should be 0xFF & 0x0F = 0x0F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by AND)");
    assert_eq!(cpu.registers().pc, RAM_START + 2, "PC should advance by 2");
    assert_eq!(cycles, 2, "ANDA immediate should take 2 cycles");
}

/// Test ANDA immediate with zero result
#[test]
fn test_anda_immediate_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // A starts as all bits set
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: ANDA #$00 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x84); // ANDA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00); // AND with zero
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0xFF & 0x00 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

/// Test ANDA immediate with negative result
#[test]
fn test_anda_immediate_negative() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // A starts as all bits set
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: ANDA #$80 instruction (result will have bit 7 set)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x84); // ANDA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x80); // AND with 10000000
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0xFF & 0x80 = 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

// ========== DIRECT ADDRESSING MODE TESTS (0x94) ==========

#[test]
fn test_anda_direct_0x94_basic() {
    // C++ Original: OpAND<1, 0x94>(A); reg = reg & ReadOperandValue8<DirectAddressing>(); 
    let mut cpu = create_test_cpu();
    
    // Setup: A=0xF0, [0xC840]=0x0F
    cpu.registers_mut().a = 0xF0;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC840, 0x0F);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x94);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x40);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0xF0 & 0x0F = 0x00
    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z); // Zero flag set
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(!cpu.registers().cc.v); // Overflow always cleared
}

#[test]
fn test_anda_direct_0x94_partial_mask() {
    // C++ Original: OpAND<1, 0x94>(A); reg = reg & ReadOperandValue8<DirectAddressing>(); 
    let mut cpu = create_test_cpu();
    
    // Setup: A=0xFF, [0xC850]=0xAA
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC850, 0xAA);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x94);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x50);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0xFF & 0xAA = 0xAA
    assert_eq!(cpu.registers().a, 0xAA);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(cpu.registers().cc.n); // Negative
    assert!(!cpu.registers().cc.v); // Overflow always cleared
}