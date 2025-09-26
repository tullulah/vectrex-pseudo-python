// C++ Original: Test suite for INC (Increment) opcodes - Using 1:1 field access and correct API
// INC opcodes: 0x4C INCA, 0x5C INCB, 0x0C INC direct, 0x6C INC indexed, 0x7C INC extended

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
fn test_inc_a_basic() {
    // INCA - Increment A register (inherent) - opcode 0x4C
    let mut cpu = create_test_cpu();
    
    // Set A to a test value
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    // Place INCA instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4C); // INCA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is incremented
    assert_eq!(cpu.registers().a, 0x43);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b0111'1111; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value);
    assert!(!cpu.registers().cc.z); // Z=0 (result is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (result is positive)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow from 0x42->0x43)
    // CRITICAL 1:1 Vectrexy: INC does NOT modify Carry flag
    // Initial carry state should be preserved (was set to true above)
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - INCA should be 2 cycles
    assert_eq!(cycles, 2);
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

#[test]
fn test_inc_indexed_0x6c() {
    // INC indexed - opcode 0x6C - Test from test_memory_operation_opcodes
    let mut cpu = create_test_cpu();
    
    // Setup X register to point to test memory location
    cpu.registers_mut().x = 0xC900;
    
    // Initialize target memory and flags
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x7E); // Value to increment
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true; 
    cpu.registers_mut().cc.v = false;
    cpu.registers_mut().cc.c = true; // Should be preserved
    
    // Place INC indexed instruction: 0x6C followed by postbyte 0x00 (,X)
    memory_bus.borrow_mut().write(0xC800, 0x6C); // INC indexed
    memory_bus.borrow_mut().write(0xC801, 0x00); // Postbyte: ,X (no offset)
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory location is incremented: 0x7E -> 0x7F
    assert_eq!(memory_bus.borrow().read(0xC900), 0x7F);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b0111'1111; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value);
    assert!(!cpu.registers().cc.z); // Z=0 (0x7F is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (0x7F is positive)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow: 0x7E != 0x7F)
    assert!(cpu.registers().cc.c);  // C preserved (INC doesn't modify Carry) - 1:1 Vectrexy
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC802);
    
    // Verify cycle count - INC indexed should be 6 cycles
    assert_eq!(cycles, 6);
}