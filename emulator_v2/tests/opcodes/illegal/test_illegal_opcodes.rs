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

fn test_illegal_opcode(opcode: u8) {
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
fn test_illegal_0x38_reserved() {
    test_illegal_opcode(0x38);
}

#[test]
fn test_illegal_0x3e_reset() {
    test_illegal_opcode(0x3E);
}

#[test]
fn test_illegal_0x41_undefined() {
    test_illegal_opcode(0x41);
}

#[test]
fn test_illegal_0x4e_clrb_illegal() {
    test_illegal_opcode(0x4E);
}

#[test]
fn test_illegal_0x71_neg_extended_illegal() {
    test_illegal_opcode(0x71);
}

#[test]
fn test_illegal_0x87_undefined() {
    test_illegal_opcode(0x87);
}

#[test]
fn test_illegal_0x8f_undefined() {
    test_illegal_opcode(0x8F);
}

#[test]
fn test_illegal_0xc7_undefined() {
    test_illegal_opcode(0xC7);
}

#[test]
fn test_illegal_0xcd_undefined() {
    test_illegal_opcode(0xCD);
}

#[test]
fn test_illegal_0xcf_undefined() {
    test_illegal_opcode(0xCF);
}
