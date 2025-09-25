// test_stack_compliance_comprehensive.rs - Comprehensive stack operation compliance tests
// Validates 1:1 C++ behavior for all stack-related opcodes and register ordering
//
// STACK OPERATIONS TESTED:
// 1. JSR/BSR/LBSR family - subroutine calls (already in separate test file)
// 2. PSHS/PULS (0x34/0x35) - System stack push/pull
// 3. PSHU/PULU (0x36/0x37) - User stack push/pull  
// 4. Future: RTS, RTI, SWI families (when implemented)
//
// COMPLIANCE FOCUS:
// - Exact register push/pull order as per C++ implementation
// - Stack byte order (HIGH/LOW) validation
// - Register bit mask behavior (CC, A, B, DP, X, Y, S/U, PC)
// - Stack pointer manipulation correctness
// - Multiple register combinations

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::rc::Rc;
use std::cell::RefCell;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

// Test helper to setup CPU with known register values for comprehensive testing
fn setup_test_cpu_with_registers() -> Cpu6809 {
    let mut cpu = create_test_cpu();
    
    // Setup known register values for testing
    cpu.registers_mut().a = 0xAA;
    cpu.registers_mut().b = 0xBB;
    cpu.registers_mut().x = 0x1122;
    cpu.registers_mut().y = 0x3344;
    cpu.registers_mut().u = 0x5566;
    cpu.registers_mut().s = 0xCF00;  // System stack in RAM
    cpu.registers_mut().pc = 0xC800; // PC in RAM area
    cpu.registers_mut().dp = 0x77;
    
    // Setup known condition codes
    let mut cc = cpu.registers().cc;
    cc.c = true;  cc.v = false; cc.z = true;  cc.n = false;
    cc.i = true;  cc.h = false; cc.f = true;  cc.e = false;
    cpu.registers_mut().cc = cc;
    
    cpu
}

// ============================================================================
// PSHS (0x34) - Push System Stack Tests
// ============================================================================

#[test]
fn test_pshs_single_register_cc() {
    // Test pushing single register (CC) validates byte order and stack decrement
    let mut cpu = setup_test_cpu_with_registers();
    
    // Write PSHS instruction with CC bit (bit 0 = 0x01)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // CC bit mask
    
    let initial_s = cpu.registers().s;
    let expected_cc = cpu.registers().cc.to_u8();
    
    // Execute PSHS
    cpu.execute_instruction(false, false);
    
    // Verify stack pointer decremented by 1 (8-bit register)
    assert_eq!(cpu.registers().s, initial_s - 1);
    
    // Verify CC pushed to stack
    let pushed_cc = cpu.memory_bus().borrow().read(cpu.registers().s);
    assert_eq!(pushed_cc, expected_cc);
}

#[test]
fn test_pshs_single_register_16bit() {
    // Test pushing 16-bit register (X) validates HIGH/LOW byte order
    let mut cpu = setup_test_cpu_with_registers();
    
    // Write PSHS instruction with X bit (bit 4 = 0x10)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x10); // X bit mask
    
    let initial_s = cpu.registers().s;
    let expected_x = cpu.registers().x; // 0x1122
    
    // Execute PSHS
    cpu.execute_instruction(false, false);
    
    // Verify stack pointer decremented by 2 (16-bit register)
    assert_eq!(cpu.registers().s, initial_s - 2);
    
    // Verify X pushed with correct byte order: HIGH at lower address, LOW at higher
    // C++ Push16: [S]=HIGH, [S+1]=LOW 
    let high_byte = cpu.memory_bus().borrow().read(cpu.registers().s);     // HIGH at S
    let low_byte = cpu.memory_bus().borrow().read(cpu.registers().s + 1);  // LOW at S+1
    
    assert_eq!(high_byte, 0x11);  // High byte of 0x1122
    assert_eq!(low_byte, 0x22);   // Low byte of 0x1122
    assert_eq!((high_byte as u16) << 8 | low_byte as u16, expected_x);
}

#[test]
fn test_pshs_register_order_compliance() {
    // Test C++ register push order: PC, U, Y, X, DP, B, A, CC (highest to lowest bit)
    // C++ pushes in BIT ORDER, so bit 7 (PC) first, then bit 6 (U), etc.
    let mut cpu = setup_test_cpu_with_registers();
    
    // Write PSHS instruction with multiple registers: PC(0x80) + A(0x02) + CC(0x01) = 0x83
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // PC + A + CC bits
    
    let initial_s = cpu.registers().s;
    let expected_a = cpu.registers().a;
    let expected_cc = cpu.registers().cc.to_u8();
    
    // Execute PSHS
    cpu.execute_instruction(false, false);
    
    // The PC that gets pushed is PC after instruction fetch (PC + 2)
    let expected_pc = 0xC802; // Initial PC (0xC800) + instruction size (2)
    
    // Verify total stack usage: PC(2) + A(1) + CC(1) = 4 bytes
    assert_eq!(cpu.registers().s, initial_s - 4);
    
    // Verify push order: PC first (highest bit), then A, then CC (lowest bit)
    let stack_base = cpu.registers().s;
    
    // C++ Push order with mask 0x83: PC(bit7), A(bit1), CC(bit0)
    // Final stack layout: [S+0]=CC, [S+1]=A, [S+2]=PC_HIGH, [S+3]=PC_LOW
    
    // CC is at S+0
    let pushed_cc = cpu.memory_bus().borrow().read(stack_base);
    assert_eq!(pushed_cc, expected_cc);
    
    // A is at S+1
    let pushed_a = cpu.memory_bus().borrow().read(stack_base + 1);
    assert_eq!(pushed_a, expected_a);
    
    // PC is at S+2 (HIGH) and S+3 (LOW)
    let pc_high = cpu.memory_bus().borrow().read(stack_base + 2);
    let pc_low = cpu.memory_bus().borrow().read(stack_base + 3);
    let pushed_pc = (pc_high as u16) << 8 | pc_low as u16;
    assert_eq!(pushed_pc, expected_pc);
}

#[test]
fn test_pshs_all_registers_full_compliance() {
    // Test pushing all registers (0xFF mask) - complete C++ order validation
    let mut cpu = setup_test_cpu_with_registers(); 
    
    // Write PSHS instruction with all registers (0xFF)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0xFF); // All registers
    
    let initial_s = cpu.registers().s;
    
    // Execute PSHS
    cpu.execute_instruction(false, false);
    
    // The PC that gets pushed is PC after instruction fetch (PC + 2)
    let expected_pc = 0xC802; // Initial PC (0xC800) + instruction size (2)
    
    // Verify total stack usage: PC(2) + U(2) + Y(2) + X(2) + DP(1) + B(1) + A(1) + CC(1) = 12 bytes
    assert_eq!(cpu.registers().s, initial_s - 12);
    
    let stack_base = cpu.registers().s;
    
    // Verify complete push order (C++ reverse stack layout - last pushed = lowest address):
    // Stack grows DOWN, so last pushed item (CC) is at lowest address [S+0]
    
    // Bit 0 (CC): [S+0] = CC (last pushed, first in memory)
    let pushed_cc = cpu.memory_bus().borrow().read(stack_base);
    assert_eq!(pushed_cc, cpu.registers().cc.to_u8());
    
    // Bit 1 (A): [S+1] = A
    let pushed_a = cpu.memory_bus().borrow().read(stack_base + 1);
    assert_eq!(pushed_a, 0xAA);
    
    // Bit 2 (B): [S+2] = B
    let pushed_b = cpu.memory_bus().borrow().read(stack_base + 2);
    assert_eq!(pushed_b, 0xBB);
    
    // Bit 3 (DP): [S+3] = DP
    let pushed_dp = cpu.memory_bus().borrow().read(stack_base + 3);
    assert_eq!(pushed_dp, 0x77);
    
    // Bit 4 (X): [S+4,S+5] = X_HIGH, X_LOW (C++ Push16: low first, high second)
    let x_high = cpu.memory_bus().borrow().read(stack_base + 4);
    let x_low = cpu.memory_bus().borrow().read(stack_base + 5);
    assert_eq!((x_high as u16) << 8 | x_low as u16, 0x1122);
    
    // Bit 5 (Y): [S+6,S+7] = Y_HIGH, Y_LOW
    let y_high = cpu.memory_bus().borrow().read(stack_base + 6);
    let y_low = cpu.memory_bus().borrow().read(stack_base + 7);
    assert_eq!((y_high as u16) << 8 | y_low as u16, 0x3344);
    
    // Bit 6 (U): [S+8,S+9] = U_HIGH, U_LOW
    let u_high = cpu.memory_bus().borrow().read(stack_base + 8);
    let u_low = cpu.memory_bus().borrow().read(stack_base + 9);
    assert_eq!((u_high as u16) << 8 | u_low as u16, 0x5566);
    
    // Bit 7 (PC): [S+10,S+11] = PC_HIGH, PC_LOW (first pushed, last in memory)
    let pc_high = cpu.memory_bus().borrow().read(stack_base + 10);
    let pc_low = cpu.memory_bus().borrow().read(stack_base + 11);
    assert_eq!((pc_high as u16) << 8 | pc_low as u16, expected_pc);
}

// ============================================================================
// PULS (0x35) - Pull System Stack Tests  
// ============================================================================

#[test]
fn test_puls_single_register_cc() {
    // Test pulling single register validates reverse order and stack increment
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup stack with known CC value
    let test_cc = 0x55;
    cpu.registers_mut().s = 0xCF00 - 1;  // Pre-decremented stack
    cpu.memory_bus().borrow_mut().write(0xCF00 - 1, test_cc);
    
    // Write PULS instruction with CC bit
    cpu.memory_bus().borrow_mut().write(0xC800, 0x35); // PULS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // CC bit mask
    
    let initial_s = cpu.registers().s;
    
    // Execute PULS
    cpu.execute_instruction(false, false);
    
    // Verify stack pointer incremented by 1
    assert_eq!(cpu.registers().s, initial_s + 1);
    
    // Verify CC pulled from stack
    assert_eq!(cpu.registers().cc.to_u8(), test_cc);
}

#[test]
fn test_puls_reverse_order_compliance() {
    // Test PULS exactly as C++ OpPUL does:
    // C++ OpPUL processes bits 0-7 sequentially: CC(bit0), A(bit1)... PC(bit7)
    // Each Pop8/Pop16 reads from stackPointer++ (current position, then increment)
    let mut cpu = setup_test_cpu_with_registers();
    
    // First do PSHS to establish correct stack layout
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x83); // PC + A + CC bits
    
    // Set known values before PSHS
    cpu.registers_mut().a = 0x56;
    cpu.registers_mut().cc.from_u8(0x78);
    
    // Execute PSHS to create proper stack layout
    cpu.execute_instruction(false, false);
    
    // Capture the correct PC that was pushed (after instruction execution)
    let pushed_pc = cpu.registers().pc; // This is where PC advanced to after PSHS
    
    // Now modify registers to test PULS restoration
    cpu.registers_mut().a = 0x00;       // Clear A
    cpu.registers_mut().cc.from_u8(0x00); // Clear CC  
    cpu.registers_mut().pc = 0xC802;     // Set to next instruction
    
    // Write PULS instruction with same mask
    cpu.memory_bus().borrow_mut().write(0xC802, 0x35); // PULS opcode
    cpu.memory_bus().borrow_mut().write(0xC803, 0x83); // Same mask: PC + A + CC
    
    let stack_before_puls = cpu.registers().s;
    
    // Execute PULS - this should read in C++ bit order: CC(0), A(1), PC(7)
    cpu.execute_instruction(false, false);
    
    // Verify stack pointer correctly incremented (4 bytes restored)
    assert_eq!(cpu.registers().s, stack_before_puls + 4);
    
    // Verify registers restored to values that were originally pushed
    // C++ OpPUL reads bit 0 (CC) first from current stack position
    assert_eq!(cpu.registers().cc.to_u8(), 0x78, "CC should be restored from PSHS");
    
    // C++ OpPUL reads bit 1 (A) second from next stack position  
    assert_eq!(cpu.registers().a, 0x56, "A should be restored from PSHS");
    
    // C++ OpPUL reads bit 7 (PC) last from remaining stack positions
    assert_eq!(cpu.registers().pc, pushed_pc, "PC should be restored to pushed value");
}

// ============================================================================
// PSHU/PULU (0x36/0x37) - User Stack Tests
// ============================================================================

#[test]
fn test_pshu_user_stack_separate_from_system() {
    // Test that PSHU uses U register instead of S register
    let mut cpu = setup_test_cpu_with_registers();
    cpu.registers_mut().u = 0xCE00;  // User stack in different area
    
    // Write PSHU instruction with A register
    cpu.memory_bus().borrow_mut().write(0xC800, 0x36); // PSHU opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x02); // A bit mask
    
    let initial_s = cpu.registers().s;  // System stack should not change
    let initial_u = cpu.registers().u;  // User stack should change
    let expected_a = cpu.registers().a;
    
    // Execute PSHU
    cpu.execute_instruction(false, false);
    
    // Verify system stack unchanged
    assert_eq!(cpu.registers().s, initial_s);
    
    // Verify user stack decremented
    assert_eq!(cpu.registers().u, initial_u - 1);
    
    // Verify A pushed to user stack (not system stack)
    let pushed_a = cpu.memory_bus().borrow().read(cpu.registers().u);
    assert_eq!(pushed_a, expected_a);
    
    // Verify system stack area unchanged by reading before/after
    // (We cannot assume memory is zero-initialized, only that PSHU doesn't modify system stack)
    // The real test is that S register is unchanged and U register changed correctly
}

#[test]
fn test_pshu_register_s_instead_of_u() {
    // Test PSHU register substitution: S register pushed instead of U register
    // C++ PSHU pushes S in place of U (bit 6), while PSHS pushes U in place of S
    let mut cpu = setup_test_cpu_with_registers();
    cpu.registers_mut().u = 0xCE00;
    
    // Write PSHU with "U" bit (0x40) - but PSHU pushes S register instead
    cpu.memory_bus().borrow_mut().write(0xC800, 0x36); // PSHU opcode  
    cpu.memory_bus().borrow_mut().write(0xC801, 0x40); // "U" bit, but pushes S
    
    let expected_s = cpu.registers().s;  // This should be pushed
    let initial_u = cpu.registers().u;
    
    // Execute PSHU
    cpu.execute_instruction(false, false);
    
    // Verify U stack decremented by 2 (16-bit register)
    assert_eq!(cpu.registers().u, initial_u - 2);
    
    // Verify S register value pushed (not U register value)
    let high_byte = cpu.memory_bus().borrow().read(cpu.registers().u);
    let low_byte = cpu.memory_bus().borrow().read(cpu.registers().u + 1);
    let pushed_value = (high_byte as u16) << 8 | low_byte as u16;
    
    assert_eq!(pushed_value, expected_s);  // S pushed, not U
}

// ============================================================================
// Cross-Stack Validation Tests
// ============================================================================

#[test]
fn test_pshs_puls_roundtrip_all_registers() {
    // Test complete roundtrip: PSHS all -> PULS all -> verify restoration
    let mut cpu = setup_test_cpu_with_registers();
    
    // Capture initial state
    let initial_state = (
        cpu.registers().a, cpu.registers().b, cpu.registers().x, cpu.registers().y,
        cpu.registers().u, cpu.registers().s, cpu.registers().dp, cpu.registers().cc.to_u8()
    );
    
    // PSHS all registers (except PC which gets modified)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS
    cpu.memory_bus().borrow_mut().write(0xC801, 0x7F); // All except PC (0xFF & !0x80)
    cpu.execute_instruction(false, false);
    
    // Modify registers to verify they get restored
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().b = 0x11;
    cpu.registers_mut().x = 0x2222;
    cpu.registers_mut().y = 0x3333;
    cpu.registers_mut().u = 0x4444;
    cpu.registers_mut().dp = 0x55;
    cpu.registers_mut().cc.from_u8(0x66);
    
    // PULS same registers back
    cpu.memory_bus().borrow_mut().write(0xC802, 0x35); // PULS (next instruction)
    cpu.memory_bus().borrow_mut().write(0xC803, 0x7F); // Same mask
    cpu.registers_mut().pc = 0xC802; // Position for PULS
    cpu.execute_instruction(false, false);
    
    // Verify complete restoration (except PC)
    assert_eq!(cpu.registers().a, initial_state.0);
    assert_eq!(cpu.registers().b, initial_state.1);  
    assert_eq!(cpu.registers().x, initial_state.2);
    assert_eq!(cpu.registers().y, initial_state.3);
    assert_eq!(cpu.registers().u, initial_state.4);
    assert_eq!(cpu.registers().s, initial_state.5);  // Stack should be restored
    assert_eq!(cpu.registers().dp, initial_state.6);
    assert_eq!(cpu.registers().cc.to_u8(), initial_state.7);
}

// ============================================================================
// Edge Cases and Error Conditions  
// ============================================================================

#[test]
fn test_stack_operations_with_zero_mask() {
    // Test behavior with zero mask (no registers) - should be no-op
    let mut cpu = setup_test_cpu_with_registers();
    
    let initial_s = cpu.registers().s;
    
    // PSHS with zero mask
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS
    cpu.memory_bus().borrow_mut().write(0xC801, 0x00); // No registers
    cpu.execute_instruction(false, false);
    
    // Verify no stack change
    assert_eq!(cpu.registers().s, initial_s);
}

#[test]
fn test_stack_boundary_behavior() {
    // Test stack operations near boundaries - validates no overflow/underflow issues
    let mut cpu = setup_test_cpu_with_registers();
    
    // Test near bottom of stack space
    cpu.registers_mut().s = 0xC801;  // Very low in RAM
    
    // PSHS single register should work
    cpu.memory_bus().borrow_mut().write(0xC800, 0x34); // PSHS
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // CC only
    
    // Should not panic or cause issues
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().s, 0xC800);
}

// ============================================================================
// JSR/BSR STACK ORDER COMPLIANCE TESTS
// ============================================================================

#[test]
fn test_jsr_direct_stack_order_compliance() {
    // C++ Original: OpJSR() calls Push16(S, PC) which is:
    // void Push16(uint16_t& S, uint16_t value) {
    //     --S; m_memory->Write(S, value & 0xFF);        // LOW byte first
    //     --S; m_memory->Write(S, (value >> 8) & 0xFF); // HIGH byte second
    // }
    // Stack layout: [HIGH][LOW] with HIGH at lower address (S), LOW at S+1
    
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup JSR Direct: PC=0xC800, target=0x2050, SP=0xCF00
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;
    cpu.registers_mut().dp = 0x20;   // Direct page for address calculation
    
    // Write JSR Direct instruction: 0x9D 0x50 (JSR $2050)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x9D);  // JSR direct opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x50);  // Direct page offset
    
    // Execute JSR instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify C++ compliance: cycles and PC jump
    assert_eq!(cycles, 7, "JSR direct should take 7 cycles");
    assert_eq!(cpu.registers().pc, 0x2050, "PC should jump to 0x2050");
    
    // Verify stack order: return address 0xC802 pushed as [0xC8][0x02]
    assert_eq!(cpu.registers().s, 0xCEFE, "Stack pointer should decrement by 2");
    
    // C++ Original: HIGH byte at S, LOW byte at S+1
    let stack_high = cpu.memory_bus().borrow().read(0xCEFE);  // HIGH at S
    let stack_low = cpu.memory_bus().borrow().read(0xCEFF);   // LOW at S+1
    
    assert_eq!(stack_high, 0xC8, "HIGH byte (0xC8) should be at stack address S");
    assert_eq!(stack_low, 0x02, "LOW byte (0x02) should be at stack address S+1");
    
    // Verify full return address reconstruction
    let return_address = (stack_high as u16) << 8 | (stack_low as u16);
    assert_eq!(return_address, 0xC802, "Return address should reconstruct to 0xC802");
}

#[test]
fn test_jsr_extended_stack_order_compliance() {
    // C++ Original: Same Push16(S, PC) behavior for all JSR variants
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup JSR Extended: PC=0xC800, target=0x4000, SP=0xCF00
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;
    
    // Write JSR Extended instruction: 0xBD 0x40 0x00 (JSR $4000)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xBD);  // JSR extended opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x40);  // Address high byte
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00);  // Address low byte
    
    // Execute JSR instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify C++ compliance
    assert_eq!(cycles, 8, "JSR extended should take 8 cycles");
    assert_eq!(cpu.registers().pc, 0x4000, "PC should jump to 0x4000");
    
    // Verify stack order: return address 0xC803 pushed as [0xC8][0x03]
    assert_eq!(cpu.registers().s, 0xCEFE, "Stack pointer should decrement by 2");
    
    let stack_high = cpu.memory_bus().borrow().read(0xCEFE);  // HIGH at S
    let stack_low = cpu.memory_bus().borrow().read(0xCEFF);   // LOW at S+1
    
    assert_eq!(stack_high, 0xC8, "HIGH byte (0xC8) should be at stack address S");
    assert_eq!(stack_low, 0x03, "LOW byte (0x03) should be at stack address S+1");
}

#[test]
fn test_bsr_stack_order_compliance() {
    // C++ Original: OpBSR() calls Push16(S, PC) same as JSR
    // void OpBSR() {
    //     int8_t offset = ReadRelativeOffset8();
    //     Push16(S, PC);
    //     PC += offset;
    // }
    
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup BSR: PC=0xC800, relative offset=+0x20, SP=0xCF00
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;
    
    // Write BSR instruction: 0x8D 0x20 (BSR +32)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8D);  // BSR opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x20);  // Relative offset +32
    
    // Execute BSR instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify C++ compliance
    assert_eq!(cycles, 7, "BSR should take 7 cycles");
    assert_eq!(cpu.registers().pc, 0xC822, "PC should be 0xC800+0x20+2=0xC822");
    
    // Verify stack order: return address 0xC802 pushed as [0xC8][0x02]
    assert_eq!(cpu.registers().s, 0xCEFE, "Stack pointer should decrement by 2");
    
    let stack_high = cpu.memory_bus().borrow().read(0xCEFE);  // HIGH at S
    let stack_low = cpu.memory_bus().borrow().read(0xCEFF);   // LOW at S+1
    
    assert_eq!(stack_high, 0xC8, "HIGH byte (0xC8) should be at stack address S");
    assert_eq!(stack_low, 0x02, "LOW byte (0x02) should be at stack address S+1");
}

#[test]
fn test_jsr_indexed_stack_order_compliance() {
    // C++ Original: JSR indexed uses same Push16(S, PC) as other JSR variants
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup JSR Indexed: PC=0xC800, X=0x3000, SP=0xCF00
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;
    cpu.registers_mut().x = 0x3000;  // Index register
    
    // Write JSR Indexed instruction: 0xAD 0x84 (JSR ,X - no offset)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xAD);  // JSR indexed opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x84);  // Postbyte: ,X (no offset)
    
    // Execute JSR instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify C++ compliance
    assert_eq!(cycles, 7, "JSR indexed should take 7 cycles");
    assert_eq!(cpu.registers().pc, 0x3000, "PC should jump to X register value");
    
    // Verify stack order: return address 0xC802 pushed as [0xC8][0x02]
    assert_eq!(cpu.registers().s, 0xCEFE, "Stack pointer should decrement by 2");
    
    let stack_high = cpu.memory_bus().borrow().read(0xCEFE);  // HIGH at S
    let stack_low = cpu.memory_bus().borrow().read(0xCEFF);   // LOW at S+1
    
    assert_eq!(stack_high, 0xC8, "HIGH byte (0xC8) should be at stack address S");
    assert_eq!(stack_low, 0x02, "LOW byte (0x02) should be at stack address S+1");
}

#[test]
fn test_multiple_jsr_calls_stack_accumulation() {
    // Test multiple JSR calls create proper stack frame accumulation
    // C++ Original: Each JSR should push its return address in sequence
    
    let mut cpu = setup_test_cpu_with_registers();
    
    // Setup initial state: PC=0xC800, SP=0xCF00
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;
    cpu.registers_mut().dp = 0xC8;   // Direct page within RAM range 0xC800-0xCFFF
    
    // First JSR Direct: JSR $C850 (DP:0x50 = 0xC8:0x50 = 0xC850)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x9D);  // JSR direct
    cpu.memory_bus().borrow_mut().write(0xC801, 0x50);  // Direct offset
    
    // Execute first JSR
    cpu.execute_instruction(false, false);
    
    // Verify first JSR stack layout
    assert_eq!(cpu.registers().pc, 0xC850, "PC should be at first JSR target");
    assert_eq!(cpu.registers().s, 0xCEFE, "Stack should have one return address");
    
    // Check first return address (0xC802) on stack
    let first_high = cpu.memory_bus().borrow().read(0xCEFE);
    let first_low = cpu.memory_bus().borrow().read(0xCEFF);
    assert_eq!(first_high, 0xC8);
    assert_eq!(first_low, 0x02);
    
    // Setup second JSR Extended at new PC location: JSR $C900 (within RAM)
    cpu.memory_bus().borrow_mut().write(0xC850, 0xBD);  // JSR extended
    cpu.memory_bus().borrow_mut().write(0xC851, 0xC9);  // Address high (RAM)
    cpu.memory_bus().borrow_mut().write(0xC852, 0x00);  // Address low
    
    // Execute second JSR
    cpu.execute_instruction(false, false);
    
    // Verify second JSR stack layout  
    assert_eq!(cpu.registers().pc, 0xC900, "PC should be at second JSR target");
    assert_eq!(cpu.registers().s, 0xCEFC, "Stack should have two return addresses");
    
    // Check second return address (0xC853) on stack - most recent at top
    let second_high = cpu.memory_bus().borrow().read(0xCEFC);  // Most recent HIGH
    let second_low = cpu.memory_bus().borrow().read(0xCEFD);   // Most recent LOW
    assert_eq!(second_high, 0xC8);
    assert_eq!(second_low, 0x53);
    
    // Verify first return address still intact below
    let preserved_first_high = cpu.memory_bus().borrow().read(0xCEFE);
    let preserved_first_low = cpu.memory_bus().borrow().read(0xCEFF);
    assert_eq!(preserved_first_high, 0xC8);
    assert_eq!(preserved_first_low, 0x02);
}