// C++ Original: Tests for CMPA indexed (0xA1)
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
fn test_cmpa_indexed_0xa1_equal() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA ,X with A=$33, X=$C890, memory[$C890]=$33 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x33;
    cpu.registers_mut().x = 0xC890;
    
    // Write CMPA ,X instruction (indexed, no offset)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xA1); // CMPA indexed
    cpu.memory_bus().borrow_mut().write(0xC801, 0x84); // Postbyte: no offset, X register
    
    // Set memory value at indexed address
    cpu.memory_bus().borrow_mut().write(0xC890, 0x33); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x33);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for indexed mode
}

#[test]
fn test_cmpa_indexed_0xa1_offset() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA 5,X with A=$77, X=$C890, memory[$C895]=$44 (A > memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x77;
    cpu.registers_mut().x = 0xC890;
    
    // Write CMPA 5,X instruction (indexed with 5-bit offset)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xA1); // CMPA indexed
    cpu.memory_bus().borrow_mut().write(0xC801, 0x05); // Postbyte: +5 offset, X register
    
    // Set memory value at indexed address (X + 5)
    cpu.memory_bus().borrow_mut().write(0xC895, 0x44); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after greater comparison (A - mem = $77 - $44 = $33 > 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x77);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for indexed mode with 5-bit offset
}

#[test]
fn test_cmpa_indexed_0xa1_with_y() {
    let mut cpu = setup_cpu_with_memory();
    
    // C++ Original: CMPA ,Y with A=$88, Y=$C8A0, memory[$C8A0]=$99 (A < memory)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x88;
    cpu.registers_mut().y = 0xC8A0;
    
    // Write CMPA ,Y instruction (indexed, no offset, Y register)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xA1); // CMPA indexed
    cpu.memory_bus().borrow_mut().write(0xC801, 0xA4); // Postbyte: no offset, Y register
    
    // Set memory value at indexed address
    cpu.memory_bus().borrow_mut().write(0xC8A0, 0x99); // Memory value to compare
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Verify flags after less comparison (A - mem = $88 - $99 = $EF < 0)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (negative result)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow occurred)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x88);     // A unchanged by compare
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for indexed mode
}