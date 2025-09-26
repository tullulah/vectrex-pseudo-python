// Test suite for MUL opcode
// C++ Original: 1:1 port from Vectrexy test infrastructure

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800; // RAM area for tests

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_mul_basic() {
    let mut cpu = create_test_cpu();
    
    // Test MUL (0x3D) - cycles: 11
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x05;
    cpu.registers_mut().b = 0x07;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 11, "MUL should take 11 cycles");
    assert_eq!(cpu.registers().d(), 0x05 * 0x07, "MUL should set D to A * B");
    assert_eq!(cpu.registers().a, 0x00, "MUL result high byte should be in A");
    assert_eq!(cpu.registers().b, 0x23, "MUL result low byte should be in B (5*7=35=0x23)");
    
    // Test flags - C++ Original: MUL affects Z and C flags
    assert!(!cpu.registers().cc.z, "Zero flag should be clear for non-zero result");
    
    // C++ Original: Carry set if bit 7 of result is 1
    assert!(!cpu.registers().cc.c, "Carry flag should be clear when bit 7 of result is 0");
}

#[test]
fn test_mul_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Test MUL with zero result
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().b = 0x55;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().d(), 0x0000, "0 * anything should be 0");
    assert!(cpu.registers().cc.z, "Zero flag should be set for zero result");
    assert!(!cpu.registers().cc.c, "Carry flag should be clear for zero result");
}

#[test]
fn test_mul_maximum_values() {
    let mut cpu = create_test_cpu();
    
    // Test MUL with maximum 8-bit values
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0xFF;  // 255
    cpu.registers_mut().b = 0xFF;  // 255
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().d(), 0xFE01, "255 * 255 = 65025 = 0xFE01");
    assert!(!cpu.registers().cc.c, "Carry should be clear for 0xFE01 (bit 7 of low byte is 0)");
    assert!(!cpu.registers().cc.z, "Zero flag should be clear for maximum result");
}
