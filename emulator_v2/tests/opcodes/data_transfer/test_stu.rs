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
fn test_stu_extended_0xff() {
    // Test STU (Store U register) extended addressing
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup U register
    cpu.registers_mut().u = 0xDEAD;

    // Set up memory: STU extended
    unsafe { &mut *memory.get() }.write(0xC820, 0xFF); // STU extended
    unsafe { &mut *memory.get() }.write(0xC821, 0xCB); // Extended address high byte (0xCB00)
    unsafe { &mut *memory.get() }.write(0xC822, 0x00); // Extended address low byte

    cpu.registers_mut().pc = 0xC820;

    // Execute STU extended
    cpu.execute_instruction(false, false).unwrap();

    // Verify U register value was stored at extended address (0xCB00)
    assert_eq!(
        unsafe { &*memory.get() }.read(0xCB00),
        0xDE,
        "High byte of U should be stored"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(0xCB01),
        0xAD,
        "Low byte of U should be stored"
    );

    // Verify PC advanced correctly
    assert_eq!(
        cpu.registers().pc,
        0xC823,
        "PC should advance by 3 bytes for extended"
    );

    // Verify condition codes
    assert_eq!(
        cpu.registers().cc.n,
        true,
        "N flag should be set for negative value"
    );
    assert_eq!(
        cpu.registers().cc.z,
        false,
        "Z flag should be clear for non-zero value"
    );
}
