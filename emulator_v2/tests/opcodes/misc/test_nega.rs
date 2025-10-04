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
fn test_neg_a_basic() {
    // NEGA - Negate A register (inherent) - opcode 0x40
    let mut cpu = create_test_cpu();
    
    // Set A to a positive test value
    cpu.registers_mut().a = 0x42; // 66 in decimal
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = true;
    
    // Place NEGA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x40); // NEGA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is negated: 0x42 -> 0xBE (two's complement)
    assert_eq!(cpu.registers().a, 0xBE); // -66 in two's complement
    
    // Verify flags - C++ Original: value = SubtractImpl(0, value, 0, CC); (which updates all flags)
    assert!(!cpu.registers().cc.z); // Z=0 (0xBE is not zero)
    assert!(cpu.registers().cc.n);  // N=1 (0xBE is negative)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow for 0x42 negation)
    assert!(cpu.registers().cc.c);  // C=1 (set by SubtractImpl for non-zero result)
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - NEGA should be 2 cycles
    assert_eq!(cycles, 2);
}

