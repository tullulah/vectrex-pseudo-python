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
fn test_and_extended_addressing_modes() {
    // Test ANDA direct (0x94)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().dp = 0xC8; // Set direct page to 0xC800

    // Setup instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x94); // ANDA direct opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x20); // direct address offset
    unsafe { &mut *memory.get() }.write(0xC820, 0xAA); // data at 0xC800 + 0x20
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xAA); // 0xFF & 0xAA = 0xAA
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);

    // Test ANDA extended (0xB4)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0xCC;

    // Setup extended addressing instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0xB4); // ANDA extended opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0xC9); // address high byte
    unsafe { &mut *memory.get() }.write(0xC802, 0x00); // address low byte
    unsafe { &mut *memory.get() }.write(0xC900, 0x88); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x88); // 0xCC & 0x88 = 0x88
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ANDA with zero result
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().dp = 0xC8; // Set direct page to 0xC800

    // Setup instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x94); // ANDA direct opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x30); // direct address offset
    unsafe { &mut *memory.get() }.write(0xC830, 0x55); // data at 0xC800 + 0x30
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00); // 0xAA & 0x55 = 0x00
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
}
