// test_stb.rs - Tests para opcodes STB (Store B register)
// C++ Original: OpST<0, opCode>(B) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_stb_direct_0xd7() {
    // C++ Original: STB direct - opcode 0xD7
    // Test storing B register to direct page memory
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: STB $30 - place in RAM
    memory_bus.borrow_mut().write(0xC800, 0xD7); // STB direct
    memory_bus.borrow_mut().write(0xC801, 0x30); // direct page address (low byte)
    
    // Set up B register with test value
    cpu.registers_mut().b = 0x42;
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify B value was stored to memory (DP + offset = $C830)
    let stored_value = memory_bus.borrow().read(0xC830);
    assert_eq!(stored_value, 0x42, "B value should be stored to direct page memory");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 4, "STB direct should take 4 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag check for stored value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}

#[test]
fn test_stb_direct_zero() {
    // Test STB with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0xD7); // STB direct
    memory_bus.borrow_mut().write(0xC811, 0x70); // direct page address
    
    cpu.registers_mut().b = 0x00; // zero value
    cpu.registers_mut().pc = 0xC810;
    cpu.registers_mut().dp = 0xC8;
    
    cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC870);
    assert_eq!(stored_value, 0x00);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set when storing zero");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_stb_direct_negative() {
    // Test STB with negative value (bit 7 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xD7); // STB direct
    memory_bus.borrow_mut().write(0xC821, 0x80); // direct page address
    
    cpu.registers_mut().b = 0x80; // negative value
    cpu.registers_mut().pc = 0xC820;
    cpu.registers_mut().dp = 0xC8;
    
    cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC880);
    assert_eq!(stored_value, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 7 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_stb_extended_0xf7() {
    // C++ Original: STB extended - opcode 0xF7  
    // Test storing to 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: STB $C8A0 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xF7); // STB extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0xA0); // low byte of address - target $C8A0
    
    cpu.registers_mut().b = 0xFF;
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC8A0);
    assert_eq!(stored_value, 0xFF, "B should be stored to extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 5, "STB extended should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xFF");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}

#[test]
fn test_stb_verify_register_independence() {
    // Verify that STB doesn't affect A register or other registers
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC850, 0xD7); // STB direct
    memory_bus.borrow_mut().write(0xC851, 0x90); // direct page address
    
    // Set initial state
    cpu.registers_mut().a = 0x12; // A should remain unchanged
    cpu.registers_mut().b = 0x34; // B value to store
    cpu.registers_mut().x = 0x5678; // X should remain unchanged
    cpu.registers_mut().pc = 0xC850;
    cpu.registers_mut().dp = 0xC8;
    
    cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC890);
    assert_eq!(stored_value, 0x34, "B value should be stored");
    
    // Verify other registers unchanged
    assert_eq!(cpu.registers().a, 0x12, "A register should remain unchanged");
    assert_eq!(cpu.registers().b, 0x34, "B register should remain unchanged");  
    assert_eq!(cpu.registers().x, 0x5678, "X register should remain unchanged");
}

// TODO: Add test for STB indexed (0xE7) - requires implementing indexed addressing mode helpers
// TODO: Add edge case tests for boundary values
// TODO: Verify exact cycle counts match CpuOpCodes.h