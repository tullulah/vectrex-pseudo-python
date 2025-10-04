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
fn test_orb_all_addressing_modes() {
    // Test ORAB with immediate mode (0xCA)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xAA;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x55); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0xFF); // 0xAA | 0x55 = 0xFF
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.v, false); // Overflow always clear for OR

    // Test ORAB with direct mode (0xDA)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xF0;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xDA); // ORAB direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x10); // direct address offset
    memory_bus.borrow_mut().write(0xC810, 0x0F); // data at 0xC800 + 0x10
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0xFF); // 0xF0 | 0x0F = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ORAB with zero result
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0x00;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x00); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set
}

