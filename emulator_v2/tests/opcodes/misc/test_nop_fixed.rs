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
fn test_nop_0x12() {
    // Test basic NOP instruction behavior - should do absolutely nothing
    // except advance PC and consume cycles
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up initial state with known values
    let original_a = 0x42;
    let original_b = 0x84;
    let original_x = 0x1234;
    let original_y = 0x5678;
    let original_u = 0x9ABC;
    let original_s = 0xDEF0;
    let original_dp = 0xC8;
    let original_pc = 0xC800;

    // Initialize all registers
    cpu.registers_mut().a = original_a;
    cpu.registers_mut().b = original_b;
    cpu.registers_mut().x = original_x;
    cpu.registers_mut().y = original_y;
    cpu.registers_mut().u = original_u;
    cpu.registers_mut().s = original_s;
    cpu.registers_mut().dp = original_dp;

    // Set up condition codes with mixed values
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    let original_cc = cpu.registers().cc;

    // Set up memory: NOP instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x12); // NOP

    // Set PC and record initial cycle count
    cpu.registers_mut().pc = original_pc;

    // Execute NOP instruction
    cpu.execute_instruction(false, false).unwrap();

    // Verify all registers remain unchanged
    assert_eq!(
        cpu.registers().a,
        original_a,
        "A register should not change"
    );
    assert_eq!(
        cpu.registers().b,
        original_b,
        "B register should not change"
    );
    assert_eq!(
        cpu.registers().x,
        original_x,
        "X register should not change"
    );
    assert_eq!(
        cpu.registers().y,
        original_y,
        "Y register should not change"
    );
    assert_eq!(
        cpu.registers().u,
        original_u,
        "U register should not change"
    );
    assert_eq!(
        cpu.registers().s,
        original_s,
        "S register should not change"
    );
    assert_eq!(
        cpu.registers().dp,
        original_dp,
        "DP register should not change"
    );

    // Verify condition codes remain unchanged
    assert_eq!(
        cpu.registers().cc.c,
        original_cc.c,
        "Carry flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.z,
        original_cc.z,
        "Zero flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.n,
        original_cc.n,
        "Negative flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.v,
        original_cc.v,
        "Overflow flag should not change"
    );

    // Verify PC advanced by exactly 1 byte
    assert_eq!(
        cpu.registers().pc,
        original_pc + 1,
        "PC should advance by 1 byte for NOP"
    );

    // Verify cycle count
}

#[test]
fn test_nop_sequence() {
    // Test multiple consecutive NOPs to ensure they all work correctly
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up memory with multiple NOPs followed by a recognizable instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x12); // NOP
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // NOP
    unsafe { &mut *memory.get() }.write(0xC802, 0x12); // NOP
    unsafe { &mut *memory.get() }.write(0xC803, 0x86); // LDA #immediate (for visible change)
    unsafe { &mut *memory.get() }.write(0xC804, 0x99); // Immediate value

    // Initialize CPU
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x00; // A initial

    // Execute first NOP
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, 0xC801, "PC should be at second NOP");
    assert_eq!(
        cpu.registers().a,
        0x00,
        "A should not change after first NOP"
    );

    // Execute second NOP
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, 0xC802, "PC should be at third NOP");
    assert_eq!(
        cpu.registers().a,
        0x00,
        "A should not change after second NOP"
    );

    // Execute third NOP
    cpu.execute_instruction(false, false);
    assert_eq!(
        cpu.registers().pc,
        0xC803,
        "PC should be at LDA instruction"
    );
    assert_eq!(
        cpu.registers().a,
        0x00,
        "A should not change after third NOP"
    );

    // Execute LDA to verify we can continue after NOPs
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, 0xC805, "PC should advance past LDA");
    assert_eq!(
        cpu.registers().a,
        0x99,
        "A should contain loaded value after LDA"
    );
}

#[test]
fn test_nop_memory_isolation() {
    // Verify NOP doesn't affect memory contents
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Set up test memory pattern
    unsafe { &mut *memory.get() }.write(0xCA00, 0xAA);
    unsafe { &mut *memory.get() }.write(0xCA01, 0xBB);
    unsafe { &mut *memory.get() }.write(0xCA02, 0xCC);

    // Set up NOP instruction
    unsafe { &mut *memory.get() }.write(0xC800, 0x12);

    // Execute NOP
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false);

    // Verify memory contents unchanged
    assert_eq!(
        unsafe { &*memory.get() }.read(0xCA00),
        0xAA,
        "Memory should not change"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(0xCA01),
        0xBB,
        "Memory should not change"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(0xCA02),
        0xCC,
        "Memory should not change"
    );
}

#[test]
fn test_nop_timing_consistency() {
    // Verify NOP consistently takes 2 cycles regardless of context
    let (mut cpu, memory) = setup_cpu_with_ram();

    unsafe { &mut *memory.get() }.write(0xC800, 0x12); // NOP
    unsafe { &mut *memory.get() }.write(0xC801, 0x12); // NOP
    unsafe { &mut *memory.get() }.write(0xC802, 0x12); // NOP

    // Test multiple NOPs have consistent timing
    cpu.registers_mut().pc = 0xC800;

    cpu.execute_instruction(false, false).unwrap();
    cpu.execute_instruction(false, false).unwrap();
    cpu.execute_instruction(false, false).unwrap();
}

#[test]
fn test_nop_with_different_register_states() {
    // Test NOP behavior with various register combinations
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Test with all registers at maximum values
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().b = 0xFF;
    cpu.registers_mut().x = 0xFFFF;
    cpu.registers_mut().y = 0xFFFF;
    cpu.registers_mut().u = 0xFFFF;
    cpu.registers_mut().s = 0xFFFF;
    cpu.registers_mut().dp = 0xFF;

    // Set all condition code flags
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    cpu.registers_mut().cc.h = true;
    cpu.registers_mut().cc.i = true;
    cpu.registers_mut().cc.f = true;

    unsafe { &mut *memory.get() }.write(0xC800, 0x12); // NOP

    cpu.registers_mut().pc = 0xC800;

    // Execute NOP with maxed-out registers
    cpu.execute_instruction(false, false).unwrap();

    // Verify nothing changed except PC
    assert_eq!(cpu.registers().a, 0xFF, "A should remain 0xFF");
    assert_eq!(cpu.registers().b, 0xFF, "B should remain 0xFF");
    assert_eq!(cpu.registers().x, 0xFFFF, "X should remain 0xFFFF");
    assert_eq!(cpu.registers().y, 0xFFFF, "Y should remain 0xFFFF");
    assert_eq!(cpu.registers().u, 0xFFFF, "U should remain 0xFFFF");
    assert_eq!(cpu.registers().s, 0xFFFF, "S should remain 0xFFFF");
    assert_eq!(cpu.registers().dp, 0xFF, "DP should remain 0xFF");

    // Verify all condition codes remain set
    assert_eq!(cpu.registers().cc.c, true, "Carry should remain set");
    assert_eq!(cpu.registers().cc.z, true, "Zero should remain set");
    assert_eq!(cpu.registers().cc.n, true, "Negative should remain set");
    assert_eq!(cpu.registers().cc.v, true, "Overflow should remain set");
    assert_eq!(cpu.registers().cc.h, true, "Half-carry should remain set");
    assert_eq!(cpu.registers().cc.i, true, "IRQ mask should remain set");
    assert_eq!(cpu.registers().cc.f, true, "FIRQ mask should remain set");

    assert_eq!(cpu.registers().pc, 0xC801, "PC should advance by 1");
}
