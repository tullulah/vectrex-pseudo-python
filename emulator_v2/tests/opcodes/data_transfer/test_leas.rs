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
fn test_leas_indexed_basic() {
    // C++ Original: LEAS - Load Effective Address into S (indexed) - opcode 0x32
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let (mut cpu, memory) = setup_cpu_with_ram();

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x3000; // Base register for indexed addressing
    cpu.registers_mut().s = 0x0000; // Clear S initially
    cpu.registers_mut().cc.z = true; // Set Z flag initially to verify it's not affected

    unsafe { &mut *memory.get() }.write(0xC800, 0x32); // LEAS indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0xC4); // ,U (no offset)

    cpu.execute_instruction(false, false).unwrap();

    // Verify S contains the effective address
    assert_eq!(cpu.registers().s, 0x3000);

    // Verify Z flag is NOT affected by LEAS - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(cpu.registers().cc.z); // Should remain true

    assert_eq!(cpu.registers().pc, 0xC802);
}
