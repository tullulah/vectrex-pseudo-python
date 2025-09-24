// Tests for RTS (Return from Subroutine) opcode implementation
// C++ Original validation: PC = Pop16(S); with 5 cycles

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
fn test_rts_basic_functionality() {
    // Test RTS opcode basic functionality: PC = Pop16(S)
    let mut cpu = create_test_cpu();
    
    // Manually set up stack with return address 0xC900 (in RAM area)
    cpu.registers_mut().s = 0xCA00; // Use RAM stack area (0xC800-0xCFFF)
    cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
    cpu.memory_bus().borrow_mut().write(cpu.registers().s, 0x00); // Low byte first
    cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
    cpu.memory_bus().borrow_mut().write(cpu.registers().s, 0xC9); // High byte second
    
    // Place RTS instruction in RAM area 
    cpu.registers_mut().pc = 0xC800; // Set PC to RAM area
    cpu.memory_bus().borrow_mut().write(0xC800, 0x39); // RTS opcode
    
    let initial_s = cpu.registers().s;
    
    // Execute RTS
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify PC was set to return address
    assert_eq!(cpu.registers().pc, 0xC900, "RTS should set PC to popped return address");
    
    // Verify stack pointer was restored (incremented by 2)
    assert_eq!(cpu.registers().s, initial_s.wrapping_add(2), "RTS should restore stack pointer by popping 2 bytes");
    
    // Verify RTS takes exactly 5 cycles
    assert_eq!(cycles, 5, "RTS should consume exactly 5 cycles");
}

#[test]
fn test_jsr_rts_roundtrip() {
    // Test complete JSR→RTS cycle to verify stack compliance
    let mut cpu = create_test_cpu();
    
    // Set up initial state
    let initial_pc = 0xC800; // Use RAM area
    let subroutine_addr = 0xC900;
    let return_addr = initial_pc + 3; // JSR takes 3 bytes (opcode + 16-bit address)
    
    cpu.registers_mut().pc = initial_pc;
    cpu.registers_mut().s = 0xCA00; // Initial stack pointer in RAM area
    
    // Place JSR instruction at 0xC800
    cpu.memory_bus().borrow_mut().write(0xC800, 0xBD); // JSR Extended opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0xC9); // High byte of subroutine address
    cpu.memory_bus().borrow_mut().write(0xC802, 0x00); // Low byte of subroutine address
    
    // Place RTS instruction at subroutine
    cpu.memory_bus().borrow_mut().write(subroutine_addr, 0x39); // RTS opcode
    
    let initial_s = cpu.registers().s;
    
    // Execute JSR
    let jsr_cycles = cpu.execute_instruction(false, false);
    
    // Verify JSR worked correctly
    assert_eq!(cpu.registers().pc, subroutine_addr, "JSR should jump to subroutine");
    assert_eq!(cpu.registers().s, initial_s.wrapping_sub(2), "JSR should push return address (2 bytes)");
    
    // Verify return address was pushed correctly
    let pushed_high = cpu.memory_bus().borrow().read(cpu.registers().s) as u16;
    let pushed_low = cpu.memory_bus().borrow().read(cpu.registers().s.wrapping_add(1)) as u16;
    let pushed_addr = (pushed_high << 8) | pushed_low;
    assert_eq!(pushed_addr, return_addr, "JSR should push correct return address");
    
    // Execute RTS
    let rts_cycles = cpu.execute_instruction(false, false);
    
    // Verify complete roundtrip
    assert_eq!(cpu.registers().pc, return_addr, "RTS should return to address after JSR");
    assert_eq!(cpu.registers().s, initial_s, "Stack pointer should be completely restored");
    assert_eq!(rts_cycles, 5, "RTS should consume exactly 5 cycles");
    
    println!("JSR→RTS roundtrip successful: JSR used {} cycles, RTS used {} cycles", jsr_cycles, rts_cycles);
}

#[test]
fn test_rts_stack_boundary_conditions() {
    // Test RTS behavior at stack boundaries
    let mut cpu = create_test_cpu();
    
    // Test near stack boundary in RAM area
    cpu.registers_mut().s = 0xCFF0; // Near top of RAM stack area (but not at very edge)
    
    // Push return address 0xC900 (valid RAM address)
    cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
    cpu.memory_bus().borrow_mut().write(cpu.registers().s, 0x00); // Low byte 
    cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
    cpu.memory_bus().borrow_mut().write(cpu.registers().s, 0xC9); // High byte
    
    // Place RTS instruction in RAM area
    cpu.registers_mut().pc = 0xC810; // Set PC to RAM area (different from return address)
    cpu.memory_bus().borrow_mut().write(0xC810, 0x39); // RTS opcode
    
    let initial_s = cpu.registers().s;
    
    // Execute RTS
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify correct behavior even near boundary
    assert_eq!(cpu.registers().pc, 0xC900, "RTS should work correctly near stack boundary");
    assert_eq!(cpu.registers().s, initial_s.wrapping_add(2), "Stack pointer should wrap correctly");
    assert_eq!(cycles, 5, "RTS should always consume exactly 5 cycles");
}

#[test]
fn test_rts_vs_cpp_reference_compliance() {
    // Verify RTS matches C++ reference exactly: PC = Pop16(S)
    let mut cpu = create_test_cpu();
    
    // Test multiple return addresses to ensure consistent behavior
    let test_addresses = [0xC800, 0xC900, 0xCA00, 0xCB00]; // Use RAM addresses (0xC800-0xCFFF)
    
    for &addr in &test_addresses {
        // Reset state
        cpu.registers_mut().s = 0xCA00; // Use RAM area
        cpu.registers_mut().pc = 0xC800; // Use RAM area
        
        // Push test address to stack (C++ order: high byte first in memory, low address)
        cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
        cpu.memory_bus().borrow_mut().write(cpu.registers().s, (addr & 0xFF) as u8); // Low byte first
        cpu.registers_mut().s = cpu.registers().s.wrapping_sub(1);
        cpu.memory_bus().borrow_mut().write(cpu.registers().s, (addr >> 8) as u8); // High byte second
        
        // Place RTS instruction
        cpu.memory_bus().borrow_mut().write(0xC800, 0x39);
        
        let stack_before = cpu.registers().s;
        
        // Execute RTS
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify C++ compliance: PC = Pop16(S)
        assert_eq!(cpu.registers().pc, addr, 
                   "RTS with address 0x{:04X} should match C++ Pop16(S) behavior", addr);
        assert_eq!(cpu.registers().s, stack_before.wrapping_add(2),
                   "Stack pointer should be incremented by 2 after Pop16(S)");
        assert_eq!(cycles, 5,
                   "RTS should always consume exactly 5 cycles");
    }
}