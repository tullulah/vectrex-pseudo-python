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
fn test_ldb_immediate_0xc6() {
    // C++ Original: LDB #immediate - opcode 0xC6
    // Test LDB with immediate value - basic functionality
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0xC6); // LDB #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x42); // immediate value

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().b,
        0x42,
        "B register should contain immediate value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past 2-byte instruction"
    ); // From CpuOpCodes.h

    assert_eq!(cpu.registers().cc.n, false, "Negative flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldb_immediate_zero() {
    // Test LDB with zero value to verify Zero flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0xC6); // LDB #immediate
    unsafe { &mut *memory.get() }.write(0xC811, 0x00); // zero value

    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().b, 0x00);
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
fn test_ldb_immediate_negative() {
    // Test LDB with negative value (bit 7 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0xC6); // LDB #immediate
    unsafe { &mut *memory.get() }.write(0xC821, 0x80); // negative value

    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().b, 0x80);
    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set for bit 7 = 1"
    );
    assert_eq!(
        cpu.registers().cc.z,
        false,
        "Zero flag should be clear for non-zero value"
    );
}

#[test]
fn test_ldb_direct_0xd6() {
    // C++ Original: LDB direct - opcode 0xD6
    // Test loading from direct page memory
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDB $40 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC830, 0xD6); // LDB direct
    unsafe { &mut *memory.get() }.write(0xC831, 0x40); // direct page address (low byte)

    // Set up target memory location (DP = 0xC8, so address = $C840) - must be in RAM range
    unsafe { &mut *memory.get() }.write(0xC840, 0x77); // value to load (matching DP + offset)

    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().b,
        0x77,
        "B should contain value from direct page memory"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC832,
        "PC should advance past instruction"
    ); // From CpuOpCodes.h

    assert_eq!(cpu.registers().cc.n, false, "Negative flag check");
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}

#[test]
fn test_ldb_extended_0xf6() {
    // C++ Original: LDB extended - opcode 0xF6
    // Test loading from 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDB $C860 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xF6); // LDB extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x60); // low byte of address - target $C860

    // Set up target memory location in RAM
    unsafe { &mut *memory.get() }.write(0xC860, 0xBB); // value to load

    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().b,
        0xBB,
        "B should contain value from extended address"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC843,
        "PC should advance past 3-byte instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set for 0xBB"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}
