use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const SWI_VECTOR: u16 = 0xFFFA;
const SWI2_VECTOR: u16 = 0xFFF4;
const SWI3_VECTOR: u16 = 0xFFF2;
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
fn test_swi_pushes_entire_state_0x3f() {
    /* C++ Original: OpSWI
    void OpSWI() {
        PushCCState(true);
        CC.InterruptMask = 1;
        CC.FastInterruptMask = 1;
        PC = Read16(InterruptVector::Swi);
        AddCycles(19);
    }
    */
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write SWI vector address
    let swi_handler = 0xE200;
    unsafe { &mut *memory.get() }.write(SWI_VECTOR, (swi_handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI_VECTOR + 1, (swi_handler & 0xFF) as u8);

    // Setup: CPU state before SWI
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3F); // SWI opcode

    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().dp = 0x56;
    cpu.registers_mut().x = 0x789A;
    cpu.registers_mut().y = 0xBCDE;
    cpu.registers_mut().u = 0x1234;
    cpu.registers_mut().cc.from_u8(0x25); // Some flags

    let original_sp = cpu.registers.s;

    // Execute SWI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "SWI should execute without error");

    // Verify: PC jumped to SWI vector
    assert_eq!(
        cpu.registers.pc, swi_handler,
        "PC should jump to SWI handler"
    );

    // Verify: I and F bits are SET (both interrupts masked)
    assert_eq!(cpu.registers.cc.i, true, "I bit should be set after SWI");
    assert_eq!(cpu.registers.cc.f, true, "F bit should be set after SWI");

    // Verify: Stack has entire state pushed (12 bytes)
    assert_eq!(
        cpu.registers.s,
        original_sp - 12,
        "Stack should have 12 bytes pushed"
    );

    // MC6809 Push order: PC, U, Y, X, DP, B, A, CC
    // Memory layout (Vectrexy: LOW byte first):
    // S+11: PC low (first byte of PC push)
    // S+10: PC high
    // S+9:  U low
    // S+8:  U high
    // S+7:  Y low
    // S+6:  Y high
    // S+5:  X low
    // S+4:  X high
    // S+3:  DP
    // S+2:  B
    // S+1:  A
    // S+0:  CC (last push) ← S points here

    let s = cpu.registers.s;

    // PC is pushed with Vectrexy order (LOW first, points to NEXT instruction after SWI)
    let expected_pc = RAM_START + 0x101; // After 1-byte SWI
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 11),
        (expected_pc & 0xFF) as u8,
        "PC low should be stacked at S+11"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 10),
        (expected_pc >> 8) as u8,
        "PC high should be stacked at S+10"
    );

    // U is pushed with Vectrexy order (LOW first)
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 9),
        0x34,
        "U low should be stacked at S+9"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 8),
        0x12,
        "U high should be stacked at S+8"
    );

    // Y is pushed with Vectrexy order (LOW first)
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 7),
        0xDE,
        "Y low should be stacked at S+7"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 6),
        0xBC,
        "Y high should be stacked at S+6"
    );

    // X is pushed with Vectrexy order (LOW first)
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 5),
        0x9A,
        "X low should be stacked at S+5"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 4),
        0x78,
        "X high should be stacked at S+4"
    );

    // DP
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 3),
        0x56,
        "DP should be stacked at S+3"
    );

    // B
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 2),
        0x34,
        "B should be stacked at S+2"
    );

    // A
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 1),
        0x12,
        "A should be stacked at S+1"
    );

    // CC (with E bit set)
    let stacked_cc = unsafe { &*memory.get() }.read(s);
    assert_eq!(
        stacked_cc & 0x80,
        0x80,
        "E bit should be set in stacked CC at S+0"
    );
    assert_eq!(
        stacked_cc & 0x7F,
        0x25,
        "CC original value (without E bit) should be 0x25"
    );
}

#[test]
fn test_swi2_no_interrupt_masking_0x10_0x3f() {
    /* C++ Original: OpSWI2
    void OpSWI2() {
        PushCCState(true);
        PC = Read16(InterruptVector::Swi2);
        AddCycles(19);
        // Note: Does NOT set I or F bits
    }
    */
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write SWI2 vector
    let swi2_handler = 0xE300;
    unsafe { &mut *memory.get() }.write(SWI2_VECTOR, (swi2_handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI2_VECTOR + 1, (swi2_handler & 0xFF) as u8);

    // Setup: Write SWI2 instruction (2 bytes: 0x10 0x3F)
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x10); // Page 1 prefix
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x3F); // SWI2 opcode

    // Setup: Initial CC with I and F clear
    cpu.registers_mut().cc.from_u8(0x00);
    cpu.registers_mut().cc.i = false;
    cpu.registers_mut().cc.f = false;

    let original_sp = cpu.registers.s;

    // Execute SWI2
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "SWI2 should execute without error");

    // Verify: PC jumped to SWI2 vector
    assert_eq!(
        cpu.registers.pc, swi2_handler,
        "PC should jump to SWI2 handler"
    );

    // Verify: I and F bits are NOT set (key difference from SWI)
    assert_eq!(cpu.registers.cc.i, false, "I bit should NOT be set by SWI2");
    assert_eq!(cpu.registers.cc.f, false, "F bit should NOT be set by SWI2");

    // Verify: Stack has entire state pushed (same as SWI)
    assert_eq!(
        cpu.registers.s,
        original_sp - 12,
        "Stack should have 12 bytes pushed"
    );

    // MC6809 Push order: PC, U, Y, X, DP, B, A, CC
    // Memory layout (Vectrexy: LOW byte first):
    // S+11: PC low (first byte of PC push)
    // S+10: PC high
    // ...
    // S+0:  CC (last push) ← S points here

    let s = cpu.registers.s;

    // Verify: E bit is set in stacked CC at S+0
    let stacked_cc = unsafe { &*memory.get() }.read(s);
    assert_eq!(
        stacked_cc & 0x80,
        0x80,
        "E bit should be set in stacked CC at S+0"
    );

    // Verify: PC on stack points past 2-byte SWI2 instruction (Vectrexy order)
    let expected_return_pc = RAM_START + 0x102; // After 2-byte SWI2
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 11),
        (expected_return_pc & 0xFF) as u8,
        "Return PC low should be correct at S+11"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 10),
        (expected_return_pc >> 8) as u8,
        "Return PC high should be correct at S+10"
    );
}

#[test]
fn test_swi3_no_interrupt_masking_0x11_0x3f() {
    /* C++ Original: OpSWI3
    void OpSWI3() {
        PushCCState(true);
        PC = Read16(InterruptVector::Swi3);
        AddCycles(19);
        // Note: Does NOT set I or F bits (like SWI2)
    }
    */
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write SWI3 vector
    let swi3_handler = 0xE400;
    unsafe { &mut *memory.get() }.write(SWI3_VECTOR, (swi3_handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI3_VECTOR + 1, (swi3_handler & 0xFF) as u8);

    // Setup: Write SWI3 instruction (2 bytes: 0x11 0x3F)
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x11); // Page 2 prefix
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x3F); // SWI3 opcode

    // Setup: Initial CC with some flags set
    cpu.registers_mut().cc.from_u8(0x12);
    cpu.registers_mut().cc.i = true; // Set I initially
    cpu.registers_mut().cc.f = true; // Set F initially

    let original_sp = cpu.registers.s;

    // Execute SWI3
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "SWI3 should execute without error");

    // Verify: PC jumped to SWI3 vector
    assert_eq!(
        cpu.registers.pc, swi3_handler,
        "PC should jump to SWI3 handler"
    );

    // Verify: I and F bits UNCHANGED (SWI3 doesn't modify them)
    // IMPORTANT: Current implementation behavior - SWI3 doesn't clear I/F
    // They remain whatever they were before
    assert_eq!(
        cpu.registers.cc.i, true,
        "I bit should remain unchanged by SWI3"
    );
    assert_eq!(
        cpu.registers.cc.f, true,
        "F bit should remain unchanged by SWI3"
    );

    // Verify: Stack has entire state pushed
    assert_eq!(
        cpu.registers.s,
        original_sp - 12,
        "Stack should have 12 bytes pushed"
    );

    // Verify: E bit is set in stacked CC (at S+0, not S+11)
    let stacked_cc = unsafe { &*memory.get() }.read(cpu.registers.s);
    assert_eq!(
        stacked_cc & 0x80,
        0x80,
        "E bit should be set in stacked CC at S+0"
    );
}

#[test]
fn test_swi_variants_use_different_vectors() {
    // Verify that SWI, SWI2, SWI3 jump to different interrupt vectors
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write distinct handler addresses for each vector
    let swi_addr = 0xE200;
    let swi2_addr = 0xE300;
    let swi3_addr = 0xE400;

    unsafe { &mut *memory.get() }.write(SWI_VECTOR, (swi_addr >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI_VECTOR + 1, (swi_addr & 0xFF) as u8);

    unsafe { &mut *memory.get() }.write(SWI2_VECTOR, (swi2_addr >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI2_VECTOR + 1, (swi2_addr & 0xFF) as u8);

    unsafe { &mut *memory.get() }.write(SWI3_VECTOR, (swi3_addr >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI3_VECTOR + 1, (swi3_addr & 0xFF) as u8);

    // Test SWI (0x3F)
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3F);
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, swi_addr, "SWI should use 0xFFFA vector");

    // Reset for SWI2
    cpu.reset();
    cpu.registers_mut().s = STACK_START;

    // Test SWI2 (0x10 0x3F)
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x10);
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x3F);
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, swi2_addr, "SWI2 should use 0xFFF4 vector");

    // Reset for SWI3
    cpu.reset();
    cpu.registers_mut().s = STACK_START;

    // Test SWI3 (0x11 0x3F)
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x11);
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x3F);
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, swi3_addr, "SWI3 should use 0xFFF2 vector");
}

#[test]
fn test_swi_interrupt_masking_behavior() {
    // Verify the critical difference between SWI and SWI2/SWI3:
    // SWI sets both I and F bits (masks all interrupts)
    // SWI2/SWI3 leave I and F unchanged

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup vectors (all point to same handler for simplicity)
    let handler = 0xE000;
    unsafe { &mut *memory.get() }.write(SWI_VECTOR, (handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI_VECTOR + 1, (handler & 0xFF) as u8);
    unsafe { &mut *memory.get() }.write(SWI2_VECTOR, (handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(SWI2_VECTOR + 1, (handler & 0xFF) as u8);

    // Test 1: SWI with interrupts initially enabled
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3F); // SWI
    cpu.registers_mut().cc.i = false;
    cpu.registers_mut().cc.f = false;

    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers.cc.i, true, "SWI should SET I bit");
    assert_eq!(cpu.registers.cc.f, true, "SWI should SET F bit");

    // Reset for SWI2
    cpu.reset();
    cpu.registers_mut().s = STACK_START;

    // Test 2: SWI2 with interrupts initially enabled
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x10); // Page 1
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x3F); // SWI2
    cpu.registers_mut().cc.i = false;
    cpu.registers_mut().cc.f = false;

    cpu.execute_instruction(false, false);

    assert_eq!(cpu.registers.cc.i, false, "SWI2 should NOT set I bit");
    assert_eq!(cpu.registers.cc.f, false, "SWI2 should NOT set F bit");
}
