// C++ Original: Test suite for SWI (Software Interrupt) opcode 0x3F - Using 1:1 field access and correct API
// SWI implementation from Vectrexy Cpu.cpp interrupt handling

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use crate::interrupt_vector_rom::InterruptVectorRom;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // C++ Original: Map interrupt vectors (0xFFF0-0xFFFF) for SWI/CWAI tests
    let vector_rom = Rc::new(RefCell::new(InterruptVectorRom::new()));
    memory_bus.borrow_mut().connect_device(vector_rom, (0xFFF0, 0xFFFF), vectrex_emulator_v2::core::memory_bus::EnableSync::False);
    
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
    // Stack grows downward, Vectrexy push order: PC, U, Y, X, DP, B, A, CC
    // CC is pushed LAST and is at the current stack pointer (lowest address)
    
    // Verify stack contents in correct positions according to Vectrexy order
    // PC was 0xC801 after reading opcode (incremented from 0xC800)
    assert_eq!(memory_bus.borrow().read(old_s - 1), 0x01, "PC low byte should be at old_s-1"); // PC low = 0x01
    assert_eq!(memory_bus.borrow().read(old_s - 2), 0xC8, "PC high byte should be at old_s-2"); // PC high = 0xC8
    assert_eq!(memory_bus.borrow().read(old_s - 3), 0x88, "U low byte should be at old_s-3"); // U low = 0x88
    assert_eq!(memory_bus.borrow().read(old_s - 4), 0x77, "U high byte should be at old_s-4"); // U high = 0x77
    assert_eq!(memory_bus.borrow().read(old_s - 5), 0x66, "Y low byte should be at old_s-5"); // Y low = 0x66
    assert_eq!(memory_bus.borrow().read(old_s - 6), 0x55, "Y high byte should be at old_s-6"); // Y high = 0x55
    assert_eq!(memory_bus.borrow().read(old_s - 7), 0x44, "X low byte should be at old_s-7"); // X low = 0x44
    assert_eq!(memory_bus.borrow().read(old_s - 8), 0x33, "X high byte should be at old_s-8"); // X high = 0x33
    assert_eq!(memory_bus.borrow().read(old_s - 9), 0x99, "DP should be at old_s-9"); // DP = 0x99
    assert_eq!(memory_bus.borrow().read(old_s - 10), 0x22, "B should be at old_s-10"); // B = 0x22
    assert_eq!(memory_bus.borrow().read(old_s - 11), 0x11, "A should be at old_s-11"); // A = 0x11
    assert_eq!(memory_bus.borrow().read(old_s - 12), 0xAA, "CC should be at old_s-12 (pushed last)"); // CC = 0xAA with E=1
    
    // Stack pointer should have decremented by 12 bytes for entire register set
    assert_eq!(cpu.registers().s, old_s - 12, "Stack pointer should have decremented by 12 bytes");
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