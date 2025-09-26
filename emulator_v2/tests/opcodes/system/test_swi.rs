// C++ Original: Test suite for SWI (Software Interrupt) opcode 0x3F - Using 1:1 field access and correct API
// SWI implementation from Vectrexy Cpu.cpp interrupt handling

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
fn test_swi_basic_interrupt() {
    let mut cpu = create_test_cpu();
    
    // Test SWI (0x3F) - Software Interrupt - cycles: 19
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = 0x9ABC;
    cpu.registers_mut().u = 0xDEF0;
    cpu.registers_mut().s = 0xCFFF;  // Stack starts at top of RAM
    cpu.registers_mut().dp = 0xAA;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x3F); // SWI
    
    let old_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 19, "SWI should take 19 cycles");
    
    // C++ Original: SWI sets I and F flags, and E flag (entire flag)
    assert!(cpu.registers().cc.i, "Interrupt mask should be set after SWI");
    assert!(cpu.registers().cc.f, "Fast interrupt mask should be set after SWI");
    
    // C++ Original: SWI sets entire flag (pushes all registers)
    assert!(cpu.registers().cc.e, "Entire flag should be set after SWI");
    
    // Stack should be decremented by 12 bytes (PC, U, Y, X, DP, B, A, CC)
    assert_eq!(cpu.registers().s, old_s - 12, "Stack pointer should be decremented by 12 bytes");
}

#[test]
fn test_swi_stack_contents() {
    let mut cpu = create_test_cpu();
    
    // Setup specific register values to verify stack saving
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().b = 0x22;
    cpu.registers_mut().x = 0x3344;
    cpu.registers_mut().y = 0x5566;
    cpu.registers_mut().u = 0x7788;
    cpu.registers_mut().s = 0xCFF0;
    cpu.registers_mut().dp = 0x99;
    cpu.registers_mut().cc.from_u8(0xAA);
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x3F); // SWI
    
    let old_s = cpu.registers().s;
    let _cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: SWI pushes registers in specific order: CC, A, B, DP, X, Y, U, PC
    // Stack grows downward, so first pushed = highest address
    let mut stack_addr = old_s - 1;
    
    // Verify stack contents (pushed in reverse order)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xAA, "CC should be pushed first"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x11, "A should be pushed second"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x22, "B should be pushed third"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x99, "DP should be pushed fourth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x33, "X high should be pushed fifth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x44, "X low should be pushed sixth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x55, "Y high should be pushed seventh"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x66, "Y low should be pushed eighth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x77, "U high should be pushed ninth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x88, "U low should be pushed tenth"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xC8, "PC high should be pushed eleventh"); stack_addr -= 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x01, "PC low should be pushed last (PC+1 due to increment)");
}

#[test] 
fn test_swi_interrupt_mask_behavior() {
    let mut cpu = create_test_cpu();
    
    // Clear interrupt masks initially
    cpu.registers_mut().cc.i = false;
    cpu.registers_mut().cc.f = false;
    cpu.registers_mut().cc.e = false;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x3F); // SWI
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: SWI sets I, F, and E flags
    assert!(cpu.registers().cc.i, "I flag should be set (mask IRQ)");
    assert!(cpu.registers().cc.f, "F flag should be set (mask FIRQ)");
    assert!(cpu.registers().cc.e, "E flag should be set (entire register set saved)");
}