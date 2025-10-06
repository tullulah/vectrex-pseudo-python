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
fn test_lda_immediate_0x86() {
    // C++ Original: LDA #immediate - opcode 0x86
    // Test loading immediate value into A register
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up memory: 0x86 0x42 (LDA #$42) - place in RAM area (0xC800+)
    unsafe { &mut *memory.get() }.write(0xC800, 0x86); // LDA #immediate
    unsafe { &mut *memory.get() }.write(0xC801, 0x42); // immediate value

    // Set PC to start of instruction
    cpu.registers_mut().pc = 0xC800;

    // Execute one instruction
    cpu.execute_instruction(false, false).unwrap();

    // Verify results - C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0;
    assert_eq!(
        cpu.registers().a,
        0x42,
        "A register should contain loaded value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    ); // From CpuOpCodes.h

    // Condition codes
    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag should be clear for positive value"
    );
    assert_eq!(
        cpu.registers().cc.z,
        false,
        "Zero flag should be clear for non-zero value"
    );
    assert_eq!(
        cpu.registers().cc.v,
        false,
        "Overflow flag should be clear (always for LD)"
    );
}

#[test]
fn test_lda_immediate_zero() {
    // Test LDA with zero value to verify Zero flag setting
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0x86); // LDA #immediate
    unsafe { &mut *memory.get() }.write(0xC811, 0x00); // zero value

    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().a, 0x00);
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
fn test_lda_immediate_negative() {
    // Test LDA with negative value (bit 7 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0x86); // LDA #immediate
    unsafe { &mut *memory.get() }.write(0xC821, 0x80); // negative value

    cpu.registers_mut().pc = 0xC820;
    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers().a, 0x80);
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
fn test_lda_direct_0x96() {
    // C++ Original: LDA direct - opcode 0x96
    // Test loading from direct page memory
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDA $20 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC830, 0x96); // LDA direct
    unsafe { &mut *memory.get() }.write(0xC831, 0x20); // direct page address (low byte)

    // Set up target memory location (DP = 0xC8, so address = $C820) - must be in RAM range
    unsafe { &mut *memory.get() }.write(0xC820, 0x55); // value to load

    cpu.registers_mut().pc = 0xC830;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().a,
        0x55,
        "A should contain value from direct page memory"
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
fn test_lda_extended_0xb6() {
    // C++ Original: LDA extended - opcode 0xB6
    // Test loading from 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: LDA $C850 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xB6); // LDA extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x50); // low byte of address - target $C850

    // Set up target memory location in RAM
    unsafe { &mut *memory.get() }.write(0xC850, 0xAA); // value to load

    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(
        cpu.registers().a,
        0xAA,
        "A should contain value from extended address"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC843,
        "PC should advance past 3-byte instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        true,
        "Negative flag should be set for 0xAA"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for LD");
}
