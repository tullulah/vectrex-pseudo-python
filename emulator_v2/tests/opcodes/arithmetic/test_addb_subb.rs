//! Tests for ADDB and SUBB opcodes (B register arithmetic)
//! MC6809 Programming Manual opcodes:
//! - ADDB: 0xCB (immediate), 0xDB (direct), 0xEB (indexed), 0xFB (extended)
//! - SUBB: 0xC0 (immediate), 0xD0 (direct), 0xE0 (indexed), 0xF0 (extended)

use std::cell::RefCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{EnableSync, MemoryBus};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu() -> (Cpu6809, Rc<RefCell<Ram>>) {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Connect RAM for entire address space
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_addb_direct_0xdb() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x42, memory at 0xC850 = 0x33
    cpu.registers_mut().b = 0x42;
    cpu.registers_mut().dp = 0xC8; // Direct page
    memory.borrow_mut().write(RAM_START + 0x50, 0x33);
    
    // Write opcode: ADDB direct
    memory.borrow_mut().write(RAM_START, 0xDB); // Opcode
    memory.borrow_mut().write(RAM_START + 1, 0x50); // Direct address (low byte)
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x42 + 0x33 = 0x75
    assert_eq!(cpu.registers().b, 0x75, "B should be 0x42 + 0x33 = 0x75");
    assert_eq!(cpu.registers().pc, RAM_START + 2, "PC should advance by 2");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}

#[test]
fn test_addb_indexed_0xeb() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x10, X = 0xC850, memory at [X] = 0x20
    cpu.registers_mut().b = 0x10;
    cpu.registers_mut().x = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x20);
    
    // Write opcode: ADDB indexed [,X] (postbyte = 0x84)
    memory.borrow_mut().write(RAM_START, 0xEB); // Opcode
    memory.borrow_mut().write(RAM_START + 1, 0x84); // Postbyte: no offset, X register
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x10 + 0x20 = 0x30
    assert_eq!(cpu.registers().b, 0x30, "B should be 0x10 + 0x20 = 0x30");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
}

#[test]
fn test_addb_extended_0xfb() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x7F, memory at 0xC900 = 0x02 (will overflow to negative)
    cpu.registers_mut().b = 0x7F;
    memory.borrow_mut().write(RAM_START + 0x100, 0x02);
    
    // Write opcode: ADDB extended
    memory.borrow_mut().write(RAM_START, 0xFB); // Opcode
    memory.borrow_mut().write(RAM_START + 1, 0xC9); // Address high
    memory.borrow_mut().write(RAM_START + 2, 0x00); // Address low
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x7F + 0x02 = 0x81 (negative in signed)
    assert_eq!(cpu.registers().b, 0x81, "B should be 0x81");
    assert_eq!(cpu.registers().pc, RAM_START + 3, "PC should advance by 3");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set");
    assert_eq!(cpu.registers().cc.v, true, "Overflow flag should be set");
}

#[test]
fn test_subb_direct_0xd0() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x50, memory at 0xC850 = 0x10
    cpu.registers_mut().b = 0x50;
    cpu.registers_mut().dp = 0xC8;
    memory.borrow_mut().write(RAM_START + 0x50, 0x10);
    
    // Write opcode: SUBB direct
    memory.borrow_mut().write(RAM_START, 0xD0);
    memory.borrow_mut().write(RAM_START + 1, 0x50);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x50 - 0x10 = 0x40
    assert_eq!(cpu.registers().b, 0x40, "B should be 0x50 - 0x10 = 0x40");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "Carry flag should be clear (no borrow)");
}

#[test]
fn test_subb_indexed_0xe0() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x20, Y = 0xC850, memory at [Y] = 0x20 (result = 0)
    cpu.registers_mut().b = 0x20;
    cpu.registers_mut().y = RAM_START + 0x50;
    memory.borrow_mut().write(RAM_START + 0x50, 0x20);
    
    // Write opcode: SUBB indexed [,Y] (postbyte = 0xA4)
    memory.borrow_mut().write(RAM_START, 0xE0);
    memory.borrow_mut().write(RAM_START + 1, 0xA4); // Postbyte: no offset, Y register
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x20 - 0x20 = 0x00
    assert_eq!(cpu.registers().b, 0x00, "B should be zero");
    assert_eq!(cpu.registers().cc.z, true, "Zero flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "Carry flag should be clear");
}

#[test]
fn test_subb_extended_0xf0() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x10, memory at 0xC900 = 0x20 (will underflow)
    cpu.registers_mut().b = 0x10;
    memory.borrow_mut().write(RAM_START + 0x100, 0x20);
    
    // Write opcode: SUBB extended
    memory.borrow_mut().write(RAM_START, 0xF0);
    memory.borrow_mut().write(RAM_START + 1, 0xC9);
    memory.borrow_mut().write(RAM_START + 2, 0x00);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    // Verify result: B = 0x10 - 0x20 = 0xF0 (borrow occurred)
    assert_eq!(cpu.registers().b, 0xF0, "B should wrap to 0xF0");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set");
    assert_eq!(cpu.registers().cc.c, true, "Carry flag should be set (borrow)");
}

#[test]
fn test_addb_overflow_flag() {
    let (mut cpu, memory) = setup_cpu();
    
    // Setup: B = 0x7F (max positive signed), add 0x01 → overflow to negative
    cpu.registers_mut().b = 0x7F;
    cpu.registers_mut().dp = 0xC8;
    memory.borrow_mut().write(RAM_START + 0x50, 0x01);
    
    memory.borrow_mut().write(RAM_START, 0xDB);
    memory.borrow_mut().write(RAM_START + 1, 0x50);
    
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x80, "B should be 0x80");
    assert_eq!(cpu.registers().cc.v, true, "Overflow flag should be set");
    assert_eq!(cpu.registers().cc.n, true, "Negative flag should be set");
}
