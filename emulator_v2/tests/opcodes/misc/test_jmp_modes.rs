// Test for JMP opcode (indexed and extended modes)
use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_emulator() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Conectar RAM para tests
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    cpu
}

#[test]
fn test_jmp_indexed_0x6e() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup: X register pointing to target address
    cpu.registers_mut().x = 0xC900;
    
    // Write JMP indexed (0x6E) with postbyte 0x84 (,X no offset)
    memory_bus.borrow_mut().write(RAM_START, 0x6E);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x84); // Indexed postbyte: ,X
    
    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;
    
    // Execute JMP
    cpu.execute_instruction(false, false);
    
    // Verify PC jumped to X register value
    assert_eq!(cpu.registers().pc, 0xC900, "PC should jump to address in X");
}

#[test]
fn test_jmp_extended_0x7e() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Write JMP extended (0x7E) with target address 0xD000
    memory_bus.borrow_mut().write(RAM_START, 0x7E);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xD0); // High byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // Low byte
    
    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;
    
    // Execute JMP
    cpu.execute_instruction(false, false);
    
    // Verify PC jumped to target address
    assert_eq!(cpu.registers().pc, 0xD000, "PC should jump to 0xD000");
}

#[test]
fn test_jmp_direct_0x0e() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup DP register to 0xC8
    cpu.registers_mut().dp = 0xC8;
    
    // Write JMP direct (0x0E) with offset 0x50 -> EA = 0xC850
    memory_bus.borrow_mut().write(RAM_START, 0x0E);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x50); // Direct page offset
    
    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;
    
    // Execute JMP
    cpu.execute_instruction(false, false);
    
    // Verify PC jumped to DP:offset
    assert_eq!(cpu.registers().pc, 0xC850, "PC should jump to DP:0x50 = 0xC850");
}
