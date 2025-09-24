// C++ Original: Test suite for LEA opcodes (Load Effective Address) - 1:1 Vectrexy port

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
fn test_leax_indexed_basic() {
    // C++ Original: LEAX - Load Effective Address into X (indexed) - opcode 0x30
    // C++ Original: reg = EA; if (&reg == &X || &reg == &Y) { CC.Zero = (reg == 0); }
    let mut cpu = create_test_cpu();
    
    // Set initial state
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x1000; // Base register for indexed addressing
    cpu.registers_mut().x = 0x0000; // Clear X initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially
    
    // Place LEAX ,Y instruction (indexed with no offset)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x30); // LEAX indexed
    memory_bus.borrow_mut().write(0xC801, 0xA4); // ,Y (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify X contains the effective address (Y register value)
    assert_eq!(cpu.registers().x, 0x1000);
    
    // Verify Z flag is cleared (X is non-zero) - C++ Original: Z flag affected by LEAX/LEAY
    assert!(!cpu.registers().cc.z);
    
    // Verify PC advanced correctly (2 bytes: opcode + postbyte)
    assert_eq!(cpu.registers().pc, 0xC802);
    
    // Verify cycle count - C++ Original: LEAX has 4 cycles
    assert_eq!(cycles, 4);
}

#[test]
fn test_leax_indexed_zero_result() {
    // C++ Original: LEAX with zero result should set Z flag
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().y = 0x0000; // Base register with zero value
    cpu.registers_mut().x = 0x1234; // X has non-zero value initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x30); // LEAX indexed
    memory_bus.borrow_mut().write(0xC801, 0xA4); // ,Y (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify X contains zero
    assert_eq!(cpu.registers().x, 0x0000);
    
    // Verify Z flag is set (X is zero) - C++ Original: CC.Zero = (reg == 0)
    assert!(cpu.registers().cc.z);
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

#[test]
fn test_leay_indexed_basic() {
    // C++ Original: LEAY - Load Effective Address into Y (indexed) - opcode 0x31
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x2000; // Base register for indexed addressing
    cpu.registers_mut().y = 0x0000; // Clear Y initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x31); // LEAY indexed
    memory_bus.borrow_mut().write(0xC801, 0x84); // ,X (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify Y contains the effective address
    assert_eq!(cpu.registers().y, 0x2000);
    
    // Verify Z flag is cleared (Y is non-zero) - C++ Original: Z flag affected by LEAX/LEAY
    assert!(!cpu.registers().cc.z);
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

#[test]
fn test_leas_indexed_basic() {
    // C++ Original: LEAS - Load Effective Address into S (indexed) - opcode 0x32
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().u = 0x3000; // Base register for indexed addressing
    cpu.registers_mut().s = 0x0000; // Clear S initially
    cpu.registers_mut().cc.z = true; // Set Z flag initially to verify it's not affected
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x32); // LEAS indexed
    memory_bus.borrow_mut().write(0xC801, 0xC4); // ,U (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify S contains the effective address
    assert_eq!(cpu.registers().s, 0x3000);
    
    // Verify Z flag is NOT affected by LEAS - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(cpu.registers().cc.z); // Should remain true
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

#[test]
fn test_leau_indexed_basic() {
    // C++ Original: LEAU - Load Effective Address into U (indexed) - opcode 0x33
    // C++ Original: Zero flag not affected by LEAU/LEAS
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0x4000; // Base register for indexed addressing
    cpu.registers_mut().u = 0x0000; // Clear U initially
    cpu.registers_mut().cc.z = false; // Clear Z flag initially to verify it's not affected
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x33); // LEAU indexed
    memory_bus.borrow_mut().write(0xC801, 0xE4); // ,S (no offset)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify U contains the effective address
    assert_eq!(cpu.registers().u, 0x4000);
    
    // Verify Z flag is NOT affected by LEAU - C++ Original: Zero flag not affected by LEAU/LEAS
    assert!(!cpu.registers().cc.z); // Should remain false
    
    assert_eq!(cpu.registers().pc, 0xC802);
    assert_eq!(cycles, 4);
}

#[test] 
fn test_leax_indexed_with_offset() {
    // C++ Original: LEAX with 8-bit offset - tests ReadIndexedEA calculation
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().x = 0x1000; // Base register
    cpu.registers_mut().cc.z = true; // Set Z initially to verify it changes
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x30); // LEAX indexed
    memory_bus.borrow_mut().write(0xC801, 0x88); // 8-bit offset,X
    memory_bus.borrow_mut().write(0xC802, 0x10); // offset = +16
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify X contains base + offset: 0x1000 + 0x10 = 0x1010
    assert_eq!(cpu.registers().x, 0x1010);
    
    // Verify Z flag is cleared (result is non-zero)
    assert!(!cpu.registers().cc.z);
    
    assert_eq!(cpu.registers().pc, 0xC803); // 3 bytes total
    assert_eq!(cycles, 5); // C++ Original: base 4 cycles + 1 for 8-bit offset indexed (0x08 case)
}
