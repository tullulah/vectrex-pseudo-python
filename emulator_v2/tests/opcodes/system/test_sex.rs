// SEX (Sign Extend) Opcode 0x1D Tests
// Tests for Sign Extend B register to A register
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
fn test_sex_positive_b_to_a() {
    // C++ Original: SEX - Sign extend from B to A (if B bit 7 = 0, A = 0x00)
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0xFF; // Will be overwritten
    cpu.registers_mut().b = 0x42; // Positive (bit 7 = 0)
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write SEX instruction: 0x1D
    memory_bus.borrow_mut().write(RAM_START, 0x1D);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x00 for positive B");
    assert_eq!(cpu.registers().b, 0x42, "B should be unchanged");
    
    // Check CC flags: N and Z based on D register
    let d_value = cpu.registers().d();
    assert_eq!(d_value, 0x0042, "D should be 0x0042");
    assert!(!cpu.registers().cc.n, "N flag should be clear (D is positive)");
    assert!(!cpu.registers().cc.z, "Z flag should be clear (D is not zero)");
}

#[test]
fn test_sex_negative_b_to_a() {
    // C++ Original: SEX - Sign extend from B to A (if B bit 7 = 1, A = 0xFF)
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x00; // Will be overwritten  
    cpu.registers_mut().b = 0x80; // Negative (bit 7 = 1)
    
    let memory_bus = cpu.memory_bus().clone();
    
    memory_bus.borrow_mut().write(RAM_START, 0x1D);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
    assert_eq!(cpu.registers().a, 0xFF, "A should be 0xFF for negative B");
    assert_eq!(cpu.registers().b, 0x80, "B should be unchanged");
    
    let d_value = cpu.registers().d();
    assert_eq!(d_value, 0xFF80, "D should be 0xFF80");
    assert!(cpu.registers().cc.n, "N flag should be set (D is negative)");
    assert!(!cpu.registers().cc.z, "Z flag should be clear (D is not zero)");
}

#[test]
fn test_sex_zero_result() {
    // C++ Original: SEX with B=0 should result in D=0 and set Z flag
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x00; // Zero
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x1D);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "SEX should take 2 cycles");
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x00");
    assert_eq!(cpu.registers().b, 0x00, "B should remain 0x00");
    assert_eq!(cpu.registers().d(), 0x0000, "D should be 0x0000");
    assert!(!cpu.registers().cc.n, "N flag should be clear");
    assert!(cpu.registers().cc.z, "Z flag should be set (D is zero)");
}