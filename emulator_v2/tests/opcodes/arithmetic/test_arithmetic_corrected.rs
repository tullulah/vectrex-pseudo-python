//! Tests for indexed arithmetic operations (ANDA, EORA, ORAA, SUBA, ADDA)
//! 
//! This test module verifies the implementation of arithmetic instructions
//! with indexed addressing modes, covering opcodes:
//! - ANDA indexed: 0xA4 (AND A with memory indexed)
//! - EORA indexed: 0xA8 (Exclusive OR A with memory indexed)  
//! - ORAA indexed: 0xAA (OR A with memory indexed)
//! - SUBA indexed: 0xA0 (Subtract memory from A indexed)
//! - ADDA indexed: 0xAB (Add memory to A indexed)

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
fn test_anda_indexed_0xa4() {
    // Test ANDA indexed - AND A register with memory value using indexed addressing
    let mut cpu = create_test_cpu();
    
    // Setup registers
    cpu.registers_mut().a = 0xFF;       // A = 0xFF (11111111)
    cpu.registers_mut().x = 0xC900;     // X points to memory location
    
    // Set up memory: ANDA indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x0F); // Value at memory location = 0x0F (00001111)
    memory_bus.borrow_mut().write(0xC800, 0xA4); // ANDA indexed
    memory_bus.borrow_mut().write(0xC801, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC800;
    
    // Execute ANDA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0xFF AND 0x0F = 0x0F
    assert_eq!(cpu.registers().a, 0x0F, "A should be result of AND operation");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "ANDA indexed should take 4 cycles");
    
    // Verify condition codes - AND operation clears V, sets N/Z based on result
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for AND operation");
}

#[test]
fn test_eora_indexed_0xa8() {
    // Test EORA indexed - Exclusive OR A register with memory value
    let mut cpu = create_test_cpu();
    
    // Setup registers
    cpu.registers_mut().a = 0xAA;       // A = 0xAA (10101010)
    cpu.registers_mut().y = 0xCA00;     // Y points to memory location
    
    // Set up memory: EORA indexed,Y
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCA00, 0x55); // Value at memory = 0x55 (01010101)
    memory_bus.borrow_mut().write(0xC810, 0xA8); // EORA indexed
    memory_bus.borrow_mut().write(0xC811, 0xA4); // ,Y postbyte (0xA4 = ,Y sin offset)
    
    cpu.registers_mut().pc = 0xC810;
    
    // Execute EORA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0xAA XOR 0x55 = 0xFF
    assert_eq!(cpu.registers().a, 0xFF, "A should be result of EOR operation");
    assert_eq!(cpu.registers().pc, 0xC812, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "EORA indexed should take 4 cycles");
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for EOR operation");
}

#[test]
fn test_oraa_indexed_0xaa() {
    // Test ORAA indexed - OR A register with memory value
    let mut cpu = create_test_cpu();
    
    // Setup registers
    cpu.registers_mut().a = 0x0F;       // A = 0x0F (00001111)
    cpu.registers_mut().u = 0xCB00;     // U points to memory location
    
    // Set up memory: ORAA indexed,U
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCB00, 0xF0); // Value at memory = 0xF0 (11110000)
    memory_bus.borrow_mut().write(0xC820, 0xAA); // ORAA indexed
    memory_bus.borrow_mut().write(0xC821, 0xC4); // ,U postbyte (0xC4 = ,U sin offset)
    
    cpu.registers_mut().pc = 0xC820;
    
    // Execute ORAA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x0F OR 0xF0 = 0xFF
    assert_eq!(cpu.registers().a, 0xFF, "A should be result of OR operation");
    assert_eq!(cpu.registers().pc, 0xC822, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "ORAA indexed should take 4 cycles");
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for OR operation");
}

#[test]
fn test_suba_indexed_0xa0() {
    // Test SUBA indexed - Subtract memory value from A register
    let mut cpu = create_test_cpu();
    
    // Setup registers
    cpu.registers_mut().a = 0x50;       // A = 0x50 (80 decimal)
    cpu.registers_mut().x = 0xCC00;     // X points to memory location
    
    // Set up memory: SUBA indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCC00, 0x30); // Value at memory = 0x30 (48 decimal)
    memory_bus.borrow_mut().write(0xC830, 0xA0); // SUBA indexed
    memory_bus.borrow_mut().write(0xC831, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC830;
    
    // Execute SUBA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x50 - 0x30 = 0x20
    assert_eq!(cpu.registers().a, 0x20, "A should be result of SUB operation");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "SUBA indexed should take 4 cycles");
    
    // Verify condition codes - subtraction sets flags based on result
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no borrow)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for normal subtraction");
}

#[test]
fn test_adda_indexed_overflow_0xab() {
    // Test ADDA indexed with overflow condition
    let mut cpu = create_test_cpu();
    
    // Setup registers for overflow test
    cpu.registers_mut().a = 0x7F;       // A = 0x7F (max positive signed 8-bit)
    cpu.registers_mut().x = 0xCD00;     // X points to memory location
    
    // Set up memory: ADDA indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCD00, 0x01); // Value = 0x01 (will cause overflow)
    memory_bus.borrow_mut().write(0xC840, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(0xC841, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC840;
    
    // Execute ADDA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x7F + 0x01 = 0x80 (overflow from positive to negative)
    assert_eq!(cpu.registers().a, 0x80, "A should be result of ADD operation");
    assert_eq!(cpu.registers().pc, 0xC842, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "ADDA indexed should take 4 cycles");
    
    // Verify condition codes - overflow should be detected
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry out)");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set for overflow");
}

#[test]
fn test_adda_indexed_carry_0xab() {
    // Test ADDA indexed with carry condition
    let mut cpu = create_test_cpu();
    
    // Setup registers for carry test
    cpu.registers_mut().a = 0xFF;       // A = 0xFF (-1 or 255)
    cpu.registers_mut().x = 0xCE00;     // X points to memory location
    
    // Set up memory: ADDA indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCE00, 0x01); // Value = 0x01
    memory_bus.borrow_mut().write(0xC850, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(0xC851, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC850;
    
    // Execute ADDA indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0xFF + 0x01 = 0x00 with carry
    assert_eq!(cpu.registers().a, 0x00, "A should be zero");
    assert_eq!(cpu.registers().pc, 0xC852, "PC should advance by 2 bytes");
    assert_eq!(cycles, 4, "ADDA indexed should take 4 cycles");
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for zero");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set for zero result");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be set for carry out");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_anda_zero_result_0xa4() {
    // Test ANDA that results in zero to verify Zero flag
    let mut cpu = create_test_cpu();
    
    // Setup for zero result
    cpu.registers_mut().a = 0xF0;       // A = 0xF0 (11110000)
    cpu.registers_mut().x = 0xCF00;     // X points to memory location
    
    // Set up memory: ANDA indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCF00, 0x0F); // Value = 0x0F (00001111)
    memory_bus.borrow_mut().write(0xC860, 0xA4); // ANDA indexed
    memory_bus.borrow_mut().write(0xC861, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC860;
    
    // Execute ANDA indexed
    cpu.execute_instruction(false, false);
    
    // Verify result: 0xF0 AND 0x0F = 0x00
    assert_eq!(cpu.registers().a, 0x00, "A should be zero");
    
    // Verify Zero flag is set
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set for zero result");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for zero");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for AND operation");
}