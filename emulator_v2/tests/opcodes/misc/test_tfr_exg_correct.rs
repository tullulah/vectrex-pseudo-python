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
fn test_tfr_a_to_b_0x1f() {
    // C++ Original: TFR A,B with postbyte 0x89 (A=8+bit3, B=9+bit3 for 8-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state - A=0x42, B=0x00
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x00;

    unsafe { &mut *memory.get() }.write(0xC800, 0x1F); // TFR opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x89); // TFR A,B: src=0(A)<<4 | dst=1(B) | 0x88 = 0x01 | 0x88 = 0x89

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    // Verify: B should now equal A, A unchanged
    assert_eq!(cpu.registers().a, 0x42, "A register should be unchanged");
    assert_eq!(cpu.registers().b, 0x42, "B register should receive A value");
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    ); // From MC6809 documentation
}

#[test]
fn test_tfr_x_to_d_0x1f() {
    // C++ Original: TFR X,D with postbyte 0x10 (X=1, D=0 for 16-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state - X=0x1234, D=0x0000
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().a = 0x00; // D = A:B combined
    cpu.registers_mut().b = 0x00;

    unsafe { &mut *memory.get() }.write(0xC800, 0x1F); // TFR opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x10); // X=1, D=0 (16-bit transfer)

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    // Verify: D should now equal X, X unchanged
    assert_eq!(cpu.registers().x, 0x1234, "X register should be unchanged");
    assert_eq!(
        cpu.registers().d(),
        0x1234,
        "D register should receive X value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    );
}

#[test]
fn test_exg_a_b_0x1e() {
    // C++ Original: EXG A,B with postbyte 0x89 (swap A and B)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state - A=0x42, B=0x33
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x33;

    unsafe { &mut *memory.get() }.write(0xC800, 0x1E); // EXG opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x89); // EXG A,B: src=0(A)<<4 | dst=1(B) | 0x88 = 0x01 | 0x88 = 0x89

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    // Verify: A and B values should be swapped
    assert_eq!(
        cpu.registers().a,
        0x33,
        "A register should contain original B value"
    );
    assert_eq!(
        cpu.registers().b,
        0x42,
        "B register should contain original A value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    ); // From MC6809 documentation
}

#[test]
fn test_exg_x_y_0x1e() {
    // C++ Original: EXG X,Y with postbyte 0x12 (swap X and Y)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state - X=0x1234, Y=0x5678
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().y = 0x5678;

    unsafe { &mut *memory.get() }.write(0xC800, 0x1E); // EXG opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // X=1, Y=2 (16-bit exchange)

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    // Verify: X and Y values should be swapped
    assert_eq!(
        cpu.registers().x,
        0x5678,
        "X register should contain original Y value"
    );
    assert_eq!(
        cpu.registers().y,
        0x1234,
        "Y register should contain original X value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    );
}

#[test]
fn test_tfr_d_to_x_0x1f() {
    // C++ Original: TFR D,X with postbyte 0x01 (D=0, X=1 for 16-bit)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state - D=0xABCD, X=0x0000
    cpu.registers_mut().a = 0xAB; // D = A:B combined = 0xABCD
    cpu.registers_mut().b = 0xCD;
    cpu.registers_mut().x = 0x0000;

    unsafe { &mut *memory.get() }.write(0xC800, 0x1F); // TFR opcode
    unsafe { &mut *memory.get() }.write(0xC801, 0x01); // D=0, X=1 (16-bit transfer)

    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false).unwrap();

    // Verify: X should now equal D, D unchanged
    assert_eq!(
        cpu.registers().d(),
        0xABCD,
        "D register should be unchanged"
    );
    assert_eq!(
        cpu.registers().x,
        0xABCD,
        "X register should receive D value"
    );
    assert_eq!(
        cpu.registers().pc,
        0xC802,
        "PC should advance past instruction"
    );
}
