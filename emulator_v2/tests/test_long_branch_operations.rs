// Test suite for long branch operations: LBRA, LBNE, LBEQ, LBCC, LBCS, LBHI, LBLS, LBVC, LBVS, LBPL, LBMI, LBGE, LBLT, LBGT, LBLE, LBRN
// C++ Reference: OpLBRA() and OpLongBranch(condFunc) from Vectrexy Cpu.cpp

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800; // RAM area for tests

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_lbra_basic_functionality() {
    // Test LBRA (Long Branch Always) - opcode 0x16
    // C++ Original: void OpLBRA() { int16_t offset = ReadRelativeOffset16(); PC += offset; }
    
    let mut cpu = create_test_cpu();
    
    // Setup: PC at RAM_START, LBRA +100 (0x0064)
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBRA instruction: 0x16 0x00 0x64 (branch forward +100)  
    memory_bus.borrow_mut().write(RAM_START, 0x16); // LBRA opcode
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00); // Offset high byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x64); // Offset low byte (+100)
    
    // Execute LBRA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cycles, 5, "LBRA should take exactly 5 cycles");
    assert_eq!(cpu.registers().pc, RAM_START + 3 + 100, "PC should be updated with offset after instruction");
}

#[test]
fn test_lbeq_conditional_branching() {
    // Test LBEQ (0x1027) - Long Branch if Equal
    // C++ Original: OpLongBranch([this] { return CC.Zero != 0; })
    
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test LBEQ when Z flag is SET (should branch)
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().cc.z = true; // Set Zero flag
    
    // Write LBEQ instruction: 0x10 0x27 0x00 0x50 (branch forward +80)
    memory_bus.borrow_mut().write(RAM_START, 0x10); // Page prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x27); // LBEQ opcode
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // Offset high byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x50); // Offset low byte (+80)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 6, "LBEQ should take 6 cycles when branch is taken (5 + 1 extra)");
    assert_eq!(cpu.registers().pc, RAM_START + 4 + 80, "PC should branch when Z flag is set");
}

#[test]
fn test_lbrn_never_branches() {
    // Test LBRN (0x1021) - Long Branch Never (always consumes offset but never branches)
    // C++ Original: OpLongBranch([] { return false; })
    
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    cpu.registers_mut().pc = RAM_START;
    
    // Write LBRN instruction: 0x10 0x21 0x10 0x00 (would branch forward +4096 if condition was true)
    memory_bus.borrow_mut().write(RAM_START, 0x10); // Page prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x21); // LBRN opcode
    memory_bus.borrow_mut().write(RAM_START + 2, 0x10); // Offset high byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x00); // Offset low byte (+4096)
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cycles, 5, "LBRN should take 5 cycles (never takes extra cycle)");
    assert_eq!(cpu.registers().pc, RAM_START + 4, "PC should advance past instruction but never branch");
}







