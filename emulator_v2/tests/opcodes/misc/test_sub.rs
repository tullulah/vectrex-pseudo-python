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
fn test_sub_extended_addressing_modes() {
    // Test SUBA direct (0x90) - already implemented, adding test for completeness
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x80;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x90); // SUBA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x60); // direct address offset
    memory_bus.borrow_mut().write(0xC860, 0x01); // data at 0xC800 + 0x60
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x7F); // 0x80 - 0x01 = 0x7F
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: negative - positive = positive
    assert_eq!(cpu.registers().cc.c, false);

    // Test SUBA extended (0xB0) - already implemented
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x00;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xB0); // SUBA extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // address low byte
    memory_bus.borrow_mut().write(0xC900, 0x01); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x00 - 0x01 = 0xFF (borrow)
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set (borrow occurred)
}

