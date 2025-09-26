// C++ Original: Test suite for TFR (Transfer Register) opcode 0x1F - Using 1:1 field access and correct API
// TFR implementation from Vectrexy Cpu.cpp lines 803-826

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
fn test_tfr_16bit_d_to_x() {
    // C++ Original: TFR D,X - transfer 16-bit D register to X register
    let mut cpu = create_test_cpu();
    
    // Setup: D = 0x1234
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write TFR D,X instruction: 0x1F followed by postbyte 0x01 (D=0, X=1)
    memory_bus.borrow_mut().write(0xC800, 0x1F);     // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // D(0) -> X(1)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: TFR takes 6 cycles
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
    
    // Verify transfer - source unchanged, destination receives value
    assert_eq!(cpu.registers().d(), 0x1234, "D register should be unchanged");
    assert_eq!(cpu.registers().x, 0x1234, "X register should receive D value");
}

#[test]
fn test_tfr_8bit_a_to_b() {
    // C++ postbyte format for 8-bit transfer A -> B:
    // For 8-bit: bit 3 AND bit 7 must both be set (0x88 base)
    // src = REG_A (0), dst = REG_B (1)
    // postbyte = (REG_A << 4) | REG_B | 0x88 = (0 << 4) | 1 | 0x88 = 0x89
    let mut cpu = create_test_cpu();
    
    // Setup test values
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x00;
    cpu.registers_mut().pc = 0xC800;
    
    // Write TFR opcode and postbyte to memory
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x89); // A -> B, 8-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x42, "A register should remain unchanged");
    assert_eq!(cpu.registers().b, 0x42, "B register should receive A value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 6, "TFR should take 6 cycles"); // Based on 6809 timing
}

#[test]
fn test_tfr_8bit_cc_to_dp() {
    // postbyte = (REG_CC << 4) | REG_DP | 0x88 = (2 << 4) | 3 | 0x88 = 0xAB
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().cc.from_u8(0x85); // Some flag values
    cpu.registers_mut().dp = 0x00;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0xAB); // CC -> DP, 8-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().cc.to_u8(), 0x85, "CC register should remain unchanged");
    assert_eq!(cpu.registers().dp, 0x85, "DP register should receive CC value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 6);
}

#[test]
fn test_tfr_16bit_x_to_y() {
    // postbyte = (REG_X << 4) | REG_Y = (1 << 4) | 2 = 0x12
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().y = 0x0000;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x12); // X -> Y, 16-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().x, 0x1234, "X register should remain unchanged");
    assert_eq!(cpu.registers().y, 0x1234, "Y register should receive X value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 6);
}

#[test]
fn test_tfr_stack_pointers() {
    // Test TFR S,U - transfer system stack pointer to user stack pointer
    // postbyte = (REG_S << 4) | REG_U = (4 << 4) | 3 = 0x43
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().s = 0xCFFF;
    cpu.registers_mut().u = 0x0000;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x43); // S -> U, 16-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().s, 0xCFFF, "S register should remain unchanged");
    assert_eq!(cpu.registers().u, 0xCFFF, "U register should receive S value");
    assert_eq!(cycles, 6);
}