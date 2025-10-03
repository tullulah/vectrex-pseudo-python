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

#[test]
fn test_andb_all_addressing_modes() {
    // Test ANDB with immediate mode (0xC4)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xFF;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC4); // ANDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xAA); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0xAA); // 0xFF & 0xAA = 0xAA
    assert_eq!(cpu.registers().cc.n, true); // Negative flag set
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false); // Overflow always clear for AND

    // Test ANDB with direct mode (0xD4)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xCC;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xD4); // ANDB direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x20); // direct address offset
    memory_bus.borrow_mut().write(0xC820, 0x88); // data at 0xC800 + 0x20
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x88); // 0xCC & 0x88 = 0x88
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ANDB with zero result
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xAA;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC4); // ANDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x55); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x00); // 0xAA & 0x55 = 0x00
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set
}

#[test]
fn test_eorb_all_addressing_modes() {
    // Test EORB with immediate mode (0xC8)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xFF;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC8); // EORB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xAA); // operand
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x55); // 0xFF ^ 0xAA = 0x55
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false); // Overflow always clear for EOR

    // Test EORB with direct mode (0xD8)
    let mut cpu = create_test_cpu();
    cpu.registers_mut().b = 0xAA;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xD8); // EORB direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x30); // direct address offset
    memory_bus.borrow_mut().write(0xC830, 0xAA); // data at 0xC800 + 0x30
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0x00); // 0xAA ^ 0xAA = 0x00
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true); // Zero flag set
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