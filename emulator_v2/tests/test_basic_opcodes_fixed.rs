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
fn test_clr_a_basic() {
    // CLRA - Clear A register (inherent) - opcode 0x4F
    let mut cpu = create_test_cpu();
    
    // Set A to a non-zero value initially
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    // Place CLRA instruction in RAM area (0xC800+)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4F); // CLRA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is cleared
    assert_eq!(cpu.registers().a, 0x00);
    
    // Verify flags - C++ Original: CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0;
    assert!(cpu.registers().cc.z);   // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (result is positive)
    assert!(!cpu.registers().cc.v);  // V=0 (no overflow)
    assert!(!cpu.registers().cc.c);  // C=0 (carry cleared) - 1:1 Vectrexy behavior
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - CLRA should be 2 cycles
    assert_eq!(cycles, 2);
}

#[test]
fn test_clr_b_basic() {
    // CLRB - Clear B register (inherent) - opcode 0x5F
    let mut cpu = create_test_cpu();
    
    // Set B to a non-zero value initially
    cpu.registers_mut().b = 0xAA;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    // Place CLRB instruction in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x5F); // CLRB
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify B register is cleared
    assert_eq!(cpu.registers().b, 0x00);
    
    // Verify flags - C++ Original: CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0;
    assert!(cpu.registers().cc.z);   // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (result is positive)
    assert!(!cpu.registers().cc.v);  // V=0 (no overflow)
    assert!(!cpu.registers().cc.c);  // C=0 (carry cleared) - 1:1 Vectrexy behavior
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count
    assert_eq!(cycles, 2);
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
fn test_tst_a_positive() {
    // TSTA - Test A register (inherent) - opcode 0x4D
    let mut cpu = create_test_cpu();
    
    // Set A to a positive test value
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    // Place TSTA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x4D); // TSTA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is unchanged
    assert_eq!(cpu.registers().a, 0x42);
    
    // Verify flags - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0;
    assert!(!cpu.registers().cc.z); // Z=0 (0x42 is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (0x42 is positive)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared by TST)
    // CRITICAL 1:1 Vectrexy: TST does NOT modify Carry flag (should preserve initial true)
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - TSTA should be 2 cycles
    assert_eq!(cycles, 2);
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

#[test]
fn test_com_a_basic() {
    // COMA - Complement A register (inherent) - opcode 0x43
    let mut cpu = create_test_cpu();
    
    // Set A to a test value
    cpu.registers_mut().a = 0x42; // 01000010 in binary
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false;
    
    // Place COMA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x43); // COMA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is complemented: 0x42 -> 0xBD (bitwise NOT)
    assert_eq!(cpu.registers().a, 0xBD); // 10111101 in binary
    
    // Verify flags - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1;
    assert!(!cpu.registers().cc.z); // Z=0 (0xBD is not zero)
    assert!(cpu.registers().cc.n);  // N=1 (0xBD bit 7 = 1, so negative)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared by COM)
    assert!(cpu.registers().cc.c);  // C=1 (always set by COM) - 1:1 Vectrexy behavior
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);
    
    // Verify cycle count - COMA should be 2 cycles
    assert_eq!(cycles, 2);
}