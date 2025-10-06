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
fn test_ldx_immediate_0x8e() {
    // C++ Original: LDX #immediate - opcode 0x8E
    // Test LDX with immediate 16-bit value - basic functionality
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0x8E); // LDX #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // high byte of immediate value
    unsafe { &mut *memory.get() }.write(0xC802, 0x34); // low byte of immediate value

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().x,
        0x1234,
        "X register should contain immediate value"
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
fn test_ldx_immediate_zero() {
    // Test LDX with zero value to verify Zero flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0x8E); // LDX #immediate
    unsafe { &mut *memory.get() }.write(0xC811, 0x00); // high byte = 0
    unsafe { &mut *memory.get() }.write(0xC812, 0x00); // low byte = 0

    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().x, 0x0000);
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
fn test_ldx_immediate_negative() {
    // Test LDX with negative value (bit 15 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0x8E); // LDX #immediate
    unsafe { &mut *memory.get() }.write(0xC821, 0x80); // high byte = 0x80 (negative)
    unsafe { &mut *memory.get() }.write(0xC822, 0x00); // low byte = 0x00

    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().x, 0x8000);
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
fn test_ldx_direct_0x9e() {
    // C++ Original: LDX direct - opcode 0x9E
    // Test loading from direct page memory (16-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDX $50 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC830, 0x9E); // LDX direct
    unsafe { &mut *memory.get() }.write(0xC831, 0x50); // direct page address (low byte)

    // Set up target memory location (DP = 0xC8, so address = $C850) - must be in RAM range
    unsafe { &mut *memory.get() }.write(0xC850, 0xAB); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC851, 0xCD); // low byte value to load

    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().x,
        0xABCD,
        "X should contain value from direct page memory"
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
fn test_ldx_extended_0xbe() {
    // C++ Original: LDX extended - opcode 0xBE
    // Test loading from 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDX $C870 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xBE); // LDX extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x70); // low byte of address - target $C870

    // Set up target memory location in RAM
    unsafe { &mut *memory.get() }.write(0xC870, 0x55); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC871, 0x77); // low byte value to load

    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().x,
        0x5577,
        "X should contain value from extended address"
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
