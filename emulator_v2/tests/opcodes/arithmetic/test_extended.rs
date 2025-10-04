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
fn test_extended_addressing_modes() {
    // Test ORAB extended (0xFA)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0x0F;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xFA); // ORAB extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // address low byte  
    memory_bus.borrow_mut().write(0xC900, 0xF0); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0xFF); // 0x0F | 0xF0 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ANDB extended (0xF4)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xFF;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xF4); // ANDB extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte  
    memory_bus.borrow_mut().write(0xC802, 0x10); // address low byte
    memory_bus.borrow_mut().write(0xC910, 0x0F); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x0F); // 0xFF & 0x0F = 0x0F
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
}

