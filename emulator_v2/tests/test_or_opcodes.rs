// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_oraa_immediate_basic() {
    // Test ORAA #$0F - OR A with immediate value
    // C++ Original: OpOR sets reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xF0; // Initial A = 11110000
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    
    // Setup: ORAA #$0F instruction at 0xC800 (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8A); // ORAA immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x0F); // OR mask
    cpu.registers_mut().pc = 0xC800;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0xFF, "A should be 0xF0 | 0x0F = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by OR)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "ORAA immediate should take 2 cycles");
}

#[test]
fn test_oraa_immediate_zero_result() {
    // Test ORAA #$00 with A=0x00 - OR resulting in zero
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x00; // A starts as zero
    
    // Setup: ORAA #$00 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8A); // ORAA immediate
    memory_bus.borrow_mut().write(0xC801, 0x00); // OR with zero
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should remain 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_orab_immediate_basic() {
    // Test ORAB #$AA - OR B with immediate value
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0x55; // Initial B = 01010101
    
    // Setup: ORAB #$AA instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCA); // ORAB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0xAA); // OR mask 10101010
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xFF, "B should be 0x55 | 0xAA = 0xFF");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_ldaa_immediate_negative() {
    // Test LDAA #$80 - Load A with negative value (sets N flag)
    // Note: This test verifies that existing LD opcodes work correctly
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x86); // LDAA immediate
    memory_bus.borrow_mut().write(0xC801, 0x80); // Negative value (bit 7 set)
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80);
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (bit 7 = 1)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_ldaa_immediate_zero() {
    // Test LDAA #$00 - Load A with zero (sets Z flag)
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x86); // LDAA immediate
    memory_bus.borrow_mut().write(0xC801, 0x00); // Zero value
    cpu.registers_mut().pc = 0xC800;
    
    cpu.registers_mut().a = 0xFF; // Set A to non-zero initially
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (value = 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_staa_direct_basic() {
    // Test STAA $50 - Store A to direct address
    // C++ Original: OpST writes sourceReg to EA, same flag updates as LD
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x42; // Value to store
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM page (0xC8xx)
    
    // Setup: STAA $50 instruction
    // Direct addressing: EA = DP:offset = 0xC8:0x50 = 0xC850 (within RAM range)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x97); // STAA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x50); // Direct page offset
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    // Verify memory was written at effective address 0xC850
    assert_eq!(memory_bus.borrow().read(0xC850), 0x42, "Memory at 0xC850 should contain 0x42");
    
    // Verify flags based on stored value
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (value != 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by ST)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
}