// test_ldu.rs - Tests para opcodes LDU (Load U register)
// C++ Original: OpLD<0, opCode>(U) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_ldu_immediate_0xce() {
    // C++ Original: LDU #immediate - opcode 0xCE 
    // Test LDU with immediate 16-bit value - basic functionality
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCE); // LDU #immediate
    memory_bus.borrow_mut().write(0xC801, 0x12); // high byte of immediate value
    memory_bus.borrow_mut().write(0xC802, 0x34); // low byte of immediate value
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().u, 0x1234, "U register should contain immediate value");
    assert_eq!(cpu.registers().pc, 0xC803, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 3, "LDU immediate should take 3 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldu_immediate_zero() {
    // Test LDU with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0xCE); // LDU #immediate
    memory_bus.borrow_mut().write(0xC811, 0x00); // high byte = 0
    memory_bus.borrow_mut().write(0xC812, 0x00); // low byte = 0
    
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().u, 0x0000);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_ldu_immediate_negative() {
    // Test LDU with negative value (bit 15 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xCE); // LDU #immediate
    memory_bus.borrow_mut().write(0xC821, 0x80); // high byte = 0x80 (negative)
    memory_bus.borrow_mut().write(0xC822, 0x00); // low byte = 0x00
    
    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().u, 0x8000);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 15 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_ldu_direct_0xde() {
    // C++ Original: LDU direct - opcode 0xDE
    // Test loading from direct page memory (16-bit)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDU $70 - place in RAM
    memory_bus.borrow_mut().write(0xC830, 0xDE); // LDU direct
    memory_bus.borrow_mut().write(0xC831, 0x70); // direct page address (low byte)
    
    // Set up target memory location (DP = 0xC8, so address = $C870) - must be in RAM range
    memory_bus.borrow_mut().write(0xC870, 0xAB); // high byte value to load
    memory_bus.borrow_mut().write(0xC871, 0xCD); // low byte value to load
    
    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().u, 0xABCD, "U should contain value from direct page memory");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance past instruction");
    assert_eq!(cycles, 5, "LDU direct should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xABCD");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldu_extended_0xfe() {
    // C++ Original: LDU extended - opcode 0xFE  
    // Test loading from 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDU $C890 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xFE); // LDU extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x90); // low byte of address - target $C890
    
    // Set up target memory location in RAM
    memory_bus.borrow_mut().write(0xC890, 0x55); // high byte value to load
    memory_bus.borrow_mut().write(0xC891, 0x77); // low byte value to load
    
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().u, 0x5577, "U should contain value from extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 6, "LDU extended should take 6 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for 0x5577");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

// TODO: Add test for LDU indexed (0xEE) - requires implementing indexed addressing mode helpers  
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add boundary condition tests (direct page boundaries, etc.)