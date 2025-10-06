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
fn test_bra_0x20() {
    // BRA (Branch Always) - opcode 0x20
    // Unconditional relative branch
    let (mut cpu, memory) = setup_cpu_with_ram();

    // BRA $10 (branch forward 16 bytes)
    unsafe { &mut *memory.get() }.write(RAM_START, 0x20); // BRA
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // offset +16

    cpu.registers_mut().pc = RAM_START;
    cpu.execute_instruction(false, false).unwrap();

    // PC should be RAM_START + 2 (instruction size) + 0x10 (offset) = RAM_START + 0x12
    assert_eq!(cpu.registers().pc, RAM_START + 0x12);
}

#[test]
fn test_bra_backward_0x20() {
    // BRA with negative offset (backward branch)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Start at RAM_START + 0x100
    let start_pc = RAM_START + 0x100;

    // BRA $FE (-2 in two's complement)
    unsafe { &mut *memory.get() }.write(start_pc, 0x20); // BRA
    unsafe { &mut *memory.get() }.write(start_pc + 1, 0xFE); // offset -2

    cpu.registers_mut().pc = start_pc;
    cpu.execute_instruction(false, false).unwrap();

    // PC should be start_pc + 2 + (-2) = start_pc
    assert_eq!(cpu.registers().pc, start_pc);
}

#[test]
fn test_beq_taken_0x27() {
    // BEQ (Branch if Equal/Zero) - opcode 0x27
    // Branches if Z flag is set
    let (mut cpu, memory) = setup_cpu_with_ram();

    // BEQ $10
    unsafe { &mut *memory.get() }.write(RAM_START, 0x27); // BEQ
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // offset +16

    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().cc.z = true; // Set Z flag

    cpu.execute_instruction(false, false).unwrap();

    // Branch should be taken
    assert_eq!(cpu.registers().pc, RAM_START + 0x12);
}

#[test]
fn test_beq_not_taken_0x27() {
    // BEQ when Z flag is clear - branch not taken
    let (mut cpu, memory) = setup_cpu_with_ram();

    // BEQ $10
    unsafe { &mut *memory.get() }.write(RAM_START, 0x27); // BEQ
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // offset +16

    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().cc.z = false; // Clear Z flag

    cpu.execute_instruction(false, false).unwrap();

    // Branch not taken, PC just advances past instruction
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}

#[test]
fn test_bne_taken_0x26() {
    // BNE (Branch if Not Equal/Zero) - opcode 0x26
    // Branches if Z flag is clear
    let (mut cpu, memory) = setup_cpu_with_ram();

    // BNE $10
    unsafe { &mut *memory.get() }.write(RAM_START, 0x26); // BNE
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // offset +16

    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().cc.z = false; // Clear Z flag

    cpu.execute_instruction(false, false).unwrap();

    // Branch should be taken
    assert_eq!(cpu.registers().pc, RAM_START + 0x12);
}

#[test]
fn test_bne_not_taken_0x26() {
    // BNE when Z flag is set - branch not taken
    let (mut cpu, memory) = setup_cpu_with_ram();

    // BNE $10
    unsafe { &mut *memory.get() }.write(RAM_START, 0x26); // BNE
    unsafe { &mut *memory.get() }.write(RAM_START + 1, 0x10); // offset +16

    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().cc.z = true; // Set Z flag

    cpu.execute_instruction(false, false).unwrap();

    // Branch not taken
    assert_eq!(cpu.registers().pc, RAM_START + 2);
}
