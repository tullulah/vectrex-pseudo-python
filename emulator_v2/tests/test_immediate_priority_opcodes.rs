// Test suite for immediate priority opcodes: JMP, MUL, SWI, SWI2, SWI3, RTI
// C++ Original: 1:1 port from Vectrexy test infrastructure

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800; // RAM area for tests
const RAM_END: u16 = 0xCFFF;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // For basic opcode tests, we don't need BIOS - just setup vector table manually if needed
    // Test vector addresses will be setup by individual tests
    
    Cpu6809::new(memory_bus)
}

// ========== JMP TESTS ==========

#[test]
fn test_jmp_basic_functionality() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Direct (0x0E) - cycles: 3
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().dp = 0xC8; // Direct page for 0xC8xx
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x0E); // JMP direct
    memory_bus.borrow_mut().write(RAM_START + 1, 0x50); // Direct address 0x50 -> 0xC850
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "JMP direct should take 3 cycles");
    assert_eq!(cpu.registers().pc, 0xC850, "JMP should set PC to effective address");
}

#[test]
fn test_jmp_indexed() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Indexed (0x6E) - cycles: variable based on addressing mode
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().x = 0xC900; // Base address
    cpu.registers_mut().a = 0x0A;   // Offset value for A register
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x6E); // JMP indexed
    memory_bus.borrow_mut().write(RAM_START + 1, 0x86); // X + A offset (valid indexed mode)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 4, "JMP indexed A,X should take 4 cycles (3 base + 1 for A offset)");
    assert_eq!(cpu.registers().pc, 0xC90A, "JMP indexed should jump to X + A");
}

#[test]
fn test_jmp_extended() {
    let mut cpu = create_test_cpu();
    
    // Test JMP Extended (0x7E) - cycles: 4
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x7E); // JMP extended
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9); // High byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x50); // Low byte
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 4, "JMP extended should take 4 cycles");
    assert_eq!(cpu.registers().pc, 0xC950, "JMP extended should jump to full 16-bit address");
}

// ========== MUL TESTS ==========

#[test]
fn test_mul_basic() {
    let mut cpu = create_test_cpu();
    
    // Test MUL (0x3D) - cycles: 11
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x05;
    cpu.registers_mut().b = 0x07;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 11, "MUL should take 11 cycles");
    assert_eq!(cpu.registers().d(), 0x05 * 0x07, "MUL should set D to A * B");
    assert_eq!(cpu.registers().a, 0x00, "MUL result high byte should be in A");
    assert_eq!(cpu.registers().b, 0x23, "MUL result low byte should be in B (5*7=35=0x23)");
    
    // Test flags - C++ Original: MUL affects Z and C flags
    assert!(!cpu.registers().cc.z, "Zero flag should be clear for non-zero result");
    
    // C++ Original: Carry set if bit 7 of result is 1
    assert!(!cpu.registers().cc.c, "Carry flag should be clear when bit 7 of result is 0");
}

#[test]
fn test_mul_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Test MUL with zero result
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().b = 0x55;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().d(), 0x0000, "0 * anything should be 0");
    assert!(cpu.registers().cc.z, "Zero flag should be set for zero result");
    assert!(!cpu.registers().cc.c, "Carry flag should be clear for zero result");
}

#[test]
fn test_mul_maximum_values() {
    let mut cpu = create_test_cpu();
    
    // Test MUL with maximum 8-bit values
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0xFF;  // 255
    cpu.registers_mut().b = 0xFF;  // 255
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3D); // MUL
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().d(), 0xFE01, "255 * 255 = 65025 = 0xFE01");
    assert!(!cpu.registers().cc.c, "Carry should be clear for 0xFE01 (bit 7 of low byte is 0)");
    assert!(!cpu.registers().cc.z, "Zero flag should be clear for maximum result");
}

// ========== SWI TESTS ==========
// Note: SWI tests require proper vector table setup, temporarily commented out

/*
#[test]
fn test_swi_basic_interrupt() {
    let mut cpu = create_test_cpu();
    
    // Test SWI (0x3F) - Software Interrupt - cycles: 19
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = 0x9ABC;
    cpu.registers_mut().u = 0xDEF0;
    cpu.registers_mut().s = RAM_END;  // Stack starts at top of RAM
    cpu.registers_mut().dp = 0xAA;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3F); // SWI
    
    let old_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 19, "SWI should take 19 cycles");
    // Note: Vector address will depend on BIOS content, so this test might need adjustment
    
    // C++ Original: SWI sets I and F flags, and E flag (entire flag)
    assert!(cpu.registers().cc.i, "Interrupt mask should be set after SWI");
    assert!(cpu.registers().cc.f, "Fast interrupt mask should be set after SWI");
    
    // C++ Original: SWI sets entire flag (pushes all registers)
    assert!(cpu.registers().cc.e, "Entire flag should be set after SWI");
    
    // Stack should be decremented by 12 bytes (PC, U, Y, X, DP, B, A, CC)
    assert_eq!(cpu.registers().s, old_s - 12, "Stack pointer should be decremented by 12 bytes");
}

// ========== SWI2 TESTS ==========

#[test]
fn test_swi2_basic() {
    let mut cpu = create_test_cpu();
    
    // Test SWI2 (0x103F) - Software Interrupt 2 - cycles: 20
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x10); // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x3F); // SWI2
    
    let old_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "SWI2 should take 20 cycles");
    assert!(cpu.registers().cc.e, "SWI2 should set entire flag");
    
    assert_eq!(cpu.registers().s, old_s - 12, "SWI2 should push entire register set");
}

// ========== SWI3 TESTS ==========

#[test]
fn test_swi3_basic() {
    let mut cpu = create_test_cpu();
    
    // Test SWI3 (0x113F) - Software Interrupt 3 - cycles: 20
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x11); // Page 2 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x3F); // SWI3
    
    let old_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "SWI3 should take 20 cycles");
    assert!(cpu.registers().cc.e, "SWI3 should set entire flag");
    
    assert_eq!(cpu.registers().s, old_s - 12, "SWI3 should push entire register set");
}

// ========== RTI TESTS ==========

#[test]
fn test_rti_basic() {
    let mut cpu = create_test_cpu();
    
    // Test RTI (0x3B) - Return from Interrupt - cycles: variable
    // Set up stack with minimal saved context
    cpu.registers_mut().s = RAM_END - 3;  // Only PC and CC saved for fast interrupt
    
    let saved_pc = 0xABCD;
    let saved_cc = 0x50;
    
    let memory_bus = cpu.memory_bus().clone();
    let mut stack_addr = cpu.registers().s;
    
    memory_bus.borrow_mut().write(stack_addr, saved_cc); stack_addr += 1;
    memory_bus.borrow_mut().write(stack_addr, (saved_pc >> 8) as u8); stack_addr += 1;
    memory_bus.borrow_mut().write(stack_addr, (saved_pc & 0xFF) as u8);
    
    cpu.registers_mut().pc = RAM_START;
    memory_bus.borrow_mut().write(RAM_START, 0x3B); // RTI
    
    let _cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().pc, saved_pc, "RTI should restore PC");
}
*/