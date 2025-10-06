use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_andcc_0x1c_clear_all() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: ANDCC can clear all flags
    cpu.registers_mut().pc = 0xC800;
    // Start with all flags set
    cpu.registers_mut().cc.from_u8(0xFF);

    // Write ANDCC #$00 (clear all flags)
    cpu.memory_bus_mut().write(0xC800, 0x1C); // ANDCC opcode
    cpu.memory_bus_mut().write(0xC801, 0x00); // Clear all

    cpu.execute_instruction(false, false).unwrap();

    // Verify all flags are cleared
    assert_eq!(cpu.registers().cc.c, false, "C flag should be cleared");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be cleared");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be cleared");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be cleared");
    assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
    assert_eq!(cpu.registers().cc.h, false, "H flag should be cleared");
    assert_eq!(cpu.registers().cc.f, false, "F flag should be cleared");
    assert_eq!(cpu.registers().cc.e, false, "E flag should be cleared");
}

#[test]
fn test_andcc_0x1c_clear_flags() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: ANDCC clears specified flags
    cpu.registers_mut().pc = 0xC800;
    // Start with all flags set
    cpu.registers_mut().cc.from_u8(0xFF);

    // Write ANDCC #$AB (clear Interrupt and Zero flags: ~0x54)
    cpu.memory_bus_mut().write(0xC800, 0x1C); // ANDCC opcode
    cpu.memory_bus_mut().write(0xC801, 0xAB); // Clear I and Z

    cpu.execute_instruction(false, false).unwrap();

    // Verify specified flags are cleared
    assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be cleared");

    // Verify other flags remain set
    assert_eq!(cpu.registers().cc.c, true, "C flag should remain set");
    assert_eq!(cpu.registers().cc.n, true, "N flag should remain set");
    assert_eq!(cpu.registers().cc.v, true, "V flag should remain set");
}

#[test]
fn test_andcc_0x1c_preserve_flags() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: ANDCC preserves specified flags
    cpu.registers_mut().pc = 0xC800;
    // Start with specific flags set
    cpu.registers_mut().cc.from_u8(0x00);
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = true;

    // Write ANDCC #$05 (preserve only C and Z flags)
    cpu.memory_bus_mut().write(0xC800, 0x1C); // ANDCC opcode
    cpu.memory_bus_mut().write(0xC801, 0x05); // Keep C and Z

    cpu.execute_instruction(false, false).unwrap();

    // Verify only specified flags remain
    assert_eq!(cpu.registers().cc.c, true, "C flag should be preserved");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be preserved");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be cleared");
    assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
}
