// C++ Original: Test suite for DEC (Decrement) opcodes - Using 1:1 field access and correct API
// DEC opcodes: 0x4A DECA, 0x5A DECB, 0x0A DEC direct, 0x6A DEC indexed, 0x7A DEC extended

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

#[test]
fn test_dec_direct_0x0a() {
    // DEC direct - opcode 0x0A - Test from test_memory_operation_opcodes
    let mut cpu = create_test_cpu();
    
    // Initialize target memory and flags
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0x80, 0x01); // Target memory (will become 0x00)
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false; // Should be preserved
    
    // Place DEC direct instruction: 0x0A followed by direct page address 0x80
    memory_bus.borrow_mut().write(0xC800, 0x0A); // DEC direct
    memory_bus.borrow_mut().write(0xC801, 0x80); // Direct page address
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory location is decremented: 0x01 -> 0x00
    assert_eq!(memory_bus.borrow().read(0x80), 0x00);
    
    // Verify flags - C++ Original: CC.Overflow = origValue == 0b1000'0000; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value);
    assert!(cpu.registers().cc.z);   // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (0x00 is not negative)
    assert!(!cpu.registers().cc.v);  // V=0 (no overflow: 0x01 != 0x80)
    assert!(!cpu.registers().cc.c);  // C preserved (DEC doesn't modify Carry) - 1:1 Vectrexy
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC802);
    
    // Verify cycle count - DEC direct should be 6 cycles
    assert_eq!(cycles, 6);
}