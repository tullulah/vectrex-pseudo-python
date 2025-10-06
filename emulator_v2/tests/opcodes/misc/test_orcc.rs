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
fn test_orcc_0x1a_preserve_existing() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: ORCC preserves existing flags
    cpu.registers_mut().pc = 0xC800;
    // Start with some flags already set
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.n = true;

    // Write ORCC #$04 (set Zero flag)
    cpu.memory_bus_mut().write(0xC800, 0x1A); // ORCC opcode
    cpu.memory_bus_mut().write(0xC801, 0x04); // Z=1

    cpu.execute_instruction(false, false).unwrap();

    // Verify new flag is set and existing flags preserved
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be preserved");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be preserved");
}

#[test]
fn test_orcc_0x1a_set_flags() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // C++ Original: ORCC with immediate operand
    cpu.registers_mut().pc = 0xC800;
    // Start with clear condition codes
    cpu.registers_mut().cc.from_u8(0x00);

    // Write ORCC #$54 (set Interrupt mask and Zero flag)
    cpu.memory_bus_mut().write(0xC800, 0x1A); // ORCC opcode
    cpu.memory_bus_mut().write(0xC801, 0x54); // I=1, Z=1

    cpu.execute_instruction(false, false).unwrap();

    // Verify flags are set
    assert_eq!(cpu.registers().cc.i, true, "I flag should be set");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.c, false, "C flag should remain clear");
    assert_eq!(cpu.registers().cc.n, false, "N flag should remain clear");
}
