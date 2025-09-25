// C++ Original: Tests for CMPA extended (0xB1)
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
fn test_cmpa_extended_0xb1_equal() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $C890 with A=$AA, memory[$C890]=$AA (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0xAA;
    
    // Write CMPA $C890 instruction (extended addressing)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xB1); // CMPA extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8); // Address high byte
    cpu.memory_bus().borrow_mut().write(0xC802, 0x90); // Address low byte
    
    // Set memory value at extended address
    cpu.memory_bus().borrow_mut().write(0xC890, 0xAA); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0xAA);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes)
    assert_eq!(cycles, 5);                   // 5 cycles for extended mode
}

#[test]
fn test_cmpa_extended_0xb1_greater() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $C890 with A=$FF, memory[$C890]=$80 (A > memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0xFF;
    
    // Write CMPA $C890 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xB1); // CMPA extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8); // Address high byte
    cpu.memory_bus().borrow_mut().write(0xC802, 0x90); // Address low byte
    
    // Set memory value at extended address
    cpu.memory_bus().borrow_mut().write(0xC890, 0x80); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after greater comparison (A - mem = $FF - $80 = $7F > 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0xFF);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes)
    assert_eq!(cycles, 5);                   // 5 cycles for extended mode
}

#[test]
fn test_cmpa_extended_0xb1_less() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA $C890 with A=$10, memory[$C890]=$90 (A < memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x10;
    
    // Write CMPA $C890 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xB1); // CMPA extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8); // Address high byte
    cpu.memory_bus().borrow_mut().write(0xC802, 0x90); // Address low byte
    
    // Set memory value at extended address
    cpu.memory_bus().borrow_mut().write(0xC890, 0x90); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after less comparison (A - mem = $10 - $90 = $80 < 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (negative result)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow occurred)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x10);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced (3 bytes)
    assert_eq!(cycles, 5);                   // 5 cycles for extended mode
}