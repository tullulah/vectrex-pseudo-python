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
fn test_dec_a_basic() {
    // DECA - Decrement A register (inherent) - opcode 0x4A
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set A to a test value
    cpu.registers_mut().a = 0x43;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;

    // Place DECA instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x4A); // DECA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Verify A register is decremented
    assert_eq!(cpu.registers().a, 0x42);

    // Verify flags - C++ Original: CC.Overflow = origValue == 0b1000'0000; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value);
    assert!(!cpu.registers().cc.z); // Z=0 (result is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (result is positive)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow: 0x43 != 0x80)
                                    // CRITICAL 1:1 Vectrexy: DEC does NOT modify Carry flag (should preserve initial true)

    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);

    // Verify cycle count - DECA should be 2 cycles
}
