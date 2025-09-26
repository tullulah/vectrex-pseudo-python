// test_ldx.rs - Tests para opcodes LDX (Load X register)
// C++ Original: OpLD<0, opCode>(X) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_ldx_immediate_0x8e() {
    // C++ Original: LDX #immediate - opcode 0x8E 
    // Test LDX with immediate 16-bit value - basic functionality
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8E); // LDX #immediate
    memory_bus.borrow_mut().write(0xC801, 0x12); // high byte of immediate value
    memory_bus.borrow_mut().write(0xC802, 0x34); // low byte of immediate value
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x1234, "X register should contain immediate value");
    assert_eq!(cpu.registers().pc, 0xC803, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 3, "LDX immediate should take 3 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldx_immediate_zero() {
    // Test LDX with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0x8E); // LDX #immediate
    memory_bus.borrow_mut().write(0xC811, 0x00); // high byte = 0
    memory_bus.borrow_mut().write(0xC812, 0x00); // low byte = 0
    
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x0000);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_ldx_immediate_negative() {
    // Test LDX with negative value (bit 15 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0x8E); // LDX #immediate
    memory_bus.borrow_mut().write(0xC821, 0x80); // high byte = 0x80 (negative)
    memory_bus.borrow_mut().write(0xC822, 0x00); // low byte = 0x00
    
    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x8000);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 15 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_ldx_direct_0x9e() {
    // C++ Original: LDX direct - opcode 0x9E
    // Test loading from direct page memory (16-bit)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDX $50 - place in RAM
    memory_bus.borrow_mut().write(0xC830, 0x9E); // LDX direct
    memory_bus.borrow_mut().write(0xC831, 0x50); // direct page address (low byte)
    
    // Set up target memory location (DP = 0xC8, so address = $C850) - must be in RAM range
    memory_bus.borrow_mut().write(0xC850, 0xAB); // high byte value to load
    memory_bus.borrow_mut().write(0xC851, 0xCD); // low byte value to load
    
    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0xABCD, "X should contain value from direct page memory");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance past instruction");
    assert_eq!(cycles, 5, "LDX direct should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xABCD");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldx_extended_0xbe() {
    // C++ Original: LDX extended - opcode 0xBE  
    // Test loading from 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDX $C870 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xBE); // LDX extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x70); // low byte of address - target $C870
    
    // Set up target memory location in RAM
    memory_bus.borrow_mut().write(0xC870, 0x55); // high byte value to load
    memory_bus.borrow_mut().write(0xC871, 0x77); // low byte value to load
    
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x5577, "X should contain value from extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 6, "LDX extended should take 6 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for 0x5577");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

// TODO: Add test for LDX indexed (0xAE) - requires implementing indexed addressing mode helpers  
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add boundary condition tests (direct page boundaries, etc.)