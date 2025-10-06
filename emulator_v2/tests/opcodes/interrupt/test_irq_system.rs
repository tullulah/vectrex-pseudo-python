use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const IRQ_VECTOR: u16 = 0xFFF8;
const FIRQ_VECTOR: u16 = 0xFFF6;
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
fn test_irq_basic_interrupt() {
    /* C++ Original: DoExecuteInstruction checks irqEnabled before fetching instruction
    if (irqEnabled && (CC.InterruptMask == 0)) {
        PushCCState(true);
        CC.InterruptMask = 1;
        PC = Read16(InterruptVector::Irq);
        AddCycles(19);
        return;
    }
    */
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write IRQ vector address (where to jump when IRQ occurs)
    let irq_handler_address = 0xE000;
    unsafe { &mut *memory.get() }.write(IRQ_VECTOR, (irq_handler_address >> 8) as u8);
    unsafe { &mut *memory.get() }.write(IRQ_VECTOR + 1, (irq_handler_address & 0xFF) as u8);

    // Setup: CPU state before interrupt
    cpu.registers_mut().pc = 0xD000;
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x84;
    cpu.registers_mut().cc.i = false; // I=0 means interrupts enabled

    // Execute with irq_enabled=true
    // Should trigger IRQ before fetching any instruction
    let result = cpu.execute_instruction(true, false);

    assert!(result.is_ok(), "IRQ should execute without error");

    // Verify: PC jumped to IRQ vector
    assert_eq!(
        cpu.registers.pc, irq_handler_address,
        "PC should jump to IRQ handler"
    );

    // Verify: I bit is SET (interrupts masked during handler)
    assert_eq!(
        cpu.registers.cc.i, true,
        "I bit should be set to mask further interrupts"
    );

    // Verify: Stack has entire state pushed (12 bytes)
    assert_eq!(
        cpu.registers.s,
        STACK_START - 12,
        "Stack should have 12 bytes pushed for entire state"
    );

    // Verify: E bit is set in stacked CC (indicates entire state)
    // CC is at S+0 (last push in push_cc_state)
    let s = cpu.registers.s;
    let stacked_cc = unsafe { &*memory.get() }.read(s);
    assert_eq!(stacked_cc & 0x80, 0x80, "E bit should be set in stacked CC");

    // Verify: Cycles consumed (19 for IRQ processing)
    // Note: cycles are reset to 0 before execute_instruction, so we can't check this directly
}

#[test]
fn test_irq_masked_no_interrupt() {
    // IRQ should NOT trigger if I bit is set (interrupts masked)
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write IRQ vector
    unsafe { &mut *memory.get() }.write(IRQ_VECTOR, 0xE0);
    unsafe { &mut *memory.get() }.write(IRQ_VECTOR + 1, 0x00);

    // Setup: Write a NOP instruction at PC
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x12); // NOP

    // Setup: Mask interrupts (I=1)
    cpu.registers_mut().cc.i = true;

    let original_pc = cpu.registers.pc;
    let original_sp = cpu.registers.s;

    // Execute with irq_enabled=true BUT I bit is set
    let result = cpu.execute_instruction(true, false);

    assert!(result.is_ok(), "Should execute NOP without error");

    // Verify: PC advanced normally (NOP), NOT jumped to IRQ vector
    assert_eq!(
        cpu.registers.pc,
        original_pc + 1,
        "PC should advance past NOP, not jump to IRQ vector"
    );

    // Verify: Stack unchanged (no interrupt processing)
    assert_eq!(
        cpu.registers.s, original_sp,
        "Stack should be unchanged when IRQ is masked"
    );

    // Verify: I bit still set
    assert_eq!(cpu.registers.cc.i, true, "I bit should remain set");
}

#[test]
fn test_cwai_sets_waiting_flag() {
    /* C++ Original: OpCWAI
    void OpCWAI() {
        uint8_t value = ReadOperandValue8<...>();
        CC.Value = CC.Value & value;
        PushCCState(true);
        ASSERT(!m_waitingForInterrupts);
        m_waitingForInterrupts = true;
    }
    */
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write CWAI instruction at PC
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3C); // CWAI opcode
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0xAF); // Mask: clear I, keep rest

    // Setup: Initial CC state
    cpu.registers_mut().cc.from_u8(0x50); // Some initial flags

    let original_sp = cpu.registers.s;

    // Execute CWAI
    let result = cpu.execute_instruction(false, false);

    assert!(result.is_ok(), "CWAI should execute without error");

    // Verify: CC was ANDed with mask
    let expected_cc = 0x50 & 0xAF;
    // Note: E bit will be set by push_cc_state
    assert_eq!(
        cpu.registers_mut().cc.to_u8() & 0x7F,
        expected_cc & 0x7F,
        "CC should be ANDed with immediate mask (ignoring E bit)"
    );

    // Verify: Entire state pushed to stack
    assert_eq!(
        cpu.registers.s,
        original_sp - 12,
        "CWAI should push entire state (12 bytes)"
    );

    // Verify: E bit set in stacked CC (CC is at S+0, last byte pushed)
    let stacked_cc = unsafe { &*memory.get() }.read(cpu.registers.s);
    assert_eq!(stacked_cc & 0x80, 0x80, "E bit should be set in stacked CC");

    // Verify: PC advanced past CWAI instruction (points to byte after mask)
    assert_eq!(
        cpu.registers.pc,
        RAM_START + 0x102,
        "PC should point past CWAI and its immediate operand"
    );
}

#[test]
fn test_cwai_continues_waiting_when_irq_masked() {
    // If CWAI mask cleared I bit (allowing IRQ), but then something set I bit,
    // CPU should continue waiting instead of accepting IRQ
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Execute CWAI
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x3C); // CWAI
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0xFF); // Mask: keep all bits (including I)

    cpu.registers_mut().cc.i = true; // I=1 (interrupts masked)

    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "CWAI should execute");

    let pc_after_cwai = cpu.registers.pc;

    // Now execute_instruction with irq_enabled=true but I=1
    // Should continue waiting (add nominal cycles and return)
    let result = cpu.execute_instruction(true, false);
    assert!(result.is_ok(), "Waiting should continue");

    // Verify: PC unchanged (still waiting, didn't jump to vector)
    assert_eq!(
        cpu.registers.pc, pc_after_cwai,
        "PC should remain unchanged while waiting with IRQ masked"
    );

    // Verify: I bit still set
    assert_eq!(cpu.registers.cc.i, true, "I bit should remain set");
}

#[test]
fn test_irq_not_triggered_when_irq_disabled() {
    // Even if I bit is clear (interrupts enabled in CC),
    // if irq_enabled parameter is false, IRQ should not trigger
    let (mut cpu, memory) = setup_cpu_with_ram();

    // Setup: Write NOP at PC
    cpu.registers_mut().pc = RAM_START + 0x100;
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x12); // NOP

    // Setup: Clear I bit (interrupts enabled in CC)
    cpu.registers_mut().cc.i = false;

    let original_pc = cpu.registers.pc;

    // Execute with irq_enabled=FALSE (no IRQ signal from VIA)
    let result = cpu.execute_instruction(false, false);
    assert!(result.is_ok(), "NOP should execute");

    // Verify: Normal execution (NOP executed, PC advanced)
    assert_eq!(
        cpu.registers.pc,
        original_pc + 1,
        "Should execute NOP normally when no IRQ signal"
    );
}
