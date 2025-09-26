// C++ Original: Test suite for SWI3 (Software Interrupt 3) opcode 0x113F - Using 1:1 field access and correct API
// SWI3 implementation from Vectrexy Cpu.cpp Page 2 opcodes

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
fn test_swi3_basic() {
    let mut cpu = create_test_cpu();
    
    // Test SWI3 (0x113F) - Software Interrupt 3 - cycles: 20
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x11); // Page 2 prefix
    memory_bus.borrow_mut().write(0xC801, 0x3F); // SWI3
    
    let old_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "SWI3 should take 20 cycles");
    assert!(cpu.registers().cc.e, "SWI3 should set entire flag");
    
    assert_eq!(cpu.registers().s, old_s - 12, "SWI3 should push entire register set");
}

#[test]
fn test_swi3_interrupt_flags() {
    let mut cpu = create_test_cpu();
    
    // Clear interrupt masks initially
    cpu.registers_mut().cc.i = false;
    cpu.registers_mut().cc.f = false;
    cpu.registers_mut().cc.e = false;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x11); // Page 2 prefix
    memory_bus.borrow_mut().write(0xC801, 0x3F); // SWI3
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: SWI3 does NOT set I and F flags (unlike SWI and SWI2)
    // It only sets the E flag to indicate entire register set is saved
    assert!(!cpu.registers().cc.i, "SWI3 should NOT set I flag");
    assert!(!cpu.registers().cc.f, "SWI3 should NOT set F flag");
    assert!(cpu.registers().cc.e, "SWI3 should set E flag");
}

#[test]
fn test_swi3_stack_contents() {
    let mut cpu = create_test_cpu();
    
    // Setup specific register values to verify stack saving
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x33;
    cpu.registers_mut().b = 0x44;
    cpu.registers_mut().x = 0x5566;
    cpu.registers_mut().y = 0x7788;
    cpu.registers_mut().u = 0x99AA;
    cpu.registers_mut().s = 0xCFF0;
    cpu.registers_mut().dp = 0xBB;
    cpu.registers_mut().cc.from_u8(0xCC);
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x11); // Page 2 prefix
    memory_bus.borrow_mut().write(0xC801, 0x3F); // SWI3
    
    let old_s = cpu.registers().s;
    let _cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: SWI3 pushes entire register set in same order as SWI: CC, A, B, DP, X, Y, U, PC
    let mut stack_addr = old_s - 1;
    
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xCC, "CC should be pushed first"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x33, "A should be pushed second"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x44, "B should be pushed third"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xBB, "DP should be pushed fourth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x55, "X high should be pushed fifth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x66, "X low should be pushed sixth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x77, "Y high should be pushed seventh"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x88, "Y low should be pushed eighth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x99, "U high should be pushed ninth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xAA, "U low should be pushed tenth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xC8, "PC high should be pushed eleventh"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x02, "PC low should be pushed last (PC+2 due to 2-byte instruction)");
}