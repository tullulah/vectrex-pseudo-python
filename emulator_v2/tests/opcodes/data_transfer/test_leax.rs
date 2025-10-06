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
fn test_leax_indexed_basic() {
    // C++ Original: LEAX - Load Effective Address into X (indexed) - opcode 0x30
    // C++ Original: reg = EA; if (&reg == &X || &reg == &Y) { CC.Zero = (reg == 0); }
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set initial state
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x1000; // Base register for indexed addressing
    cpu.registers_mut().x = 0x0000; // Clear X initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially

    // Place LEAX ,Y instruction (indexed with no offset)
    unsafe { &mut *memory.get() }.write(0xC800, 0x30); // LEAX indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0xA4); // ,Y (no offset)

    cpu.execute_instruction(false, false).unwrap();

    // Verify X contains the effective address (Y register value)
    assert_eq!(cpu.registers().x, 0x1000);

    // Verify Z flag is cleared (X is non-zero) - C++ Original: Z flag affected by LEAX/LEAY
    assert!(!cpu.registers().cc.z);

    // Verify PC advanced correctly (2 bytes: opcode + postbyte)
    assert_eq!(cpu.registers().pc, 0xC802);

    // Verify cycle count - C++ Original: LEAX has 4 cycles
}

#[test]
fn test_leax_indexed_with_offset() {
    // C++ Original: LEAX with 8-bit offset - tests ReadIndexedEA calculation
    let (mut cpu, memory) = setup_cpu_with_ram();

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x1000; // Base register
    cpu.registers_mut().cc.z = true; // Set Z initially to verify it changes

    unsafe { &mut *memory.get() }.write(0xC800, 0x30); // LEAX indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0x88); // 8-bit offset,X
    unsafe { &mut *memory.get() }.write(0xC802, 0x10); // offset = +16

    cpu.execute_instruction(false, false).unwrap();

    // Verify X contains base + offset: 0x1000 + 0x10 = 0x1010
    assert_eq!(cpu.registers().x, 0x1010);

    // Verify Z flag is cleared (result is non-zero)
    assert!(!cpu.registers().cc.z);

    assert_eq!(cpu.registers().pc, 0xC803); // 3 bytes total
}

#[test]
fn test_leax_indexed_zero_result() {
    // C++ Original: LEAX with zero result should set Z flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x0000; // Base register with zero value
    cpu.registers_mut().x = 0x1234; // X has non-zero value initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially

    unsafe { &mut *memory.get() }.write(0xC800, 0x30); // LEAX indexed
    unsafe { &mut *memory.get() }.write(0xC801, 0xA4); // ,Y (no offset)

    cpu.execute_instruction(false, false).unwrap();

    // Verify X contains zero
    assert_eq!(cpu.registers().x, 0x0000);

    // Verify Z flag is set (X is zero) - C++ Original: CC.Zero = (reg == 0)
    assert!(cpu.registers().cc.z);

    assert_eq!(cpu.registers().pc, 0xC802);
}
