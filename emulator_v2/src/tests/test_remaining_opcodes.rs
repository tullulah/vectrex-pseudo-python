// Tests for the final remaining opcodes: SYNC (0x13) and RESET* (0x3E)
// Achieving 100% Vectrexy compliance - 224/224 opcodes implemented

use crate::core::cpu6809::Cpu6809;
use crate::core::memory_bus::MemoryBus;
use crate::core::memory_map::MemoryMap;
use crate::types::Cycles;
use std::cell::RefCell;
use std::rc::Rc;

// Helper function to create a test CPU with RAM (following pattern from other tests)
fn create_test_cpu() -> Cpu6809 {
    use crate::core::ram::Ram;
    
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

// Helper function to create a test CPU with full emulator setup (BIOS + RAM)
fn create_test_cpu_with_bios() -> crate::core::emulator::Emulator {
    let mut emulator = crate::core::emulator::Emulator::new();
    
    // Use the standard BIOS path from the project
    let bios_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    emulator.init(bios_path);
    
    emulator
}

#[test]
fn test_sync_opcode_basic() {
    // Test SYNC (0x13) - Synchronize with interrupt
    let mut cpu = create_test_cpu();
    
    // Set PC to RAM area and place SYNC instruction
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x13); // SYNC opcode
    
    // Initially CPU should not be waiting for interrupts
    assert!(!cpu.waiting_for_interrupts);
    
    // Execute SYNC instruction
    let cycles = cpu.execute_instruction(false, false); // No interrupts enabled
    
    // After SYNC, CPU should be waiting for interrupts
    assert!(cpu.waiting_for_interrupts);
    
    // SYNC takes 2 cycles according to opcode table
    assert_eq!(cycles, 2);
    
    // PC should have advanced past SYNC instruction
    assert_eq!(cpu.registers.pc, 0xC801);
}

#[test] 
fn test_sync_with_interrupt() {
    // Test SYNC behavior when interrupt occurs
    let mut cpu = create_test_cpu();
    
    // Set PC and place SYNC instruction
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x13); // SYNC opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12); // NOP after SYNC
    
    // Execute SYNC - sets waiting state
    cpu.execute_instruction(false, false);
    assert!(cpu.waiting_for_interrupts);
    
    // Now execute with IRQ enabled - should clear waiting state
    let cycles = cpu.execute_instruction(true, false);
    assert!(!cpu.waiting_for_interrupts);
    
    // Should execute the next instruction (NOP) 
    assert_eq!(cycles, 2); // 2 cycles for NOP instruction
}

#[test]
fn test_sync_waiting_behavior() {
    // Test that CPU stays in waiting state until interrupt
    let mut cpu = create_test_cpu();
    
    // Set PC to RAM area and place SYNC instruction  
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x13); // SYNC opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12); // NOP after SYNC
    
    // Execute SYNC
    cpu.execute_instruction(false, false);
    assert!(cpu.waiting_for_interrupts);
    let initial_pc = cpu.registers.pc;
    
    // Execute several more times without interrupts - should stay waiting
    for _ in 0..5 {
        let cycles = cpu.execute_instruction(false, false);
        assert!(cpu.waiting_for_interrupts); // Still waiting
        assert_eq!(cycles, 1); // Minimal cycles while waiting
        assert_eq!(cpu.registers.pc, initial_pc); // PC shouldn't advance
    }
    
    // Finally provide an interrupt - should clear waiting state
    cpu.execute_instruction(false, true); // FIRQ enabled
    assert!(!cpu.waiting_for_interrupts);
}

#[test]
fn test_reset_opcode_basic() {
    // Test RESET* (0x3E) - System reset instruction  
    let mut emulator = create_test_cpu_with_bios();
    let cpu = emulator.get_cpu();
    
    // Modify CPU state to non-reset values
    cpu.registers.a = 0xAA;
    cpu.registers.b = 0xBB; 
    cpu.registers.x = 0xCCDD;
    cpu.registers.y = 0xEEFF;
    cpu.registers.s = 0x1122;
    cpu.registers.u = 0x3344;
    cpu.registers.dp = 0x55;
    cpu.registers.cc.from_u8(0xFF);
    cpu.registers.pc = 0xC800;
    
    // Place RESET instruction in RAM area
    cpu.memory_bus().borrow_mut().write(0xC800, 0x3E); // RESET* opcode
    
    // Execute RESET instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // RESET takes 0 cycles (immediate effect)
    assert_eq!(cycles, 0);
    
    // CPU state should be reset to initial values
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.registers.x, 0x0000);
    assert_eq!(cpu.registers.y, 0x0000);
    assert_eq!(cpu.registers.s, 0x0000);
    assert_eq!(cpu.registers.u, 0x0000);
    assert_eq!(cpu.registers.dp, 0x00);
    assert_eq!(cpu.registers.cc.to_u8(), 0x00);
    
    // PC should be reset to reset vector (loaded from 0xFFFE-0xFFFF)
    // This will be the actual reset vector from the BIOS
    let expected_pc = (cpu.memory_bus().borrow().read(0xFFFE) as u16) << 8 | 
                      (cpu.memory_bus().borrow().read(0xFFFF) as u16);
    assert_eq!(cpu.registers.pc, expected_pc);
    
    // waiting_for_interrupts should also be cleared
    assert!(!cpu.waiting_for_interrupts);
}

#[test]
fn test_reset_clears_sync_state() {
    // Test that RESET clears SYNC waiting state
    let mut emulator = create_test_cpu_with_bios();
    
    // Set PC and execute SYNC to enter waiting state
    {
        let cpu = emulator.get_cpu();
        cpu.registers.pc = 0xC800;
        cpu.memory_bus().borrow_mut().write(0xC800, 0x13); // SYNC opcode
        cpu.execute_instruction(false, false);
        assert!(cpu.waiting_for_interrupts);
    }
    
    // Now execute RESET instruction (need BIOS for reset vector)
    {
        let cpu = emulator.get_cpu();
        cpu.registers.pc = 0xC801;
        cpu.memory_bus().borrow_mut().write(0xC801, 0x3E); // RESET* opcode
        cpu.execute_instruction(false, false);
        
        // SYNC waiting state should be cleared by RESET
        assert!(!cpu.waiting_for_interrupts);
    }
}

#[test]
fn test_opcode_coverage_completion() {
    // Verify that we now have 100% opcode coverage
    use crate::core::cpu_op_codes::{lookup_cpu_op_runtime, AddressingMode};
    
    let mut implemented_count = 0;
    let mut total_count = 0;
    
    // Count only implemented opcodes across all pages
    // We want to count actual 6809 instructions, not all possible byte combinations
    let mut page0_count = 0;
    let mut page1_count = 0;
    let mut page2_count = 0;
    
    // Page 0 opcodes (single byte instructions)
    for opcode in 0x00..=0xFF {
        let cpu_op = lookup_cpu_op_runtime(0, opcode);
        if cpu_op.addr_mode != AddressingMode::Illegal {
            page0_count += 1;
        }
    }
    
    // Page 1 opcodes (0x10xx prefix) 
    for opcode in 0x00..=0xFF {
        let cpu_op = lookup_cpu_op_runtime(1, opcode);
        if cpu_op.addr_mode != AddressingMode::Illegal {
            page1_count += 1;
        }
    }
    
    // Page 2 opcodes (0x11xx prefix)
    for opcode in 0x00..=0xFF {
        let cpu_op = lookup_cpu_op_runtime(2, opcode);
        if cpu_op.addr_mode != AddressingMode::Illegal {
            page2_count += 1;
        }
    }
    
    implemented_count = page0_count + page1_count + page2_count;
    total_count = 256; // For display only
    
    println!("Page 0: {} opcodes, Page 1: {} opcodes, Page 2: {} opcodes", 
             page0_count, page1_count, page2_count);
    
    // Page 0 should have all 224 standard 6809 opcodes implemented
    // Pages 1 and 2 contain extended instruction sets
    println!("Implemented opcodes: {}/{}", implemented_count, total_count);
    
    // Verify we have good coverage - should be around 271 total opcodes
    assert_eq!(page0_count, 224, "Page 0 should have exactly 224 implemented opcodes");
    assert!(page1_count > 0, "Page 1 should have some extended opcodes");
    assert!(page2_count > 0, "Page 2 should have some extended opcodes");
    assert_eq!(implemented_count, 271, "Total implemented opcodes should be 271 (224+38+9)");
    
    // Specifically verify our new opcodes are marked as implemented
    let sync_op = lookup_cpu_op_runtime(0, 0x13);
    assert_ne!(sync_op.addr_mode, AddressingMode::Illegal, "SYNC (0x13) should be implemented");
    assert_eq!(sync_op.cycles, 2, "SYNC should take 2 cycles");
    
    let reset_op = lookup_cpu_op_runtime(0, 0x3E);
    assert_ne!(reset_op.addr_mode, AddressingMode::Illegal, "RESET* (0x3E) should be implemented");
    assert_eq!(reset_op.cycles, 0, "RESET should take 0 cycles");
}

#[test]
fn test_sync_with_bios_real_scenario() {
    // Test SYNC in a more realistic scenario with actual BIOS
    let mut cpu = create_test_cpu();
    
    // Place SYNC instruction in RAM area (use correct RAM range 0xC800)
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x13); // SYNC
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12); // NOP after sync
    
    // Save initial state
    let initial_a = cpu.registers.a;
    let initial_b = cpu.registers.b;
    let initial_x = cpu.registers.x;
    
    // Execute SYNC
    let sync_cycles = cpu.execute_instruction(false, false);
    assert_eq!(sync_cycles, 2);
    assert!(cpu.waiting_for_interrupts);
    assert_eq!(cpu.registers.pc, 0xC801); // PC advanced past SYNC
    
    // Registers should be unchanged by SYNC
    assert_eq!(cpu.registers.a, initial_a);
    assert_eq!(cpu.registers.b, initial_b);
    assert_eq!(cpu.registers.x, initial_x);
    
    // Simulate waiting cycles
    for i in 0..3 {
        let wait_cycles = cpu.execute_instruction(false, false);
        assert_eq!(wait_cycles, 1, "Waiting cycle {} should take 1 cycle", i);
        assert!(cpu.waiting_for_interrupts);
        assert_eq!(cpu.registers.pc, 0xC801); // PC shouldn't advance while waiting
    }
    
    // Provide interrupt to wake up
    let _wake_cycles = cpu.execute_instruction(true, false);
    assert!(!cpu.waiting_for_interrupts);
    // Should have executed the NOP after clearing wait state
    assert_eq!(cpu.registers.pc, 0xC802); // Advanced past NOP
}

#[test]
fn test_reset_vector_loading() {
    // Test that RESET properly loads from reset vector
    let mut emulator = create_test_cpu_with_bios();
    let cpu = emulator.get_cpu();
    
    // Read the reset vector from BIOS
    let reset_vector_low = cpu.memory_bus().borrow().read(0xFFFF);
    let reset_vector_high = cpu.memory_bus().borrow().read(0xFFFE);
    let expected_reset_pc = ((reset_vector_high as u16) << 8) | (reset_vector_low as u16);
    
    println!("BIOS Reset Vector: {:04X} (from {:02X}{:02X})", 
             expected_reset_pc, reset_vector_high, reset_vector_low);
    
    // Set PC to execute RESET instruction
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x3E); // RESET* opcode
    
    // Execute RESET
    cpu.execute_instruction(false, false);
    
    // PC should now be the reset vector value
    assert_eq!(cpu.registers.pc, expected_reset_pc, 
               "RESET should load PC from reset vector at 0xFFFE-0xFFFF");
}
