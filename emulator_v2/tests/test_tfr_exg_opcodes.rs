// test_tfr_exg_opcodes.rs - Tests for TFR/EXG register transfer opcodes
// C++ Original Analysis from Vectrexy Cpu.cpp lines 803-826:
//
// void ExchangeOrTransfer(bool exchange) {
//     uint8_t postbyte = ReadPC8();
//     ASSERT(!!(postbyte & BITS(3)) == !!(postbyte & BITS(7))); // 8-bit to 8-bit or 16-bit to 16-bit only
//
//     uint8_t src = (postbyte >> 4) & 0b111;
//     uint8_t dst = postbyte & 0b111;
//
//     if (postbyte & BITS(3)) {
//         ASSERT(src < 4 && dst < 4); // Only first 4 are valid 8-bit register indices
//         uint8_t* const reg[]{&A, &B, &CC.Value, &DP};
//         if (exchange)
//             std::swap(*reg[dst], *reg[src]);
//         else
//             *reg[dst] = *reg[src];
//     } else {
//         ASSERT(src < 6 && dst < 6); // Only first 6 are valid 16-bit register indices
//         uint16_t* const reg[]{&D, &X, &Y, &U, &S, &PC};
//         if (exchange)
//             std::swap(*reg[dst], *reg[src]);
//         else
//             *reg[dst] = *reg[src];
//     }
// }
// void OpEXG() { ExchangeOrTransfer(true); }  // 0x1E
// void OpTFR() { ExchangeOrTransfer(false); } // 0x1F

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
fn test_tfr_16bit_d_to_s() {
    // postbyte = (REG_D << 4) | REG_S = (0 << 4) | 4 = 0x04
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x12; // D = A:B
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().s = 0x0000;
    cpu.registers_mut().pc = 0xC800;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x04); // D -> S, 16-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    let d_value = (cpu.registers().a as u16) << 8 | cpu.registers().b as u16;
    assert_eq!(d_value, 0x1234, "D register should remain unchanged");
    assert_eq!(cpu.registers().s, 0x1234, "S register should receive D value");
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 6);
}

#[test]
fn test_exg_8bit_a_b() {
    // postbyte = (REG_A << 4) | REG_B | 0x08 = (0 << 4) | 1 | 0x08 = 0x09
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
fn test_tfr_16bit_pc_to_x() {
    // postbyte = (REG_PC << 4) | REG_X = (5 << 4) | 1 = 0x51
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x0000;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x51); // PC -> X, 16-bit transfer
    
    let cycles = cpu.execute_instruction(false, false);
    
    // PC should have advanced to 0xC802 after reading the postbyte
    // X gets the PC value after advancement
    assert_eq!(cpu.registers().x, 0xC802, "X should receive PC value after instruction fetch");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance normally");
    assert_eq!(cycles, 6);
}
