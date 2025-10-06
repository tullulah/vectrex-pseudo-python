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
fn test_clra_minimal() {
    // C++ Original: CLRA - Clear A register
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Place CLRA instruction in RAM
    unsafe { &mut *memory.get() }.write(0xC800, 0x4F); // CLRA

    // Set initial state
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x42; // Non-zero value

    // Execute instruction
    cpu.execute_instruction(false, false).unwrap();

    // Verify results
    assert_eq!(cpu.registers().a, 0x00); // CLRA is 2 cycles
    assert!(cpu.registers().cc.z); // Zero flag should be set
    assert!(!cpu.registers().cc.n); // Negative flag should be clear
    assert!(!cpu.registers().cc.v); // Overflow flag should be clear
}
