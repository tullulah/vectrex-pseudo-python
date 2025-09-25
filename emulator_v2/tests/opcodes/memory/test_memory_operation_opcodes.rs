// Test suite for newly implemented memory operation opcodes
// Tests NEG, COM, INC, DEC, TST, CLR, DAA opcodes across all addressing modes

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
fn test_neg_direct_0x00() {
    let mut cpu = create_test_cpu();
    
    // Setup: NEG $12 - Negate value at direct page address $12
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x00); // NEG direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x12); // Direct page address ($12)
    memory_bus.borrow_mut().write(0xC812, 0x05); // Value to negate: 5 (use RAM area)
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: Negating is 0 - value, uses subtract_impl
    assert_eq!(memory_bus.borrow().read(0xC812), 0xFB); // -5 in two's complement
    assert_eq!(cpu.registers().cc.n, true); // Negative result
    assert_eq!(cpu.registers().cc.z, false); // Not zero
    assert_eq!(cpu.registers().cc.c, true); // Carry set (borrow)
    assert_eq!(cpu.registers().pc, 0xC802);
}

#[test]
fn test_daa_bcd_adjustment_0x19() {
    let mut cpu = create_test_cpu();
    
    // Setup: DAA - Decimal Adjust Accumulator A
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x19); // DAA opcode
    
    // Test BCD adjustment after addition: 09 + 01 = 0A (needs +6 correction)
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().a = 0x0A; // Result needing BCD adjustment
    cpu.registers_mut().cc.h = false; // No half-carry
    cpu.registers_mut().cc.c = false; // No carry
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: LSN > 9, so add 6 to LSN. MSN <= 9, so no MSN correction
    assert_eq!(cpu.registers().a, 0x10); // 0x0A + 0x06 = 0x10 (BCD corrected)
    assert_eq!(cpu.registers().pc, 0xC801);
}

// Test COM Extended (0x73)
#[test]
fn test_com_extended_0x73() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store value 0x55 in memory location within RAM
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x55); // Use RAM area
    memory_bus.borrow_mut().write(0xC800, 0x73);  // COM extended
    memory_bus.borrow_mut().write(0xC801, 0xC9);  // high byte (RAM area)
    memory_bus.borrow_mut().write(0xC802, 0x00);  // low byte
    
    cpu.registers_mut().pc = 0xC800;
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify memory was complemented: ~0x55 = 0xAA
    assert_eq!(memory_bus.borrow().read(0xC900), 0xAA);
    // Verify condition codes: N=1 (MSB set), Z=0, V=0, C=1 (always set)
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, true);
    assert_eq!(cpu.registers().pc, 0xC803);
}

// Test INC Indexed (0x6C)
#[test]
fn test_inc_indexed_0x6c() {
    let mut cpu = create_test_cpu();
    
    // Setup test: X register pointing to memory location in RAM
    let memory_bus = cpu.memory_bus().clone();
    cpu.registers_mut().x = 0xC900; // Point to RAM area
    memory_bus.borrow_mut().write(0xC900, 0xFF); // Will overflow to 0x00
    memory_bus.borrow_mut().write(0xC800, 0x6C);  // INC indexed
    memory_bus.borrow_mut().write(0xC801, 0x84);  // ,X postbyte
    
    cpu.registers_mut().pc = 0xC800;
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify memory was incremented: 0xFF + 1 = 0x00 (overflow)
    assert_eq!(memory_bus.borrow().read(0xC900), 0x00);
    // Verify condition codes: N=0, Z=1, V=0 (overflow but not 2's complement overflow)
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().pc, 0xC802);  
}

// Test DEC Direct (0x0A)
#[test]
fn test_dec_direct_0x0a() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store value 0x01 in memory location within direct page (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC880, 0x01); // Use RAM area
    memory_bus.borrow_mut().write(0xC800, 0x0A);  // DEC direct
    memory_bus.borrow_mut().write(0xC801, 0x80);  // address
    
    cpu.registers_mut().pc = 0xC800;
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify memory was decremented: 0x01 - 1 = 0x00
    assert_eq!(memory_bus.borrow().read(0xC880), 0x00);
    // Verify condition codes: N=0, Z=1, V=0
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().pc, 0xC802);
}

// Test TST Extended (0x7D)
#[test]
fn test_tst_extended_0x7d() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store negative value in memory location (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC950, 0x80); // MSB set (negative), use RAM area
    memory_bus.borrow_mut().write(0xC800, 0x7D);  // TST extended
    memory_bus.borrow_mut().write(0xC801, 0xC9);  // high byte (RAM area)
    memory_bus.borrow_mut().write(0xC802, 0x50);  // low byte
    
    cpu.registers_mut().pc = 0xC800;
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify memory unchanged
    assert_eq!(memory_bus.borrow().read(0xC950), 0x80);
    // Verify condition codes: N=1 (MSB set), Z=0, V=0, C=0 (always cleared)
    assert_eq!(cpu.registers().cc.n, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().pc, 0xC803);
}

// Test CLR Extended (0x7F)
#[test]
fn test_clr_extended_0x7f() {
    let mut cpu = create_test_cpu();
    
    // Setup test: store non-zero value in memory location (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCA00, 0x55); // Use RAM area
    memory_bus.borrow_mut().write(0xC800, 0x7F);  // CLR extended
    memory_bus.borrow_mut().write(0xC801, 0xCA);  // high byte (RAM area)
    memory_bus.borrow_mut().write(0xC802, 0x00);  // low byte
    
    cpu.registers_mut().pc = 0xC800;
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify memory was cleared
    assert_eq!(memory_bus.borrow().read(0xCA00), 0x00);
    // Verify condition codes: N=0, Z=1, V=0, C=0
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.v, false);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().pc, 0xC803);
}

// Stack order compliance tests (as requested)
#[test]
fn test_memory_operations_stack_independence() {
    let mut cpu = create_test_cpu();
    
    // Memory operations should not affect stack
    let initial_sp = 0xCFFF;
    cpu.registers_mut().s = initial_sp;
    
    // Test multiple memory operations
    let operations = [
        (0x00, 0x12), // NEG direct
        (0x03, 0x12), // COM direct
        (0x0C, 0x12), // INC direct
        (0x0A, 0x12), // DEC direct
        (0x0D, 0x12), // TST direct
        (0x0F, 0x12), // CLR direct
    ];
    
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup direct page to point to RAM area and initialize test memory
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    memory_bus.borrow_mut().write(0xC812, 0x42); // Initialize test location
    
    let mut pc = 0xC800;
    for (opcode, operand) in operations {
        memory_bus.borrow_mut().write(pc, opcode);
        memory_bus.borrow_mut().write(pc + 1, operand);
        cpu.registers_mut().pc = pc;
        cpu.execute_instruction(false, false);
        
        // Verify stack pointer unchanged
        assert_eq!(cpu.registers().s, initial_sp, 
            "Stack pointer changed during opcode 0x{:02X}", opcode);
        
        pc += 2;
    }
}