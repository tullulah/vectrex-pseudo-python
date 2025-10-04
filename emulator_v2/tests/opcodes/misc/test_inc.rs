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
fn test_inc_overflow() {
    // INCA with overflow (0x7F -> 0x80)
    let mut cpu = create_test_cpu();
    
    // Set A to 0x7F (127 in signed)
    cpu.registers_mut().a = 0x7F;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = false;
    
    // Place INCA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4C); // INCA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Result: 0x7F + 1 = 0x80 (128 in unsigned, -128 in signed)
    assert_eq!(cpu.registers().a, 0x80);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b0111'1111; (0x7F case)
    assert!(!cpu.registers().cc.z); // Z=0 (result 0x80 is not zero)
    assert!(cpu.registers().cc.n);  // N=1 (0x80 is negative in signed arithmetic)
    assert!(cpu.registers().cc.v);  // V=1 (overflow: 0x7F is exactly the overflow condition)
    // CRITICAL 1:1 Vectrexy: INC does NOT modify Carry flag (should preserve initial false)
    
    assert_eq!(cycles, 2);
}

