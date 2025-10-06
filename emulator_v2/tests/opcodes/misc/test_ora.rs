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
fn test_ora_extended_addressing_modes() {
    // Test ORAA direct (0x9A)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().dp = 0xC8; // Set direct page register to 0xC8

    // Setup instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x9A); // ORAA direct opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x10); // direct address offset
    unsafe { &mut *memory.get() }.write(0xC810, 0x55); // data at 0xC800 + 0x10
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0xAA | 0x55 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);

    // Test ORAA extended (0xBA)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0x0F;

    // Setup extended addressing instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0xBA); // ORAA extended opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0xC9); // address high byte
    unsafe { &mut *memory.get() }.write(0xC802, 0x50); // address low byte
    unsafe { &mut *memory.get() }.write(0xC950, 0xF0); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x0F | 0xF0 = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);

    // Test ORAA indexed (placeholder - implementation depends on indexed addressing)
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0x33;
    cpu.registers_mut().x = 0xC900;

    // Setup indexed addressing instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0xAA); // ORAA indexed opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x84); // indexed postbyte (,X no offset)
    unsafe { &mut *memory.get() }.write(0xC900, 0xCC); // data at X
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x33 | 0xCC = 0xFF
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
}
