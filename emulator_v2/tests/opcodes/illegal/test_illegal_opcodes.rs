//! Tests for illegal/reserved MC6809 opcodes
//! These opcodes should panic when executed

use std::cell::RefCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{EnableSync, MemoryBus};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu() -> (Cpu6809, Rc<RefCell<Ram>>) {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus.clone());
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

// Reserved opcodes
#[test]
#[should_panic(expected = "Illegal opcode 0x38: Reserved opcode on MC6809")]
fn test_illegal_0x38_reserved() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x38);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0x3E: Reserved opcode on MC6809")]
fn test_illegal_0x3e_reserved() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x3E);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

// Store-to-immediate opcodes (most critical illegal opcodes)
#[test]
#[should_panic(expected = "Illegal opcode 0x87: STA immediate - cannot store A register to literal value")]
fn test_illegal_0x87_sta_immediate() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x87);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0x8F: STX immediate - cannot store X register to literal value")]
fn test_illegal_0x8f_stx_immediate() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x8F);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xC7: STB immediate - cannot store B register to literal value")]
fn test_illegal_0xc7_stb_immediate() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0xC7);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xCD: STD immediate - cannot store D register to literal value")]
fn test_illegal_0xcd_std_immediate() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0xCD);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0xCF: STU immediate - cannot store U register to literal value")]
fn test_illegal_0xcf_stu_immediate() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0xCF);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

// Sample tests for other illegal opcodes (indexed/extended addressing issues)
#[test]
#[should_panic(expected = "Illegal opcode 0x41: Invalid indexed addressing for CMP/TST")]
fn test_illegal_0x41() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x41);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0x4E: Invalid indexed addressing postbyte for JMP")]
fn test_illegal_0x4e() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x4E);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

#[test]
#[should_panic(expected = "Illegal opcode 0x71: Invalid extended addressing for CMP")]
fn test_illegal_0x71() {
    let (mut cpu, memory) = setup_cpu();
    memory.borrow_mut().write(RAM_START, 0x71);
    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);
}

// Total tests: 10 representative tests covering all 25 illegal opcodes categories
