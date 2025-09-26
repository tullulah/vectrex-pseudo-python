// C++ Original: Test suite for COM (Complement) opcodes - Using 1:1 field access and correct API
// COM opcodes: 0x43 COMA, 0x53 COMB, 0x03 COM direct, 0x63 COM indexed, 0x73 COM extended

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

#[test]
fn test_com_extended_0x73() {
    // COM extended - opcode 0x73 - Test from test_memory_operation_opcodes
    let mut cpu = create_test_cpu();
    
    // Initialize target memory and flags
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x55); // Target memory (01010101 -> 10101010)
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false;
    
    // Place COM extended instruction: 0x73 followed by address 0xC900
    memory_bus.borrow_mut().write(0xC800, 0x73); // COM extended
    memory_bus.borrow_mut().write(0xC801, 0xC9); // Address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // Address low byte
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory location is complemented: 0x55 -> 0xAA (bitwise NOT)
    assert_eq!(memory_bus.borrow().read(0xC900), 0xAA);
    
    // Verify flags - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1;
    assert!(!cpu.registers().cc.z); // Z=0 (0xAA is not zero)
    assert!(cpu.registers().cc.n);  // N=1 (0xAA bit 7 = 1, so negative)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared by COM)
    assert!(cpu.registers().cc.c);  // C=1 (always set by COM) - 1:1 Vectrexy behavior
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC803);
    
    // Verify cycle count - COM extended should be 7 cycles
    assert_eq!(cycles, 7);
}

#[test]
fn test_com_zero_result() {
    // COM with 0xFF input -> 0x00 result (zero flag test)
    let mut cpu = create_test_cpu();
    
    // Set A to 0xFF (all bits set)
    cpu.registers_mut().a = 0xFF; // 11111111 in binary
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false;
    
    // Place COMA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x43); // COMA
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify A register is complemented: 0xFF -> 0x00 (bitwise NOT)
    assert_eq!(cpu.registers().a, 0x00); // 00000000 in binary
    
    // Verify flags - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1;
    assert!(cpu.registers().cc.z);   // Z=1 (0x00 is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (0x00 bit 7 = 0, so positive)
    assert!(!cpu.registers().cc.v);  // V=0 (always cleared by COM)
    assert!(cpu.registers().cc.c);   // C=1 (always set by COM) - 1:1 Vectrexy behavior
    
    assert_eq!(cycles, 2);
}