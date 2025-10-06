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
fn test_jsr_indexed_0xad_simple() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    let subroutine_addr = RAM_START + 0x100;

    // Setup: X points to subroutine address
    cpu.registers_mut().x = subroutine_addr;

    // Write opcode: JSR indexed [,X] (postbyte = 0x84)
    unsafe { &mut *memory.get() }.write(RAM_START, 0xAD);
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x84); // No offset, X register

    cpu.registers_mut().pc = RAM_START;
    let initial_s = cpu.registers().s;

    cpu.execute_instruction(false, false);

    // Verify: PC jumped to subroutine
    assert_eq!(
        cpu.registers().pc,
        subroutine_addr,
        "PC should jump to subroutine address"
    );

    // Verify: Return address (PC after JSR instruction = RAM_START + 2) pushed to stack
    let expected_return = RAM_START + 2;
    assert_eq!(
        cpu.registers().s,
        initial_s - 2,
        "Stack pointer should decrement by 2"
    );

    // Stack should have return address (high byte first, then low byte)
    let stacked_high = unsafe { &*memory.get() }.read(cpu.registers().s);
    let stacked_low = unsafe { &*memory.get() }.read(cpu.registers().s + 1);
    let stacked_pc = ((stacked_high as u16) << 8) | (stacked_low as u16);

    assert_eq!(
        stacked_pc, expected_return,
        "Return address should be on stack"
    );
}

#[test]
fn test_jsr_indexed_0xad_with_offset() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: X = 0xC850, offset = 0x10, subroutine at 0xC860
    cpu.registers_mut().x = RAM_START + 0x50;
    let offset: i8 = 0x10;
    let subroutine_addr = (RAM_START + 0x50).wrapping_add_signed(offset as i16);

    // Write opcode: JSR indexed with 5-bit offset (postbyte = 0b000xxxxx where xxxxx = offset)
    unsafe { &mut *memory.get() }.write(RAM_START, 0xAD);
    // For 8-bit offset: postbyte = 0x88, followed by offset byte
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x88); // 8-bit offset mode, X register
    unsafe { &mut *memory.get() }.write(RAM_START + 2, offset as u8); // Offset value

    cpu.registers_mut().pc = RAM_START;
    let initial_s = cpu.registers().s;

    cpu.execute_instruction(false, false);

    // Verify: PC jumped to X + offset
    assert_eq!(
        cpu.registers().pc,
        subroutine_addr,
        "PC should jump to X + offset"
    );

    // Verify: Return address pushed (PC after 3-byte instruction = RAM_START + 3)
    let expected_return = RAM_START + 3;
    let stacked_high = unsafe { &*memory.get() }.read(cpu.registers().s);
    let stacked_low = unsafe { &*memory.get() }.read(cpu.registers().s + 1);
    let stacked_pc = ((stacked_high as u16) << 8) | (stacked_low as u16);

    assert_eq!(
        stacked_pc, expected_return,
        "Return address should account for 3-byte instruction"
    );
    assert_eq!(
        cpu.registers().s,
        initial_s - 2,
        "Stack pointer should decrement by 2"
    );
}

#[test]
fn test_jsr_indexed_0xad_nested_calls() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // First JSR: call subroutine at 0xC900
    cpu.registers_mut().x = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START, 0xAD);
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x84);

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);

    let first_return = RAM_START + 2;
    let first_s = cpu.registers().s;

    // Second JSR: nested call from within first subroutine
    cpu.registers_mut().y = RAM_START + 0x200;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0xAD);
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0xA4); // Y register

    cpu.execute_instruction(false, false);

    let second_return = RAM_START + 0x102;

    // Verify: Two return addresses on stack
    assert_eq!(
        cpu.registers().s,
        first_s - 2,
        "Stack should have two entries"
    );

    // Second return address (most recent) at current SP
    let stacked_high2 = unsafe { &*memory.get() }.read(cpu.registers().s);
    let stacked_low2 = unsafe { &*memory.get() }.read(cpu.registers().s + 1);
    let stacked_pc2 = ((stacked_high2 as u16) << 8) | (stacked_low2 as u16);
    assert_eq!(
        stacked_pc2, second_return,
        "Second return address should be on top of stack"
    );

    // First return address below it
    let stacked_high1 = unsafe { &*memory.get() }.read(first_s);
    let stacked_low1 = unsafe { &*memory.get() }.read(first_s + 1);
    let stacked_pc1 = ((stacked_high1 as u16) << 8) | (stacked_low1 as u16);
    assert_eq!(
        stacked_pc1, first_return,
        "First return address should be below second"
    );
}

#[test]
fn test_jsr_indexed_0xad_indirect() {
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: X points to address that contains target address
    cpu.registers_mut().x = RAM_START + 0x50;
    let target_addr = RAM_START + 0x200;

    // Store target address at [X]
    unsafe { &mut *memory.get() }.write(RAM_START + 0x50, (target_addr >> 8) as u8); // High byte
    unsafe { &mut *memory.get() }.write(RAM_START + 0x51, (target_addr & 0xFF) as u8); // Low byte

    // Write opcode: JSR indexed indirect [[,X]]
    unsafe { &mut *memory.get() }.write(RAM_START, 0xAD);
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x94); // Indirect mode, X register

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false);

    // Verify: PC jumped to address stored at [X]
    assert_eq!(
        cpu.registers().pc,
        target_addr,
        "PC should jump to indirect target"
    );
}
