// Test suite for JMP opcodes
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
fn test_jmp_basic_functionality() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Direct (0x0E) - cycles: 3
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().dp = 0xC8; // Direct page for 0xC8xx
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x0E); // JMP direct
    memory_bus.borrow_mut().write(RAM_START + 1, 0x50); // Direct address 0x50 -> 0xC850
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "JMP direct should take 3 cycles");
    assert_eq!(cpu.registers().pc, 0xC850, "JMP should set PC to effective address");
}

#[test]
fn test_jmp_indexed() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Indexed (0x6E) - cycles: variable based on addressing mode
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().x = 0xC900; // Base address
    cpu.registers_mut().a = 0x0A;   // Offset value for A register
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x6E); // JMP indexed
    memory_bus.borrow_mut().write(RAM_START + 1, 0x86); // X + A offset (valid indexed mode)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 4, "JMP indexed A,X should take 4 cycles (3 base + 1 for A offset)");
    assert_eq!(cpu.registers().pc, 0xC90A, "JMP indexed should jump to X + A");
}

#[test]
fn test_jmp_extended() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Extended (0x7E) - cycles: 4
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x7E); // JMP extended
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9); // High byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50); // Low byte
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 4, "JMP extended should take 4 cycles");
    assert_eq!(cpu.registers().pc, 0xC950, "JMP extended should jump to full 16-bit address");
}
