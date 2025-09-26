// Test for indexed addressing with D register (0x0B)
// Port 1:1 from Vectrexy C++ implementation
// C++ Original: Indexed addressing mode using D register as offset

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_indexed_d_register_0x0b() {
    // C++ Original: Indexed addressing using D register as offset
    // Format: [,D] where D = A:B concatenated
    // NOTE: Postbyte must be 0x8B (bit 7=1) for D register mode, not 0x0B
    let mut cpu = create_test_cpu();
    let program = vec![
        0xA6, 0x8B  // LDA [,D] - Load A using D register as index offset
    ];
    
    // Load program into RAM
    for (i, &byte) in program.iter().enumerate() {
        cpu.memory_bus().borrow_mut().write(RAM_START + i as u16, byte);
    }
    cpu.registers.pc = RAM_START;
    
    // Set up indexed addressing: EA = X + D
    // Postbyte 0x8B: bit 7=1 (extended mode), bits 6-5=00 (X register), bits 3-0=0x0B (D offset)
    cpu.registers.x = 0xC800;  // Base register X points to RAM start
    cpu.registers.a = 0x00;    // High byte of D 
    cpu.registers.b = 0x50;    // Low byte of D -> D = 0x0050 as offset
    
    // Put test value at target location X + D = 0xC800 + 0x50 = 0xC850
    cpu.memory_bus().borrow_mut().write(0xC850, 0x42);
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify indexed addressing worked correctly
    assert_eq!(cpu.registers.a, 0x42);  // Should load value from EA = X + D
    assert_eq!(cpu.registers.pc, RAM_START + 2);
    assert!(cycles > 0);
}