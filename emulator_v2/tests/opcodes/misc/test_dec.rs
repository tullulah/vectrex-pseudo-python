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
fn test_dec_overflow() {
    // DECA with overflow (0x80 -> 0x7F) - Test 1:1 Vectrexy behavior
    let mut cpu = create_test_cpu();
    
    // Set A to 0x80 (-128 in signed, will overflow to +127)
    cpu.registers_mut().a = 0x80;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = false;
    cpu.registers_mut().cc.c = true; // Set to verify DEC doesn't modify Carry
    
    // Place DECA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4A); // DECA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Result: 0x80 - 1 = 0x7F (overflow from negative to positive)
    assert_eq!(cpu.registers().a, 0x7F);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b1000'0000; (0x80 case)
    assert!(!cpu.registers().cc.z); // Z=0 (0x7F is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (0x7F is positive)
    assert!(cpu.registers().cc.v);  // V=1 (overflow: 0x80 is exactly the overflow condition)
    assert!(cpu.registers().cc.c);  // C unchanged (DEC doesn't modify Carry) - 1:1 Vectrexy
    
    assert_eq!(cycles, 2);
}

