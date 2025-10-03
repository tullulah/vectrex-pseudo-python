//! Tests for ANDB, EORB, ORAB opcodes (B register logic operations)
//! MC6809 Programming Manual opcodes:
//! - ANDB: 0xC4 (immediate), 0xD4 (direct), 0xE4 (indexed), 0xF4 (extended)
//! - EORB: 0xC8 (immediate), 0xD8 (direct), 0xE8 (indexed), 0xF8 (extended)
//! - ORAB: 0xCA (immediate), 0xDA (direct), 0xEA (indexed), 0xFA (extended)

use std::cell::RefCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{EnableSync, MemoryBus};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu() -> (Cpu6809, Rc<RefCell<Ram>>) {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_andb_indexed_0xe4() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0xFF, X = 0xC850, memory at [X] = 0x0F
    cpu.registers_mut().b = 0xFF;
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x0F);
    
    // Write opcode: ANDB indexed [,X]
    memory.borrow_mut().write(RAM_START, 0xE4);
    memory.borrow_mut().write(RAM_START + 1, 0x84); // Postbyte: X register, no offset
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0xFF AND 0x0F = 0x0F
    assert_eq!(cpu.registers().b, 0x0F, "B should be 0xFF AND 0x0F = 0x0F");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow flag should always be clear for AND");
}

#[test]
fn test_andb_zero_result() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x0F, memory = 0xF0 (no common bits)
    cpu.registers_mut().b = 0x0F;
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0xF0);
    
    memory.borrow_mut().write(RAM_START, 0xE4);
    memory.borrow_mut().write(RAM_START + 1, 0x84);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00, "B should be zero");
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}

#[test]
fn test_eorb_indexed_0xe8() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0xAA (10101010), memory = 0x55 (01010101)
    cpu.registers_mut().b = 0xAA;
    cpu.registers_mut().y = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x55);
    
    // Write opcode: EORB indexed [,Y]
    memory.borrow_mut().write(RAM_START, 0xE8);
    memory.borrow_mut().write(RAM_START + 1, 0xA4); // Postbyte: Y register
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0xAA XOR 0x55 = 0xFF
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0xAA XOR 0x55 = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow flag should always be clear for EOR");
}

#[test]
fn test_eorb_extended_0xf8() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x12, memory at 0xC900 = 0x12 (XOR with itself = 0)
    cpu.registers_mut().b = 0x12;
    memory.borrow_mut().write(RAM_START + 0x100, 0x12);
    
    // Write opcode: EORB extended
    memory.borrow_mut().write(RAM_START, 0xF8);
    memory.borrow_mut().write(RAM_START + 1, 0xC9);
    memory.borrow_mut().write(RAM_START + 2, 0x00);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x12 XOR 0x12 = 0x00
    assert_eq!(cpu.registers().b, 0x00, "B should be zero (XOR with itself)");
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}

#[test]
fn test_orab_indexed_0xea() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x0F, X = 0xC850, memory = 0xF0
    cpu.registers_mut().b = 0x0F;
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0xF0);
    
    // Write opcode: ORAB indexed [,X]
    memory.borrow_mut().write(RAM_START, 0xEA);
    memory.borrow_mut().write(RAM_START + 1, 0x84);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x0F OR 0xF0 = 0xFF
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0x0F OR 0xF0 = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow flag should always be clear for OR");
}

#[test]
fn test_orab_no_change() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0xFF, memory = 0x00 (OR with 0 = no change)
    cpu.registers_mut().b = 0xFF;
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x00);
    
    memory.borrow_mut().write(RAM_START, 0xEA);
    memory.borrow_mut().write(RAM_START + 1, 0x84);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xFF, "B should remain 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set");
}

#[test]
fn test_logic_ops_clear_overflow() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: Force overflow flag set first
    cpu.registers_mut().b = 0x7F;
    cpu.registers_mut().cc.v = true; // Manually set overflow
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x01);
    
    // Execute ANDB - should clear overflow flag
    memory.borrow_mut().write(RAM_START, 0xE4);
    memory.borrow_mut().write(RAM_START + 1, 0x84);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().cc.v, false, "Overflow flag should be cleared by logic operation");
}
