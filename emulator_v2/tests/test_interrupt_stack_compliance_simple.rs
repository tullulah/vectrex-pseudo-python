// Interrupt Stack Order Compliance Tests - SIMPLIFIED VERSION
// Tests for CWAI (0x3C), SWI (0x3F), RTI (0x3B) stack operations
// Following Vectrexy 1:1 compliance rules for interrupt stack handling

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // Add additional RAM for ROM/vector space (0xE000-0xFFFF) to allow vector writes in tests
    let rom_ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(rom_ram, (0xE000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus.clone());
    // Don't call reset() as it requires ROM vectors - just set PC manually
    cpu.registers_mut().pc = RAM_START;
    
    cpu
}

// ======= CWAI Stack Order Test =======

#[test]
fn test_cwai_basic_stack_push() {
    // Test that CWAI pushes the complete register state to stack
    let mut cpu = create_test_cpu();
    
    // Set up known register values
    cpu.registers_mut().pc = RAM_START; // PC must be in RAM area
    cpu.registers_mut().u = 0x5678;
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().b = 0xBB;
    cpu.registers_mut().cc.from_u8(0x55);
    cpu.registers_mut().s = 0xC900; // Stack starts here
    
    // Write CWAI instruction: 0x3C followed by immediate mask value
    {
        let memory_bus = cpu.memory_bus();
        memory_bus.borrow_mut().write(RAM_START, 0x3C);     // CWAI opcode
        memory_bus.borrow_mut().write(RAM_START + 1, 0xFF); // AND mask (no change to CC)
    }
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: CWAI takes 20 cycles 
    assert_eq!(cycles, 20, "CWAI should take 20 cycles");
    
    // Verify stack pointer moved down (12 bytes for complete state)
    assert_eq!(cpu.registers().s, 0xC900 - 12, "Stack pointer should move down by 12 bytes");
    
    // Verify some key registers were pushed to stack
    {
        let memory_bus = cpu.memory_bus();
        // CC is pushed last (at lowest address)
        let pushed_cc = memory_bus.borrow().read(0xC900 - 12);
        assert!((pushed_cc & 0x80) != 0, "CC should have Entire bit set when pushed by CWAI");
        
        // A is pushed second to last
        assert_eq!(memory_bus.borrow().read(0xC900 - 11), 0xAA, "A should be pushed correctly");
        
        // B is pushed third to last
        assert_eq!(memory_bus.borrow().read(0xC900 - 10), 0xBB, "B should be pushed correctly");
    }
}

// ======= SWI Stack Order Test =======

#[test]
fn test_swi_basic_stack_push() {
    // Test that SWI pushes complete register state and sets interrupt masks
    let mut cpu = create_test_cpu();
    
    // Set up initial values
    cpu.registers_mut().pc = RAM_START; // PC must be in RAM area
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().cc.from_u8(0x44);
    cpu.registers_mut().s = 0xC900;
    
    // Set up SWI vector and instruction
    {
        let memory_bus = cpu.memory_bus();
        memory_bus.borrow_mut().write(0xFFFA, 0xC8); // SWI vector high byte  
        memory_bus.borrow_mut().write(0xFFFA + 1, 0x10); // SWI vector low byte = 0xC810 (in RAM)
        memory_bus.borrow_mut().write(RAM_START, 0x3F); // SWI opcode
    }
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: SWI takes 19 cycles
    assert_eq!(cycles, 19, "SWI should take 19 cycles");
    
    // Verify PC was changed to SWI vector
    assert_eq!(cpu.registers().pc, 0xC810, "PC should be set to SWI vector");
    
    // Verify interrupt masks were set
    assert!(cpu.registers().cc.i, "Interrupt mask should be set");
    assert!(cpu.registers().cc.f, "Fast interrupt mask should be set");
    
    // Verify stack operation
    assert_eq!(cpu.registers().s, 0xC900 - 12, "Stack pointer should move down by 12 bytes");
}

// ======= RTI Stack Order Test =======

#[test]
fn test_rti_entire_stack_pop() {
    // Test RTI with entire state restoration (when Entire bit is set)
    let mut cpu = create_test_cpu();
    
    // Set up stack pointer as if we're returning from interrupt
    cpu.registers_mut().s = 0xC900 - 12;
    
    // Set up stack with known values (simulate interrupt stack frame)
    {
        let memory_bus = cpu.memory_bus();
        
        // CC with Entire bit set (pushed last, at lowest address)
        memory_bus.borrow_mut().write(0xC900 - 12, 0x80 | 0x44); // CC with Entire bit
        
        // A register (pushed second to last)
        memory_bus.borrow_mut().write(0xC900 - 11, 0x33);
        
        // B register (pushed third to last)
        memory_bus.borrow_mut().write(0xC900 - 10, 0x22);
        
        // DP register
        memory_bus.borrow_mut().write(0xC900 - 9, 0x11);
        
        // X register (16-bit, high byte first)
        memory_bus.borrow_mut().write(0xC900 - 8, 0xDE);
        memory_bus.borrow_mut().write(0xC900 - 7, 0xF0);
        
        // Y register (16-bit, high byte first)
        memory_bus.borrow_mut().write(0xC900 - 6, 0x9A);
        memory_bus.borrow_mut().write(0xC900 - 5, 0xBC);
        
        // U register (16-bit, high byte first)
        memory_bus.borrow_mut().write(0xC900 - 4, 0x56);
        memory_bus.borrow_mut().write(0xC900 - 3, 0x78);
        
        // PC register (16-bit, high byte first, pushed first)
        memory_bus.borrow_mut().write(0xC900 - 2, 0x12);
        memory_bus.borrow_mut().write(0xC900 - 1, 0x34);
        
        // Write RTI instruction
        memory_bus.borrow_mut().write(RAM_START, 0x3B); // RTI opcode
    }
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: RTI takes 15 cycles when entire state is popped
    assert_eq!(cycles, 15, "RTI should take 15 cycles when popping entire state");
    
    // Verify stack pointer restored
    assert_eq!(cpu.registers().s, 0xC900, "Stack pointer should be restored");
    
    // Verify registers were restored from stack
    assert_eq!(cpu.registers().pc, 0x1234, "PC should be restored from stack");
    assert_eq!(cpu.registers().u, 0x5678, "U should be restored from stack");
    assert_eq!(cpu.registers().y, 0x9ABC, "Y should be restored from stack");
    assert_eq!(cpu.registers().x, 0xDEF0, "X should be restored from stack");
    assert_eq!(cpu.registers().dp, 0x11, "DP should be restored from stack");
    assert_eq!(cpu.registers().a, 0x33, "A should be restored from stack");
    assert_eq!(cpu.registers().b, 0x22, "B should be restored from stack");
    
    // Verify CC was restored (without Entire bit)
    let restored_cc = cpu.registers().cc.to_u8() & 0x7F;
    assert_eq!(restored_cc, 0x44, "CC should be restored without Entire bit");
}

#[test]
fn test_rti_partial_stack_pop() {
    // Test RTI with partial state restoration (when Entire bit is NOT set)
    let mut cpu = create_test_cpu();
    
    // Set up initial register values (should not change except CC)
    cpu.registers_mut().u = 0x1111;
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().s = 0xC900 - 2; // Only CC on stack
    
    // Set up stack with CC without Entire bit
    {
        let memory_bus = cpu.memory_bus();
        memory_bus.borrow_mut().write(0xC900 - 2, 0x77); // CC without Entire bit (0x80 NOT set)
        memory_bus.borrow_mut().write(RAM_START, 0x3B); // RTI opcode
    }
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: RTI takes 6 cycles when only CC is popped
    assert_eq!(cycles, 6, "RTI should take 6 cycles when popping partial state");
    
    // Verify other registers were NOT changed
    assert_eq!(cpu.registers().u, 0x1111, "U should not change in partial RTI");
    assert_eq!(cpu.registers().a, 0x55, "A should not change in partial RTI");
    
    // Verify CC was restored from stack
    assert_eq!(cpu.registers().cc.to_u8() & 0x7F, 0x77, "CC should be restored from stack");
}

// ======= Integration Test =======

#[test]
fn test_cwai_rti_round_trip() {
    // Test that CWAI followed by RTI correctly saves and restores state
    let mut cpu = create_test_cpu();
    
    // Set up initial state
    let original_u = 0x1234;
    let original_a = 0xAA;
    let original_cc = 0x55;
    
    cpu.registers_mut().u = original_u;
    cpu.registers_mut().a = original_a;
    cpu.registers_mut().cc.from_u8(original_cc);
    cpu.registers_mut().s = 0xC900;
    
    // Execute CWAI
    {
        let memory_bus = cpu.memory_bus();
        memory_bus.borrow_mut().write(RAM_START, 0x3C);     // CWAI opcode
        memory_bus.borrow_mut().write(RAM_START + 1, 0xFF); // No CC masking
    }
    
    let cwai_cycles = cpu.execute_instruction(false, false);
    assert_eq!(cwai_cycles, 20, "CWAI should take 20 cycles");
    
    // Execute RTI to restore state
    {
        let memory_bus = cpu.memory_bus();
        memory_bus.borrow_mut().write(cpu.registers().pc, 0x3B); // RTI opcode
    }
    
    let rti_cycles = cpu.execute_instruction(false, false);
    assert_eq!(rti_cycles, 15, "RTI should take 15 cycles for entire state");
    
    // Verify registers were restored
    assert_eq!(cpu.registers().u, original_u, "U should be restored after CWAI/RTI round trip");
    assert_eq!(cpu.registers().a, original_a, "A should be restored after CWAI/RTI round trip");
    assert_eq!(cpu.registers().s, 0xC900, "S should be restored after CWAI/RTI round trip");
    
    // CC should be restored (masking out Entire bit for comparison)
    let restored_cc = cpu.registers().cc.to_u8() & 0x7F;
    assert_eq!(restored_cc, original_cc & 0x7F, "CC should be restored after CWAI/RTI round trip");
}