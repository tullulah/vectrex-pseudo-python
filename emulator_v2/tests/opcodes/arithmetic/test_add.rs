// Tests para opcodes ADD y SUB - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x80, 0x8B, 0xC0, 0xCB

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_add_sub_comprehensive_flags() {
    // Test various flag combinations with ADD/SUB
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Test 1: ADD negative result
    cpu.registers_mut().a = 0x70; // 112
    cpu.memory_bus_mut().write(RAM_START, 0x8B); // ADDA immediate
    cpu.memory_bus_mut().write(RAM_START + 1, 0x20); // Add 32 = 144 = 0x90 (negative)
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x90, "A should be 0x90");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");

    // Test 2: SUB overflow: negative - positive = positive (should set V)
    cpu.registers_mut().b = 0x80; // -128 in signed
    cpu.memory_bus_mut().write(RAM_START + 0x10, 0xC0); // SUBB immediate
    cpu.memory_bus_mut().write(RAM_START + 0x11, 0x01); // Subtract 1
    cpu.registers_mut().pc = RAM_START + 0x10;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().b, 0x7F, "B should be 0x7F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(
        cpu.registers().cc.v,
        true,
        "V flag should be set (overflow)"
    );
}
