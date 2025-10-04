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
fn test_leas_indexed_basic() {
    // C++ Original: LEAS - Load Effective Address into S (indexed) - opcode 0x32
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x3000; // Base register for indexed addressing
    cpu.registers_mut().s = 0x0000; // Clear S initially
    cpu.registers_mut().cc.z = true; // Set Z flag initially to verify it's not affected
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x32); // LEAS indexed
    memory_bus.borrow_mut().write(0xC801, 0xC4); // ,U (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify S contains the effective address
    assert_eq!(cpu.registers().s, 0x3000);
    
    // Verify Z flag is NOT affected by LEAS - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(cpu.registers().cc.z); // Should remain true
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

