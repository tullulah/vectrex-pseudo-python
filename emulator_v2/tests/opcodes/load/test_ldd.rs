// test_ldd.rs - Tests para opcodes LDD (Load D register)
// C++ Original: OpLD<0, opCode>(D) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_ldd_immediate_0xcc() {
    // C++ Original: LDD #immediate - opcode 0xCC 
    // Test LDD with immediate 16-bit value - loads into A:B combined (D register)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCC); // LDD #immediate
    memory_bus.borrow_mut().write(0xC801, 0x12); // high byte of immediate value (A)
    memory_bus.borrow_mut().write(0xC802, 0x34); // low byte of immediate value (B)
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x12, "A register should contain high byte");
    assert_eq!(cpu.registers().b, 0x34, "B register should contain low byte");
    assert_eq!(cpu.registers().d(), 0x1234, "D register should contain combined value");
    assert_eq!(cpu.registers().pc, 0xC803, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 3, "LDD immediate should take 3 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldd_immediate_zero() {
    // Test LDD with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0xCC); // LDD #immediate
    memory_bus.borrow_mut().write(0xC811, 0x00); // high byte = 0
    memory_bus.borrow_mut().write(0xC812, 0x00); // low byte = 0
    
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().d(), 0x0000);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_ldd_immediate_negative() {
    // Test LDD with negative value (bit 15 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xCC); // LDD #immediate
    memory_bus.borrow_mut().write(0xC821, 0x80); // high byte = 0x80 (negative)
    memory_bus.borrow_mut().write(0xC822, 0x00); // low byte = 0x00
    
    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().d(), 0x8000);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 15 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_ldd_direct_0xdc() {
    // C++ Original: LDD direct - opcode 0xDC
    // Test loading from direct page memory (16-bit)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDD $60 - place in RAM
    memory_bus.borrow_mut().write(0xC830, 0xDC); // LDD direct
    memory_bus.borrow_mut().write(0xC831, 0x60); // direct page address (low byte)
    
    // Set up target memory location (DP = 0xC8, so address = $C860) - must be in RAM range
    memory_bus.borrow_mut().write(0xC860, 0xAB); // high byte value to load
    memory_bus.borrow_mut().write(0xC861, 0xCD); // low byte value to load
    
    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xAB, "A should contain high byte from memory");
    assert_eq!(cpu.registers().b, 0xCD, "B should contain low byte from memory");
    assert_eq!(cpu.registers().d(), 0xABCD, "D should contain combined value from direct page memory");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance past instruction");
    assert_eq!(cycles, 5, "LDD direct should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xABCD");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldd_extended_0xfc() {
    // C++ Original: LDD extended - opcode 0xFC  
    // Test loading from 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDD $C880 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xFC); // LDD extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x80); // low byte of address - target $C880
    
    // Set up target memory location in RAM
    memory_bus.borrow_mut().write(0xC880, 0x55); // high byte value to load
    memory_bus.borrow_mut().write(0xC881, 0x77); // low byte value to load
    
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55, "A should contain high byte from extended address");
    assert_eq!(cpu.registers().b, 0x77, "B should contain low byte from extended address");
    assert_eq!(cpu.registers().d(), 0x5577, "D should contain combined value from extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 6, "LDD extended should take 6 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for 0x5577");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

// TODO: Add test for LDD indexed (0xEC) - requires implementing indexed addressing mode helpers  
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add boundary condition tests (direct page boundaries, etc.)