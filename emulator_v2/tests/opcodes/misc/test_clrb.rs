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
fn test_clr_b_basic() {
    // CLRB - Clear B register (inherent) - opcode 0x5F
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set B to a non-zero value initially
    cpu.registers_mut().b = 0xAA;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;

    // Place CLRB instruction in RAM area
    unsafe { &mut *memory.get() }.write(0xC800, 0x5F); // CLRB
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();

    // Verify B register is cleared
    assert_eq!(cpu.registers().b, 0x00);

    // Verify flags - C++ Original: CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0;
    assert!(cpu.registers().cc.z); // Z=1 (result is zero)
    assert!(!cpu.registers().cc.n); // N=0 (result is positive)
    assert!(!cpu.registers().cc.v); // V=0 (no overflow)
    assert!(!cpu.registers().cc.c); // C=0 (carry cleared) - 1:1 Vectrexy behavior

    // Verify PC advanced
    assert_eq!(cpu.registers().pc, 0xC801);

    // Verify cycle count
}
