// Tests para opcodes ADD y SUB - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x80, 0x8B, 0xC0, 0xCB

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
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_add_sub_comprehensive_flags() {
    // Test various flag combinations with ADD/SUB
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test 1: ADD negative result
    cpu.registers_mut().a = 0x70; // 112
    memory_bus.borrow_mut().write(0xC800, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x20); // Add 32 = 144 = 0x90 (negative)
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x90, "A should be 0x90");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    
    // Test 2: SUB overflow: negative - positive = positive (should set V)
    cpu.registers_mut().b = 0x80; // -128 in signed
    memory_bus.borrow_mut().write(0xC810, 0xC0); // SUBB immediate
    memory_bus.borrow_mut().write(0xC811, 0x01); // Subtract 1
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x7F, "B should be 0x7F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set (overflow)");
}

