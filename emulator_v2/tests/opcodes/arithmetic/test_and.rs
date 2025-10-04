// Tests para opcodes AND y EOR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x84, 0xC4, 0x88, 0xC8

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
fn test_and_eor_flag_comprehensive() {
    // Test flag behavior with various AND/EOR combinations
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test 1: AND negative result
    cpu.registers_mut().a = 0xFF;
    memory_bus.borrow_mut().write(0xC800, 0x84); // ANDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x80); // AND with 0x80
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    
    // Test 2: EOR zero result
    cpu.registers_mut().b = 0x33;
    memory_bus.borrow_mut().write(0xC810, 0xC8); // EORB immediate
    memory_bus.borrow_mut().write(0xC811, 0x33); // EOR with same value
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00, "B should be 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

