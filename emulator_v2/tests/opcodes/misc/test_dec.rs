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
fn test_dec_overflow() {
    // DECA with overflow (0x80 -> 0x7F) - Test 1:1 Vectrexy behavior
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set A to 0x80 (-128 in signed, will overflow to +127)
    cpu.registers_mut().a = 0x80;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = false;
    cpu.registers_mut().cc.c = true; // Set to verify DEC doesn't modify Carry

    // Place DECA instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x4A); // DECA
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Result: 0x80 - 1 = 0x7F (overflow from negative to positive)
    assert_eq!(cpu.registers().a, 0x7F);

    // Verify flags - C++ Original: CC.Overflow = origValue == 0b1000'0000; (0x80 case)
    assert!(!cpu.registers().cc.z); // Z=0 (0x7F is not zero)
    assert!(!cpu.registers().cc.n); // N=0 (0x7F is positive)
    assert!(cpu.registers().cc.v); // V=1 (overflow: 0x80 is exactly the overflow condition)
    assert!(cpu.registers().cc.c); // C unchanged (DEC doesn't modify Carry) - 1:1 Vectrexy
}
