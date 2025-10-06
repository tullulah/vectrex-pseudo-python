// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_orab_immediate_basic() {
    // Test ORAB #$AA - OR B with immediate value
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Initial state
    cpu.registers_mut().b = 0x55; // Initial B = 01010101

    // Setup: ORAB #$AA instruction
    cpu.memory_bus_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    cpu.memory_bus_mut().write(0xC801, 0xAA); // OR mask 10101010
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().b, 0xFF, "B should be 0x55 | 0xAA = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}
