// C++ Original: Test suite for CLR (Clear) opcodes - Using 1:1 field access and correct API
// CLR opcodes: 0x4F CLRA, 0x5F CLRB, 0x0F CLR direct, 0x6F CLR indexed, 0x7F CLR extended

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
fn test_clr_extended_0x7f() {
    // CLR extended - opcode 0x7F - Test from test_memory_operation_opcodes
    let mut cpu = create_test_cpu();
    
    // Initialize test values
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = true;
    
    // Set target memory location to non-zero
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0xFF); // Target memory
    
    // Place CLR extended instruction: 0x7F followed by address 0xC900
    memory_bus.borrow_mut().write(0xC800, 0x7F); // CLR extended
    memory_bus.borrow_mut().write(0xC801, 0xC9); // Address high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // Address low byte
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify memory location is cleared
    assert_eq!(memory_bus.borrow().read(0xC900), 0x00);
    
    // Verify flags - C++ Original: CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0;
    assert!(cpu.registers().cc.z);   // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n);  // N=0 (result is positive)
    assert!(!cpu.registers().cc.v);  // V=0 (no overflow)
    assert!(!cpu.registers().cc.c);  // C=0 (carry cleared) - 1:1 Vectrexy behavior
    
    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC803);
    
    // Verify cycle count - CLR extended should be 7 cycles
    assert_eq!(cycles, 7);
}