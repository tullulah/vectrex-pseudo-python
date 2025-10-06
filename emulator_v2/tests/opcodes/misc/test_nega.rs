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
fn test_neg_a_basic() {
    // NEGA - Negate A register (inherent) - opcode 0x40
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set A to a positive test value
    cpu.registers_mut().a = 0x42; // 66 in decimal
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.c = true;

    // Place NEGA instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x40); // NEGA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Verify A register is negated: 0x42 -> 0xBE (two's complement)
    assert_eq!(cpu.registers().a, 0xBE); // -66 in two's complement

    // Verify flags - C++ Original: value = SubtractImpl(0, value, 0, CC); (which updates all flags)
    assert!(!cpu.registers().cc.z); // Z=0 (0xBE is not zero)
    assert!(cpu.registers().cc.n); // N=1 (0xBE is negative)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow for 0x42 negation)
    assert!(cpu.registers().cc.c); // C=1 (set by SubtractImpl for non-zero result)

    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);

    // Verify cycle count - NEGA should be 2 cycles
}
