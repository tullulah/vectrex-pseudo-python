// Test suite for SWI2/SWI3/RTI stack order compliance
// C++ Original: 1:1 port from Vectrexy interrupt handling stack operations
// Validates exact stack byte order compliance with C++ PushCCState/PopCCState

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;
const VECTOR_SWI2: u16 = 0xFFF2;  // SWI2 vector address (Vectrexy compliant)
const VECTOR_SWI3: u16 = 0xFFF4;  // SWI3 vector address (Vectrexy compliant)

fn create_test_cpu_with_vectors() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    // Add additional RAM for ROM/vector space (0xE000-0xFFFF) to allow vector writes in tests
    let rom_ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(rom_ram, (0xE000, 0xFFFF), EnableSync::False);
    
    let cpu = Cpu6809::new(memory_bus.clone());
    
    // Setup interrupt vectors for SWI2/SWI3 (no reset needed)
    // SWI2 vector -> points to a safe address
    memory_bus.borrow_mut().write(VECTOR_SWI2, 0xDE);
    memory_bus.borrow_mut().write(VECTOR_SWI2 + 1, 0xAD);
    // SWI3 vector -> points to a safe address  
    memory_bus.borrow_mut().write(VECTOR_SWI3, 0xBE);
    memory_bus.borrow_mut().write(VECTOR_SWI3 + 1, 0xEF);
    
    cpu
}

// ========== SWI2 STACK ORDER COMPLIANCE TESTS ==========

#[test]
fn test_swi2_stack_order_compliance() {
    let mut cpu = create_test_cpu_with_vectors();
    
    // C++ Original: PushCCState(true) - pushes entire register set in specific order
    // C++ Original stack order: PC, U, Y, X, DP, B, A, CC (12 bytes total)
    
    // Setup initial register state for verification
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().dp = 0x56;
    cpu.registers_mut().x = 0x789A;
    cpu.registers_mut().y = 0xBCDE;
    cpu.registers_mut().u = 0xF012;
    cpu.registers_mut().s = 0xD000;  // System stack pointer
    cpu.registers_mut().cc.from_u8(0x78);
    
    let memory_bus = cpu.memory_bus().clone();
    // Write SWI2 instruction: 0x10 0x3F
    memory_bus.borrow_mut().write(RAM_START, 0x10);  // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x3F);  // SWI2
    
    let original_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles and basic state
    assert_eq!(cycles, 20, "SWI2 should take 20 cycles");
    assert_eq!(cpu.registers().s, original_s - 12, "SWI2 should push 12 bytes (entire register set)");
    assert!(cpu.registers().cc.e, "SWI2 should set entire flag");
    assert_eq!(cpu.registers().pc, 0xDEAD, "SWI2 should jump to vector address");
    
    // C++ Original stack push order verification: PC, U, Y, X, DP, B, A, CC
    // Stack grows downward: first pushed item goes to highest address
    // Layout after push: [PC_high PC_low U_high U_low Y_high Y_low X_high X_low DP B A CC]
    //                    0xCFFF  0xCFFE 0xCFFD 0xCFFC 0xCFFB 0xCFFA 0xCFF9 0xCFF8 0xCFF7 0xCFF6 0xCFF5 0xCFF4
    
    // Verify stack contents in exact C++ order - reading from high to low addresses
    // PC (16-bit): original PC + 2 (after SWI2 instruction)
    // C++ Push16: writes low byte first (higher addr), then high byte (lower addr)
    let expected_return_pc = RAM_START + 2;
    assert_eq!(memory_bus.borrow().read(original_s - 1), (expected_return_pc & 0xFF) as u8, "Stack[0xCFFF]: PC low byte");
    assert_eq!(memory_bus.borrow().read(original_s - 2), (expected_return_pc >> 8) as u8, "Stack[0xCFFE]: PC high byte");
    
    // U (16-bit): C++ Push16 writes low byte first, then high byte
    assert_eq!(memory_bus.borrow().read(original_s - 3), 0x12, "Stack[0xCFFD]: U low byte");
    assert_eq!(memory_bus.borrow().read(original_s - 4), 0xF0, "Stack[0xCFFC]: U high byte");
    
    // Y (16-bit): C++ Push16 writes low byte first, then high byte
    assert_eq!(memory_bus.borrow().read(original_s - 5), 0xDE, "Stack[0xCFFB]: Y low byte");
    assert_eq!(memory_bus.borrow().read(original_s - 6), 0xBC, "Stack[0xCFFA]: Y high byte");
    
    // X (16-bit): C++ Push16 writes low byte first, then high byte
    assert_eq!(memory_bus.borrow().read(original_s - 7), 0x9A, "Stack[0xCFF9]: X low byte");
    assert_eq!(memory_bus.borrow().read(original_s - 8), 0x78, "Stack[0xCFF8]: X high byte");
    
    // DP (8-bit)
    assert_eq!(memory_bus.borrow().read(original_s - 9), 0x56, "Stack[0xCFF7]: DP");
    
    // B (8-bit)
    assert_eq!(memory_bus.borrow().read(original_s - 10), 0x34, "Stack[0xCFF6]: B");
    
    // A (8-bit)
    assert_eq!(memory_bus.borrow().read(original_s - 11), 0x12, "Stack[0xCFF5]: A");
    
    // CC (8-bit) with E flag set
    let expected_cc = 0x78 | 0x80;  // Original CC with E flag set
    assert_eq!(memory_bus.borrow().read(original_s - 12), expected_cc, "Stack[0xCFF4]: CC with E flag");
}

// ========== SWI3 STACK ORDER COMPLIANCE TESTS ==========

#[test]
fn test_swi3_stack_order_compliance() {
    let mut cpu = create_test_cpu_with_vectors();
    
    // C++ Original: PushCCState(true) - same stack order as SWI2
    // Verify SWI3 uses identical stack push pattern
    
    // Setup different register state to distinguish from SWI2 test
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().b = 0xBB;
    cpu.registers_mut().dp = 0xCC;
    cpu.registers_mut().x = 0xDDEE;
    cpu.registers_mut().y = 0xFF00;
    cpu.registers_mut().u = 0x1122;
    cpu.registers_mut().s = 0xD000;
    cpu.registers_mut().cc.from_u8(0x45);
    
    let memory_bus = cpu.memory_bus().clone();
    // Write SWI3 instruction: 0x11 0x3F
    memory_bus.borrow_mut().write(RAM_START, 0x11);  // Page 2 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x3F);  // SWI3
    
    let original_s = cpu.registers().s;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles and basic state
    assert_eq!(cycles, 20, "SWI3 should take 20 cycles");
    assert_eq!(cpu.registers().s, original_s - 12, "SWI3 should push 12 bytes (entire register set)");
    assert!(cpu.registers().cc.e, "SWI3 should set entire flag");
    assert_eq!(cpu.registers().pc, 0xBEEF, "SWI3 should jump to vector address");
    
    // Verify identical stack order as SWI2 - stack grows downward from high to low addresses
    let expected_return_pc = RAM_START + 2;
    
    // Verify stack contents match expected register values - C++ Push16 byte order
    assert_eq!(memory_bus.borrow().read(original_s - 1), (expected_return_pc & 0xFF) as u8, "SWI3 Stack[0xCFFF]: PC low");
    assert_eq!(memory_bus.borrow().read(original_s - 2), (expected_return_pc >> 8) as u8, "SWI3 Stack[0xCFFE]: PC high");
    assert_eq!(memory_bus.borrow().read(original_s - 3), 0x22, "SWI3 Stack[0xCFFD]: U low");
    assert_eq!(memory_bus.borrow().read(original_s - 4), 0x11, "SWI3 Stack[0xCFFC]: U high");
    assert_eq!(memory_bus.borrow().read(original_s - 5), 0x00, "SWI3 Stack[0xCFFB]: Y low");
    assert_eq!(memory_bus.borrow().read(original_s - 6), 0xFF, "SWI3 Stack[0xCFFA]: Y high");
    assert_eq!(memory_bus.borrow().read(original_s - 7), 0xEE, "SWI3 Stack[0xCFF9]: X low");
    assert_eq!(memory_bus.borrow().read(original_s - 8), 0xDD, "SWI3 Stack[0xCFF8]: X high");
    assert_eq!(memory_bus.borrow().read(original_s - 9), 0xCC, "SWI3 Stack[0xCFF7]: DP");
    assert_eq!(memory_bus.borrow().read(original_s - 10), 0xBB, "SWI3 Stack[0xCFF6]: B");
    assert_eq!(memory_bus.borrow().read(original_s - 11), 0xAA, "SWI3 Stack[0xCFF5]: A");
    
    let expected_cc = 0x45 | 0x80;  // Original CC with E flag set
    assert_eq!(memory_bus.borrow().read(original_s - 12), expected_cc, "SWI3 Stack[0xCFF4]: CC with E flag");
}

// ========== RTI STACK ORDER COMPLIANCE TESTS ==========

#[test]
fn test_rti_full_context_restore() {
    let mut cpu = create_test_cpu_with_vectors();
    
    // C++ Original: PopCCState(poppedEntire) - pops in reverse order of push
    // C++ Original pop order: CC, A, B, DP, X, Y, U, PC (reverse of push)
    // Test full interrupt context restore (12 bytes)
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().s = 0xD000 - 12;  // Stack with full context
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup stack with full interrupt context (as if from SWI/IRQ)
    // C++ Original push order was: PC, U, Y, X, DP, B, A, CC
    // Stack layout: PC at high addresses, CC at low address (S points here)
    let original_s = 0xD000;  // S before push
    
    // Prepare expected values to restore
    let restore_pc = 0x1234;
    let restore_u = 0x5678;
    let restore_y = 0x9ABC;
    let restore_x = 0xDEF0;
    let restore_dp = 0x11;
    let restore_b = 0x22;
    let restore_a = 0x33;
    let restore_cc = 0x44 | 0x80;  // E flag set indicates full context
    
    // Write stack data matching real push layout (Vectrexy Push16 writes low byte first, then high byte)
    // C++ Original: Push16(S, PC) writes: Write(--S, PC_low); Write(--S, PC_high);
    memory_bus.borrow_mut().write(original_s - 1, (restore_pc & 0xFF) as u8); // PC low at 0xCFFF  
    memory_bus.borrow_mut().write(original_s - 2, (restore_pc >> 8) as u8);   // PC high at 0xCFFE
    memory_bus.borrow_mut().write(original_s - 3, (restore_u & 0xFF) as u8);  // U low at 0xCFFD
    memory_bus.borrow_mut().write(original_s - 4, (restore_u >> 8) as u8);    // U high at 0xCFFC  
    memory_bus.borrow_mut().write(original_s - 5, (restore_y & 0xFF) as u8);  // Y low at 0xCFFB
    memory_bus.borrow_mut().write(original_s - 6, (restore_y >> 8) as u8);    // Y high at 0xCFFA
    memory_bus.borrow_mut().write(original_s - 7, (restore_x & 0xFF) as u8);  // X low at 0xCFF9
    memory_bus.borrow_mut().write(original_s - 8, (restore_x >> 8) as u8);    // X high at 0xCFF8
    memory_bus.borrow_mut().write(original_s - 9, restore_dp);                // DP at 0xCFF7
    memory_bus.borrow_mut().write(original_s - 10, restore_b);                // B at 0xCFF6
    memory_bus.borrow_mut().write(original_s - 11, restore_a);                // A at 0xCFF5
    memory_bus.borrow_mut().write(original_s - 12, restore_cc);               // CC at 0xCFF4 (S points here)
    
    // Write RTI instruction
    memory_bus.borrow_mut().write(RAM_START, 0x3B);  // RTI
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify RTI cycles for full context restore (E=1)
    assert_eq!(cycles, 15, "RTI should take 15 cycles for full context restore");
    
    // Verify all registers restored in correct order
    assert_eq!(cpu.registers().pc, restore_pc, "RTI should restore PC");
    assert_eq!(cpu.registers().u, restore_u, "RTI should restore U"); 
    assert_eq!(cpu.registers().y, restore_y, "RTI should restore Y");
    assert_eq!(cpu.registers().x, restore_x, "RTI should restore X");
    assert_eq!(cpu.registers().dp, restore_dp, "RTI should restore DP");
    assert_eq!(cpu.registers().b, restore_b, "RTI should restore B");
    assert_eq!(cpu.registers().a, restore_a, "RTI should restore A");
    assert_eq!(cpu.registers().cc.to_u8(), restore_cc, "RTI should restore CC exactly");
    
    // Verify stack pointer advanced by 12 bytes
    assert_eq!(cpu.registers().s, 0xD000, "RTI should advance stack pointer by 12 bytes");
    assert!(cpu.registers().cc.e, "RTI should preserve E flag from restored CC");
}

#[test]
fn test_rti_fast_interrupt_context() {
    let mut cpu = create_test_cpu_with_vectors();
    
    // Test fast interrupt context restore (CC.E=0, only PC and CC saved)
    // C++ Original: PopCCState with poppedEntire=false -> 6 cycles
    
    cpu.registers_mut().pc = RAM_START;
    cpu.registers_mut().s = 0xD000 - 3;  // Stack with fast context (PC + CC only = 3 bytes)
    
    let memory_bus = cpu.memory_bus().clone();
    let original_s = 0xD000;
    
    // Setup minimal interrupt context (as if from fast interrupt)
    let restore_pc = 0xABCD;
    let restore_cc = 0x25;  // E flag clear (0x25 & 0x80 = 0)
    
    // Fast interrupt stack order: PC low, PC high, CC (3 bytes total) 
    // C++ Original: Push16(S, PC) writes low byte first, then high byte
    memory_bus.borrow_mut().write(original_s - 1, (restore_pc & 0xFF) as u8); // PC low at 0xCFFF
    memory_bus.borrow_mut().write(original_s - 2, (restore_pc >> 8) as u8);   // PC high at 0xCFFE
    memory_bus.borrow_mut().write(original_s - 3, restore_cc);                 // CC at 0xCFFD (S points here)
    
    // Write RTI instruction
    memory_bus.borrow_mut().write(RAM_START, 0x3B);  // RTI
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify RTI cycles for fast interrupt restore (E=0)
    assert_eq!(cycles, 6, "RTI should take 6 cycles for fast interrupt restore");
    
    // Verify minimal restoration
    assert_eq!(cpu.registers().pc, restore_pc, "RTI should restore PC from fast interrupt");
    assert_eq!(cpu.registers().cc.to_u8(), restore_cc, "RTI should restore CC from fast interrupt");
    assert!(!cpu.registers().cc.e, "RTI should clear E flag for fast interrupt");
    
    // Verify stack pointer advanced by 3 bytes (fast interrupt context size)
    assert_eq!(cpu.registers().s, 0xD000, "RTI should advance stack pointer by fast context size (3 bytes)");
}

#[test]
fn test_swi2_rti_round_trip() {
    let mut cpu = create_test_cpu_with_vectors();
    
    // Test complete SWI2 -> RTI round trip to verify stack symmetry
    // C++ Original: PushCCState(true) followed by PopCCState should restore exactly
    
    // Setup initial CPU state
    let initial_pc = RAM_START;
    let initial_a = 0x77;
    let initial_b = 0x88;
    let initial_dp = 0x99;
    let initial_x = 0xAABB;
    let initial_y = 0xCCDD;
    let initial_u = 0xEEFF;
    let initial_s = 0xD000;
    let initial_cc = 0x35;
    
    cpu.registers_mut().pc = initial_pc;
    cpu.registers_mut().a = initial_a;
    cpu.registers_mut().b = initial_b;
    cpu.registers_mut().dp = initial_dp;
    cpu.registers_mut().x = initial_x;
    cpu.registers_mut().y = initial_y;
    cpu.registers_mut().u = initial_u;
    cpu.registers_mut().s = initial_s;
    cpu.registers_mut().cc.from_u8(initial_cc);
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Write SWI2 instruction
    memory_bus.borrow_mut().write(RAM_START, 0x10);  // Page 1 prefix
    memory_bus.borrow_mut().write(RAM_START + 1, 0x3F);  // SWI2
    
    // Execute SWI2
    let swi_cycles = cpu.execute_instruction(false, false);
    assert_eq!(swi_cycles, 20, "SWI2 should take 20 cycles");
    
    // Verify SWI2 effects
    assert_eq!(cpu.registers().pc, 0xDEAD, "SWI2 should jump to vector");
    assert_eq!(cpu.registers().s, initial_s - 12, "SWI2 should push 12 bytes");
    assert!(cpu.registers().cc.e, "SWI2 should set E flag");
    
    // Now setup RTI to return from interrupt
    let return_address = 0xDEAD;  // Current PC (interrupt handler)
    memory_bus.borrow_mut().write(return_address, 0x3B);  // RTI
    
    // Execute RTI
    let rti_cycles = cpu.execute_instruction(false, false);
    assert_eq!(rti_cycles, 15, "RTI should take 15 cycles for full restore");
    
    // Verify complete restoration (except PC which should be return address)
    let expected_return_pc = initial_pc + 2;  // After SWI2 instruction
    assert_eq!(cpu.registers().pc, expected_return_pc, "RTI should restore return address");
    assert_eq!(cpu.registers().a, initial_a, "RTI should restore A");
    assert_eq!(cpu.registers().b, initial_b, "RTI should restore B");
    assert_eq!(cpu.registers().dp, initial_dp, "RTI should restore DP");
    assert_eq!(cpu.registers().x, initial_x, "RTI should restore X");
    assert_eq!(cpu.registers().y, initial_y, "RTI should restore Y");
    assert_eq!(cpu.registers().u, initial_u, "RTI should restore U");
    assert_eq!(cpu.registers().s, initial_s, "RTI should restore stack pointer");
    
    // CC should be restored with E flag preserved from push
    let expected_cc = initial_cc | 0x80;  // Original CC with E flag set
    assert_eq!(cpu.registers().cc.to_u8(), expected_cc, "RTI should restore CC with E flag");
}