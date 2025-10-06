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
fn test_sta_direct_0x97() {
    // C++ Original: STA direct - opcode 0x97
    // Test storing A register to direct page memory
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: STA $40 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC800, 0x97); // STA direct
    unsafe { &mut *memory.get() }.write(0xC801, 0x40); // direct page address (low byte)

    // Set up A register with test value
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)

    cpu.execute_instruction(false, false).unwrap();

    // Verify A value was stored to memory (DP + offset = $C840)
    let stored_value = unsafe { &*memory.get() }.read(0xC840);
    assert_eq!(
        stored_value, 0x55,
        "A value should be stored to direct page memory"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    ); // From CpuOpCodes.h

    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag check for stored value"
    );
    assert_eq!(cpu.registers().cc.z, false, "Zero flag check");
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}

#[test]
fn test_sta_direct_zero() {
    // Test STA with zero value to verify Zero flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC810, 0x97); // STA direct
    unsafe { &mut *memory.get() }.write(0xC811, 0x50); // direct page address

    cpu.registers_mut().a = 0x00; // zero value
    cpu.registers_mut().pc = 0xC810;
    cpu.registers_mut().dp = 0xC8;

    cpu.execute_instruction(false, false);

    let stored_value = unsafe { &*memory.get() }.read(0xC850);
    assert_eq!(stored_value, 0x00);
    assert_eq!(
        cpu.registers().cc.z,
        true,
        "Zero flag should be set when storing zero"
    );
    assert_eq!(
        cpu.registers().cc.n,
        false,
        "Negative flag should be clear for zero"
    );
}

#[test]
fn test_sta_direct_negative() {
    // Test STA with negative value (bit 7 set) to verify Negative flag
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC820, 0x97); // STA direct
    unsafe { &mut *memory.get() }.write(0xC821, 0x60); // direct page address

    cpu.registers_mut().a = 0x80; // negative value
    cpu.registers_mut().pc = 0xC820;
    cpu.registers_mut().dp = 0xC8;

    cpu.execute_instruction(false, false);

    let stored_value = unsafe { &*memory.get() }.read(0xC860);
    assert_eq!(stored_value, 0x80);
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
fn test_sta_extended_0xb7() {
    // C++ Original: STA extended - opcode 0xB7
    // Test storing to 16-bit absolute address
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up instruction: STA $C890 - place in RAM
    unsafe { &mut *memory.get() }.write(0xC840, 0xB7); // STA extended
    unsafe { &mut *memory.get() }.write(0xC841, 0xC8); // high byte of address
    unsafe { &mut *memory.get() }.write(0xC842, 0x90); // low byte of address - target $C890

    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().pc = 0xC840;

    cpu.execute_instruction(false, false).unwrap();

    let stored_value = unsafe { &*memory.get() }.read(0xC890);
    assert_eq!(stored_value, 0xAA, "A should be stored to extended address");
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
    assert_eq!(cpu.registers().cc.v, false, "Overflow always clear for ST");
}
