// C++ Original: Test suite for NEG (Negate) opcodes - Using 1:1 field access and correct API
// NEG opcodes: 0x40 NEGA, 0x50 NEGB, 0x00 NEG direct, 0x60 NEG indexed, 0x70 NEG extended

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

#[test]
fn test_neg_direct_0x00() {
    // NEG direct - opcode 0x00 - Test from test_memory_operation_opcodes
    let mut cpu = create_test_cpu();
    
    // Initialize target memory and flags
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0x80, 0x01); // Target memory (will become 0xFF)
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false;
    
    // Place NEG direct instruction: 0x00 followed by direct page address 0x80
    memory_bus.borrow_mut().write(0xC800, 0x00); // NEG direct
    memory_bus.borrow_mut().write(0xC801, 0x80); // Direct page address
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory location is negated: 0x01 -> 0xFF (two's complement)
    assert_eq!(memory_bus.borrow().read(0x80), 0xFF);
    
    // Verify flags - C++ Original: value = SubtractImpl(0, value, 0, CC); (which updates all flags)
    assert!(!cpu.registers().cc.z); // Z=0 (0xFF is not zero)
    assert!(cpu.registers().cc.n);  // N=1 (0xFF is negative)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow for 0x01 negation)
    assert!(cpu.registers().cc.c);  // C=1 (set by SubtractImpl for non-zero result)
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC802);
    
    // Verify cycle count - NEG direct should be 6 cycles
    assert_eq!(cycles, 6);
}

#[test]
fn test_neg_zero_special_case() {
    // NEG with zero value - special case where 0x00 -> 0x00
    let mut cpu = create_test_cpu();
    
    // Set A to zero
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = true;
    
    // Place NEGA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x40); // NEGA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register remains zero: 0x00 -> 0x00 (negation of zero is zero)
    assert_eq!(cpu.registers().a, 0x00);
    
    // Verify flags - C++ Original: value = SubtractImpl(0, value, 0, CC); (special case for zero)
    assert!(cpu.registers().cc.z);   // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (0x00 is not negative)
    assert!(!cpu.registers().cc.v);  // V=0 (no overflow)
    assert!(!cpu.registers().cc.c);  // C=0 (no borrow for zero) - 1:1 Vectrexy behavior
    
    assert_eq!(cycles, 2);
}