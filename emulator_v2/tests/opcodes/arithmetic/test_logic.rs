//! Tests for ANDB, EORB, ORAB opcodes (B register logic operations)
//! MC6809 Programming Manual opcodes:
//! - ANDB: 0xC4 (immediate), 0xD4 (direct), 0xE4 (indexed), 0xF4 (extended)
//! - EORB: 0xC8 (immediate), 0xD8 (direct), 0xE8 (indexed), 0xF8 (extended)
//! - ORAB: 0xCA (immediate), 0xDA (direct), 0xEA (indexed), 0xFA (extended)

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_logic_ops_clear_overflow() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Force overflow flag set first
    cpu.registers_mut().b = 0x7F;
    cpu.registers_mut().cc.v = true; // Manually set overflow
    cpu.registers_mut().x = RAM_START + 0x50;
    cpu.memory_bus_mut().write(RAM_START + 0x50, 0x01);

    // Execute ANDB - should clear overflow flag
    cpu.memory_bus_mut().write(RAM_START, 0xE4);
    cpu.memory_bus_mut().write(RAM_START + 1, 0x84);

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().cc.v,
        false,
        "Overflow flag should be cleared by logic operation"
    );
}
