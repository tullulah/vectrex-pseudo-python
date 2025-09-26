// ABX (Add B to X) Opcode 0x3A Tests
// Tests for Add B register to X register (unsigned addition)
// Following Vectrexy 1:1 compliance rules

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().pc = RAM_START;
    cpu
}

#[test]
fn test_abx_simple_addition() {
    // C++ Original: ABX - Add B register to X register (X = X + B, unsigned)
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().b = 0x42;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3A);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ABX should take 3 cycles");
    assert_eq!(cpu.registers().x, 0x1042, "X should be X + B");
    assert_eq!(cpu.registers().b, 0x42, "B should be unchanged");
    // ABX does not affect condition codes
}

#[test]
fn test_abx_with_overflow() {
    // C++ Original: ABX with 16-bit overflow  
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0xFFFF;
    cpu.registers_mut().b = 0x01;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3A);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ABX should take 3 cycles");
    assert_eq!(cpu.registers().x, 0x0000, "X should wrap around to 0x0000");
    // ABX does not set overflow flags
}

#[test]
fn test_abx_zero_addition() {
    // C++ Original: ABX with B=0 should leave X unchanged
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().b = 0x00;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3A);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ABX should take 3 cycles");
    assert_eq!(cpu.registers().x, 0x5678, "X should remain unchanged");
    assert_eq!(cpu.registers().b, 0x00, "B should remain 0x00");
}