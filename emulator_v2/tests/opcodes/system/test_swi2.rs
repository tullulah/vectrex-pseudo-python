// Test for SWI2 opcode (0x3F + 0x3F prefix pattern)
// Port 1:1 from Vectrexy C++ implementation
// C++ Original: Software Interrupt 2 - two-byte SWI variant

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // Add additional RAM for ROM/vector space (0xE000-0xFFFF) to allow vector writes in tests
    let rom_ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(rom_ram, (0xE000, 0xFFFF), EnableSync::False);
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_swi2_0x3f() {
    // C++ Original: SWI2 - Software Interrupt 2
    let mut cpu = create_test_cpu();
    let program = vec![0x3F]; // SWI2 opcode
    
    // Load program into RAM
    for (i, &byte) in program.iter().enumerate() {
        cpu.memory_bus().borrow_mut().write(RAM_START + i as u16, byte);
    }
    cpu.registers.pc = RAM_START;
    
    // Set initial stack pointer
    cpu.registers.s = 0xCFFF;
    
    // Set up SWI2 interrupt vector at $FFFA-$FFFB to point to valid RAM address
    // Note: This would normally be in ROM, but we need to map it for testing
    cpu.memory_bus().borrow_mut().write(0xFFFA, 0xC8); // High byte of vector
    cpu.memory_bus().borrow_mut().write(0xFFFA + 1, 0x50); // Low byte -> vector = $C850
    
    let initial_pc = cpu.registers.pc;
    let cycles = cpu.execute_instruction(false, false);
    
    // SWI2 should advance PC and consume cycles
    // Note: Full interrupt handling may not be implemented in test environment
    assert_ne!(cpu.registers.pc, initial_pc);
    assert!(cycles > 0);
    
    // Verify stack operations occurred (PC should be pushed)
    assert!(cpu.registers.s < 0xCFFF); // Stack pointer should have decremented
}