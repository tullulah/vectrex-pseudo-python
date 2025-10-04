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
fn test_leau_indexed_basic() {
    // C++ Original: LEAU - Load Effective Address into U (indexed) - opcode 0x33
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0x4000; // Base register for indexed addressing
    cpu.registers_mut().u = 0x0000; // Clear U initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially to verify it's not affected
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x33); // LEAU indexed
    memory_bus.borrow_mut().write(0xC801, 0xE4); // ,S (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify U contains the effective address
    assert_eq!(cpu.registers().u, 0x4000);
    
    // Verify Z flag is NOT affected by LEAU - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(!cpu.registers().cc.z); // Should remain false
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

