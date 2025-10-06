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
fn test_sub_extended_addressing_modes() {
    // Test SUBA direct (0x90) - already implemented, adding test for completeness
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0x80;
    cpu.registers_mut().dp = 0xC8; // Set direct page register to 0xC8

    // Setup instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x90); // SUBA direct opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x60); // direct address offset
    unsafe { &mut *memory.get() }.write(0xC860, 0x01); // data at 0xC800 + 0x60
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x7F); // 0x80 - 0x01 = 0x7F
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: negative - positive = positive
    assert_eq!(cpu.registers().cc.c, false);

    // Test SUBA extended (0xB0) - already implemented
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0x00;

    // Setup extended addressing instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0xB0); // SUBA extended opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0xC9); // address high byte
    unsafe { &mut *memory.get() }.write(0xC802, 0x00); // address low byte
    unsafe { &mut *memory.get() }.write(0xC900, 0x01); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xFF); // 0x00 - 0x01 = 0xFF (borrow)
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set (borrow occurred)
}
