// SUBB Operation Tests - 1:1 compliance with Vectrexy C++ implementation  
// C++ Original: template<int page, uint8_t opCode> void OpSUB(uint8_t& reg)
// C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>(), 0, CC);

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    Cpu6809::new(memory_bus)
}

#[test]
fn test_subb_direct_0xd0() {
    // C++ Original: OpSUB<0, 0xD0>(B); - SUBB direct
    let mut cpu = create_test_cpu();
    
    // Setup: B=0x50, memory[$10]=0x20 -> B=0x30, carry=0
    cpu.registers_mut().b = 0x50;
    cpu.registers_mut().dp = 0xC8; // Set direct page to $C8xx
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START + 0x10, 0x20); // Store operand in memory
    memory_bus.borrow_mut().write(RAM_START, 0xD0);        // SUBB direct
    memory_bus.borrow_mut().write(RAM_START + 1, 0x10);    // direct address offset
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC);
    assert_eq!(cpu.registers().b, 0x30);
    assert!(!cpu.registers().cc.c); // $50 - $20 = $30 -> no carry
    assert!(!cpu.registers().cc.z); // Result not zero
    assert!(!cpu.registers().cc.n); // $30 has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // No overflow
}

#[test]
fn test_subb_direct_underflow() {
    let mut cpu = create_test_cpu();
    
    // Setup: B=0x10, memory[$15]=0x20 -> B=0xF0, carry=1 (underflow)
    cpu.registers_mut().b = 0x10;
    cpu.registers_mut().dp = 0xC8;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START + 0x15, 0x20); // Store operand in memory
    memory_bus.borrow_mut().write(RAM_START, 0xD0);        // SUBB direct
    memory_bus.borrow_mut().write(RAM_START + 1, 0x15);    // direct address offset
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0xF0);
    assert!(cpu.registers().cc.c); // $10 - $20 causes underflow -> carry set
    assert!(!cpu.registers().cc.z); // Result not zero
    assert!(cpu.registers().cc.n); // $F0 has bit 7 set -> negative
}

#[test]
fn test_subb_indexed_0xe0() {
    // C++ Original: OpSUB<0, 0xE0>(B); - SUBB indexed
    let mut cpu = create_test_cpu();
    
    // Setup: B=0x80, X=$C820, memory[$C820]=0x30 -> B=0x50
    cpu.registers_mut().b = 0x80;
    cpu.registers_mut().x = RAM_START + 0x20;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START + 0x20, 0x30); // Store operand in memory
    memory_bus.borrow_mut().write(RAM_START, 0xE0);        // SUBB indexed
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00);    // indexed postbyte (X register, no offset)
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x50);
    assert!(!cpu.registers().cc.c); // $80 - $30 = $50 -> no carry
    assert!(!cpu.registers().cc.z); // Result not zero
    assert!(!cpu.registers().cc.n); // $50 has bit 7 clear -> not negative
    assert!(cpu.registers().cc.v); // Overflow: -128 - 48 = -176 (out of signed range) -> overflow
}

#[test]
fn test_subb_extended_0xf0() {
    // C++ Original: OpSUB<0, 0xF0>(B); - SUBB extended
    let mut cpu = create_test_cpu();
    
    // Setup: B=0x7F, memory[$C900]=0x7F -> B=0x00, zero flag set
    cpu.registers_mut().b = 0x7F;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x7F);          // Store operand in memory
    memory_bus.borrow_mut().write(RAM_START, 0xF0);       // SUBB extended
    memory_bus.borrow_mut().write(RAM_START + 1, 0xC9);   // extended address high byte
    memory_bus.borrow_mut().write(RAM_START + 2, 0x00);   // extended address low byte
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x00);
    assert!(!cpu.registers().cc.c); // $7F - $7F = $00 -> no carry
    assert!(cpu.registers().cc.z); // Result is zero -> zero flag set
    assert!(!cpu.registers().cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // No overflow
}

#[test]
fn test_subb_comprehensive_flags() {
    // Test comprehensive flag behavior for SUBB operations following Vectrexy pattern
    let mut cpu = create_test_cpu();
    
    let test_cases = [
        // (B_initial, operand, expected_result, expected_c, expected_z, expected_n, expected_v, description)
        (0x80, 0x01, 0x7F, false, false, false, true, "positive overflow"),
        (0x01, 0x80, 0x81, true, false, true, true, "negative underflow - fixed overflow flag"),
        (0x00, 0x01, 0xFF, true, false, true, false, "zero minus one"),
        (0xFF, 0xFF, 0x00, false, true, false, false, "same values zero result"),
        (0x50, 0x30, 0x20, false, false, false, false, "normal subtraction"),
    ];
    
    for (i, (b_initial, operand, expected_result, expected_c, expected_z, expected_n, expected_v, description)) in test_cases.iter().enumerate() {
        let test_addr = RAM_START + (i as u16 * 10);
        
        // Test SUBB direct for each case
        cpu.registers_mut().b = *b_initial;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = test_addr;
        
        let direct_offset = 0x50 + (i as u8); // Use different offsets to avoid collision
        let memory_bus = cpu.memory_bus().clone();
        
        // Write operand at DP:direct_offset location
        let operand_address = (0xC8 << 8) | (direct_offset as u16);
        memory_bus.borrow_mut().write(operand_address, *operand); // Store operand in memory
        memory_bus.borrow_mut().write(test_addr, 0xD0);            // SUBB direct
        memory_bus.borrow_mut().write(test_addr + 1, direct_offset);  // direct address offset
        
        cpu.execute_instruction(false, false);
        
        assert_eq!(cpu.registers().b, *expected_result, "SUBB {}: Result mismatch", description);
        assert_eq!(cpu.registers().cc.c, *expected_c, "SUBB {}: Carry flag mismatch", description);
        assert_eq!(cpu.registers().cc.z, *expected_z, "SUBB {}: Zero flag mismatch", description);
        assert_eq!(cpu.registers().cc.n, *expected_n, "SUBB {}: Negative flag mismatch", description);
        assert_eq!(cpu.registers().cc.v, *expected_v, "SUBB {}: Overflow flag mismatch", description);
    }
}