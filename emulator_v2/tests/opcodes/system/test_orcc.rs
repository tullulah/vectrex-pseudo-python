// ORCC (OR Condition Code) Opcode 0x1A Tests
// Tests for OR immediate with Condition Code register
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
fn test_orcc_set_interrupt_mask() {
    // C++ Original: ORCC - OR immediate with CC register
    let mut cpu = create_test_cpu();
    
    // Clear all flags initially
    cpu.registers_mut().cc.from_u8(0x00);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ORCC #$10 (set interrupt mask bit - bit 4)
    memory_bus.borrow_mut().write(RAM_START, 0x1A);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x10); // 00010000 - sets bit 4 (I flag)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ORCC should take 3 cycles");
    assert!(cpu.registers().cc.i, "I flag should be set");
    assert!(!cpu.registers().cc.c, "C flag should remain clear");
    assert!(!cpu.registers().cc.n, "N flag should remain clear");
}

#[test]
fn test_orcc_set_multiple_flags() {
    // C++ Original: ORCC setting multiple flags
    let mut cpu = create_test_cpu();
    
    // Start with some flags set
    cpu.registers_mut().cc.c = true;  // Carry already set
    cpu.registers_mut().cc.z = false; // Zero clear
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ORCC #$0A - set Z flag (bit 2) and V flag (bit 1) 
    memory_bus.borrow_mut().write(RAM_START, 0x1A);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x06); // 00000110 - sets Z and V
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ORCC should take 3 cycles");
    assert!(cpu.registers().cc.c, "C flag should remain set");
    assert!(cpu.registers().cc.z, "Z flag should be set by ORCC");
    assert!(cpu.registers().cc.v, "V flag should be set by ORCC");
}

#[test]
fn test_orcc_preserve_existing_flags() {
    // C++ Original: ORCC with mask that doesn't affect existing flags
    let mut cpu = create_test_cpu();
    
    // Set specific flags
    cpu.registers_mut().cc.n = true;  // Negative set
    cpu.registers_mut().cc.f = true;  // FIRQ mask set
    
    let memory_bus = cpu.memory_bus().clone();
    
    // ORCC #$00 (OR with zero - should preserve all flags)
    memory_bus.borrow_mut().write(RAM_START, 0x1A);
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00); // 00000000 - preserves all
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 3, "ORCC should take 3 cycles");
    assert!(cpu.registers().cc.n, "N flag should remain set");
    assert!(cpu.registers().cc.f, "F flag should remain set");
    assert!(!cpu.registers().cc.z, "Z flag should remain clear");
}