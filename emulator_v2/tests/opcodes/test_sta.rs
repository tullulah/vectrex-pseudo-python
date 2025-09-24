// test_sta.rs - Tests para opcodes STA (Store A register)
// C++ Original: OpST<0, opCode>(A) en Cpu.cpp - Port 1:1 desde Vectrexy

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
fn test_sta_direct_0x97() {
    // C++ Original: STA direct - opcode 0x97
    // Test storing A register to direct page memory
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: STA $40 - place in RAM
    memory_bus.borrow_mut().write(0xC800, 0x97); // STA direct
    memory_bus.borrow_mut().write(0xC801, 0x40); // direct page address (low byte)
    
    // Set up A register with test value
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A value was stored to memory (DP + offset = $C840)
    let stored_value = memory_bus.borrow().read(0xC840);
    assert_eq!(stored_value, 0x55, "A value should be stored to direct page memory");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 4, "STA direct should take 4 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, false, "Negative flag check for stored value");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}

#[test]
fn test_sta_direct_zero() {
    // Test STA with zero value to verify Zero flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC810, 0x97); // STA direct
    memory_bus.borrow_mut().write(0xC811, 0x50); // direct page address
    
    cpu.registers_mut().a = 0x00; // zero value
    cpu.registers_mut().pc = 0xC810;
    cpu.registers_mut().dp = 0xC8;
    
    cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC850);
    assert_eq!(stored_value, 0x00);
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set when storing zero");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear for zero");
}

#[test]
fn test_sta_direct_negative() {
    // Test STA with negative value (bit 7 set) to verify Negative flag
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0x97); // STA direct
    memory_bus.borrow_mut().write(0xC821, 0x60); // direct page address
    
    cpu.registers_mut().a = 0x80; // negative value
    cpu.registers_mut().pc = 0xC820;
    cpu.registers_mut().dp = 0xC8;
    
    cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC860);
    assert_eq!(stored_value, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for bit 7 = 1");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear for non-zero value");
}

#[test]
fn test_sta_extended_0xb7() {
    // C++ Original: STA extended - opcode 0xB7  
    // Test storing to 16-bit absolute address
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Set up instruction: STA $C890 - place in RAM
    memory_bus.borrow_mut().write(0xC840, 0xB7); // STA extended
    memory_bus.borrow_mut().write(0xC841, 0xC8); // high byte of address
    memory_bus.borrow_mut().write(0xC842, 0x90); // low byte of address - target $C890
    
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().pc = 0xC840;
    
    let cycles = cpu.execute_instruction(false, false);
    
    let stored_value = memory_bus.borrow().read(0xC890);
    assert_eq!(stored_value, 0xAA, "A should be stored to extended address");
    assert_eq!(cpu.registers().pc, 0xC843, "PC should advance past 3-byte instruction");
    assert_eq!(cycles, 5, "STA extended should take 5 cycles"); // From CpuOpCodes.h
    
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set for 0xAA");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}

// TODO: Add test for STA indexed (0xA7) - requires implementing indexed addressing mode helpers  
// TODO: Verify cycle counts match CpuOpCodes.h exactly
// TODO: Add comprehensive edge case tests