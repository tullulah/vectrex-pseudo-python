// test_ldb.rs - Tests para opcodes LDB (Load B register)
// C++ Original: OpLD<0, opCode>(B) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_ldb_immediate_0xc6() {
    // C++ Original: LDB #immediate - opcode 0xC6 
    // Test LDB with immediate value - basic functionality
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC6); // LDB #immediate
    memory_bus.borrow_mut().write(0xC801, 0x42); // immediate value
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x42, "B register should contain immediate value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past 2-byte instruction");
    assert_eq!(cycles, 2, "LDB immediate should take 2 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldb_immediate_zero() {
    // Test LDB with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0xC6); // LDB #immediate
    memory_bus.borrow_mut().write(0xC811, 0x00); // zero value
    
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set for zero value");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_ldb_immediate_negative() {
    // Test LDB with negative value (bit 7 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xC6); // LDB #immediate
    memory_bus.borrow_mut().write(0xC821, 0x80); // negative value
    
    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 7 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_ldb_direct_0xd6() {
    // C++ Original: LDB direct - opcode 0xD6
    // Test loading from direct page memory
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDB $40 - place in RAM
    memory_bus.borrow_mut().write(0xC830, 0xD6); // LDB direct
    memory_bus.borrow_mut().write(0xC831, 0x40); // direct page address (low byte)
    
    // Set up target memory location (DP = 0xC8, so address = $C840) - must be in RAM range
    memory_bus.borrow_mut().write(0xC840, 0x77); // value to load (matching DP + offset)
    
    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x77, "B should contain value from direct page memory");
    assert_eq!(cpu.registers().pc, 0xC832, "PC should advance past instruction");
    assert_eq!(cycles, 4, "LDB direct should take 4 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag check");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldb_extended_0xf6() {
    // C++ Original: LDB extended - opcode 0xF6  
    // Test loading from 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: LDB $C860 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xF6); // LDB extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x60); // low byte of address - target $C860
    
    // Set up target memory location in RAM
    memory_bus.borrow_mut().write(0xC860, 0xBB); // value to load
    
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xBB, "B should contain value from extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 5, "LDB extended should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xBB");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

// TODO: Add test for LDB indexed (0xE6) - requires implementing indexed addressing mode helpers
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add boundary condition tests (direct page boundaries, etc.)