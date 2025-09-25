// test_jsr_bsr_opcodes.rs - Tests for JSR/BSR subroutine call opcodes
// C++ Original Analysis from Vectrexy Cpu.cpp lines 353-357 and 789-799:
//
// template <int page, uint8_t opCode>
// void OpJSR() {
//     uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>();
//     Push16(S, PC);
//     PC = EA;
// }
//
// void OpBSR() {
//     int8_t offset = ReadRelativeOffset8();
//     Push16(S, PC);
//     PC += offset;
// }
//
// void OpLBSR() {
//     int16_t offset = ReadRelativeOffset16();
//     Push16(S, PC);
//     PC += offset;
// }
//
// Key Points:
// 1. JSR: Jump to Subroutine - absolute addressing (direct, indexed, extended)
// 2. BSR: Branch to Subroutine - relative 8-bit offset
// 3. LBSR: Long Branch to Subroutine - relative 16-bit offset  
// 4. All push return address (current PC) to system stack BEFORE jumping
// 5. Push16(S, PC) pushes high byte first, then low byte (stack grows down)
// 6. Opcodes: JSR Direct(0x9D), JSR Indexed(0xAD), JSR Extended(0xBD), BSR(0x8D), LBSR(0x17)
// 7. Cycles: JSR Direct/Indexed=7, JSR Extended=8, BSR=7, LBSR=9

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
fn test_jsr_direct_0x9d() {
    // C++ Original: OpJSR<0, 0x9D>(); - JSR Direct addressing
    // Tests JSR with direct page addressing: EA = DP:immediate_byte
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF, DP=0x20
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;  // System stack in RAM area
    cpu.registers_mut().dp = 0x20;   // Direct page
    
    // Write JSR Direct instruction: 0x9D 0x50 (JSR $2050)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x9D);  // JSR direct opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x50);  // Direct page offset
    
    // Execute instruction  
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles (C++ Original: JSR direct = 7 cycles)
    assert_eq!(cycles, 7);
    
    // Verify PC jumped to EA = DP:immediate = 0x20:0x50 = 0x2050
    assert_eq!(cpu.registers().pc, 0x2050);
    
    // Verify return address (PC after JSR instruction = 0xC802) pushed to stack
    // C++ Original: Push16(S, PC) - high byte first, then low byte
    assert_eq!(cpu.registers().s, 0xCFFD);  // Stack pointer decremented by 2
    
    // Check stack contents: high byte at 0xCFFD, low byte at 0xCFFE
    // C++ Push16: --S, write LOW; --S, write HIGH. So HIGH at S, LOW at S+1
    let return_high = cpu.memory_bus().borrow().read(0xCFFD);  // HIGH byte at S
    let return_low = cpu.memory_bus().borrow().read(0xCFFE);   // LOW byte at S+1
    assert_eq!(return_high, 0xC8);  // High byte of 0xC802
    assert_eq!(return_low, 0x02);   // Low byte of 0xC802
}

#[test]  
fn test_jsr_indexed_0xad() {
    // C++ Original: OpJSR<0, 0xAD>(); - JSR Indexed addressing
    // Tests JSR with indexed addressing using X register
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF, X=0x3000
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    cpu.registers_mut().x = 0x3000;  // Index register
    
    // Write JSR Indexed instruction: 0xAD 0x84 (JSR ,X - no offset)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xAD);  // JSR indexed opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x84);  // Postbyte: ,X (no offset)
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles (C++ Original: JSR indexed = 7 cycles base + indexed cycles)
    assert_eq!(cycles, 7);
    
    // Verify PC jumped to EA = X = 0x3000
    assert_eq!(cpu.registers().pc, 0x3000);
    
    // Verify return address pushed to stack
    assert_eq!(cpu.registers().s, 0xCFFD);
    let return_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                      (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    assert_eq!(return_addr, 0xC802);  // PC after JSR instruction
}

#[test]
fn test_jsr_extended_0xbd() {
    // C++ Original: OpJSR<0, 0xBD>(); - JSR Extended addressing  
    // Tests JSR with extended addressing (16-bit absolute address)
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    
    // Write JSR Extended instruction: 0xBD 0x40 0x00 (JSR $4000)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xBD);  // JSR extended opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x40);  // High byte of address
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00);  // Low byte of address
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles (C++ Original: JSR extended = 8 cycles)
    assert_eq!(cycles, 8);
    
    // Verify PC jumped to EA = 0x4000
    assert_eq!(cpu.registers().pc, 0x4000);
    
    // Verify return address pushed to stack
    assert_eq!(cpu.registers().s, 0xCFFD);
    let return_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                      (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    assert_eq!(return_addr, 0xC803);  // PC after 3-byte JSR instruction
}

#[test]
fn test_bsr_relative_0x8d() {
    // C++ Original: OpBSR(); - BSR relative addressing
    // Tests BSR with 8-bit signed relative offset
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    
    // Write BSR instruction: 0x8D 0x10 (BSR +16)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8D);  // BSR opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x10);  // +16 offset
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles (C++ Original: BSR = 7 cycles)
    assert_eq!(cycles, 7);
    
    // Verify PC = PC + offset = 0xC802 + 0x10 = 0xC812
    assert_eq!(cpu.registers().pc, 0xC812);
    
    // Verify return address pushed to stack
    assert_eq!(cpu.registers().s, 0xCFFD);
    let return_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                      (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    assert_eq!(return_addr, 0xC802);  // PC after BSR instruction
}

#[test]
fn test_bsr_negative_offset() {
    // C++ Original: OpBSR(); - BSR with negative relative offset
    // Tests BSR backward branch with 8-bit signed offset
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    
    // Write BSR instruction: 0x8D 0xF0 (BSR -16)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x8D);  // BSR opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0xF0);  // -16 offset (0xF0 = -16 in signed 8-bit)
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles
    assert_eq!(cycles, 7);
    
    // Verify PC = PC + offset = 0xC802 + (-16) = 0xC7F2
    assert_eq!(cpu.registers().pc, 0xC7F2);
    
    // Verify return address pushed to stack
    assert_eq!(cpu.registers().s, 0xCFFD);
    let return_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                      (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    assert_eq!(return_addr, 0xC802);
}

#[test]
fn test_lbsr_relative_0x17() {
    // C++ Original: OpLBSR(); - LBSR with 16-bit relative offset
    // Tests LBSR (Long Branch to Subroutine) with 16-bit signed offset
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    
    // Write LBSR instruction: 0x17 0x12 0x34 (LBSR +0x1234)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x17);  // LBSR opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12);  // High byte of offset
    cpu.memory_bus().borrow_mut().write(0xC802, 0x34);  // Low byte of offset
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify cycles (C++ Original: LBSR = 9 cycles)
    assert_eq!(cycles, 9);
    
    // Verify PC = PC + offset = 0xC803 + 0x1234 = 0xDA37
    assert_eq!(cpu.registers().pc, 0xDA37);
    
    // Verify return address pushed to stack
    assert_eq!(cpu.registers().s, 0xCFFD);
    let return_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                      (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    assert_eq!(return_addr, 0xC803);  // PC after 3-byte LBSR instruction
}

#[test]
fn test_stack_grows_down() {
    // Verify stack behavior: multiple JSR calls should decrement stack pointer
    let mut cpu = create_test_cpu();
    
    // Setup: PC at 0xC800, Stack at 0xCFFF
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCFFF;
    
    // First JSR: 0x9D 0x10 (JSR $2010) - using direct page addressing
    cpu.memory_bus().borrow_mut().write(0xC800, 0x9D);
    cpu.memory_bus().borrow_mut().write(0xC801, 0x10);
    cpu.registers_mut().dp = 0x20;  // Direct page = 0x20, so EA = 0x2010
    
    cpu.execute_instruction(false, false);
    
    // After first JSR: stack at 0xCFFD, PC at 0x2010
    assert_eq!(cpu.registers().s, 0xCFFD);
    assert_eq!(cpu.registers().pc, 0x2010);
    
    // Use BSR for second call to stay within executable range
    // Setup BSR with small positive offset: BSR +6 (jump ahead 6 bytes)
    cpu.registers_mut().pc = 0xC820;  // Move PC to safe area within RAM
    cpu.memory_bus().borrow_mut().write(0xC820, 0x8D);  // BSR opcode
    cpu.memory_bus().borrow_mut().write(0xC821, 0x06);  // Offset +6
    
    cpu.execute_instruction(false, false);
    
    // After second BSR: stack at 0xCFFB, PC at 0xC828 (0xC822 + 6)
    assert_eq!(cpu.registers().s, 0xCFFB);
    assert_eq!(cpu.registers().pc, 0xC828);
    
    // Verify nested return addresses on stack
    // First return address (from JSR): pushed to 0xCFFD-0xCFFE
    let return1_addr = ((cpu.memory_bus().borrow().read(0xCFFD) as u16) << 8) | 
                       (cpu.memory_bus().borrow().read(0xCFFE) as u16);
    // Second return address (from BSR): pushed to 0xCFFB-0xCFFC  
    let return2_addr = ((cpu.memory_bus().borrow().read(0xCFFB) as u16) << 8) | 
                       (cpu.memory_bus().borrow().read(0xCFFC) as u16);
                       
    assert_eq!(return1_addr, 0xC802);  // Return from first JSR  
    assert_eq!(return2_addr, 0xC822);  // Return from BSR
}

#[test]
fn test_push16_exact_c_plus_plus_behavior() {
    // Test exact C++ Push16 behavior with specific values - validates 1:1 compliance
    // C++ Push16: m_memoryBus->Write(--stackPointer, LOW); m_memoryBus->Write(--stackPointer, HIGH);
    // Result: [SP]=HIGH, [SP+1]=LOW (stack grows down)
    let mut cpu = create_test_cpu();
    
    // Setup: PC at test address, stack at known position  
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().s = 0xCF00;  // Stack pointer in RAM area
    
    // Write JSR instruction that will push PC (0xC803 after reading instruction)
    cpu.memory_bus().borrow_mut().write(0xC800, 0xBD);  // JSR Extended
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC8);  // High byte of target (RAM area)
    cpu.memory_bus().borrow_mut().write(0xC802, 0x50);  // Low byte of target
    
    // Execute - this will push return address 0xC803 to stack
    cpu.execute_instruction(false, false);
    
    // Verify C++ Push16 exact behavior:
    // C++ does: --stackPointer (CEFF), write LOW(0x03); --stackPointer (CEFE), write HIGH(0xC8)
    // Result: [CEFE]=0xC8 (HIGH), [CEFF]=0x03 (LOW), S=CEFE
    
    assert_eq!(cpu.registers().s, 0xCEFE);  // Stack decremented by 2
    
    let high_byte = cpu.memory_bus().borrow().read(0xCEFE);  // HIGH at S
    let low_byte = cpu.memory_bus().borrow().read(0xCEFF);   // LOW at S+1
    
    assert_eq!(high_byte, 0xC8);  // HIGH byte of return address 0xC803
    assert_eq!(low_byte, 0x03);   // LOW byte of return address 0xC803
    
    // Verify PC jumped to target address
    assert_eq!(cpu.registers().pc, 0xC850);
}

#[test]
fn test_multiple_pushes_stack_pattern() {
    // Test that multiple pushes create correct C++ stack pattern - critical for nested calls
    // Validates stack frame layout matches C++ exactly for RTS and interrupt handling
    let mut cpu = create_test_cpu();
    
    // Start at specific stack position
    cpu.registers_mut().s = 0xCF20;
    
    // First push: JSR to push return address
    cpu.registers_mut().pc = 0xC850;  // In RAM area
    cpu.memory_bus().borrow_mut().write(0xC850, 0xBD);  // JSR Extended 
    cpu.memory_bus().borrow_mut().write(0xC851, 0xC8);  // Target high (RAM)
    cpu.memory_bus().borrow_mut().write(0xC852, 0x80);  // Target low
    
    cpu.execute_instruction(false, false);
    
    // Second push: BSR to push next address  
    cpu.registers_mut().pc = 0xC880;  // In RAM area
    cpu.memory_bus().borrow_mut().write(0xC880, 0x8D);  // BSR
    cpu.memory_bus().borrow_mut().write(0xC881, 0x10);  // Offset
    
    cpu.execute_instruction(false, false);
    
    // Verify stack layout follows C++ pattern:
    // First push (0xC853): [CF1E]=0xC8, [CF1F]=0x53
    // Second push (0xC882): [CF1C]=0xC8, [CF1D]=0x82
    // Stack pointer at CF1C
    
    assert_eq!(cpu.registers().s, 0xCF1C);
    
    // Check first pushed value (deeper in stack) - HIGH/LOW pattern
    assert_eq!(cpu.memory_bus().borrow().read(0xCF1E), 0xC8);  // HIGH of 0xC853
    assert_eq!(cpu.memory_bus().borrow().read(0xCF1F), 0x53);  // LOW of 0xC853
    
    // Check second pushed value (top of stack) - HIGH/LOW pattern  
    assert_eq!(cpu.memory_bus().borrow().read(0xCF1C), 0xC8);  // HIGH of 0xC882
    assert_eq!(cpu.memory_bus().borrow().read(0xCF1D), 0x82);  // LOW of 0xC882
}

#[test]
fn test_jsr_bsr_comprehensive() {
    // Comprehensive test covering all JSR/BSR variations
    let mut cpu = create_test_cpu();
    
    // Test all opcodes execute without panic and modify PC correctly
    let test_cases = vec![
        (0x9D, "JSR Direct", 7),
        (0xAD, "JSR Indexed", 7),  
        (0xBD, "JSR Extended", 8),
        (0x8D, "BSR Relative", 7),
        (0x17, "LBSR Long Relative", 9),
    ];
    
    for (opcode, name, expected_cycles) in test_cases {
        // Reset for each test
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().s = 0xCFFF;
        cpu.registers_mut().dp = 0x20;
        cpu.registers_mut().x = 0x3000;
        
        // Write appropriate instruction bytes
        match opcode {
            0x9D => {  // JSR Direct
                cpu.memory_bus().borrow_mut().write(0xC800, 0x9D);
                cpu.memory_bus().borrow_mut().write(0xC801, 0x50);
            },
            0xAD => {  // JSR Indexed
                cpu.memory_bus().borrow_mut().write(0xC800, 0xAD);
                cpu.memory_bus().borrow_mut().write(0xC801, 0x84);  // ,X
            },
            0xBD => {  // JSR Extended
                cpu.memory_bus().borrow_mut().write(0xC800, 0xBD);
                cpu.memory_bus().borrow_mut().write(0xC801, 0x40);
                cpu.memory_bus().borrow_mut().write(0xC802, 0x00);
            },
            0x8D => {  // BSR
                cpu.memory_bus().borrow_mut().write(0xC800, 0x8D);
                cpu.memory_bus().borrow_mut().write(0xC801, 0x10);
            },
            0x17 => {  // LBSR
                cpu.memory_bus().borrow_mut().write(0xC800, 0x17);
                cpu.memory_bus().borrow_mut().write(0xC801, 0x12);
                cpu.memory_bus().borrow_mut().write(0xC802, 0x34);
            },
            _ => panic!("Unknown opcode in test")
        }
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify cycles match expected
        assert_eq!(cycles, expected_cycles, "{} cycles mismatch", name);
        
        // Verify PC changed (jumped somewhere)
        assert_ne!(cpu.registers().pc, 0xC800, "{} PC should have changed", name);
        
        // Verify stack pointer decremented by 2 (return address pushed)
        assert_eq!(cpu.registers().s, 0xCFFD, "{} stack should be decremented", name);
        
        println!("✅ {} test passed", name);
    }
}
