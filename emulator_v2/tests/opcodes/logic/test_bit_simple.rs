// Minimal BIT Operations Test - 1:1 compliance with Vectrexy
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    Cpu6809::new(memory_bus)
}

#[test]
fn test_bita_immediate_basic() {
    let mut cpu = create_test_cpu();
    
    // Test A=$FF, operand=$0F -> result=$0F (not zero, not negative)
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x85);     // BITA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // operand
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers().a, 0xFF); // Register A should not change
    assert!(!cpu.registers().cc.z); // $FF & $0F = $0F -> not zero
    assert!(!cpu.registers().cc.n); // $0F has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // BIT always clears overflow
}

#[test]
fn test_bita_immediate_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Test A=$55, operand=$AA -> result=$00 (zero)
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x85);     // BITA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0xAA); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55); // Register A should not change
    assert!(cpu.registers().cc.z); // $55 & $AA = $00 -> zero
    assert!(!cpu.registers().cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // BIT always clears overflow
}

#[test]
fn test_bita_immediate_negative_result() {
    let mut cpu = create_test_cpu();
    
    // Test A=$80, operand=$80 -> result=$80 (negative)
    cpu.registers_mut().a = 0x80;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x85);     // BITA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x80); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80); // Register A should not change
    assert!(!cpu.registers().cc.z); // $80 & $80 = $80 -> not zero
    assert!(cpu.registers().cc.n); // $80 has bit 7 set -> negative
    assert!(!cpu.registers().cc.v); // BIT always clears overflow
}

#[test]
fn test_bitb_immediate_basic() {
    let mut cpu = create_test_cpu();
    
    // Test B=$7F, operand=$7F -> result=$7F (not zero, not negative)
    cpu.registers_mut().b = 0x7F;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0xC5);     // BITB immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x7F); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x7F); // Register B should not change
    assert!(!cpu.registers().cc.z); // $7F & $7F = $7F -> not zero
    assert!(!cpu.registers().cc.n); // $7F has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // BIT always clears overflow
}