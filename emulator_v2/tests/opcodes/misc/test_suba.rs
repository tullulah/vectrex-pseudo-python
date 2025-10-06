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
fn test_suba_immediate() {
    // C++ Original: SUBA #immediate - Subtract immediate value from A
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0x80); // SUBA #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x05); // immediate value

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x15; // Initial value

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x10); // 0x15 - 0x05 = 0x10
    assert!(!cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
    assert!(!cpu.registers().cc.v);
    assert!(!cpu.registers().cc.c);
}
