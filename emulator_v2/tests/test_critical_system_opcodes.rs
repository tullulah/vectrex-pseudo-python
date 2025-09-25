// Critical System Opcodes Compliance Tests
// Tests for TFR (0x1F), EXG (0x1E), SEX (0x1D), ANDCC (0x1C), ORCC (0x1A), ABX (0x3A), CWAI (0x3C)
// Following Vectrexy 1:1 compliance rules

use vectrex_emulator_v2::core::emulator::Emulator;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;
const RAM_END: u16 = 0xCFFF;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    let mut cpu = Cpu6809::new(memory_bus.clone());
    // Don't call reset() as it requires ROM vectors - just set PC manually
    cpu.registers_mut().pc = RAM_START;
    
    cpu
}

// ======= TFR (Transfer Register) Tests =======

#[test]
fn test_tfr_16bit_d_to_x() {
    // C++ Original: TFR D,X - transfer 16-bit D register to X register
    let mut cpu = create_test_cpu();
    
    // Setup: D = 0x1234
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write TFR D,X instruction: 0x1F followed by postbyte 0x01 (D=0, X=1)
    memory_bus.borrow_mut().write(RAM_START, 0x1F);     // TFR opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // D(0) -> X(1)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: TFR takes 6 cycles
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
    
    // Verify D register unchanged, X register = D value
    assert_eq!(cpu.registers().d(), 0x1234, "D register should be unchanged");
    assert_eq!(cpu.registers().x, 0x1234, "X register should receive D value");
    assert_eq!(cpu.registers().pc, RAM_START + 2, "PC should advance by 2");
}

#[test]
fn test_tfr_8bit_a_to_b() {
    // C++ Original: TFR A,B - transfer 8-bit A register to B register  
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x00;
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write TFR A,B: 0x1F followed by postbyte 0x89 (A=8, B=9, bit 3 set for 8-bit)
    memory_bus.borrow_mut().write(RAM_START, 0x1F);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x89);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
    assert_eq!(cpu.registers().a, 0x42, "A register should be unchanged");
    assert_eq!(cpu.registers().b, 0x42, "B register should receive A value");
}

#[test] 
fn test_tfr_pc_to_u() {
    // C++ Original: TFR PC,U - transfer PC to U register
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = RAM_START + 4; // Use a different offset in RAM
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write TFR PC,U: postbyte 0x53 (PC=5, U=3)
    memory_bus.borrow_mut().write(RAM_START + 4, 0x1F);
    memory_bus.borrow_mut().write(RAM_START + 5, 0x53);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
    // PC advances after reading postbyte, so U gets PC+2
    assert_eq!(cpu.registers().u, RAM_START + 6, "U should receive PC+2 value");
    assert_eq!(cpu.registers().pc, RAM_START + 6, "PC should advance by 2");
}

// ======= EXG (Exchange Register) Tests =======

#[test]
fn test_exg_16bit_d_x_exchange() {
    // C++ Original: EXG D,X - exchange D and X registers
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34; // D = 0x1234
    cpu.registers_mut().x = 0x5678;
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write EXG D,X: 0x1E followed by postbyte 0x01
    memory_bus.borrow_mut().write(RAM_START, 0x1E);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 8, "EXG should take 8 cycles");
    assert_eq!(cpu.registers().d(), 0x5678, "D should now contain old X value");
    assert_eq!(cpu.registers().x, 0x1234, "X should now contain old D value");
}

#[test]
fn test_exg_8bit_a_b_exchange() {
    // C++ Original: EXG A,B - exchange A and B registers
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().b = 0xBB;
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write EXG A,B: postbyte 0x89 (A=8, B=9, bit 3 set)
    memory_bus.borrow_mut().write(RAM_START, 0x1E);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x89);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 8, "EXG should take 8 cycles");
    assert_eq!(cpu.registers().a, 0xBB, "A should now contain old B value");
    assert_eq!(cpu.registers().b, 0xAA, "B should now contain old A value");
}

// ======= SEX (Sign Extend) Tests =======

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

// ======= ABX (Add B to X) Tests =======

#[test]
fn test_abx_simple_addition() {
    // C++ Original: ABX - Add B register to X register (X = X + B, unsigned)
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().b = 0x42;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3A);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ABX should take 3 cycles");
    assert_eq!(cpu.registers().x, 0x1042, "X should be X + B");
    assert_eq!(cpu.registers().b, 0x42, "B should be unchanged");
    // ABX does not affect condition codes
}

#[test]
fn test_abx_with_overflow() {
    // C++ Original: ABX with 16-bit overflow
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().x = 0xFFFF;
    cpu.registers_mut().b = 0x01;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x3A);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ABX should take 3 cycles");
    assert_eq!(cpu.registers().x, 0x0000, "X should wrap around to 0x0000");
    // ABX does not set overflow flags
}

// ======= ANDCC Tests =======

#[test]
fn test_andcc_clear_interrupt_mask() {
    // C++ Original: ANDCC - AND immediate with CC register
    let mut cpu = create_test_cpu();
    
    // Set some flags initially
    cpu.registers_mut().cc.i = true;  // Interrupt mask  
    cpu.registers_mut().cc.c = true;  // Carry
    cpu.registers_mut().cc.n = true;  // Negative
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ANDCC #$EF (clear interrupt mask bit - bit 4)
    memory_bus.borrow_mut().write(RAM_START, 0x1C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xEF); // 11101111 - clears bit 4 (I flag)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    assert!(!cpu.registers().cc.i, "I flag should be cleared");
    assert!(cpu.registers().cc.c, "C flag should remain set");
    assert!(cpu.registers().cc.n, "N flag should remain set");
}

#[test]
fn test_andcc_clear_multiple_flags() {
    // C++ Original: ANDCC clearing multiple flags
    let mut cpu = create_test_cpu();
    
    // Set all flags
    cpu.registers_mut().cc.from_u8(0xFF);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ANDCC #$C0 - keep only E and F flags (bits 7,6)
    memory_bus.borrow_mut().write(RAM_START, 0x1C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC0);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    
    let cc_value = cpu.registers().cc.to_u8();
    assert_eq!(cc_value & 0x3F, 0x00, "Lower 6 bits should be cleared");
    assert_eq!(cc_value & 0xC0, 0xC0, "Upper 2 bits should remain set");
}

// ======= Stack Order Compliance Tests =======

#[test]
fn test_tfr_exg_register_encoding_compliance() {
    // C++ Original register encoding verification
    // 16-bit: D=0, X=1, Y=2, U=3, S=4, PC=5
    // 8-bit: A=8, B=9, CC=10, DP=11 (with bit 3 set in postbyte)
    
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test D(0) to Y(2): postbyte = 0x02
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().b = 0x22; // D = 0x1122
    cpu.registers_mut().y = 0x0000;
    
    memory_bus.borrow_mut().write(RAM_START, 0x1F); // TFR
    memory_bus.borrow_mut().write(RAM_START + 1, 0x02); // D -> Y
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().y, 0x1122, "TFR D,Y should work with encoding 0x02");
    
    // Test A(8) to DP(11): postbyte = 0x8B (bit 3 set for 8-bit)
    cpu.registers_mut().pc = RAM_START + 2;
    cpu.registers_mut().a = 0x33;
    cpu.registers_mut().dp = 0x00;
    
    memory_bus.borrow_mut().write(RAM_START + 2, 0x1F); // TFR
    memory_bus.borrow_mut().write(RAM_START + 3, 0x8B); // A -> DP (8-bit)
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().dp, 0x33, "TFR A,DP should work with 8-bit encoding");
}