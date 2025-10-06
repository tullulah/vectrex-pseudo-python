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
fn test_leau_indexed_basic() {
    // C++ Original: LEAU - Load Effective Address into U (indexed) - opcode 0x33
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let (mut cpu, memory) = setup_cpu_with_ram();

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0x4000; // Base register for indexed addressing
    cpu.registers_mut().u = 0x0000; // Clear U initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially to verify it's not affected

    unsafe { &mut *memory.get() }.write(0xC800, 0x33); // LEAU indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0xE4); // ,S (no offset)

    cpu.execute_instruction(false, false).unwrap();

    // Verify U contains the effective address
    assert_eq!(cpu.registers().u, 0x4000);

    // Verify Z flag is NOT affected by LEAU - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(!cpu.registers().cc.z); // Should remain false

    assert_eq!(cpu.registers().pc, 0xC802);
}
