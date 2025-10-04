//! Tests for 16-bit Store operations (STX, STD, STU) with indexed addressing
//! 
//! This test module verifies the implementation of 16-bit store instructions
//! with indexed addressing modes, covering opcodes:
//! - STX: 0x9F (direct), 0xAF (indexed), 0xBF (extended)  
//! - STD: 0xDD (direct), 0xED (indexed), 0xFD (extended)
//! - STU: 0xDF (direct), 0xEF (indexed), 0xFF (extended)

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
fn test_stu_extended_0xff() {
    // Test STU (Store U register) extended addressing
    let mut cpu = create_test_cpu();
    
    // Setup U register  
    cpu.registers_mut().u = 0xDEAD;
    
    // Set up memory: STU extended
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC820, 0xFF); // STU extended
    memory_bus.borrow_mut().write(0xC821, 0xCB); // Extended address high byte (0xCB00)
    memory_bus.borrow_mut().write(0xC822, 0x00); // Extended address low byte
    
    cpu.registers_mut().pc = 0xC820;
    
    // Execute STU extended
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify U register value was stored at extended address (0xCB00)
    assert_eq!(memory_bus.borrow().read(0xCB00), 0xDE, "High byte of U should be stored");
    assert_eq!(memory_bus.borrow().read(0xCB01), 0xAD, "Low byte of U should be stored");
    
    // Verify PC advanced correctly
    assert_eq!(cpu.registers().pc, 0xC823, "PC should advance by 3 bytes for extended");
    assert_eq!(cycles, 6, "STU extended should take 6 cycles");
    
    // Verify condition codes
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
}

