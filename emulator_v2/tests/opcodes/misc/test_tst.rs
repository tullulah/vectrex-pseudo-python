// C++ Original: Test suite for basic 6809 opcodes - Using 1:1 field access and correct API

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
fn test_tst_zero_and_negative() {
    // TSTA with zero value - Test 1:1 Vectrexy behavior
    let mut cpu = create_test_cpu();
    
    // Test zero case
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false; // Will verify TST doesn't change this
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4D); // TSTA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0;
    assert!(cpu.registers().cc.z);   // Z=1 (value is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (zero is not negative)
    assert!(!cpu.registers().cc.v);  // V=0 (always cleared)
    assert!(!cpu.registers().cc.c);  // C unchanged - 1:1 Vectrexy behavior
    
    // Test negative case
    cpu.registers_mut().a = 0x80;    // Negative value
    cpu.registers_mut().cc.c = true; // Different carry to verify preservation
    cpu.registers_mut().pc = 0xC800; // Reset PC
    
    let cycles2 = cpu.execute_instruction(false, false);
    
    assert!(!cpu.registers().cc.z);  // Z=0 (0x80 is not zero)
    assert!(cpu.registers().cc.n);   // N=1 (0x80 is negative)
    assert!(!cpu.registers().cc.v);  // V=0 (always cleared)
    assert!(cpu.registers().cc.c);   // C unchanged - 1:1 Vectrexy behavior
    
    assert_eq!(cycles, 2);
    assert_eq!(cycles2, 2);
}

