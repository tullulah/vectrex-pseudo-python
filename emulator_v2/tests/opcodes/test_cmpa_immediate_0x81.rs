// C++ Original: Tests for CMPA immediate (0x81)
// Port 1:1 from Vectrexy test patterns

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn setup_cpu_with_memory() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_cmpa_immediate_0x81_equal() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA #$50 with A=$50 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x50;
    
    // Write CMPA #$50 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x50); // Compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison (A - $50 = 0)
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x50);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 2);                   // 2 cycles for immediate mode
}

#[test]
fn test_cmpa_immediate_0x81_greater() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA #$30 with A=$50 (A > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x50;
    
    // Write CMPA #$30 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x30); // Compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after greater comparison (A - $30 = $20 > 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x50);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 2);                   // 2 cycles for immediate mode
}

#[test]
fn test_cmpa_immediate_0x81_less() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA #$70 with A=$50 (A < operand, result negative)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x50;
    
    // Write CMPA #$70 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x70); // Compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after less comparison (A - $70 = $E0 < 0, signed)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (negative result)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow occurred)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x50);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 2);                   // 2 cycles for immediate mode
}

#[test]
fn test_cmpa_immediate_0x81_overflow() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA with signed overflow case (0x80 - 0x01 = 0x7F, overflow)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x80; // -128 in signed
    
    // Write CMPA #$01 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // Compare value
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify overflow flag after signed overflow
    // 0x80 - 0x01 = 0x7F: -128 - 1 = 127 (signed overflow from negative to positive)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no unsigned borrow)
    assert_eq!(cpu.registers().cc.v, true);  // Overflow flag set (signed overflow)
    assert_eq!(cpu.registers().a, 0x80);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 2);                   // 2 cycles for immediate mode
}