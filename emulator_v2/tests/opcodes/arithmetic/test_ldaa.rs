// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

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
fn test_ldaa_immediate_negative() {
    // Test LDAA #$80 - Load A with negative value (sets N flag)
    // Note: This test verifies that existing LD opcodes work correctly
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x86); // LDAA immediate
    memory_bus.borrow_mut().write(0xC801, 0x80); // Negative value (bit 7 set)
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_ldaa_immediate_zero() {
    // Test LDAA #$00 - Load A with zero (sets Z flag)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x86); // LDAA immediate
    memory_bus.borrow_mut().write(0xC801, 0x00); // Zero value
    cpu.registers_mut().pc = 0xC800;
    
    cpu.registers_mut().a = 0xFF; // Set A to non-zero initially
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (value = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

