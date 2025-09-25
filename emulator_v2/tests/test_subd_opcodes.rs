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

/// C++ Original: OpSUB<0, 0x83>(D); - SUBD immediate
/// Reference: { 0x83, "SUBD", AddressingMode::Immediate, 4, 3, "Subtract Double acc." }
/// Test SUBD immediate - Subtract 16-bit immediate from D register
#[test]
fn test_subd_immediate_0x83() {
    let mut cpu = create_test_cpu();
    
    // Set initial D register value (A=0x12, B=0x34 -> D=0x1234)
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    
    // SUBD #$0100 - subtract 0x0100 from 0x1234 = 0x1134
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x83); // SUBD immediate
    memory_bus.borrow_mut().write(0xC801, 0x01); // high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // low byte
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x1234 - 0x0100 = 0x1134
    assert_eq!(cpu.registers().d(), 0x1134);
    assert_eq!(cycles, 4); // 4 cycles for immediate mode
    assert_eq!(cpu.registers().pc, 0xC803); // PC advanced by 3 bytes
    
    // Verify flags
    assert!(!cpu.registers().cc.n); // Result is positive
    assert!(!cpu.registers().cc.z); // Result is non-zero
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
}

/// C++ Original: OpSUB<0, 0x93>(D); - SUBD direct
/// Reference: { 0x93, "SUBD", AddressingMode::Direct, 6, 2, "Subtract Double acc." }
/// Test SUBD direct - Subtract 16-bit value from direct page from D register
#[test]
fn test_subd_direct_0x93() {
    let mut cpu = create_test_cpu();
    
    // Set initial D register value
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().b = 0x00;
    
    // Set direct page register
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM area
    
    // Store value at direct page address 0xC880
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC880, 0x05); // high byte
    memory_bus.borrow_mut().write(0xC881, 0x00); // low byte
    
    // SUBD $80 - subtract value at DP:$80 from D
    memory_bus.borrow_mut().write(0xC800, 0x93); // SUBD direct
    memory_bus.borrow_mut().write(0xC801, 0x80); // direct page offset
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x2000 - 0x0500 = 0x1B00
    assert_eq!(cpu.registers().d(), 0x1B00);
    assert_eq!(cycles, 6); // 6 cycles for direct mode
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced by 2 bytes
    
    // Verify flags
    assert!(!cpu.registers().cc.n); // Result is positive
    assert!(!cpu.registers().cc.z); // Result is non-zero
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
}

/// C++ Original: OpSUB<0, 0xA3>(D); - SUBD indexed
/// Reference: { 0xA3, "SUBD", AddressingMode::Indexed, 6, 2, "Subtract Double acc." }
/// Test SUBD indexed - Subtract 16-bit value using indexed addressing from D register
#[test]
fn test_subd_indexed_0xa3() {
    let mut cpu = create_test_cpu();
    
    // Set initial D register value
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    
    // Set X register for indexed addressing to RAM area
    cpu.registers_mut().x = 0xC850;
    
    // Store value at indexed address 0xC850 (X register points directly to data)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC850, 0x01); // high byte
    memory_bus.borrow_mut().write(0xC851, 0x00); // low byte
    
    // SUBD ,X - subtract value at X from D (no offset)
    memory_bus.borrow_mut().write(0xC800, 0xA3); // SUBD indexed
    memory_bus.borrow_mut().write(0xC801, 0x84); // ,X addressing mode (no offset)
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0x1234 - 0x0100 = 0x1134
    assert_eq!(cpu.registers().d(), 0x1134);
    assert_eq!(cycles, 6); // 6 cycles for indexed mode
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced by 2 bytes
    
    // Verify flags
    assert!(!cpu.registers().cc.n); // Result is positive
    assert!(!cpu.registers().cc.z); // Result is non-zero
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
}

/// C++ Original: OpSUB<0, 0xB3>(D); - SUBD extended
/// Reference: { 0xB3, "SUBD", AddressingMode::Extended, 7, 3, "Subtract Double acc." }
/// Test SUBD extended - Subtract 16-bit value from extended address from D register
#[test]
fn test_subd_extended_0xb3() {
    let mut cpu = create_test_cpu();
    
    // Set initial D register value
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().b = 0xFF;
    
    // Store value at extended address in RAM area
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x00); // high byte
    memory_bus.borrow_mut().write(0xC901, 0x01); // low byte
    
    // SUBD $C900 - subtract value at extended address from D
    memory_bus.borrow_mut().write(0xC800, 0xB3); // SUBD extended
    memory_bus.borrow_mut().write(0xC801, 0xC9); // high byte of address
    memory_bus.borrow_mut().write(0xC802, 0x00); // low byte of address
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify result: 0xFFFF - 0x0001 = 0xFFFE
    assert_eq!(cpu.registers().d(), 0xFFFE);
    assert_eq!(cycles, 7); // 7 cycles for extended mode
    assert_eq!(cpu.registers().pc, 0xC803); // PC advanced by 3 bytes
    
    // Verify flags
    assert!(cpu.registers().cc.n); // Result is negative (MSB set)
    assert!(!cpu.registers().cc.z); // Result is non-zero
    assert!(!cpu.registers().cc.c); // No borrow
    assert!(!cpu.registers().cc.v); // No overflow
}

/// Test SUBD with zero result to verify Z flag
#[test]
fn test_subd_zero_result() {
    let mut cpu = create_test_cpu();
    
    // Set D register to 0x1234
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    
    // SUBD #$1234 - subtract same value to get zero
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x83); // SUBD immediate
    memory_bus.borrow_mut().write(0xC801, 0x12); // high byte
    memory_bus.borrow_mut().write(0xC802, 0x34); // low byte
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    // Verify zero result and Z flag
    assert_eq!(cpu.registers().d(), 0x0000);
    assert!(cpu.registers().cc.z); // Zero flag should be set
    assert!(!cpu.registers().cc.n); // Negative flag should be clear
}

/// Test SUBD with carry/borrow condition
#[test]
fn test_subd_with_borrow() {
    let mut cpu = create_test_cpu();
    
    // Set D register to smaller value
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().b = 0x00;
    
    // SUBD #$2000 - subtract larger value to create borrow
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x83); // SUBD immediate
    memory_bus.borrow_mut().write(0xC801, 0x20); // high byte
    memory_bus.borrow_mut().write(0xC802, 0x00); // low byte
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    // Verify result: 0x1000 - 0x2000 = 0xF000 (with borrow)
    assert_eq!(cpu.registers().d(), 0xF000);
    assert!(cpu.registers().cc.c); // Carry flag should be set (indicates borrow)
    assert!(cpu.registers().cc.n); // Negative flag should be set
    assert!(!cpu.registers.cc.z); // Zero flag should be clear
}