// Test de shift/rotate operations - Port 1:1 desde Vectrexy C++
// C++ Original reference: vectrexy_backup/libs/emulator/src/Cpu6809.cpp OpLSR/OpASR/OpROR/OpROL/OpASL

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

// Test LSR (Logical Shift Right) - 0x04 Direct, 0x44 Register A, 0x54 Register B, 0x64 Indexed, 0x74 Extended
#[test]
fn test_lsr_register_a_0x44() {
    let mut cpu = create_test_cpu();
    cpu.registers_mut().a = 0b1100_1010; // C=1, A=CA
    cpu.registers_mut().cc.c = false;

    // Write opcode LSR A (0x44) at RAM location
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0x44);
    cpu.registers_mut().pc = test_addr;
    
    cpu.execute_instruction(false, false); // Execute LSR A
    
    // C++ Original: OpLSR - Set C to bit 0, Z to result, N to 0
    assert_eq!(cpu.registers().a, 0b0110_0101);  // Shifted right
    assert_eq!(cpu.registers().cc.c, false);     // Original bit 0 was 0
    assert_eq!(cpu.registers().cc.z, false);     // Result not zero
    assert_eq!(cpu.registers().cc.n, false);     // N always 0 for LSR
}

// Comprehensive test for basic shift/rotate register operations
#[test]
fn test_shift_rotate_comprehensive() {
    let mut cpu = create_test_cpu();
    let test_addr = 0xC800;
    
    // Test LSR A (0x44) - Original: 1100_1010 (0xCA), expect: 0110_0101 (0x65), C=0
    cpu.registers_mut().a = 0b1100_1010;
    cpu.registers_mut().cc.c = false;
    cpu.write8(test_addr, 0x44);
    cpu.registers_mut().pc = test_addr;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0b0110_0101);
    assert_eq!(cpu.registers().cc.c, false); // Original bit 0 was 0
    assert_eq!(cpu.registers().cc.n, false); // LSR always clears N
    assert_eq!(cpu.registers().cc.z, false);
    
    // Test LSR B (0x54) - Original: 1010_0001 (0xA1), expect: 0101_0000 (0x50), C=1
    cpu.registers_mut().b = 0b1010_0001;
    cpu.write8(test_addr + 1, 0x54);
    cpu.registers_mut().pc = test_addr + 1;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().b, 0b0101_0000);
    assert_eq!(cpu.registers().cc.c, true); // Original bit 0 was 1
    assert_eq!(cpu.registers().cc.n, false);
    
    // Test ASR A (0x47) - Original: 1100_1011 (0xCB), expect: 1110_0101 (0xE5), C=1, N=1
    cpu.registers_mut().a = 0b1100_1011; // Negative number
    cpu.write8(test_addr + 2, 0x47);
    cpu.registers_mut().pc = test_addr + 2;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0b1110_0101); // Sign bit preserved
    assert_eq!(cpu.registers().cc.c, true); // Original bit 0 was 1
    assert_eq!(cpu.registers().cc.n, true); // Result negative
    
    // Test ROR A (0x46) - Test with carry set
    cpu.registers_mut().a = 0b1010_1100; // 0xAC
    cpu.registers_mut().cc.c = true; // Set carry
    cpu.write8(test_addr + 3, 0x46);
    cpu.registers_mut().pc = test_addr + 3;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0b1101_0110); // Carry into bit 7, bit 0 out
    assert_eq!(cpu.registers().cc.c, false); // Original bit 0 was 0
    assert_eq!(cpu.registers().cc.n, true); // Result negative (bit 7 = 1)
    
    // Test ROL A (0x49) - Test overflow detection
    cpu.registers_mut().a = 0b0101_1010; // 0x5A
    cpu.registers_mut().cc.c = true; // Set carry
    cpu.write8(test_addr + 4, 0x49);
    cpu.registers_mut().pc = test_addr + 4;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0b1011_0101); // Shift left, carry in
    assert_eq!(cpu.registers().cc.c, false); // Original bit 7 was 0
    assert_eq!(cpu.registers().cc.v, true); // Overflow: sign changed 0->1
    assert_eq!(cpu.registers().cc.n, true); // Result negative
    
    // Test ASL A (0x48) - Uses add_impl_u8
    cpu.registers_mut().a = 0x50; // 80 decimal
    cpu.write8(test_addr + 5, 0x48);
    cpu.registers_mut().pc = test_addr + 5;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0xA0); // 160 decimal (80 * 2)
    assert_eq!(cpu.registers().cc.n, true); // Result negative
    assert_eq!(cpu.registers().cc.v, true); // Overflow (sign changed)
    
    // Test zero result sets Z flag - LSR of 0x01
    cpu.registers_mut().a = 0x01;
    cpu.write8(test_addr + 6, 0x44); // LSR A
    cpu.registers_mut().pc = test_addr + 6;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.c, true); // Bit 0 was 1
}