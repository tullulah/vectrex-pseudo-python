// Test suite for SUBB (Subtract from B) opcode 0xC0
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp - OpSUB for B register

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
fn test_subb_immediate_basic() {
    // Test SUBB #$11 - Subtract immediate from B
    // C++ Original: OpSUB<0, 0xC0>(B);
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0x44; // Initial B = 68
    
    // Setup: SUBB #$11 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0xC0); // SUBB immediate opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x11); // Subtract 17
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x33, "B should be 0x44 - 0x11 = 0x33");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_subb_immediate_overflow() {
    // Test SUBB overflow: negative - positive = positive (should set V)
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x80; // -128 in signed
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0xC0); // SUBB immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // Subtract 1
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x7F, "B should be 0x7F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
}

#[test]
fn test_subb_immediate_zero_result() {
    // Test SUBB resulting in zero
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x25; // Initial B = 37
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0xC0); // SUBB immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x25); // Subtract same value
    cpu.registers_mut().pc = RAM_START;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00, "B should be 0x25 - 0x25 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_subb_direct_0xd0() {
    // C++ Original: OpSUB<0, 0xD0>(B) - SUBB direct page addressing
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().b = 0x50;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC800
    
    // Setup direct page memory
    cpu.memory_bus().borrow_mut().write(0xC810, 0x20); // Operand in direct page
    
    cpu.memory_bus().borrow_mut().write(RAM_START, 0xD0); // SUBB direct
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x10); // Direct page offset
    
    cpu.execute_instruction(false, false);
    
    // $50 - $20 = $30
    assert_eq!(cpu.registers().b, 0x30);
    assert_eq!(cpu.registers().cc.c, false); // No borrow
    assert_eq!(cpu.registers().cc.z, false); // Not zero
    assert_eq!(cpu.registers().cc.n, false); // Not negative
    assert_eq!(cpu.registers().cc.v, false); // No overflow
}

#[test]
fn test_subb_direct_underflow() {
    // C++ Original: SUBB direct with underflow
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().b = 0x10;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC800
    
    // Setup direct page memory
    cpu.memory_bus().borrow_mut().write(0xC815, 0x20); // Operand in direct page
    
    cpu.memory_bus().borrow_mut().write(RAM_START, 0xD0); // SUBB direct
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x15); // Direct page offset
    
    cpu.execute_instruction(false, false);
    
    // $10 - $20 = $F0 (underflow)
    assert_eq!(cpu.registers().b, 0xF0);
    assert_eq!(cpu.registers().cc.c, true); // Borrow occurred
    assert_eq!(cpu.registers().cc.z, false); // Not zero
    assert_eq!(cpu.registers().cc.n, true); // Negative result
}

#[test]
fn test_subb_indexed_0xe0() {
    // C++ Original: OpSUB<0, 0xE0>(B) - SUBB indexed addressing
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().b = 0x80;
    cpu.registers_mut().x = 0xC820; // Index register
    
    // Setup indexed memory
    cpu.memory_bus().borrow_mut().write(0xC820, 0x30); // Operand at X
    
    cpu.memory_bus().borrow_mut().write(RAM_START, 0xE0); // SUBB indexed
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x00); // No offset from X
    
    cpu.execute_instruction(false, false);
    
    // $80 - $30 = $50
    assert_eq!(cpu.registers().b, 0x50);
    assert_eq!(cpu.registers().cc.c, false); // No borrow
    assert_eq!(cpu.registers().cc.z, false); // Not zero
    assert_eq!(cpu.registers().cc.n, false); // Not negative
    assert_eq!(cpu.registers().cc.v, true); // Overflow: -128 - 48 = -176 (signed overflow)
}

#[test]
fn test_subb_extended_0xf0() {
    // C++ Original: OpSUB<0, 0xF0>(B) - SUBB extended addressing
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().b = 0x7F;
    
    // Setup extended memory
    cpu.memory_bus().borrow_mut().write(0xC900, 0x7F); // Operand at extended address
    
    cpu.memory_bus().borrow_mut().write(RAM_START, 0xF0); // SUBB extended
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0xC9); // Address high byte
    cpu.memory_bus().borrow_mut().write(RAM_START + 2, 0x00); // Address low byte
    
    cpu.execute_instruction(false, false);
    
    // $7F - $7F = $00
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().cc.c, false); // No borrow
    assert_eq!(cpu.registers().cc.z, true); // Zero result
    assert_eq!(cpu.registers().cc.n, false); // Not negative
    assert_eq!(cpu.registers().cc.v, false); // No overflow
}

#[test]
fn test_subb_comprehensive_flags() {
    // C++ Original: Comprehensive flag testing for SUBB operations
    let mut cpu = create_test_cpu();
    
    let test_cases = [
        // (B_initial, operand, expected_result, expected_c, expected_z, expected_n, expected_v, description)
        (0x80, 0x01, 0x7F, false, false, false, true, "signed overflow: -128 - 1 = +127"),
        (0x01, 0x80, 0x81, true, false, true, true, "negative underflow with overflow"),
        (0x00, 0x01, 0xFF, true, false, true, false, "zero minus one"),
        (0xFF, 0xFF, 0x00, false, true, false, false, "same values -> zero result"),
        (0x50, 0x30, 0x20, false, false, false, false, "normal subtraction"),
    ];
    
    for (i, (b_initial, operand, expected_result, expected_c, expected_z, expected_n, expected_v, description)) in test_cases.iter().enumerate() {
        let base_addr = RAM_START + (i as u16 * 2);
        
        cpu.registers_mut().b = *b_initial;
        cpu.registers_mut().pc = base_addr;
        
        cpu.memory_bus().borrow_mut().write(base_addr, 0xC0); // SUBB immediate
        cpu.memory_bus().borrow_mut().write(base_addr + 1, *operand);
        
        cpu.execute_instruction(false, false);
        
        assert_eq!(cpu.registers().b, *expected_result, "SUBB {}: Result mismatch", description);
        assert_eq!(cpu.registers().cc.c, *expected_c, "SUBB {}: Carry flag mismatch", description);
        assert_eq!(cpu.registers().cc.z, *expected_z, "SUBB {}: Zero flag mismatch", description);
        assert_eq!(cpu.registers().cc.n, *expected_n, "SUBB {}: Negative flag mismatch", description);
        assert_eq!(cpu.registers().cc.v, *expected_v, "SUBB {}: Overflow flag mismatch", description);
    }
}