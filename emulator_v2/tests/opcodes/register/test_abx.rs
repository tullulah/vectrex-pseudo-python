// Test for ABX (0x3A) - Add B to X (unsigned, no flags affected)
// C++ Original from Cpu.cpp line 831: void OpABX() { X += B; }
// Category: Register operations
// Opcode: 0x3A (inherent)
// Cycles: 3
// Flags: None affected

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_abx_basic_0x3a() {
    // C++ Original: X += B (unsigned addition, no flags)
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    // Setup: X = 0xC800, B = 0x10
    cpu.registers_mut().x = 0xC800;
    cpu.registers_mut().b = 0x10;
    
    // Write opcode: ABX
    unsafe { &mut *memory.get() }.write(RAM_START, 0x3A);
    
    // Execute
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify: X should be 0xC800 + 0x10 = 0xC810
    assert_eq!(cpu.registers().x, 0xC810);
    assert_eq!(cpu.registers().b, 0x10); // B unchanged
}

#[test]
fn test_abx_zero_addition_0x3a() {
    // Test: X + 0 = X
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().b = 0x00;
    
    unsafe { &mut *memory.get() }.write(RAM_START, 0x3A);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x1234);
}

#[test]
fn test_abx_overflow_wrap_0x3a() {
    // Test: Wrapping behavior (0xFFFF + 0x01 = 0x0000)
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    cpu.registers_mut().x = 0xFFFF;
    cpu.registers_mut().b = 0x01;
    
    unsafe { &mut *memory.get() }.write(RAM_START, 0x3A);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // C++ Original: X += B (wrapping_add behavior)
    assert_eq!(cpu.registers().x, 0x0000);
}

#[test]
fn test_abx_large_offset_0x3a() {
    // Test: X + 0xFF (max B value)
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().b = 0xFF;
    
    unsafe { &mut *memory.get() }.write(RAM_START, 0x3A);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x10FF);
}

#[test]
fn test_abx_no_flags_affected_0x3a() {
    // Verify that ABX does NOT affect any flags
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    // Set initial flag state
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().b = 0x50;
    
    unsafe { &mut *memory.get() }.write(RAM_START, 0x3A);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify flags remain unchanged
    assert_eq!(cpu.registers().cc.c, true);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.v, true);
    
    // Verify X was updated
    assert_eq!(cpu.registers().x, 0x1050);
}
