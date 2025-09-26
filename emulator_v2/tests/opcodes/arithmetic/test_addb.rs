// Test ADDB (ADD B register) opcodes - Port 1:1 desde Vectrexy C++
// C++ Original: vectrexy_backup/libs/emulator/src/Cpu.cpp OpADD

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

// ADDB (ADD to B register with memory/immediate value)
// C++ Original: OpADD - Uses AddImpl for flag calculation including carry, overflow, half-carry
// Opcodes: 0xCB (immediate), 0xDB (direct), 0xEB (indexed), 0xFB (extended)

#[test]
fn test_addb_immediate_0xcb() {
    // C++ Original: case 0xCB: OpADD<0, 0xCB>(B);
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x10; // Initial B = 16 decimal
    cpu.registers_mut().cc.c = false; // Clear carry flag
    
    // Setup: ADDB #$20 instruction at 0xC800 (RAM area)
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xCB);     // ADDB immediate opcode
    cpu.write8(test_addr + 1, 0x20); // Immediate value = 32 decimal
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ADDB takes 2 cycles (immediate)
    assert_eq!(cycles, 2);
    
    // C++ Original: AddImpl handles the addition (10 + 20 = 30)
    assert_eq!(cpu.registers().b, 0x30, "B should be 10 + 20 = 30");
    
    // C++ Original: AddImpl sets flags based on result
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (result positive)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (result not zero)");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (no overflow)");
    
    // PC should advance by 2 (opcode + immediate value)
    assert_eq!(cpu.registers().pc, test_addr + 2);
}

#[test]
fn test_addb_immediate_with_carry() {
    // C++ Original: Test ADD operation with carry generation
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0xFF; // Initial B = 255 decimal
    
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xCB);     // ADDB immediate opcode
    cpu.write8(test_addr + 1, 0x01); // Immediate value = 1 decimal
    cpu.registers_mut().pc = test_addr;
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: AddImpl handles overflow (FF + 01 = 00 with carry)
    assert_eq!(cpu.registers().b, 0x00, "B should be FF + 01 = 00 (with carry)");
    
    // C++ Original: AddImpl sets carry flag
    assert_eq!(cpu.registers().cc.c, true, "C flag should be set (carry occurred)");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result zero)");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (unsigned overflow, not signed)");
}

#[test]
fn test_addb_immediate_with_overflow() {
    // C++ Original: Test ADD operation with signed overflow
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x7F; // Initial B = 127 decimal (max positive signed 8-bit)
    
    let test_addr = 0xC800;
    cpu.write8(test_addr, 0xCB);     // ADDB immediate opcode
    cpu.write8(test_addr + 1, 0x01); // Immediate value = 1 decimal
    cpu.registers_mut().pc = test_addr;
    
    cpu.execute_instruction(false, false);
    
    // C++ Original: AddImpl detects signed overflow (7F + 01 = 80)
    assert_eq!(cpu.registers().b, 0x80, "B should be 7F + 01 = 80");
    
    // C++ Original: AddImpl sets overflow flag for signed overflow
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set (signed overflow)");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no unsigned overflow)");
}

#[test]
fn test_addb_direct_0xdb() {
    // C++ Original: case 0xDB: OpADD<0, 0xDB>(B); - direct addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x25; // Initial B = 37 decimal
    cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8 (RAM area)
    
    let test_addr = 0xC800;
    let direct_addr = 0x10;        // Direct page offset
    let full_addr = 0xC810;        // DP (0xC8) + offset (0x10)
    
    // Setup memory: store test value at direct address
    cpu.write8(full_addr, 0x15);   // Value to add = 21 decimal
    
    // Setup: ADDB direct instruction
    cpu.write8(test_addr, 0xDB);     // ADDB direct opcode
    cpu.write8(test_addr + 1, direct_addr); // Direct page offset
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ADDB direct takes 4 cycles
    assert_eq!(cycles, 4);
    
    // C++ Original: AddImpl handles addition (25 + 15 = 3A)
    assert_eq!(cpu.registers().b, 0x3A, "B should be 25 + 15 = 3A");
    
    // C++ Original: AddImpl sets flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_addb_extended_0xfb() {
    // C++ Original: case 0xFB: OpADD<0, 0xFB>(B); - extended addressing mode
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().b = 0x40; // Initial B = 64 decimal
    
    let test_addr = 0xC800;
    let extended_addr = 0xC820;    // Extended address in RAM
    
    // Setup memory: store test value at extended address
    cpu.write8(extended_addr, 0x30); // Value to add = 48 decimal
    
    // Setup: ADDB extended instruction
    cpu.write8(test_addr, 0xFB);     // ADDB extended opcode
    cpu.write8(test_addr + 1, 0xC8); // Extended address high byte
    cpu.write8(test_addr + 2, 0x20); // Extended address low byte
    cpu.registers_mut().pc = test_addr;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // C++ Original: ADDB extended takes 5 cycles
    assert_eq!(cycles, 5);
    
    // C++ Original: AddImpl handles addition (40 + 30 = 70)
    assert_eq!(cpu.registers().b, 0x70, "B should be 40 + 30 = 70");
    
    // C++ Original: AddImpl sets flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
    
    // PC should advance by 3 (opcode + 2 address bytes)
    assert_eq!(cpu.registers().pc, test_addr + 3);
}