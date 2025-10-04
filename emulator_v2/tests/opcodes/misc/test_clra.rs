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
fn test_clra_minimal() {
    // C++ Original: CLRA - Clear A register
    let mut cpu = create_test_cpu();

    // Place CLRA instruction in RAM
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4F); // CLRA

    // Set initial state
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x42; // Non-zero value

    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);

    // Verify results
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cycles, 2); // CLRA is 2 cycles
    assert!(cpu.registers().cc.z); // Zero flag should be set
    assert!(!cpu.registers().cc.n); // Negative flag should be clear
    assert!(!cpu.registers().cc.v); // Overflow flag should be clear
}

