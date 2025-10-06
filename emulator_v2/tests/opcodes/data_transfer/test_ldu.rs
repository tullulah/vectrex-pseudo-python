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
fn test_ldu_immediate_0xce() {
    // C++ Original: LDU #immediate - opcode 0xCE
    // Test LDU with immediate 16-bit value - basic functionality
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0xCE); // LDU #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // high byte of immediate value
    unsafe { &mut *memory.get() }.write(0xC802, 0x34); // low byte of immediate value

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().u,
        0x1234,
        "U register should contain immediate value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC803,
        "PC should advance past 3-byte instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag should be clear for positive value"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldu_immediate_zero() {
    // Test LDU with zero value to verify Zero flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0xCE); // LDU #immediate
    unsafe { &mut *memory.get() }.write(0xC811, 0x00); // high byte = 0
    unsafe { &mut *memory.get() }.write(0xC812, 0x00); // low byte = 0

    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().u, 0x0000);
    assert_eq!(
        cpu.registers().cc.z,
        true,
        "Zero flag should be set for zero value"
    );
    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag should be clear for zero"
    );
}

#[test]
fn test_ldu_immediate_negative() {
    // Test LDU with negative value (bit 15 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0xCE); // LDU #immediate
    unsafe { &mut *memory.get() }.write(0xC821, 0x80); // high byte = 0x80 (negative)
    unsafe { &mut *memory.get() }.write(0xC822, 0x00); // low byte = 0x00

    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().u, 0x8000);
    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set for bit 15 = 1"
    );
    assert_eq!(
        cpu.registers().cc.z,
        false,
        "Zero flag should be clear for non-zero value"
    );
}

#[test]
fn test_ldu_direct_0xde() {
    // C++ Original: LDU direct - opcode 0xDE
    // Test loading from direct page memory (16-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDU $70 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC830, 0xDE); // LDU direct
    unsafe { &mut *memory.get() }.write(0xC831, 0x70); // direct page address (low byte)

    // Set up target memory location (DP = 0xC8, so address = $C870) - must be in RAM range
    unsafe { &mut *memory.get() }.write(0xC870, 0xAB); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC871, 0xCD); // low byte value to load

    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().u,
        0xABCD,
        "U should contain value from direct page memory"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC832,
        "PC should advance past instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set for 0xABCD"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldu_extended_0xfe() {
    // C++ Original: LDU extended - opcode 0xFE
    // Test loading from 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDU $C890 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xFE); // LDU extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x90); // low byte of address - target $C890

    // Set up target memory location in RAM
    unsafe { &mut *memory.get() }.write(0xC890, 0x55); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC891, 0x77); // low byte value to load

    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().u,
        0x5577,
        "U should contain value from extended address"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC843,
        "PC should advance past 3-byte instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag should be clear for 0x5577"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}
