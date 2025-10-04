// C++ Original: Test suite for LEA opcodes (Load Effective Address) - 1:1 Vectrexy port

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
fn test_leay_indexed_basic() {
    // C++ Original: LEAY - Load Effective Address into Y (indexed) - opcode 0x31
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x2000; // Base register for indexed addressing
    cpu.registers_mut().y = 0x0000; // Clear Y initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x31); // LEAY indexed
    memory_bus.borrow_mut().write(0xC801, 0x84); // ,X (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify Y contains the effective address
    assert_eq!(cpu.registers().y, 0x2000);
    
    // Verify Z flag is cleared (Y is non-zero) - C++ Original: Z flag affected by LEAX/LEAY
    assert!(!cpu.registers().cc.z);
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

