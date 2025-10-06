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
fn test_add_extended_addressing_modes() {
    // Test ADDA direct (0x9B) - already implemented, adding test for completeness
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0x7F;
    cpu.registers_mut().dp = 0xC8; // Set direct page to 0xC800

    // Setup instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x9B); // ADDA direct opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x50); // direct address offset
    unsafe { &mut *memory.get() }.write(0xC850, 0x01); // data at 0xC800 + 0x50
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x80); // 0x7F + 0x01 = 0x80
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, true); // Overflow: positive + positive = negative
    assert_eq!(cpu.registers().cc.c, false);

    // Test ADDA extended (0xBB) - already implemented
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().a = 0xFF;

    // Setup extended addressing instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0xBB); // ADDA extended opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0xC9); // address high byte
    unsafe { &mut *memory.get() }.write(0xC802, 0x00); // address low byte
    unsafe { &mut *memory.get() }.write(0xC900, 0x01); // data at extended address
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00); // 0xFF + 0x01 = 0x00 (wrap)
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true); // Carry flag set
}
