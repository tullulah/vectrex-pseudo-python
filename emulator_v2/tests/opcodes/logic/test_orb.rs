// Test ORB (OR B register) opcodes - Port 1:1 desde Vectrexy C++
// C++ Original: vectrexy_backup/libs/emulator/src/Cpu.cpp OpOR

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

// ORB (OR B register with memory/immediate value)
// C++ Original: OpOR - reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
// Opcodes: 0xCA (immediate), 0xDA (direct), 0xEA (indexed), 0xFA (extended)

#[test]
fn test_orb_immediate_0xca() {
    // C++ Original: case 0xCA: OpOR<0, 0xCA>(B);
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0xF0; // Initial B = 11110000
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    
    // Setup: ORB #$0F instruction at 0xC800 (RAM area)
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xCA);     // ORB immediate opcode
    cpu.write8(test_addr + 1, 0x0F); // Immediate value = 00001111
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ORB takes 2 cycles (immediate)
    assert_eq!(cycles, 2);
    
    // C++ Original: reg = reg | value (F0 | 0F = FF)
    assert_eq!(cpu.registers().b, 0xFF, "B should be F0 | 0F = FF");
    
    // C++ Original: CC.Negative = CalcNegative(reg), CC.Zero = CalcZero(reg), CC.Overflow = 0
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (result not zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by OR)");
    
    // PC should advance by 2 (opcode + immediate value)
    assert_eq!(cpu.registers().pc, test_addr + 2);
}

#[test]
fn test_orb_immediate_zero_result() {
    // C++ Original: Test OR operation resulting in zero - should set Z flag
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x00; // Initial B = 00000000
    cpu.registers_mut().cc.z = false; // Clear Z flag to verify it gets set
    cpu.registers_mut().cc.n = true;  // Set N flag to verify it gets cleared
    
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xCA);     // ORB immediate opcode
    cpu.write8(test_addr + 1, 0x00); // Immediate value = 00000000
    cpu.registers_mut().pc = test_addr;
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: reg = reg | value (00 | 00 = 00)
    assert_eq!(cpu.registers().b, 0x00, "B should remain 0x00");
    
    // C++ Original: CC flags
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result zero)");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (result not negative)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_orb_direct_0xda() {
    // C++ Original: case 0xDA: OpOR<0, 0xDA>(B); - direct addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x55; // Initial B = 01010101
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let test_addr = 0xC800;
    let direct_addr = 0x10;        // Direct page offset
    let full_addr = 0xC810;        // DP (0xC8) + offset (0x10)
    
    // Setup memory: store test value at direct address
    cpu.write8(full_addr, 0xAA);   // Value to OR = 10101010
    
    // Setup: ORB direct instruction
    cpu.write8(test_addr, 0xDA);     // ORB direct opcode
    cpu.write8(test_addr + 1, direct_addr); // Direct page offset
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ORB direct takes 4 cycles
    assert_eq!(cycles, 4);
    
    // C++ Original: reg = reg | value (55 | AA = FF)
    assert_eq!(cpu.registers().b, 0xFF, "B should be 55 | AA = FF");
    
    // C++ Original: CC flags for negative result
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_orb_extended_0xfa() {
    // C++ Original: case 0xFA: OpOR<0, 0xFA>(B); - extended addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x33; // Initial B = 00110011
    
    let test_addr = 0xC800;
    let extended_addr = 0xC820;    // Extended address in RAM
    
    // Setup memory: store test value at extended address
    cpu.write8(extended_addr, 0xCC); // Value to OR = 11001100
    
    // Setup: ORB extended instruction
    cpu.write8(test_addr, 0xFA);     // ORB extended opcode
    cpu.write8(test_addr + 1, 0xC8); // Extended address high byte
    cpu.write8(test_addr + 2, 0x20); // Extended address low byte
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ORB extended takes 5 cycles
    assert_eq!(cycles, 5);
    
    // C++ Original: reg = reg | value (33 | CC = FF)
    assert_eq!(cpu.registers().b, 0xFF, "B should be 33 | CC = FF");
    
    // C++ Original: CC flags
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    
    // PC should advance by 3 (opcode + 2 address bytes)
    assert_eq!(cpu.registers().pc, test_addr + 3);
}