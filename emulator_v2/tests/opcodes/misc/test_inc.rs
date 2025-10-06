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
fn test_inc_overflow() {
    // INCA with overflow (0x7F -> 0x80)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set A to 0x7F (127 in signed)
    cpu.registers_mut().a = 0x7F;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = false;

    // Place INCA instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x4C); // INCA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Result: 0x7F + 1 = 0x80 (128 in unsigned, -128 in signed)
    assert_eq!(cpu.registers().a, 0x80);

    // Verify flags - C++ Original: CC.Overflow = origValue == 0b0111'1111; (0x7F case)
    assert!(!cpu.registers().cc.z); // Z=0 (result 0x80 is not zero)
    assert!(cpu.registers().cc.n); // N=1 (0x80 is negative in signed arithmetic)
    assert!(cpu.registers().cc.v); // V=1 (overflow: 0x7F is exactly the overflow condition)
                                   // CRITICAL 1:1 Vectrexy: INC does NOT modify Carry flag (should preserve initial false)
}
