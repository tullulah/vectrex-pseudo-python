// test_reset_stack_compliance.rs - Stack compliance tests for RESET* opcode
// Tests verify stack behavior during RESET* execution, particularly during SYNC state
//
// CRITICAL BEHAVIOR TESTED:
// 1. RESET* during normal execution - no stack push (direct reset)
// 2. RESET* during SYNC waiting state - special interaction we implemented
// 3. RESET* stack pointer behavior vs other reset mechanisms
//
// C++ Original Analysis: Vectrexy Reset() at line 79-94:
// - Resets all registers: X=0, Y=0, U=0, S=0, DP=0
// - Sets CC.InterruptMask=1, CC.FastInterruptMask=1  
// - Loads PC from reset vector Read16(InterruptVector::Reset)
// - Clears m_waitingForInterrupts = false
// - NO stack operations during reset
//
// Our Implementation Enhancement:
// - RESET* can execute during SYNC (improvement over Vectrexy limitation)
// - Must verify no unintended stack corruption during SYNC+RESET interaction

use crate::core::cpu6809::Cpu6809;
use crate::core::memory_bus::MemoryBus;
use crate::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

const RAM_START: u16 = 0xC800;
const RAM_END: u16 = 0xCFFF;

fn create_test_cpu_with_bios() -> Cpu6809 {
    use crate::core::memory_bus::EnableSync;
    
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // Add ROM/BIOS space (0xE000-0xFFFF) as RAM for testing
    let rom_ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(rom_ram, (0xE000, 0xFFFF), EnableSync::False);
    
    // Load BIOS ROM data for reset vector
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    
    // Map BIOS to 0xE000-0xFFFF (8K ROM)
    for (i, &byte) in bios_data.iter().enumerate() {
        if i < 0x2000 { // 8K limit
            memory_bus.borrow_mut().write(0xE000 + i as u16, byte);
        }
    }
    
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().pc = RAM_START; // Start in RAM for testing
    
    cpu
}

#[test] 
fn test_reset_stack_no_push_normal_execution() {
    // C++ Original: Reset() doesn't push anything to stack - direct register reset
    // Test RESET* during normal execution leaves stack untouched
    let mut cpu = create_test_cpu_with_bios();
    
    // Setup initial state with stack data
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().s = 0xCF00; // System stack
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().x = 0x1234;
    
    // Pre-populate stack with test data to verify it's not touched
    cpu.memory_bus().borrow_mut().write(0xCEFF, 0xDE); // Stack[-1]
    cpu.memory_bus().borrow_mut().write(0xCEFE, 0xAD); // Stack[-2] 
    cpu.memory_bus().borrow_mut().write(0xCEFD, 0xBE); // Stack[-3]
    cpu.memory_bus().borrow_mut().write(0xCEFC, 0xEF); // Stack[-4]
    
    // Write RESET* instruction
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x3E); // RESET* opcode
    
    let initial_stack_data = [
        cpu.memory_bus().borrow().read(0xCEFF),
        cpu.memory_bus().borrow().read(0xCEFE),
        cpu.memory_bus().borrow().read(0xCEFD),
        cpu.memory_bus().borrow().read(0xCEFC),
    ];
    
    // Execute RESET*
    cpu.execute_instruction(false, false);
    
    // Verify registers reset as expected
    assert_eq!(cpu.registers().a, 0, "A should be reset to 0");
    assert_eq!(cpu.registers().x, 0, "X should be reset to 0");
    assert_eq!(cpu.registers().s, 0, "S should be reset to 0");
    
    // CRITICAL: Verify stack data unchanged - RESET* doesn't push anything
    let final_stack_data = [
        cpu.memory_bus().borrow().read(0xCEFF),
        cpu.memory_bus().borrow().read(0xCEFE),
        cpu.memory_bus().borrow().read(0xCEFD),
        cpu.memory_bus().borrow().read(0xCEFC),
    ];
    
    assert_eq!(initial_stack_data, final_stack_data, 
               "RESET* should not modify any stack data during normal execution");
}

#[test]
fn test_reset_during_sync_stack_integrity() {
    // Test our enhanced behavior: RESET* execution during SYNC state
    // Verify no unintended stack operations occur during this special interaction
    let mut cpu = create_test_cpu_with_bios();
    
    // Setup initial state with stack in specific location
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().s = 0xCE00; // System stack  
    cpu.registers_mut().u = 0xCD00; // User stack
    cpu.registers_mut().a = 0x55;
    cpu.registers_mut().b = 0x66;
    
    // Pre-populate both stacks with sentinel values
    let system_stack_sentinels = [0x11, 0x22, 0x33, 0x44];
    let user_stack_sentinels = [0xAA, 0xBB, 0xCC, 0xDD];
    
    for (i, &val) in system_stack_sentinels.iter().enumerate() {
        cpu.memory_bus().borrow_mut().write(0xCE00 - 1 - i as u16, val);
    }
    
    for (i, &val) in user_stack_sentinels.iter().enumerate() {
        cpu.memory_bus().borrow_mut().write(0xCD00 - 1 - i as u16, val);
    }
    
    // Step 1: Execute SYNC to enter waiting state
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x13);     // SYNC opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x3E); // RESET* opcode (next)
    
    cpu.execute_instruction(false, false);
    assert!(cpu.waiting_for_interrupts, "CPU should be waiting for interrupts after SYNC");
    
    // Step 2: Execute RESET* during SYNC waiting state (our enhancement)
    cpu.execute_instruction(false, false);
    
    // Verify RESET* cleared waiting state
    assert!(!cpu.waiting_for_interrupts, "RESET* should clear waiting_for_interrupts state");
    
    // Verify registers reset as expected
    assert_eq!(cpu.registers().a, 0, "A should be reset to 0");
    assert_eq!(cpu.registers().b, 0, "B should be reset to 0");
    assert_eq!(cpu.registers().s, 0, "S should be reset to 0");
    assert_eq!(cpu.registers().u, 0, "U should be reset to 0");
    
    // CRITICAL: Verify both stacks completely untouched during SYNC+RESET interaction
    for (i, &expected) in system_stack_sentinels.iter().enumerate() {
        let actual = cpu.memory_bus().borrow().read(0xCE00 - 1 - i as u16);
        assert_eq!(actual, expected, 
                   "System stack position {} should be unchanged after SYNC+RESET", i);
    }
    
    for (i, &expected) in user_stack_sentinels.iter().enumerate() {
        let actual = cpu.memory_bus().borrow().read(0xCD00 - 1 - i as u16);
        assert_eq!(actual, expected,
                   "User stack position {} should be unchanged after SYNC+RESET", i);
    }
}

#[test]
fn test_reset_vs_interrupt_stack_behavior() {
    // Compare RESET* behavior vs normal interrupt handling
    // RESET* should NOT push state like interrupts do
    let mut cpu = create_test_cpu_with_bios();
    
    // Setup initial state identical to interrupt test scenarios
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().s = 0xCE00;
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().b = 0x22;
    cpu.registers_mut().x = 0x3344;
    cpu.registers_mut().y = 0x5566;
    cpu.registers_mut().u = 0x7788;
    cpu.registers_mut().dp = 0x99;
    cpu.registers_mut().cc.from_u8(0xAA);
    
    // Record initial stack pointer and area around it
    let initial_s = cpu.registers().s;
    let stack_area: Vec<u8> = (0..16).map(|i| {
        cpu.memory_bus().borrow().read(initial_s - i)
    }).collect();
    
    // Write and execute RESET*
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x3E); // RESET* opcode
    cpu.execute_instruction(false, false);
    
    // Verify PC loaded from reset vector (not just advanced)
    let reset_vector_low = cpu.memory_bus().borrow().read(0xFFFF);
    let reset_vector_high = cpu.memory_bus().borrow().read(0xFFFE);
    let expected_pc = ((reset_vector_high as u16) << 8) | (reset_vector_low as u16);
    assert_eq!(cpu.registers().pc, expected_pc, "PC should be loaded from reset vector");
    
    // CRITICAL: Verify stack area completely unchanged (no push occurred)
    for i in 0..16 {
        let current_value = cpu.memory_bus().borrow().read(initial_s - i);
        assert_eq!(current_value, stack_area[i as usize], 
                   "Stack memory at offset {} should be unchanged by RESET*", i);
    }
    
    // Verify S register reset to 0 (not decremented like during push operations)
    assert_eq!(cpu.registers().s, 0, "S should be reset to 0, not decremented");
}

#[test]
fn test_reset_stack_pointer_reset_behavior() {
    // Test specific stack pointer behavior during RESET*
    // Verify S register is reset to 0, not manipulated like push/pull operations
    let mut cpu = create_test_cpu_with_bios();
    
    // Setup various stack pointer values to test reset behavior
    let test_stack_values = [0xC800, 0xCFFF, 0xD000, 0x0100, 0x8000, 0xFF00];
    
    for &initial_s in &test_stack_values {
        // Reset CPU state for each test
        cpu.registers_mut().pc = RAM_START;
        cpu.registers_mut().s = initial_s;
        cpu.registers_mut().u = initial_s.wrapping_add(0x100); // Different user stack (avoid overflow)
        
        // Execute RESET*
        cpu.memory_bus().borrow_mut().write(RAM_START, 0x3E); // RESET* opcode
        cpu.execute_instruction(false, false);
        
        // Verify both stack pointers reset to 0 (C++ Original behavior)
        assert_eq!(cpu.registers().s, 0, 
                   "System stack should reset to 0 from initial value 0x{:04X}", initial_s);
        assert_eq!(cpu.registers().u, 0,
                   "User stack should reset to 0 from initial value 0x{:04X}", initial_s.wrapping_add(0x100));
        
        // Prepare for next iteration - restore PC
        cpu.registers_mut().pc = RAM_START;
    }
}