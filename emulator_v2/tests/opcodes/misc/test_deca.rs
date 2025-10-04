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
fn test_dec_a_basic() {
    // DECA - Decrement A register (inherent) - opcode 0x4A
    let mut cpu = create_test_cpu();
    
    // Set A to a test value
    cpu.registers_mut().a = 0x43;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    
    // Place DECA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4A); // DECA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is decremented
    assert_eq!(cpu.registers().a, 0x42);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b1000'0000; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value);
    assert!(!cpu.registers().cc.z); // Z=0 (result is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (result is positive)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow: 0x43 != 0x80)
    // CRITICAL 1:1 Vectrexy: DEC does NOT modify Carry flag (should preserve initial true)
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - DECA should be 2 cycles
    assert_eq!(cycles, 2);
}

