// CWAI (Clear and Wait for Interrupt) Opcode 0x3C Tests
// Tests for Clear CC bits and Wait for Interrupt
// Following Vectrexy 1:1 compliance rules

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use crate::interrupt_vector_rom::InterruptVectorRom;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // C++ Original: Map interrupt vectors (0xFFF0-0xFFFF) for SWI/CWAI tests
    let vector_rom = Rc::new(RefCell::new(InterruptVectorRom::new()));
    memory_bus.borrow_mut().connect_device(vector_rom, (0xFFF0, 0xFFFF), vectrex_emulator_v2::core::memory_bus::EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().pc = RAM_START;
    cpu
}

#[test]
fn test_cwai_basic_cc_masking() {
    // C++ Original: CWAI - Clear bits in CC register and wait for interrupt
    let mut cpu = create_test_cpu();
    
    // Set all flags initially
    cpu.registers_mut().cc.from_u8(0xFF);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // CWAI #$EF (clear interrupt mask bit 4, keep others)
    memory_bus.borrow_mut().write(RAM_START, 0x3C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xEF); // 11101111 - clears bit 4 (I flag)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "CWAI should take 20 cycles");
    assert!(!cpu.registers().cc.i, "I flag should be cleared");
    assert!(cpu.registers().cc.c, "C flag should remain set");
    assert!(cpu.registers().cc.n, "N flag should remain set");
    assert!(cpu.registers().cc.e, "E flag should be set by CWAI");
}

#[test]
fn test_cwai_clear_multiple_flags() {
    // C++ Original: CWAI clearing multiple flags
    let mut cpu = create_test_cpu();
    
    // Set all flags
    cpu.registers_mut().cc.from_u8(0xFF);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // CWAI #$C0 - keep only E and F flags (bits 7,6)
    memory_bus.borrow_mut().write(RAM_START, 0x3C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC0);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "CWAI should take 20 cycles");
    
    let cc_value = cpu.registers().cc.to_u8();
    assert_eq!(cc_value & 0x3F, 0x00, "Lower 6 bits should be cleared");
    assert!(cpu.registers().cc.e, "E flag should be set by CWAI");
    assert!(cpu.registers().cc.f, "F flag should remain set");
}

#[test]
fn test_cwai_waiting_state() {
    // C++ Original: CWAI sets CPU into waiting state
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().cc.from_u8(0x00);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // CWAI #$FF (preserve all flags, just enter wait state)
    memory_bus.borrow_mut().write(RAM_START, 0x3C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xFF);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 20, "CWAI should take 20 cycles");
    assert!(cpu.registers().cc.e, "E flag should be set by CWAI");
    // Note: actual waiting state verification would require interrupt handling
}