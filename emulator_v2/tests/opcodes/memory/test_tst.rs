// Test suite for TST (Test) opcode 0x7D
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpTST for memory test

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

/// Test TST extended (0x7D) - Test memory value (sets flags without changing value)
#[test]
fn test_tst_extended_0x7D() {
    // C++ Original: OpTST - Sets N, Z flags based on value; V=0, C=0 always
    let mut cpu = create_test_cpu();
    
    // Setup test: store negative value in memory location (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC950, 0x80); // MSB set (negative), use RAM area
    memory_bus.borrow_mut().write(RAM_START, 0x7D);  // TST extended opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9);  // high byte (RAM area)
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50);  // low byte
    
    cpu.registers_mut().pc = RAM_START;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory unchanged
    assert_eq!(memory_bus.borrow().read(0xC950), 0x80, "TST should not modify memory");
    // Verify condition codes: N=1 (MSB set), Z=0, V=0, C=0 (always cleared)
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (0x80 is negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (value is not zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().pc, RAM_START + 3, "PC should advance by 3 for extended addressing");
    assert!(cycles > 0);
}

/// Test TST extended with zero value
#[test]
fn test_tst_extended_zero() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store zero value in memory
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC950, 0x00); // Zero value
    memory_bus.borrow_mut().write(RAM_START, 0x7D);  // TST extended opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9);  // high byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50);  // low byte
    
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    // Verify memory unchanged
    assert_eq!(memory_bus.borrow().read(0xC950), 0x00, "TST should not modify memory");
    // Verify condition codes for zero value
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (0x00 is positive)");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (value is zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().pc, RAM_START + 3);
}

/// Test TST extended with positive value
#[test]
fn test_tst_extended_positive() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store positive value in memory
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC950, 0x42); // Positive value (0x42 = 01000010)
    memory_bus.borrow_mut().write(RAM_START, 0x7D);  // TST extended opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9);  // high byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50);  // low byte
    
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    // Verify memory unchanged
    assert_eq!(memory_bus.borrow().read(0xC950), 0x42, "TST should not modify memory");
    // Verify condition codes for positive value
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (0x42 is positive)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (value is not zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (always 0 for TST)");
    assert_eq!(cpu.registers().pc, RAM_START + 3);
}

/// Test TST extended flag behavior (per Vectrexy: sets N,Z, clears V, leaves C unchanged)
#[test]
fn test_tst_extended_clears_flags() {
    let mut cpu = create_test_cpu();
    
    // Setup test: pre-set V and C flags to verify they get cleared
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC950, 0x7F); // Positive value
    memory_bus.borrow_mut().write(RAM_START, 0x7D);  // TST extended opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9);  // high byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50);  // low byte
    
    cpu.registers_mut().pc = RAM_START;
    // Pre-set flags that should be cleared
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = true;
    
    cpu.execute_instruction(false, false);
    
    // Verify memory unchanged
    assert_eq!(memory_bus.borrow().read(0xC950), 0x7F, "TST should not modify memory");
    // Verify TST sets N, Z, clears V, but leaves C unchanged (per Vectrexy)
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (0x7F is positive)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (value is not zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be cleared by TST");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be unchanged by TST (per Vectrexy)");
    assert_eq!(cpu.registers().pc, RAM_START + 3);
}