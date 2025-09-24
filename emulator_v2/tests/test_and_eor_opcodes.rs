// Tests para opcodes AND y EOR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x84, 0xC4, 0x88, 0xC8

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
fn test_anda_immediate_basic() {
    // Test ANDA #$0F - AND A with immediate value
    // C++ Original: OpAND sets reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // Initial A = 11111111
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    
    // Setup: ANDA #$0F instruction at 0xC800 (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x84); // ANDA immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x0F); // AND mask 00001111
    cpu.registers_mut().pc = 0xC800;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x0F, "A should be 0xFF & 0x0F = 0x0F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by AND)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "ANDA immediate should take 2 cycles");
}

#[test]
fn test_anda_immediate_zero_result() {
    // Test ANDA #$00 - AND resulting in zero
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // A starts as all bits set
    
    // Setup: ANDA #$00 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x84); // ANDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x00); // AND with zero
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0xFF & 0x00 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_andb_immediate_basic() {
    // Test ANDB #$AA - AND B with immediate value
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0xFF; // Initial B = 11111111
    
    // Setup: ANDB #$AA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC4); // ANDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xAA); // AND mask 10101010
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xAA, "B should be 0xFF & 0xAA = 0xAA");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_eora_immediate_basic() {
    // Test EORA #$FF - EOR A with immediate value (toggle all bits)
    // C++ Original: OpEOR sets reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xAA; // Initial A = 10101010
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    
    // Setup: EORA #$FF instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x88); // EORA immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xFF); // EOR mask 11111111
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55, "A should be 0xAA ^ 0xFF = 0x55");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by EOR)");
}

#[test]
fn test_eora_immediate_zero_result() {
    // Test EORA with same value - EOR resulting in zero
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x42; // A = 0x42
    
    // Setup: EORA #$42 instruction (same as A)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x88); // EORA immediate
    memory_bus.borrow_mut().write(0xC801, 0x42); // EOR with same value
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x42 ^ 0x42 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_eorb_immediate_basic() {
    // Test EORB #$55 - EOR B with immediate value
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0xAA; // Initial B = 10101010
    
    // Setup: EORB #$55 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC8); // EORB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x55); // EOR mask 01010101
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0xAA ^ 0x55 = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_and_eor_flag_comprehensive() {
    // Test flag behavior with various AND/EOR combinations
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test 1: AND negative result
    cpu.registers_mut().a = 0xFF;
    memory_bus.borrow_mut().write(0xC800, 0x84); // ANDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x80); // AND with 0x80
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    
    // Test 2: EOR zero result
    cpu.registers_mut().b = 0x33;
    memory_bus.borrow_mut().write(0xC810, 0xC8); // EORB immediate
    memory_bus.borrow_mut().write(0xC811, 0x33); // EOR with same value
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00, "B should be 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}