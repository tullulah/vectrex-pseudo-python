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
fn test_com_a_basic() {
    // COMA - Complement A register (inherent) - opcode 0x43
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set A to a test value
    cpu.registers_mut().a = 0x42; // 01000010 in binary
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = false;

    // Place COMA instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x43); // COMA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Verify A register is complemented: 0x42 -> 0xBD (bitwise NOT)
    assert_eq!(cpu.registers().a, 0xBD); // 10111101 in binary

    // Verify flags - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1;
    assert!(!cpu.registers().cc.z); // Z=0 (0xBD is not zero)
    assert!(cpu.registers().cc.n); // N=1 (0xBD bit 7 = 1, so negative)
    assert!(!cpu.registers().cc.v); // V=0 (always cleared by COM)
    assert!(cpu.registers().cc.c); // C=1 (always set by COM) - 1:1 Vectrexy behavior

    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);

    // Verify cycle count - COMA should be 2 cycles
}
