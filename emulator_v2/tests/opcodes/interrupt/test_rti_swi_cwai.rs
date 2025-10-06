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
fn test_swi_pushes_entire_state_0x3F() {
    // SWI - Software Interrupt
    // MC6809 Spec: Pushes CC, A, B, DP, X, Y, U, PC on stack (sets E bit)

    let (mut cpu, memory) = setup_cpu_with_ram();
    let ram = unsafe { &mut *memory.get() };

    // Setup initial CPU state
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().dp = 0x56;
    cpu.registers_mut().x = 0x789A;
    cpu.registers_mut().y = 0xBCDE;
    cpu.registers_mut().u = 0xF012;
    cpu.registers_mut().cc.from_u8(0x00); // Clear CC initially

    // Write SWI opcode (0x3F) at current PC
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3F);

    // Write SWI vector (0xFFFA) - point to some handler
    let swi_handler = 0xE000;
    unsafe { &mut *memory.get() }.write(0xFFFA, (swi_handler >> 8) as u8);
    unsafe { &mut *memory.get() }.write(0xFFFB, (swi_handler & 0xFF) as u8);

    // Execute SWI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "SWI should execute without error");

    // Verify I bit (interrupt mask) is set
    assert_eq!(cpu.registers.cc.i, true, "I bit should be set");

    // Verify F bit (fast interrupt mask) is set
    assert_eq!(cpu.registers.cc.f, true, "F bit should be set");

    // Verify stack pointer moved down 12 bytes (entire state)
    assert_eq!(
        cpu.registers.s,
        STACK_START - 12,
        "Stack should have 12 bytes pushed"
    );

    // Stack grows down: FIRST push is at highest address (S+11), LAST push is at lowest address (S)
    // Vectrexy Push16 order: LOW byte first, HIGH byte second
    // PC pushed: LOW at S+11, HIGH at S+10
    // Memory layout (growing down):
    // S+11: PC low  (first byte of PC push)
    // S+10: PC high (second byte of PC push)
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

    // PC (first push, highest address) - should point to next instruction after SWI
    let expected_pc = RAM_START + 0x101;
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 11),
        (expected_pc & 0xFF) as u8,
        "PC low byte should be at S+11"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 10),
        (expected_pc >> 8) as u8,
        "PC high byte should be at S+10"
    );

    // U
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 9),
        (0xF012 & 0xFF) as u8,
        "U low byte should be at S+9"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 8),
        (0xF012 >> 8) as u8,
        "U high byte should be at S+8"
    );

    // Y
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 7),
        (0xBCDE & 0xFF) as u8,
        "Y low byte should be at S+7"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 6),
        (0xBCDE >> 8) as u8,
        "Y high byte should be at S+6"
    );

    // X
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 5),
        (0x789A & 0xFF) as u8,
        "X low byte should be at S+5"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 4),
        (0x789A >> 8) as u8,
        "X high byte should be at S+4"
    );

    // DP
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 3),
        0x56,
        "DP should be at S+3"
    );

    // B
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 2),
        0x34,
        "B should be at S+2"
    );

    // A
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 1),
        0x12,
        "A should be at S+1"
    );

    // CC (last push, lowest address)
    let stacked_cc = unsafe { &*memory.get() }.read(s);
    assert_eq!(
        stacked_cc & 0x80,
        0x80,
        "E bit should be set in stacked CC at S+0"
    );

    // Verify PC jumped to handler
    assert_eq!(
        cpu.registers.pc, swi_handler,
        "PC should jump to SWI vector address"
    );
}

#[test]
fn test_rti_pops_entire_state_0x3B() {
    // RTI - Return from Interrupt
    // MC6809 Spec: Pops entire state if E bit set

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup stack with full interrupt frame (E bit set)
    // After SWI push with Vectrexy order (LOW byte first per push16):
    // Push writes bytes in this order: push8(LOW) writes to SP-1, push8(HIGH) writes to SP-2
    // SP ends pointing to HIGH byte. When popping, we read HIGH first (from SP), then LOW (from SP+1)
    // Memory layout AFTER push (SP points to HIGH byte of each 16-bit value):
    // S+11: PC high ← (last byte of PC push, SP advanced past it)
    // S+10: PC low  ← SP after PC push points here
    // S+9:  U high
    // S+8:  U low   ← SP after U push points here
    // S+7:  Y high
    // S+6:  Y low   ← SP after Y push points here
    // S+5:  X high
    // S+4:  X low   ← SP after X push points here
    // S+3:  DP
    // S+2:  B
    // S+1:  A
    // S+0:  CC ← S points here (final SP)

    cpu.registers_mut().s = STACK_START - 12;

    // Write stack content as SWI would leave it
    // RTI pops from S upwards: CC, A, B, DP, X, Y, U, PC
    let s = cpu.registers.s;
    unsafe { &mut *memory.get() }.write(s, 0x85); // S+0: CC with E bit set (0x80) and Z+C bits
    unsafe { &mut *memory.get() }.write(s + 1, 0xAA); // S+1: A
    unsafe { &mut *memory.get() }.write(s + 2, 0xBB); // S+2: B
    unsafe { &mut *memory.get() }.write(s + 3, 0xCC); // S+3: DP
    unsafe { &mut *memory.get() }.write(s + 4, 0x12); // S+4: X high ← SP points here when popping X
    unsafe { &mut *memory.get() }.write(s + 5, 0x34); // S+5: X low (0x1234)
    unsafe { &mut *memory.get() }.write(s + 6, 0x56); // S+6: Y high
    unsafe { &mut *memory.get() }.write(s + 7, 0x78); // S+7: Y low (0x5678)
    unsafe { &mut *memory.get() }.write(s + 8, 0x9A); // S+8: U high
    unsafe { &mut *memory.get() }.write(s + 9, 0xBC); // S+9: U low (0x9ABC)
    unsafe { &mut *memory.get() }.write(s + 10, 0xE0); // S+10: PC high
    unsafe { &mut *memory.get() }.write(s + 11, 0x00); // S+11: PC low (0xE000)

    // Write RTI opcode (0x3B) at current PC
    let current_pc = RAM_START + 0x200;
    cpu.registers_mut().pc = current_pc;
    unsafe { &mut *memory.get() }.write(current_pc, 0x3B);

    // Execute RTI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "RTI should execute without error");

    // Verify all registers restored
    assert_eq!(cpu.registers.a, 0xAA, "A should be restored");
    assert_eq!(cpu.registers.b, 0xBB, "B should be restored");
    assert_eq!(cpu.registers.dp, 0xCC, "DP should be restored");
    assert_eq!(cpu.registers.x, 0x1234, "X should be restored");
    assert_eq!(cpu.registers.y, 0x5678, "Y should be restored");
    assert_eq!(cpu.registers.u, 0x9ABC, "U should be restored");
    assert_eq!(cpu.registers.pc, 0xE000, "PC should be restored");

    // Verify CC restored (E bit, Z bit, C bit)
    assert_eq!(cpu.registers.cc.e, true, "E bit should be set");
    assert_eq!(cpu.registers.cc.z, true, "Z bit should be set");
    assert_eq!(cpu.registers.cc.c, true, "C bit should be set");

    // Verify stack pointer restored
    assert_eq!(
        cpu.registers.s, STACK_START,
        "Stack should be restored to original position"
    );
}

#[test]
fn test_rti_firq_mode_0x3B() {
    // RTI - Return from Interrupt (FIRQ mode, E bit clear)
    // MC6809 Spec: Only pops CC and PC if E bit is clear

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup stack with minimal interrupt frame (E bit clear)
    // RTI pops: CC, then PC
    // Memory layout after FIRQ-style push (push16 leaves SP pointing to HIGH byte):
    // S+2: PC low
    // S+1: PC high ← SP after PC push points here
    // S+0: CC (last push) ← S points here

    let s = STACK_START - 3;
    cpu.registers_mut().s = s;

    // Write stack content (FIRQ only pushes CC + PC)
    unsafe { &mut *memory.get() }.write(s, 0x01); // S+0: CC without E bit (just C bit set)
    unsafe { &mut *memory.get() }.write(s + 1, 0xF0); // S+1: PC high (0xF000) ← SP points here when popping PC
    unsafe { &mut *memory.get() }.write(s + 2, 0x00); // S+2: PC low

    // Write RTI opcode (0x3B) at current PC
    let current_pc = RAM_START + 0x200;
    cpu.registers_mut().pc = current_pc;
    unsafe { &mut *memory.get() }.write(current_pc, 0x3B);

    // Set some register values that should NOT be restored
    cpu.registers_mut().a = 0x99;
    cpu.registers_mut().b = 0x88;
    cpu.registers_mut().x = 0x7777;

    // Execute RTI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "RTI should execute without error");

    // Verify only PC and CC restored
    assert_eq!(cpu.registers.pc, 0xF000, "PC should be restored");
    assert_eq!(
        cpu.registers.cc.c, true,
        "C bit should be set from stacked CC"
    );
    assert_eq!(cpu.registers.cc.e, false, "E bit should be clear");

    // Verify other registers NOT modified
    assert_eq!(cpu.registers.a, 0x99, "A should not be modified");
    assert_eq!(cpu.registers.b, 0x88, "B should not be modified");
    assert_eq!(cpu.registers.x, 0x7777, "X should not be modified");

    // Verify stack pointer moved only 3 bytes
    assert_eq!(cpu.registers.s, STACK_START, "Stack should move 3 bytes");
}

#[test]
fn test_swi_rti_roundtrip() {
    // Full cycle: SWI → ISR → RTI
    // Uses MemoryBus for correct address translation

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial state
    cpu.registers_mut().pc = RAM_START + 0x200;
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x24;
    cpu.registers_mut().x = 0xABCD;
    cpu.registers_mut().cc.from_u8(0x02); // Only overflow bit set (V=bit 1)

    // Write SWI opcode at PC
    cpu.memory_bus_mut().write(RAM_START + 0x200, 0x3F);

    // Setup SWI vector to point to ISR at RAM_START + 0x300
    // SWI vector is at 0xFFFA-0xFFFB (big-endian: high byte first)
    let isr_address = RAM_START + 0x300;
    cpu.memory_bus_mut().write(0xFFFA, (isr_address >> 8) as u8); // High byte
    cpu.memory_bus_mut()
        .write(0xFFFB, (isr_address & 0xFF) as u8); // Low byte

    // Write RTI opcode at ISR
    cpu.memory_bus_mut().write(RAM_START + 0x300, 0x3B);

    // Execute SWI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "SWI should execute");

    // Verify we're at ISR
    assert_eq!(
        cpu.registers().pc,
        isr_address,
        "PC should be at ISR (0x{:04X})",
        isr_address
    );

    // Execute RTI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "RTI should execute");

    // Verify we're back at original PC + 1 (after SWI opcode)
    assert_eq!(
        cpu.registers().pc,
        RAM_START + 0x201,
        "PC should return to next instruction"
    );

    // Verify all registers restored
    assert_eq!(cpu.registers().a, 0x42, "A should be preserved");
    assert_eq!(cpu.registers().b, 0x24, "B should be preserved");
    assert_eq!(cpu.registers().x, 0xABCD, "X should be preserved");
    assert_eq!(
        cpu.registers().cc.v,
        true,
        "Overflow bit should be preserved"
    );

    // Verify stack returned to original position
    assert_eq!(cpu.registers().s, STACK_START, "Stack should be balanced");
}

#[test]
fn test_cwai_clears_cc_and_pushes_state_0x3C() {
    // CWAI - Clear and Wait for Interrupt
    // MC6809 Spec: AND CC with immediate, push entire state, wait for interrupt

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup initial CC with some bits set
    cpu.registers_mut().cc.from_u8(0xFF); // All flags set

    // Setup registers to verify they're pushed
    cpu.registers_mut().pc = RAM_START + 0x400;
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().b = 0x22;
    cpu.registers_mut().dp = 0x33;
    cpu.registers_mut().x = 0x4455;
    cpu.registers_mut().y = 0x6677;
    cpu.registers_mut().u = 0x8899;

    // Write CWAI opcode (0x3C) with mask 0x00 (clear all CC bits)
    unsafe { &mut *memory.get() }.write(RAM_START + 0x400, 0x3C);
    unsafe { &mut *memory.get() }.write(RAM_START + 0x401, 0x00); // Mask: clear all bits

    // Execute CWAI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "CWAI should execute");

    // Verify stack pointer moved down 12 bytes (entire state)
    assert_eq!(
        cpu.registers.s,
        STACK_START - 12,
        "Stack should have 12 bytes pushed"
    );

    // Verify entire state pushed (same as SWI)
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

    // PC should be PC+2 (next instruction after CWAI opcode+immediate)
    let expected_pc = RAM_START + 0x402;
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 11),
        (expected_pc & 0xFF) as u8,
        "PC low should be at S+11"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 10),
        (expected_pc >> 8) as u8,
        "PC high should be at S+10"
    );

    // Verify registers pushed at correct offsets
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 3),
        0x33,
        "DP should be at S+3"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 2),
        0x22,
        "B should be at S+2"
    );
    assert_eq!(
        unsafe { &*memory.get() }.read(s + 1),
        0x11,
        "A should be at S+1"
    );

    // CC should have E bit set (0x80) even though we cleared everything
    // CC is at lowest address (S+0)
    let stacked_cc = unsafe { &*memory.get() }.read(s);
    assert_eq!(
        stacked_cc & 0x80,
        0x80,
        "E bit should be set in stacked CC at S+0"
    );
}

#[test]
fn test_cwai_with_partial_mask_0x3C() {
    // CWAI with mask that preserves some CC bits

    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup CC: 0b11111111 (all bits set)
    cpu.registers_mut().cc.from_u8(0xFF);
    cpu.registers_mut().pc = RAM_START + 0x500;

    // Write CWAI with mask 0x55 (0b01010101) - preserves alternating bits
    unsafe { &mut *memory.get() }.write(RAM_START + 0x500, 0x3C);
    unsafe { &mut *memory.get() }.write(RAM_START + 0x501, 0x55);

    // Execute CWAI
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "CWAI should execute");

    // Verify CC was ANDed: 0xFF & 0x55 = 0x55, then E bit set = 0xD5
    let s = cpu.registers.s;

    // CC is at lowest address (S+0) in MC6809 push order
    let stacked_cc = unsafe { &*memory.get() }.read(s);

    // Expected: (0xFF & 0x55) | 0x80 = 0x55 | 0x80 = 0xD5
    assert_eq!(stacked_cc, 0xD5, "CC should be 0xFF & 0x55 with E bit set");
}
