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
fn test_ldd_immediate_0xcc() {
    // C++ Original: LDD #immediate - opcode 0xCC
    // Test LDD with immediate 16-bit value - loads into A:B combined (D register)
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0xCC); // LDD #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // high byte of immediate value (A)
    unsafe { &mut *memory.get() }.write(0xC802, 0x34); // low byte of immediate value (B)

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().a,
        0x12,
        "A register should contain high byte"
    );
    assert_eq!(
        cpu.registers().b,
        0x34,
        "B register should contain low byte"
    );
    assert_eq!(
        cpu.registers().d(),
        0x1234,
        "D register should contain combined value"
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
fn test_ldd_immediate_zero() {
    // Test LDD with zero value to verify Zero flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0xCC); // LDD #immediate
    unsafe { &mut *memory.get() }.write(0xC811, 0x00); // high byte = 0
    unsafe { &mut *memory.get() }.write(0xC812, 0x00); // low byte = 0

    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().d(), 0x0000);
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
fn test_ldd_immediate_negative() {
    // Test LDD with negative value (bit 15 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0xCC); // LDD #immediate
    unsafe { &mut *memory.get() }.write(0xC821, 0x80); // high byte = 0x80 (negative)
    unsafe { &mut *memory.get() }.write(0xC822, 0x00); // low byte = 0x00

    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(cpu.registers().b, 0x00);
    assert_eq!(cpu.registers().d(), 0x8000);
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
fn test_ldd_direct_0xdc() {
    // C++ Original: LDD direct - opcode 0xDC
    // Test loading from direct page memory (16-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDD $60 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC830, 0xDC); // LDD direct
    unsafe { &mut *memory.get() }.write(0xC831, 0x60); // direct page address (low byte)

    // Set up target memory location (DP = 0xC8, so address = $C860) - must be in RAM range
    unsafe { &mut *memory.get() }.write(0xC860, 0xAB); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC861, 0xCD); // low byte value to load

    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().a,
        0xAB,
        "A should contain high byte from memory"
    );
    assert_eq!(
        cpu.registers().b,
        0xCD,
        "B should contain low byte from memory"
    );
    assert_eq!(
        cpu.registers().d(),
        0xABCD,
        "D should contain combined value from direct page memory"
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
fn test_ldd_extended_0xfc() {
    // C++ Original: LDD extended - opcode 0xFC
    // Test loading from 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDD $C880 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xFC); // LDD extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x80); // low byte of address - target $C880

    // Set up target memory location in RAM
    unsafe { &mut *memory.get() }.write(0xC880, 0x55); // high byte value to load
    unsafe { &mut *memory.get() }.write(0xC881, 0x77); // low byte value to load

    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().a,
        0x55,
        "A should contain high byte from extended address"
    );
    assert_eq!(
        cpu.registers().b,
        0x77,
        "B should contain low byte from extended address"
    );
    assert_eq!(
        cpu.registers().d(),
        0x5577,
        "D should contain combined value from extended address"
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
