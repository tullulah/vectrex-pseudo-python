// Test suite for EORA (Exclusive OR with A) opcode 0x88
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpEOR for A register

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

/// Test EORA immediate (0x88) - EOR A with immediate value
#[test]
fn test_eora_immediate_0x88() {
    // C++ Original: OpEOR sets reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xAA; // Initial A = 10101010
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: EORA #$FF instruction (toggle all bits)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x88); // EORA immediate opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0xFF); // EOR mask 11111111
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55, "A should be 0xAA ^ 0xFF = 0x55");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by EOR)");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

/// Test EORA immediate with zero result
#[test]
fn test_eora_immediate_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x42; // A = 0x42
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: EORA #$42 instruction (same as A)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x88); // EORA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x42); // EOR with same value
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x42 ^ 0x42 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

/// Test EORA immediate with negative result
#[test]
fn test_eora_immediate_negative() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x7F; // A = 01111111
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: EORA #$FF instruction (flip all bits - result will be 10000000)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x88); // EORA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0xFF); // EOR with all ones
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0x7F ^ 0xFF = 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

/// Test EORA immediate toggle operation
#[test]
fn test_eora_immediate_toggle() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x33; // A = 00110011
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: EORA #$0F instruction (toggle lower nibble)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x88); // EORA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // EOR with 00001111
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x3C, "A should be 0x33 ^ 0x0F = 0x3C");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

// ========== DIRECT ADDRESSING MODE TESTS (0x98) ==========

#[test]
fn test_eora_direct_0x98_basic() {
    // C++ Original: OpEOR<1, 0x98>(A); reg = reg ^ ReadOperandValue8<DirectAddressing>();
    let mut cpu = create_test_cpu();
    
    // Setup: A=0xF0, [0xC860]=0x0F
    cpu.registers_mut().a = 0xF0;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC860, 0x0F);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x98);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x60);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0xF0 ^ 0x0F = 0xFF
    assert_eq!(cpu.registers().a, 0xFF);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(cpu.registers().cc.n); // Negative
    assert!(!cpu.registers().cc.v); // Overflow always cleared
}

#[test]
fn test_eora_direct_0x98_self_cancel() {
    // C++ Original: OpEOR<1, 0x98>(A); reg = reg ^ ReadOperandValue8<DirectAddressing>();
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x55, [0xC870]=0x55 (XOR with self = 0)
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC870, 0x55);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x98);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x70);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x55 ^ 0x55 = 0x00
    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z); // Zero flag set
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(!cpu.registers().cc.v); // Overflow always cleared
}