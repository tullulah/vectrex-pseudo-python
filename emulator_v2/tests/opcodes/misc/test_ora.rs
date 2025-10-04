// Test file for A register extended addressing modes
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
    
    let mut cpu = Cpu6809::new(memory_bus);
    
    // Set DP to 0xC8 for direct page addressing 
    cpu.registers_mut().dp = 0xC8;
    
    cpu
}

#[test]
fn test_ora_extended_addressing_modes() {
    // Test ORAA direct (0x9A)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xAA;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x9A); // ORAA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x10); // direct address offset
    memory_bus.borrow_mut().write(0xC810, 0x55); // data at 0xC800 + 0x10
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0xAA | 0x55 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);

    // Test ORAA extended (0xBA)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x0F;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xBA); // ORAA extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x50); // address low byte  
    memory_bus.borrow_mut().write(0xC950, 0xF0); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x0F | 0xF0 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ORAA indexed (placeholder - implementation depends on indexed addressing)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x33;
    cpu.registers_mut().x = 0xC900;
    
    // Setup indexed addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xAA); // ORAA indexed opcode
    memory_bus.borrow_mut().write(0xC801, 0x84); // indexed postbyte (,X no offset)
    memory_bus.borrow_mut().write(0xC900, 0xCC); // data at X
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x33 | 0xCC = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
}

