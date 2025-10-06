// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_ldaa_immediate_negative() {
    // Test LDAA #$80 - Load A with negative value (sets N flag)
    // Note: This test verifies that existing LD opcodes work correctly
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.memory_bus_mut().write(0xC800, 0x86); // LDAA immediate
    cpu.memory_bus_mut().write(0xC801, 0x80); // Negative value (bit 7 set)
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(
        cpu.registers().cc.n,
        true,
        "N flag should be set (bit 7 = 1)"
    );
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_ldaa_immediate_zero() {
    // Test LDAA #$00 - Load A with zero (sets Z flag)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.memory_bus_mut().write(0xC800, 0x86); // LDAA immediate
    cpu.memory_bus_mut().write(0xC801, 0x00); // Zero value
    cpu.registers_mut().pc = 0xC800;

    cpu.registers_mut().a = 0xFF; // Set A to non-zero initially

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(
        cpu.registers().cc.z,
        true,
        "Z flag should be set (value = 0)"
    );
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}
