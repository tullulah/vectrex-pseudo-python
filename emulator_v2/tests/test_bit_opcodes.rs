// BIT Operation Tests - 1:1 compliance with Vectrexy C++ implementation
// C++ Original: template<int cpuOp, uint8_t opCode, typename RegOrAccType> void OpBIT(RegOrAccType& reg)
// C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0x8000;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_bita_immediate_0x85() {
    // C++ Original: OpBIT<0, 0x85>(A); - BITA immediate
    let mut cpu = create_test_cpu();
    
    // Test case 1: A=$FF, operand=$0F -> result=$0F (NZ=10)
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x85);     // BITA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x0F); // operand
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers().a, 0xFF); // Register A should not change
    assert!(!cpu.registers().cc.z); // $FF & $0F = $0F -> not zero
    assert!(!cpu.registers().cc.n); // $0F has bit 7 clear -> not negative
    assert!(!cpu.registers().cc.v); // BIT always clears overflow
    
    // Test case 2: A=$80, operand=$80 -> result=$80 (NZ=11)
    cpu.registers.a = 0x80;
    cpu.write8(0x8002, 0x85); // BITA immediate
    cpu.write8(0x8003, 0x80); // operand
    cpu.registers.pc = 0x8002;
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0x80); // Register A should not change
    assert!(!cpu.registers.cc.z); // $80 & $80 = $80 -> not zero
    assert!(cpu.registers.cc.n); // $80 has bit 7 set -> negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
    
    // Test case 3: A=$55, operand=$AA -> result=$00 (NZ=01)
    cpu.registers.a = 0x55;
    cpu.write8(0x8004, 0x85); // BITA immediate
    cpu.write8(0x8005, 0xAA); // operand
    cpu.registers.pc = 0x8004;
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0x55); // Register A should not change
    assert!(cpu.registers.cc.z); // $55 & $AA = $00 -> zero
    assert!(!cpu.registers.cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bita_direct_0x95() {
    // C++ Original: OpBIT<0, 0x95>(A); - BITA direct
    let mut cpu = create_test_cpu();
    
    // Test with A=$F0, memory[$10]=$0F -> result=$00
    cpu.registers.a = 0xF0;
    cpu.registers.dp = 0x80; // Set direct page to $80xx
    cpu.write8(0x8010, 0x0F); // Store operand in memory
    cpu.write8(0x8000, 0x95); // BITA direct
    cpu.write8(0x8001, 0x10); // direct address offset
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.a, 0xF0); // Register A should not change
    assert!(cpu.registers.cc.z); // $F0 & $0F = $00 -> zero
    assert!(!cpu.registers.cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bita_indexed_0xa5() {
    // C++ Original: OpBIT<0, 0xA5>(A); - BITA indexed
    let mut cpu = create_test_cpu();
    
    // Test with A=$33, X=$8010, memory[$8010]=$30 -> result=$30
    cpu.registers.a = 0x33;
    cpu.registers.x = 0x8010;
    cpu.write8(0x8010, 0x30); // Store operand in memory
    cpu.write8(0x8000, 0xA5); // BITA indexed
    cpu.write8(0x8001, 0x00); // indexed postbyte (no offset)
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.a, 0x33); // Register A should not change
    assert!(!cpu.registers.cc.z); // $33 & $30 = $30 -> not zero
    assert!(!cpu.registers.cc.n); // $30 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bita_extended_0xb5() {
    // C++ Original: OpBIT<0, 0xB5>(A); - BITA extended
    let mut cpu = create_test_cpu();
    
    // Test with A=$C0, memory[$8020]=$C0 -> result=$C0
    cpu.registers.a = 0xC0;
    cpu.write8(0x8020, 0xC0); // Store operand in memory
    cpu.write8(0x8000, 0xB5); // BITA extended
    cpu.write8(0x8001, 0x80); // extended address high byte
    cpu.write8(0x8002, 0x20); // extended address low byte
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.a, 0xC0); // Register A should not change
    assert!(!cpu.registers.cc.z); // $C0 & $C0 = $C0 -> not zero
    assert!(cpu.registers.cc.n); // $C0 has bit 7 set -> negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bitb_immediate_0xc5() {
    // C++ Original: OpBIT<0, 0xC5>(B); - BITB immediate
    let mut cpu = create_test_cpu();
    
    // Test case 1: B=$7F, operand=$7F -> result=$7F (NZ=10)
    cpu.registers.b = 0x7F;
    cpu.write8(0x8000, 0xC5); // BITB immediate
    cpu.write8(0x8001, 0x7F); // operand
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.b, 0x7F); // Register B should not change
    assert!(!cpu.registers.cc.z); // $7F & $7F = $7F -> not zero
    assert!(!cpu.registers.cc.n); // $7F has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
    
    // Test case 2: B=$01, operand=$FE -> result=$00 (NZ=01)
    cpu.registers.b = 0x01;
    cpu.write8(0x8002, 0xC5); // BITB immediate
    cpu.write8(0x8003, 0xFE); // operand
    cpu.registers.pc = 0x8002;
    
    cpu.step();
    
    assert_eq!(cpu.registers.b, 0x01); // Register B should not change
    assert!(cpu.registers.cc.z); // $01 & $FE = $00 -> zero
    assert!(!cpu.registers.cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bitb_direct_0xd5() {
    // C++ Original: OpBIT<0, 0xD5>(B); - BITB direct
    let mut cpu = create_test_cpu();
    
    // Test with B=$88, memory[$8015]=$08 -> result=$08
    cpu.registers.b = 0x88;
    cpu.registers.dp = 0x80; // Set direct page to $80xx
    cpu.write8(0x8015, 0x08); // Store operand in memory
    cpu.write8(0x8000, 0xD5); // BITB direct
    cpu.write8(0x8001, 0x15); // direct address offset
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.b, 0x88); // Register B should not change
    assert!(!cpu.registers.cc.z); // $88 & $08 = $08 -> not zero
    assert!(!cpu.registers.cc.n); // $08 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bitb_indexed_0xe5() {
    // C++ Original: OpBIT<0, 0xE5>(B); - BITB indexed
    let mut cpu = create_test_cpu();
    
    // Test with B=$AA, Y=$8025, memory[$8025]=$55 -> result=$00
    cpu.registers.b = 0xAA;
    cpu.registers.y = 0x8025;
    cpu.write8(0x8025, 0x55); // Store operand in memory
    cpu.write8(0x8000, 0xE5); // BITB indexed
    cpu.write8(0x8001, 0x20); // indexed postbyte (Y register, no offset)
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.b, 0xAA); // Register B should not change
    assert!(cpu.registers.cc.z); // $AA & $55 = $00 -> zero
    assert!(!cpu.registers.cc.n); // $00 has bit 7 clear -> not negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bitb_extended_0xf5() {
    // C++ Original: OpBIT<0, 0xF5>(B); - BITB extended
    let mut cpu = create_test_cpu();
    
    // Test with B=$FF, memory[$8030]=$80 -> result=$80
    cpu.registers.b = 0xFF;
    cpu.write8(0x8030, 0x80); // Store operand in memory
    cpu.write8(0x8000, 0xF5); // BITB extended
    cpu.write8(0x8001, 0x80); // extended address high byte
    cpu.write8(0x8002, 0x30); // extended address low byte
    cpu.registers.pc = 0x8000;
    
    cpu.step();
    
    // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    assert_eq!(cpu.registers.b, 0xFF); // Register B should not change
    assert!(!cpu.registers.cc.z); // $FF & $80 = $80 -> not zero
    assert!(cpu.registers.cc.n); // $80 has bit 7 set -> negative
    assert!(!cpu.registers.cc.v); // BIT always clears overflow
}

#[test]
fn test_bit_operations_comprehensive() {
    // Comprehensive test to verify BIT operations follow exact Vectrexy behavior
    let mut cpu = create_test_cpu();
    
    // Test all flag combinations for BIT operations
    let test_cases = [
        // (reg_val, operand, expected_z, expected_n, description)
        (0x00, 0x00, true, false, "zero & zero -> zero"),
        (0x00, 0xFF, true, false, "zero & anything -> zero"),
        (0xFF, 0x00, true, false, "anything & zero -> zero"),
        (0x7F, 0x7F, false, false, "positive & positive -> positive"),
        (0x80, 0x80, false, true, "negative & negative -> negative"),
        (0xFF, 0x7F, false, false, "mixed high bits -> positive"),
        (0xFF, 0x80, false, true, "mixed high bits -> negative"),
        (0x55, 0xAA, true, false, "alternating bits -> zero"),
        (0xAA, 0x55, true, false, "alternating bits -> zero"),
        (0x01, 0x01, false, false, "single bit -> positive"),
    ];
    
    for (i, (reg_val, operand, expected_z, expected_n, description)) in test_cases.iter().enumerate() {
        let base_addr = 0x8000 + (i as u16 * 4);
        
        // Test BITA immediate
        cpu.registers.a = *reg_val;
        cpu.write8(base_addr, 0x85); // BITA immediate
        cpu.write8(base_addr + 1, *operand);
        cpu.registers.pc = base_addr;
        
        cpu.step();
        
        assert_eq!(cpu.registers.cc.z, *expected_z, "BITA {}: Zero flag mismatch", description);
        assert_eq!(cpu.registers.cc.n, *expected_n, "BITA {}: Negative flag mismatch", description);
        assert!(!cpu.registers.cc.v, "BITA {}: Overflow should always be clear", description);
        assert_eq!(cpu.registers.a, *reg_val, "BITA {}: Register A should not change", description);
        
        // Test BITB immediate
        cpu.registers.b = *reg_val;
        cpu.write8(base_addr + 2, 0xC5); // BITB immediate
        cpu.write8(base_addr + 3, *operand);
        cpu.registers.pc = base_addr + 2;
        
        cpu.step();
        
        assert_eq!(cpu.registers.cc.z, *expected_z, "BITB {}: Zero flag mismatch", description);
        assert_eq!(cpu.registers.cc.n, *expected_n, "BITB {}: Negative flag mismatch", description);
        assert!(!cpu.registers.cc.v, "BITB {}: Overflow should always be clear", description);
        assert_eq!(cpu.registers.b, *reg_val, "BITB {}: Register B should not change", description);
    }
}