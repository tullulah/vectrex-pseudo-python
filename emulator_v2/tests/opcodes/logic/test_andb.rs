// Test ANDB (AND B register) opcodes - Port 1:1 desde Vectrexy C++
// C++ Original: vectrexy_backup/libs/emulator/src/Cpu.cpp OpAND

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

// ANDB (AND B register with memory/immediate value)
// C++ Original: OpAND - reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
// Opcodes: 0xC4 (immediate), 0xD4 (direct), 0xE4 (indexed), 0xF4 (extended)

#[test]
fn test_andb_immediate_0xc4() {
    // C++ Original: case 0xC4: OpAND<0, 0xC4>(B);
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0xFF; // Initial B = 11111111
    cpu.registers_mut().cc.v = true; // Set V flag to verify it gets cleared
    
    // Setup: ANDB #$F0 instruction at 0xC800 (RAM area)
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xC4);     // ANDB immediate opcode
    cpu.write8(test_addr + 1, 0xF0); // Immediate value = 11110000
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ANDB takes 2 cycles (immediate)
    assert_eq!(cycles, 2);
    
    // C++ Original: reg = reg & value (FF & F0 = F0)
    assert_eq!(cpu.registers().b, 0xF0, "B should be FF & F0 = F0");
    
    // C++ Original: CC.Negative = CalcNegative(reg), CC.Zero = CalcZero(reg), CC.Overflow = 0
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (result not zero)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by AND)");
    
    // PC should advance by 2 (opcode + immediate value)
    assert_eq!(cpu.registers().pc, test_addr + 2);
}

#[test]
fn test_andb_immediate_zero_result() {
    // C++ Original: Test AND operation resulting in zero - should set Z flag
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x0F; // Initial B = 00001111
    cpu.registers_mut().cc.z = false; // Clear Z flag to verify it gets set
    cpu.registers_mut().cc.n = true;  // Set N flag to verify it gets cleared
    
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xC4);     // ANDB immediate opcode
    cpu.write8(test_addr + 1, 0xF0); // Immediate value = 11110000
    cpu.registers_mut().pc = test_addr;
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: reg = reg & value (0F & F0 = 00)
    assert_eq!(cpu.registers().b, 0x00, "B should be 0F & F0 = 00");
    
    // C++ Original: CC flags
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result zero)");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (result not negative)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_andb_direct_0xd4() {
    // C++ Original: case 0xD4: OpAND<0, 0xD4>(B); - direct addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0xAA; // Initial B = 10101010
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let test_addr = 0xC800;
    let direct_addr = 0x10;        // Direct page offset
    let full_addr = 0xC810;        // DP (0xC8) + offset (0x10)
    
    // Setup memory: store test value at direct address
    cpu.write8(full_addr, 0x55);   // Value to AND = 01010101
    
    // Setup: ANDB direct instruction
    cpu.write8(test_addr, 0xD4);     // ANDB direct opcode
    cpu.write8(test_addr + 1, direct_addr); // Direct page offset
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ANDB direct takes 4 cycles
    assert_eq!(cycles, 4);
    
    // C++ Original: reg = reg & value (AA & 55 = 00)
    assert_eq!(cpu.registers().b, 0x00, "B should be AA & 55 = 00");
    
    // C++ Original: CC flags for zero result
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_andb_extended_0xf4() {
    // C++ Original: case 0xF4: OpAND<0, 0xF4>(B); - extended addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0xCC; // Initial B = 11001100
    
    let test_addr = 0xC800;
    let extended_addr = 0xC820;    // Extended address in RAM
    
    // Setup memory: store test value at extended address
    cpu.write8(extended_addr, 0x3C); // Value to AND = 00111100
    
    // Setup: ANDB extended instruction
    cpu.write8(test_addr, 0xF4);     // ANDB extended opcode
    cpu.write8(test_addr + 1, 0xC8); // Extended address high byte
    cpu.write8(test_addr + 2, 0x20); // Extended address low byte
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ANDB extended takes 5 cycles
    assert_eq!(cycles, 5);
    
    // C++ Original: reg = reg & value (CC & 3C = 0C)
    assert_eq!(cpu.registers().b, 0x0C, "B should be CC & 3C = 0C");
    
    // C++ Original: CC flags (result is positive, non-zero)
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    
    // PC should advance by 3 (opcode + 2 address bytes)
    assert_eq!(cpu.registers().pc, test_addr + 3);
}