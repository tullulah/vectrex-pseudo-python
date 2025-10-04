//! Minimal test for NOP opcode to verify basic CPU functionality

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
fn test_nop_minimal() {
    // C++ Original: NOP - does nothing but consume cycles
    let mut cpu = create_test_cpu();

    // Place NOP instruction in RAM area (0xC800+)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x12); // NOP

    // Set PC to start of instruction
    cpu.registers_mut().pc = 0xC800;
    let initial_pc = cpu.registers().pc;

    // Execute one instruction
    let cycles = cpu.execute_instruction(false, false);

    // Verify results
    assert_eq!(cpu.registers().pc, initial_pc + 1);
    assert_eq!(cycles, 2); // NOP is 2 cycles
}

