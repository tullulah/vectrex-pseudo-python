//! Tests for 16-bit Store operations (STX, STD, STU) with indexed addressing
//! 
//! This test module verifies the implementation of 16-bit store instructions
//! with indexed addressing modes, covering opcodes:
//! - STX: 0x9F (direct), 0xAF (indexed), 0xBF (extended)  
//! - STD: 0xDD (direct), 0xED (indexed), 0xFD (extended)
//! - STU: 0xDF (direct), 0xEF (indexed), 0xFF (extended)

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
fn test_stx_direct_0x9f() {
    // Test STX direct addressing - stores X register to direct page location
    let mut cpu = create_test_cpu();
    
    // Setup X register with test value
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (pointing to RAM area)
    
    // Set up memory: STX direct at 0xC800
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x9F); // STX direct
    memory_bus.borrow_mut().write(0xC801, 0x50); // Direct address offset (0xC850)
    
    cpu.registers_mut().pc = 0xC800;
    
    // Execute STX direct
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify X register value was stored at direct address (0xC8 << 8 + 0x50 = 0xC850)
    assert_eq!(memory_bus.borrow().read(0xC850), 0x12, "High byte of X should be stored");
    assert_eq!(memory_bus.borrow().read(0xC851), 0x34, "Low byte of X should be stored");
    
    // Verify PC advanced correctly
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2 bytes");
    assert_eq!(cycles, 5, "STX direct should take 5 cycles");
    
    // STX only affects N and Z flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (not affected by STX)");
}

#[test]
fn test_stx_indexed_0xaf() {
    // Test STX indexed addressing - stores X register using indexed mode
    let mut cpu = create_test_cpu();
    
    // Setup registers
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = 0xC900; // Y points to target memory location
    
    // Set up memory: STX indexed,Y
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xAF); // STX indexed
    memory_bus.borrow_mut().write(0xC801, 0xA4); // ,Y addressing mode postbyte (0xA4 = ,Y sin offset)
    
    cpu.registers_mut().pc = 0xC800;
    
    // Execute STX indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify X register value was stored at Y address (0xC900)
    assert_eq!(memory_bus.borrow().read(0xC900), 0x56, "High byte of X should be stored at Y");
    assert_eq!(memory_bus.borrow().read(0xC901), 0x78, "Low byte of X should be stored at Y+1");
    
    // Verify PC advanced correctly
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2 bytes");
    assert_eq!(cycles, 5, "STX indexed should take 5 cycles"); // Base cycles for indexed
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
}

#[test]
fn test_std_indexed_0xed() {
    // Test STD (Store D register) indexed addressing
    let mut cpu = create_test_cpu();
    
    // Setup D register (A:B combined)
    cpu.registers_mut().a = 0x9A;  // High byte of D
    cpu.registers_mut().b = 0xBC;  // Low byte of D
    cpu.registers_mut().x = 0xCA00; // X points to target memory location
    
    // Set up memory: STD indexed,X
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0xED); // STD indexed
    memory_bus.borrow_mut().write(0xC811, 0x84); // ,X addressing mode postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC810;
    
    // Execute STD indexed
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify D register value was stored at X address (0xCA00)
    assert_eq!(memory_bus.borrow().read(0xCA00), 0x9A, "High byte of D (A) should be stored at X");
    assert_eq!(memory_bus.borrow().read(0xCA01), 0xBC, "Low byte of D (B) should be stored at X+1");
    
    // Verify PC advanced correctly
    assert_eq!(cpu.registers().pc, 0xC812, "PC should advance by 2 bytes");
    assert_eq!(cycles, 5, "STD indexed should take 5 cycles");
    
    // Verify condition codes - STD affects N and Z based on 16-bit D value
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative 16-bit value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (not affected by STD)");
}

#[test]
fn test_stu_extended_0xff() {
    // Test STU (Store U register) extended addressing
    let mut cpu = create_test_cpu();
    
    // Setup U register  
    cpu.registers_mut().u = 0xDEAD;
    
    // Set up memory: STU extended
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xFF); // STU extended
    memory_bus.borrow_mut().write(0xC821, 0xCB); // Extended address high byte (0xCB00)
    memory_bus.borrow_mut().write(0xC822, 0x00); // Extended address low byte
    
    cpu.registers_mut().pc = 0xC820;
    
    // Execute STU extended
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify U register value was stored at extended address (0xCB00)
    assert_eq!(memory_bus.borrow().read(0xCB00), 0xDE, "High byte of U should be stored");
    assert_eq!(memory_bus.borrow().read(0xCB01), 0xAD, "Low byte of U should be stored");
    
    // Verify PC advanced correctly
    assert_eq!(cpu.registers().pc, 0xC823, "PC should advance by 3 bytes for extended");
    assert_eq!(cycles, 6, "STU extended should take 6 cycles");
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
}

#[test]
fn test_stx_zero_value() {
    // Test STX with zero value to verify Zero flag setting
    let mut cpu = create_test_cpu();
    
    // Setup X register with zero
    cpu.registers_mut().x = 0x0000;
    cpu.registers_mut().dp = 0xC8;
    
    // Set up memory: STX direct
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC830, 0x9F); // STX direct
    memory_bus.borrow_mut().write(0xC831, 0x80); // Direct address offset
    
    cpu.registers_mut().pc = 0xC830;
    
    // Execute STX
    cpu.execute_instruction(false, false);
    
    // Verify zero was stored
    assert_eq!(memory_bus.borrow().read(0xC880), 0x00, "High byte should be zero");
    assert_eq!(memory_bus.borrow().read(0xC881), 0x00, "Low byte should be zero");
    
    // Verify Zero flag is set
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for zero value");
}

#[test]
fn test_std_negative_value() {
    // Test STD with negative value to verify Negative flag setting
    let mut cpu = create_test_cpu();
    
    // Setup D register with negative value (high bit set)
    cpu.registers_mut().a = 0x80;  // High byte - makes 16-bit value negative
    cpu.registers_mut().b = 0x00;  // Low byte
    cpu.registers_mut().x = 0xCC00;
    
    // Set up memory: STD indexed
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC840, 0xED); // STD indexed
    memory_bus.borrow_mut().write(0xC841, 0x84); // ,X postbyte (0x84 = ,X sin offset)
    
    cpu.registers_mut().pc = 0xC840;
    
    // Execute STD
    cpu.execute_instruction(false, false);
    
    // Verify negative value was stored
    assert_eq!(memory_bus.borrow().read(0xCC00), 0x80, "High byte should be 0x80");
    assert_eq!(memory_bus.borrow().read(0xCC01), 0x00, "Low byte should be 0x00");
    
    // Verify Negative flag is set
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative 16-bit value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
}