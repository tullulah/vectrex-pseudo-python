// C++ Original: Tests for CMPA opcodes (0x81, 0x91, 0xA1, 0xB1)
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
fn test_cmpa_immediate_0x81() {
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
fn test_cmpa_immediate_greater() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA #$30 with A=$50 (A > operand, result positive)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x50;

    // Write CMPA #$30 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x30); // Compare value

    cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after greater comparison (A - $30 = $20)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (positive result)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear (no borrow)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x50);     // A unchanged by compare
}

#[test]
fn test_cmpa_immediate_less() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA #$70 with A=$50 (A < operand, borrow needed)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x50;

    // Write CMPA #$70 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x70); // Compare value

    cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after less comparison (A - $70 needs borrow)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear (not equal)
    assert_eq!(cpu.registers().cc.n, true);  // Negative flag set (result $E0)
    assert_eq!(cpu.registers().cc.c, true);  // Carry flag set (borrow occurred)
    assert_eq!(cpu.registers().cc.v, false); // Overflow flag clear
    assert_eq!(cpu.registers().a, 0x50);     // A unchanged by compare
}

#[test]
fn test_cmpa_direct_0x91() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA $20 with A=$40, memory[$C820]=$40 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x40;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8

    // Write test data to memory
    cpu.memory_bus().borrow_mut().write(0xC820, 0x40); // Memory value to compare

    // Write CMPA direct instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x91); // CMPA direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x20); // Direct page address

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().a, 0x40);     // A unchanged
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for direct mode
}

#[test]
fn test_cmpa_extended_0xB1() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA $C820 with A=$80, memory[$C820]=$80 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x80;

    // Write test data to memory
    cpu.memory_bus().borrow_mut().write(0xC820, 0x80); // Memory value to compare

    // Write CMPA extended instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xB1); // CMPA extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8); // High byte of address
    cpu.memory_bus().borrow_mut().write(0xC802, 0x20); // Low byte of address

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear (result 0x00 is not negative)
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().a, 0x80);     // A unchanged
    assert_eq!(cpu.registers().pc, 0xC803);  // PC advanced
    assert_eq!(cycles, 5);                   // 5 cycles for extended mode
}

#[test]
fn test_cmpa_indexed_0xA1() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA ,X with A=$60, X=$C850, memory[$C850]=$60 (equal)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x60;
    cpu.registers_mut().x = 0xC850;

    // Write test data to memory
    cpu.memory_bus().borrow_mut().write(0xC850, 0x60); // Memory value to compare

    // Write CMPA indexed instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0xA1); // CMPA indexed
    cpu.memory_bus().borrow_mut().write(0xC801, 0x84); // ,X addressing mode

    let cycles = cpu.execute_instruction(false, false);

    // C++ Original: Verify flags after equal comparison
    assert_eq!(cpu.registers().cc.z, true);  // Zero flag set (equal)
    assert_eq!(cpu.registers().cc.n, false); // Negative flag clear
    assert_eq!(cpu.registers().cc.c, false); // Carry flag clear
    assert_eq!(cpu.registers().a, 0x60);     // A unchanged
    assert_eq!(cpu.registers().pc, 0xC802);  // PC advanced
    assert_eq!(cycles, 4);                   // 4 cycles for indexed mode
}

#[test]
fn test_cmpa_overflow_case() {
    let mut cpu = setup_cpu_with_memory();

    // C++ Original: CMPA with overflow condition (0x80 - 0x01 = 0x7F, pos - pos = pos but sign change)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x80; // -128 in signed

    // Write CMPA #$01 instruction
    cpu.memory_bus().borrow_mut().write(0xC800, 0x81); // CMPA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // Compare value

    cpu.execute_instruction(false, false);

    // C++ Original: Verify flags - subtract 0x80 - 0x01 = 0x7F (overflow occurs: -128 - 1 = -129, wraps to +127)
    assert_eq!(cpu.registers().cc.z, false); // Zero flag clear
    assert_eq!(cpu.registers().cc.n, false); // Result is positive (0x7F)
    assert_eq!(cpu.registers().cc.c, false); // No borrow needed
    assert_eq!(cpu.registers().cc.v, true);  // Overflow occurs (signed arithmetic overflow)
    assert_eq!(cpu.registers().a, 0x80);     // A unchanged
}