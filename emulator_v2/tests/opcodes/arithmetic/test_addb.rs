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
fn test_addb_all_addressing_modes() {
    // Test ADDB with immediate mode (0xCB)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0x7F;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCB); // ADDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x80); // 0x7F + 0x01 = 0x80
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set (0x80 has bit 7 set)
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: positive + positive = negative
    assert_eq!(cpu.registers().cc.c, false);

    // Test ADDB with carry generation
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xFF;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCB); // ADDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x00); // 0xFF + 0x01 = 0x00 (wrap)
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set
}

