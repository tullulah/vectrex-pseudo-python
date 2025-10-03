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

#[test]
fn test_add_extended_addressing_modes() {
    // Test ADDA direct (0x9B) - already implemented, adding test for completeness
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0x7F;
    
    // Setup instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x9B); // ADDA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x50); // direct address offset
    memory_bus.borrow_mut().write(0xC850, 0x01); // data at 0xC800 + 0x50
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x80); // 0x7F + 0x01 = 0x80
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: positive + positive = negative
    assert_eq!(cpu.registers().cc.c, false);

    // Test ADDA extended (0xBB) - already implemented
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0xFF;
    
    // Setup extended addressing instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xBB); // ADDA extended opcode
    memory_bus.borrow_mut().write(0xC801, 0xC9); // address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // address low byte
    memory_bus.borrow_mut().write(0xC900, 0x01); // data at extended address
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00); // 0xFF + 0x01 = 0x00 (wrap)
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set
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
