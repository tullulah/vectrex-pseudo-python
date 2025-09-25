// C++ Original: Tests for CMPA direct (0x91)
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
fn test_cmpa_direct_0x91_equal() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $80 with A=$42, memory[$80]=$42 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8xx
    
    // Write CMPA $80 instruction (direct page)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x91); // CMPA direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x80); // Direct address offset
    
    // Set memory value at direct page address
    cpu.memory_bus().borrow_mut().write(0xC880, 0x42); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x42);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for direct mode
}

#[test]
fn test_cmpa_direct_0x91_greater() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $80 with A=$60, memory[$80]=$40 (A > memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x60;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8xx
    
    // Write CMPA $80 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x91); // CMPA direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x80); // Direct address offset
    
    // Set memory value at direct page address
    cpu.memory_bus().borrow_mut().write(0xC880, 0x40); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after greater comparison (A - mem = $60 - $40 = $20 > 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x60);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for direct mode
}

#[test]
fn test_cmpa_direct_0x91_less() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $80 with A=$30, memory[$80]=$50 (A < memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8xx
    
    // Write CMPA $80 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x91); // CMPA direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x80); // Direct address offset
    
    // Set memory value at direct page address
    cpu.memory_bus().borrow_mut().write(0xC880, 0x50); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after less comparison (A - mem = $30 - $50 = $E0 < 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (negative result)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow occurred)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x30);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for direct mode
}