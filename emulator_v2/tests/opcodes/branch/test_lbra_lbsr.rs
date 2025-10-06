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
fn test_lbra_forward_0x16() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Write LBRA with offset +0x1000 (forward jump)
    unsafe { &mut *memory.get() }.write(RAM_START, 0x16); // LBRA opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // High byte of offset
    unsafe { &mut *memory.get() }.write(RAM_START + 2, 0x00); // Low byte of offset

    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;

    // PC after reading opcode and offset = 0xC800 + 3 = 0xC803
    // Expected PC = 0xC803 + 0x1000 = 0xD803

    // Execute LBRA
    cpu.execute_instruction(false, false);

    // Verify PC jumped forward by 0x1000 from end of instruction
    assert_eq!(cpu.registers().pc, 0xD803, "PC should jump to 0xD803");
}

#[test]
fn test_lbra_backward_0x16() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Write LBRA with offset -256 (0xFF00 in two's complement)
    unsafe { &mut *memory.get() }.write(RAM_START, 0x16); // LBRA opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0xFF); // High byte (-1)
    unsafe { &mut *memory.get() }.write(RAM_START + 2, 0x00); // Low byte (0)

    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;

    // PC after reading = 0xC803
    // Expected PC = 0xC803 + (-256) = 0xC803 - 0x100 = 0xC703

    // Execute LBRA
    cpu.execute_instruction(false, false);

    // Verify PC jumped backward
    assert_eq!(cpu.registers().pc, 0xC703, "PC should jump to 0xC703");
}

#[test]
fn test_lbsr_pushes_return_address_0x17() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Write LBSR with offset +0x0500
    unsafe { &mut *memory.get() }.write(RAM_START, 0x17); // LBSR opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x05); // High byte of offset
    unsafe { &mut *memory.get() }.write(RAM_START + 2, 0x00); // Low byte of offset

    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;
    let initial_sp = cpu.registers().s;

    // Execute LBSR
    cpu.execute_instruction(false, false);

    // Verify return address was pushed onto stack
    // Stack grows down: SP-1 has low byte, SP-2 has high byte (last pushed)
    let pushed_low = unsafe { &mut *memory.get() }.read(initial_sp - 1);
    let pushed_high = unsafe { &mut *memory.get() }.read(initial_sp - 2);
    let return_address = ((pushed_high as u16) << 8) | (pushed_low as u16);

    assert_eq!(return_address, 0xC803, "Return address should be 0xC803");
    assert_eq!(
        cpu.registers().s,
        initial_sp - 2,
        "Stack pointer should decrease by 2"
    );

    // Verify PC jumped to target (0xC803 + 0x0500 = 0xCD03)
    assert_eq!(cpu.registers().pc, 0xCD03, "PC should jump to 0xCD03");
}

#[test]
fn test_lbsr_rts_roundtrip() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Setup: LBSR at 0xC800, subroutine at 0xC900 with RTS
    unsafe { &mut *memory.get() }.write(RAM_START, 0x17); // LBSR opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x00); // Offset high
    unsafe { &mut *memory.get() }.write(RAM_START + 2, 0xFD); // Offset low (+253 = to 0xC900)

    unsafe { &mut *memory.get() }.write(0xC900, 0x39); // RTS opcode at subroutine

    // Set PC to start of LBSR
    cpu.registers_mut().pc = RAM_START;
    let initial_sp = cpu.registers().s;

    // Execute LBSR
    cpu.execute_instruction(false, false);

    // Should be at subroutine (0xC803 + 0x00FD = 0xC900)
    assert_eq!(cpu.registers().pc, 0xC900, "PC should be at subroutine");

    // Execute RTS
    cpu.execute_instruction(false, false);

    // Should return to instruction after LBSR (0xC803)
    assert_eq!(cpu.registers().pc, 0xC803, "PC should return to 0xC803");
    assert_eq!(
        cpu.registers().s,
        initial_sp,
        "Stack pointer should be restored"
    );
}

#[test]
fn test_lbra_zero_offset() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Write LBRA with offset 0 (infinite loop scenario)
    unsafe { &mut *memory.get() }.write(RAM_START, 0x16); // LBRA opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x00); // High byte
    unsafe { &mut *memory.get() }.write(RAM_START + 2, 0x00); // Low byte

    // Set PC to start of instruction
    cpu.registers_mut().pc = RAM_START;

    // Execute LBRA
    cpu.execute_instruction(false, false);

    // PC after instruction = 0xC803 + 0 = 0xC803
    assert_eq!(
        cpu.registers().pc,
        0xC803,
        "PC should advance to next instruction"
    );
}
