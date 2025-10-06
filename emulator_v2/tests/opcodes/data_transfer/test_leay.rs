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
fn test_leay_indexed_basic() {
    // C++ Original: LEAY - Load Effective Address into Y (indexed) - opcode 0x31
    let (mut cpu, memory) = setup_cpu_with_ram();

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x2000; // Base register for indexed addressing
    cpu.registers_mut().y = 0x0000; // Clear Y initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially

    unsafe { &mut *memory.get() }.write(0xC800, 0x31); // LEAY indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0x84); // ,X (no offset)

    cpu.execute_instruction(false, false).unwrap();

    // Verify Y contains the effective address
    assert_eq!(cpu.registers().y, 0x2000);

    // Verify Z flag is cleared (Y is non-zero) - C++ Original: Z flag affected by LEAX/LEAY
    assert!(!cpu.registers().cc.z);

    assert_eq!(cpu.registers().pc, 0xC802);
}
