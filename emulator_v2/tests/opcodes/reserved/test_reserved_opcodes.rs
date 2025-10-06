use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, CpuError, EnableSync, MemoryBus, Ram};

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

fn test_reserved_opcode(opcode: u8) {
    let (mut cpu, memory) = setup_cpu_with_ram();
    unsafe { &mut *memory.get() }.write(RAM_START, opcode);
    cpu.registers_mut().pc = RAM_START;

    let result = cpu.execute_instruction(false, false);
    assert!(result.is_err());

    match result.unwrap_err() {
        CpuError::IllegalInstruction(op) => assert_eq!(op, opcode),
        _ => panic!("Expected IllegalInstruction error"),
    }
}

#[test]
fn test_reserved_0x01() {
    test_reserved_opcode(0x01);
}

#[test]
fn test_reserved_0x02() {
    test_reserved_opcode(0x02);
}

#[test]
fn test_reserved_0x05() {
    test_reserved_opcode(0x05);
}

#[test]
fn test_reserved_0x0b() {
    test_reserved_opcode(0x0B);
}

#[test]
fn test_reserved_0x14() {
    test_reserved_opcode(0x14);
}

#[test]
fn test_reserved_0x15() {
    test_reserved_opcode(0x15);
}

#[test]
fn test_reserved_0x18() {
    test_reserved_opcode(0x18);
}

#[test]
fn test_reserved_0x1b() {
    test_reserved_opcode(0x1B);
}
