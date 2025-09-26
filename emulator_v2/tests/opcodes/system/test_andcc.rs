// ANDCC (AND Condition Code) Opcode 0x1C Tests
// Tests for AND immediate with Condition Code register
// Following Vectrexy 1:1 compliance rules

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().pc = RAM_START;
    cpu
}

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

#[test]
fn test_andcc_preserve_flags() {
    // C++ Original: ANDCC with mask that preserves all flags
    let mut cpu = create_test_cpu();
    
    // Set specific flags
    cpu.registers_mut().cc.z = true;  // Zero
    cpu.registers_mut().cc.v = true;  // Overflow
    cpu.registers_mut().cc.f = false; // FIRQ mask off
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ANDCC #$FF (preserve all flags)
    memory_bus.borrow_mut().write(RAM_START, 0x1C);
    memory_bus.borrow_mut().write(RAM_START + 1, 0xFF); // 11111111 - preserves all
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    assert!(cpu.registers().cc.z, "Z flag should remain set");
    assert!(cpu.registers().cc.v, "V flag should remain set");
    assert!(!cpu.registers().cc.f, "F flag should remain cleared");
}