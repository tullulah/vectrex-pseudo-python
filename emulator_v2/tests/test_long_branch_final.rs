// Test comprensivo simplificado para long branch operations
// Validación básica de las operaciones implementadas

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    let mut cpu = Cpu6809::new(memory_bus);
    // Don't call reset() - just initialize PC directly to avoid needing BIOS ROM
    cpu.registers_mut().pc = RAM_START;
    
    cpu
}

#[test]
fn test_lbra_basic() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBRA instruction: 0x16 0x01 0x00 (branch forward +256)
    memory_bus.borrow_mut().write(RAM_START, 0x16);     // LBRA
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // High byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // Low byte
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 5, "LBRA should take 5 cycles");
    assert_eq!(cpu.registers().pc, RAM_START + 3 + 0x0100, "LBRA should branch correctly");
}

#[test]
fn test_lbeq_when_zero_set() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBEQ instruction: 0x10 0x27 0x00 0x80 (branch forward +128)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x27); // LBEQ
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x80); // Low byte
    
    // Set Z flag
    cpu.registers_mut().cc.z = true;
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 6, "LBEQ should take 6 cycles when branching");
    assert_eq!(cpu.registers().pc, RAM_START + 4 + 0x0080, "LBEQ should branch when Z=1");
}

#[test] 
fn test_lbeq_when_zero_clear() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBEQ instruction: 0x10 0x27 0x00 0x80 (branch forward +128)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x27); // LBEQ
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x80); // Low byte
    
    // Clear Z flag
    cpu.registers_mut().cc.z = false;
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 5, "LBEQ should take 5 cycles when not branching");
    assert_eq!(cpu.registers().pc, RAM_START + 4, "LBEQ should not branch when Z=0");
}

#[test]
fn test_lbrn_never_branches() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBRN instruction: 0x10 0x21 0x01 0x00 (would branch forward +256)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x21); // LBRN
    memory_bus.borrow_mut().write(RAM_START + 2, 0x01); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x00); // Low byte
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 5, "LBRN should take 5 cycles (never branches)");
    assert_eq!(cpu.registers().pc, RAM_START + 4, "LBRN should never branch");
}

#[test]
fn test_lbhi_unsigned_higher() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBHI instruction: 0x10 0x22 0x00 0x40 (branch forward +64)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x22); // LBHI
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x40); // Low byte
    
    // Set condition for unsigned higher: C=0 and Z=0
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().cc.z = false;
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 6, "LBHI should take 6 cycles when branching");
    assert_eq!(cpu.registers().pc, RAM_START + 4 + 0x0040, "LBHI should branch when unsigned higher");
}

#[test]
fn test_lbcc_carry_clear() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBCC instruction: 0x10 0x24 0x00 0x60 (branch forward +96)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x24); // LBCC
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x60); // Low byte
    
    // Set condition for carry clear: C=0
    cpu.registers_mut().cc.c = false;
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 6, "LBCC should take 6 cycles when branching");
    assert_eq!(cpu.registers().pc, RAM_START + 4 + 0x0060, "LBCC should branch when carry clear");
}

#[test]
fn test_lbge_signed_greater_equal() {
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBGE instruction: 0x10 0x2C 0x00 0x20 (branch forward +32)
    memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x2C); // LBGE
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 3, 0x20); // Low byte
    
    // Set condition for signed greater/equal: N=V
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = true;
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 6, "LBGE should take 6 cycles when branching");
    assert_eq!(cpu.registers().pc, RAM_START + 4 + 0x0020, "LBGE should branch when N=V");
}

#[test]
fn test_negative_offset_lbra() {
    let mut cpu = create_test_cpu();
    
    // Start at higher address to test backward jump
    cpu.registers_mut().pc = RAM_START + 0x0200;
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBRA instruction with negative offset (-256 = 0xFF00)
    memory_bus.borrow_mut().write(RAM_START + 0x0200, 0x16);     // LBRA
    memory_bus.borrow_mut().write(RAM_START + 0x0200 + 1, 0xFF); // High byte
    memory_bus.borrow_mut().write(RAM_START + 0x0200 + 2, 0x00); // Low byte
    
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 5, "LBRA with negative offset should take 5 cycles");
    assert_eq!(cpu.registers().pc, RAM_START + 0x0200 + 3 - 0x0100, "LBRA should handle negative offset correctly");
}

#[test]
fn test_all_long_branch_opcodes_recognized() {
    // This test verifies that all long branch opcodes are properly recognized
    // and don't result in illegal instruction panics
    
    let long_branch_opcodes = vec![
        0x21, // LBRN
        0x22, // LBHI  
        0x23, // LBLS
        0x24, // LBCC
        0x25, // LBCS
        0x26, // LBNE
        0x27, // LBEQ
        0x28, // LBVC
        0x29, // LBVS
        0x2A, // LBPL
        0x2B, // LBMI
        0x2C, // LBGE
        0x2D, // LBLT
        0x2E, // LBGT
        0x2F, // LBLE
    ];
    
    for opcode in long_branch_opcodes {
        let mut cpu = create_test_cpu();
        
        let memory_bus = cpu.memory_bus().clone();
        // Write long branch instruction: 0x10 [opcode] 0x00 0x10 (branch forward +16)
        memory_bus.borrow_mut().write(RAM_START, 0x10);     // Page 1 prefix
        memory_bus.borrow_mut().write(RAM_START + 1, opcode); // Long branch opcode
        memory_bus.borrow_mut().write(RAM_START + 2, 0x00); // High byte
        memory_bus.borrow_mut().write(RAM_START + 3, 0x10); // Low byte
        
        // Execute should not panic (instruction exists)
        let cycles = cpu.execute_instruction(false, false);
        
        // All long branches should take either 5 or 6 cycles
        assert!(cycles == 5 || cycles == 6, "Long branch opcode 0x{:02X} should take 5 or 6 cycles, got {}", opcode, cycles);
    }
}

#[test]
fn test_lbra_opcode_recognized() {
    // Test that LBRA (0x16) is recognized as a valid instruction
    let mut cpu = create_test_cpu();
    
    let memory_bus = cpu.memory_bus().clone();
    // Write LBRA instruction: 0x16 0x00 0x04 (branch forward +4)
    memory_bus.borrow_mut().write(RAM_START, 0x16);     // LBRA
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00); // High byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x04); // Low byte
    
    // Should not panic - instruction should be recognized
    let cycles = cpu.execute_instruction(false, false);
    assert_eq!(cycles, 5, "LBRA should take exactly 5 cycles");
    assert_eq!(cpu.registers().pc, RAM_START + 3 + 4, "LBRA should branch correctly");
}