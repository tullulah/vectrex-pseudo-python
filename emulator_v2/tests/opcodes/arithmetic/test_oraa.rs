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

