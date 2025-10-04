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

