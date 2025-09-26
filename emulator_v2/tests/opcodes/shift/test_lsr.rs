// LSR (Logical Shift Right) Opcode Tests  
// Tests for LSR opcodes: 0x04 (direct), 0x44 (register A), 0x54 (register B), 0x64 (indexed), 0x74 (extended)
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
fn test_lsr_register_a_0x44() {
    // C++ Original: OpLSR - Set C to bit 0, Z to result, N to 0
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0b1100_1010; // 0xCA
    cpu.registers_mut().cc.c = false;

    // Write opcode LSR A (0x44) at RAM location
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x44);
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "LSR A should take 2 cycles");
    assert_eq!(cpu.registers().a, 0b0110_0101);  // 0x65 - Shifted right
    assert_eq!(cpu.registers().cc.c, false);     // Original bit 0 was 0
    assert_eq!(cpu.registers().cc.z, false);     // Result not zero
    assert_eq!(cpu.registers().cc.n, false);     // N always 0 for LSR
}

#[test]
fn test_lsr_register_a_with_carry() {
    // C++ Original: LSR sets carry to original bit 0
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0b1100_1011; // 0xCB - bit 0 = 1
    cpu.registers_mut().cc.c = false;

    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x44); // LSR A
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "LSR A should take 2 cycles");
    assert_eq!(cpu.registers().a, 0b0110_0101);  // 0x65 - Shifted right
    assert_eq!(cpu.registers().cc.c, true);      // Original bit 0 was 1
    assert_eq!(cpu.registers().cc.z, false);     // Result not zero
    assert_eq!(cpu.registers().cc.n, false);     // N always 0 for LSR
}

#[test]
fn test_lsr_register_a_zero_result() {
    // C++ Original: LSR producing zero result sets Z flag
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0b0000_0001; // 0x01 - will become 0
    cpu.registers_mut().cc.c = false;

    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x44); // LSR A
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 2, "LSR A should take 2 cycles");
    assert_eq!(cpu.registers().a, 0x00);         // Result is zero
    assert_eq!(cpu.registers().cc.c, true);      // Original bit 0 was 1
    assert_eq!(cpu.registers().cc.z, true);      // Zero flag set
    assert_eq!(cpu.registers().cc.n, false);     // N always 0 for LSR
}