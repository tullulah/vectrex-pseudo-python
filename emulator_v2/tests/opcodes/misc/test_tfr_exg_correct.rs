//! Tests for TFR/EXG (Transfer/Exchange) opcodes  
//! Port-1:1 tests based on Vectrexy CPU implementation
//! 
//! C++ Original: ExchangeOrTransfer function in Cpu.cpp lines 803-826
//! TFR: dst = src (0x1F)
//! EXG: swap(dst, src) (0x1E)

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
fn test_tfr_a_to_b_0x1f() {
    // C++ Original: TFR A,B with postbyte 0x89 (A=8+bit3, B=9+bit3 for 8-bit)
    let mut cpu = create_test_cpu();
    
    // Setup initial state - A=0x42, B=0x00
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x00;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode  
    memory_bus.borrow_mut().write(0xC801, 0x89); // TFR A,B: src=0(A)<<4 | dst=1(B) | 0x88 = 0x01 | 0x88 = 0x89
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: B should now equal A, A unchanged
    assert_eq!(cpu.registers().a, 0x42, "A register should be unchanged");
    assert_eq!(cpu.registers().b, 0x42, "B register should receive A value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 6, "TFR should take 6 cycles"); // From MC6809 documentation
}

#[test]
fn test_tfr_x_to_d_0x1f() {
    // C++ Original: TFR X,D with postbyte 0x10 (X=1, D=0 for 16-bit)
    let mut cpu = create_test_cpu();
    
    // Setup initial state - X=0x1234, D=0x0000
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().a = 0x00; // D = A:B combined
    cpu.registers_mut().b = 0x00;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x10); // X=1, D=0 (16-bit transfer)
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: D should now equal X, X unchanged
    assert_eq!(cpu.registers().x, 0x1234, "X register should be unchanged");
    assert_eq!(cpu.registers().d(), 0x1234, "D register should receive X value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
}

#[test]
fn test_exg_a_b_0x1e() {
    // C++ Original: EXG A,B with postbyte 0x89 (swap A and B)
    let mut cpu = create_test_cpu();
    
    // Setup initial state - A=0x42, B=0x33
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x33;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x89); // EXG A,B: src=0(A)<<4 | dst=1(B) | 0x88 = 0x01 | 0x88 = 0x89
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: A and B values should be swapped
    assert_eq!(cpu.registers().a, 0x33, "A register should contain original B value");
    assert_eq!(cpu.registers().b, 0x42, "B register should contain original A value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 8, "EXG should take 8 cycles"); // From MC6809 documentation
}

#[test]
fn test_exg_x_y_0x1e() {
    // C++ Original: EXG X,Y with postbyte 0x12 (swap X and Y)
    let mut cpu = create_test_cpu();
    
    // Setup initial state - X=0x1234, Y=0x5678
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().y = 0x5678;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1E); // EXG opcode
    memory_bus.borrow_mut().write(0xC801, 0x12); // X=1, Y=2 (16-bit exchange)
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: X and Y values should be swapped
    assert_eq!(cpu.registers().x, 0x5678, "X register should contain original Y value");
    assert_eq!(cpu.registers().y, 0x1234, "Y register should contain original X value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 8, "EXG should take 8 cycles");
}

#[test]
fn test_tfr_d_to_x_0x1f() {
    // C++ Original: TFR D,X with postbyte 0x01 (D=0, X=1 for 16-bit)
    let mut cpu = create_test_cpu();
    
    // Setup initial state - D=0xABCD, X=0x0000
    cpu.registers_mut().a = 0xAB; // D = A:B combined = 0xABCD
    cpu.registers_mut().b = 0xCD;
    cpu.registers_mut().x = 0x0000;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x1F); // TFR opcode
    memory_bus.borrow_mut().write(0xC801, 0x01); // D=0, X=1 (16-bit transfer)
    
    cpu.registers_mut().pc = 0xC800;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: X should now equal D, D unchanged
    assert_eq!(cpu.registers().d(), 0xABCD, "D register should be unchanged");
    assert_eq!(cpu.registers().x, 0xABCD, "X register should receive D value");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance past instruction");
    assert_eq!(cycles, 6, "TFR should take 6 cycles");
}