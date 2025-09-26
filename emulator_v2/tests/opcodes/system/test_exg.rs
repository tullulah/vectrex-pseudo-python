// C++ Original: Test suite for EXG (Exchange Register) opcode 0x1E - Using 1:1 field access and correct API
// EXG implementation from Vectrexy Cpu.cpp lines 803-826

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

// Register indices for 8-bit transfers (postbyte & 0x08 != 0)
// uint8_t* const reg[]{&A, &B, &CC.Value, &DP};
const REG_A: u8 = 0;
const REG_B: u8 = 1;
const REG_CC: u8 = 2;
const REG_DP: u8 = 3;

// Register indices for 16-bit transfers (postbyte & 0x08 == 0)
// uint16_t* const reg[]{&D, &X, &Y, &U, &S, &PC};
const REG_D: u8 = 0;
const REG_X: u8 = 1;
const REG_Y: u8 = 2;
const REG_U: u8 = 3;
const REG_S: u8 = 4;
const REG_PC: u8 = 5;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_exg_8bit_a_b() {
    // postbyte = (REG_A << 4) | REG_B | 0x88 = (0 << 4) | 1 | 0x88 = 0x89
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x73;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x89); // A <-> B, 8-bit exchange
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x73, "A should receive B's original value");
    assert_eq!(cpu.registers().b, 0x42, "B should receive A's original value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 8, "EXG should take 8 cycles"); // Based on 6809 timing
}

#[test]
fn test_exg_16bit_x_y() {
    // postbyte = (REG_X << 4) | REG_Y = (1 << 4) | 2 = 0x12
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().y = 0x5678;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x12); // X <-> Y, 16-bit exchange
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x5678, "X should receive Y's original value");
    assert_eq!(cpu.registers().y, 0x1234, "Y should receive X's original value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 8);
}

#[test]
fn test_exg_16bit_s_u() {
    // postbyte = (REG_S << 4) | REG_U = (4 << 4) | 3 = 0x43
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().s = 0xAABB;
    cpu.registers_mut().u = 0xCCDD;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x43); // S <-> U, 16-bit exchange
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().s, 0xCCDD, "S should receive U's original value");
    assert_eq!(cpu.registers().u, 0xAABB, "U should receive S's original value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 8);
}

#[test]
fn test_exg_8bit_cc_dp() {
    // postbyte = (REG_CC << 4) | REG_DP | 0x88 = (2 << 4) | 3 | 0x88 = 0xAB
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().cc.from_u8(0x85); // Some flag values
    cpu.registers_mut().dp = 0x42;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0xAB); // CC <-> DP, 8-bit exchange
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().cc.to_u8(), 0x42, "CC should receive DP's original value");
    assert_eq!(cpu.registers().dp, 0x85, "DP should receive CC's original value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 8);
}

#[test]
fn test_exg_16bit_d_x() {
    // postbyte = (REG_D << 4) | REG_X = (0 << 4) | 1 = 0x01
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x12; // D = A:B = 0x1234
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // D <-> X, 16-bit exchange
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().d(), 0x5678, "D should receive X's original value");
    assert_eq!(cpu.registers().x, 0x1234, "X should receive D's original value");
    assert_eq!(cpu.registers().a, 0x56, "A should contain high byte of exchanged value");
    assert_eq!(cpu.registers().b, 0x78, "B should contain low byte of exchanged value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 8);
}