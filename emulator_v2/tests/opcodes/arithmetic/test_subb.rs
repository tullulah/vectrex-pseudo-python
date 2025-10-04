// Test file for B register arithmetic and logical operations
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
fn test_subb_all_addressing_modes() {
    // Test SUBB with immediate mode (0xC0)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0x80;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC0); // SUBB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x7F); // 0x80 - 0x01 = 0x7F
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: negative - positive = positive
    assert_eq!(cpu.registers().cc.c, false);

    // Test SUBB with borrow (negative result wrapping)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0x00;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC0); // SUBB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0xFF); // 0x00 - 0x01 = 0xFF (borrow)
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set (borrow occurred)
}

