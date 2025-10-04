// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

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
fn test_orab_immediate_basic() {
    // Test ORAB #$AA - OR B with immediate value
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0x55; // Initial B = 01010101
    
    // Setup: ORAB #$AA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xAA); // OR mask 10101010
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0x55 | 0xAA = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

