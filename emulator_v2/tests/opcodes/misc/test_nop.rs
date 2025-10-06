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
fn test_nop_minimal() {
    // C++ Original: NOP - does nothing but consume cycles
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Place NOP instruction in RAM area (0xC800+)
    unsafe { &mut *memory.get() }.write(0xC800, 0x12); // NOP

    // Set PC to start of instruction
    cpu.registers_mut().pc = 0xC800;
    let initial_pc = cpu.registers().pc;

    // Execute one instruction
    cpu.execute_instruction(false, false).unwrap();

    // Verify results
    assert_eq!(cpu.registers().pc, initial_pc + 1); // NOP is 2 cycles
}
