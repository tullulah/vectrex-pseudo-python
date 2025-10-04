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
fn test_eor_extended_addressing_modes() {
    // Test EORA direct (0x98)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xFF;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x98); // EORA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x40); // direct address offset
    memory_bus.borrow_mut().write(0xC840, 0xAA); // data at 0xC800 + 0x40
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x55); // 0xFF ^ 0xAA = 0x55
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);

    // Test EORA extended (0xB8)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xAA;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xB8); // EORA extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // address low byte
    memory_bus.borrow_mut().write(0xC900, 0xAA); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00); // 0xAA ^ 0xAA = 0x00
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);

    // Test EORA indexed (placeholder)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x0F;
    cpu.registers_mut().y = 0xC930;
    
    // Setup indexed addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xA8); // EORA indexed opcode
    memory_bus.borrow_mut().write(0xC801, 0xA4); // indexed postbyte (,Y no offset)
    memory_bus.borrow_mut().write(0xC930, 0xF0); // data at Y
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x0F ^ 0xF0 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
}

