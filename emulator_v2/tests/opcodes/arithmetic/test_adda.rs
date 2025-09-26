// Test suite for ADDA (Add to A) opcode 0x8B
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpADD without carry

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

/// Test ADDA immediate (0x8B) - Add immediate to A, basic case
#[test]
fn test_adda_immediate_0x8B() {
    // C++ Original: OpADD<0, 0x8B>(A); reg = AddImpl(reg, b, 0, CC);
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x10; // Initial A = 16
    cpu.registers_mut().cc.c = true; // Set carry to verify it's updated correctly
    cpu.registers_mut().cc.v = true; // Set overflow to verify it's updated correctly
    
    // Setup: ADDA #$0F instruction at RAM_START
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x8B); // ADDA immediate opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // Add 15
    cpu.registers_mut().pc = RAM_START;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x1F, "A should be 0x10 + 0x0F = 0x1F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (no overflow)");
    assert_eq!(cpu.registers().pc, RAM_START + 2, "PC should advance by 2");
    assert!(cycles > 0);
}

/// Test ADDA immediate with carry generation
#[test]
fn test_adda_immediate_carry() {
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // Initial A = 255
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: ADDA #$01 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // Add 1
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0xFF + 0x01 = 0x00 (wrapped)");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be set (carry occurred)");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
    assert!(cycles > 0);
}

/// Test ADDA immediate with overflow
#[test]
fn test_adda_immediate_overflow() {
    // Test ADDA with overflow: positive + positive = negative
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x7F; // 127 (max positive signed 8-bit)
    cpu.registers_mut().pc = RAM_START;
    
    // Setup: ADDA #$01 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // Add 1
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0x7F + 0x01 = 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result is negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry in unsigned)");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set (overflow occurred)");
    assert_eq!(cpu.registers().pc, RAM_START + 2);
    assert!(cycles > 0);
}

// ========== DIRECT ADDRESSING MODE TESTS (0x9B) ==========

#[test]
fn test_adda_direct_0x9b_basic() {
    // C++ Original: OpADD<1, 0x9B>(A); reg = AddImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x30, [0xC8A0]=0x20
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC8A0, 0x20);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x9B);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0xA0);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x30 + 0x20 = 0x50
    assert_eq!(cpu.registers().a, 0x50);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(!cpu.registers().cc.c); // No carry
    assert!(!cpu.registers().cc.v); // No overflow
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

#[test]
fn test_adda_direct_0x9b_carry() {
    // C++ Original: OpADD<1, 0x9B>(A); reg = AddImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0xFF, [0xC8B0]=0x01 (carry case)
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC8B0, 0x01);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x9B);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0xB0);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0xFF + 0x01 = 0x00 (with carry)
    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z); // Zero flag set
    assert!(!cpu.registers().cc.n); // Not negative
    assert!(cpu.registers().cc.c); // Carry occurred
    assert!(!cpu.registers().cc.v); // No overflow
}

#[test]
fn test_adda_direct_0x9b_overflow() {
    // C++ Original: OpADD<1, 0x9B>(A); reg = AddImpl(reg, ReadOperandValue8<DirectAddressing>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x7F, [0xC8C0]=0x01 (positive overflow case)
    cpu.registers_mut().a = 0x7F; // Max positive signed 8-bit
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    // Setup memory
    cpu.memory_bus().borrow_mut().write(0xC8C0, 0x01);
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x9B);
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0xC0);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Result: A = 0x7F + 0x01 = 0x80 (signed overflow: +127 + 1 = -128)
    assert_eq!(cpu.registers().a, 0x80);
    assert!(!cpu.registers().cc.z); // Not zero
    assert!(cpu.registers().cc.n); // Negative (0x80 = -128 signed)
    assert!(!cpu.registers().cc.c); // No carry
    assert!(cpu.registers().cc.v); // Overflow occurred
}