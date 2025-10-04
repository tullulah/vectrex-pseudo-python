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
fn test_and_extended_addressing_modes() {
    // Test ANDA direct (0x94)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xFF;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x94); // ANDA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x20); // direct address offset
    memory_bus.borrow_mut().write(0xC820, 0xAA); // data at 0xC800 + 0x20
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xAA); // 0xFF & 0xAA = 0xAA
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);

    // Test ANDA extended (0xB4)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xCC;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xB4); // ANDA extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // address low byte
    memory_bus.borrow_mut().write(0xC900, 0x88); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x88); // 0xCC & 0x88 = 0x88  
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ANDA with zero result
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xAA;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x94); // ANDA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x30); // direct address offset
    memory_bus.borrow_mut().write(0xC830, 0x55); // data at 0xC800 + 0x30
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00); // 0xAA & 0x55 = 0x00
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
}

