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
fn test_tst_zero_and_negative() {
    // TSTA with zero value - Test 1:1 Vectrexy behavior
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Test zero case
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false; // Will verify TST doesn't change this

    unsafe { &mut *memory.get() }.write(0xC800, 0x4D); // TSTA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0;
    assert!(cpu.registers().cc.z); // Z=1 (value is zero)
    assert!(!cpu.registers().cc.n); // N=0 (zero is not negative)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared)
    assert!(!cpu.registers().cc.c); // C unchanged - 1:1 Vectrexy behavior

    // Test negative case
    cpu.registers_mut().a = 0x80; // Negative value
    cpu.registers_mut().cc.c = true; // Different carry to verify preservation
    cpu.registers_mut().pc = 0xC800; // Reset PC

    cpu.execute_instruction(false, false).unwrap();

    assert!(!cpu.registers().cc.z); // Z=0 (0x80 is not zero)
    assert!(cpu.registers().cc.n); // N=1 (0x80 is negative)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared)
    assert!(cpu.registers().cc.c); // C unchanged - 1:1 Vectrexy behavior
}
