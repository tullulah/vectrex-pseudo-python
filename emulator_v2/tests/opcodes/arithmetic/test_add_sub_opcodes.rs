// Tests para opcodes ADD y SUB - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x80, 0x8B, 0xC0, 0xCB

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
fn test_adda_immediate_basic() {
    // Test ADDA #$0F - Add immediate to A
    // C++ Original: OpADD<0, 0x8B>(A); reg = AddImpl(reg, b, 0, CC);
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x10; // Initial A = 16
    cpu.registers_mut().cc.c = true; // Set carry to verify it's updated correctly
    cpu.registers_mut().cc.v = true; // Set overflow to verify it's updated correctly
    
    // Setup: ADDA #$0F instruction at 0xC800 (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8B); // ADDA immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x0F); // Add 15
    cpu.registers_mut().pc = 0xC800;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x1F, "A should be 0x10 + 0x0F = 0x1F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (no overflow)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "ADDA immediate should take 2 cycles");
}

#[test]
fn test_adda_immediate_carry() {
    // Test ADDA with carry generation
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0xFF; // Initial A = 255
    
    // Setup: ADDA #$01 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x01); // Add 1
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0xFF + 0x01 = 0x00 (wrapped)");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be set (carry occurred)");
}

#[test]
fn test_adda_immediate_overflow() {
    // Test ADDA with overflow: positive + positive = negative
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x7F; // 127 (max positive signed 8-bit)
    
    // Setup: ADDA #$01 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x01); // Add 1
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x80, "A should be 0x7F + 0x01 = 0x80");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result is negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry in unsigned)");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set (overflow occurred)");
}

#[test]
fn test_addb_immediate_basic() {
    // Test ADDB #$22 - Add immediate to B
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0x33; // Initial B = 51
    
    // Setup: ADDB #$22 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xCB); // ADDB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x22); // Add 34
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x55, "B should be 0x33 + 0x22 = 0x55");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_suba_immediate_basic() {
    // Test SUBA #$0F - Subtract immediate from A
    // C++ Original: OpSUB<0, 0x80>(A); reg = SubtractImpl(reg, ReadOperandValue8<addrMode>(), 0, CC);
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x20; // Initial A = 32
    
    // Setup: SUBA #$0F instruction at 0xC800 (RAM area)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x80); // SUBA immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x0F); // Subtract 15
    cpu.registers_mut().pc = 0xC800;
    
    // Execute instruction
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify results
    assert_eq!(cpu.registers().a, 0x11, "A should be 0x20 - 0x0F = 0x11");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (no overflow)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "SUBA immediate should take 2 cycles");
}

#[test]
fn test_suba_immediate_borrow() {
    // Test SUBA with borrow (underflow)
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x05; // Initial A = 5
    
    // Setup: SUBA #$10 instruction (subtract 16 from 5)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x80); // SUBA immediate
    memory_bus.borrow_mut().write(0xC801, 0x10); // Subtract 16
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xF5, "A should be 0x05 - 0x10 = 0xF5 (wrapped)");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set (result is negative)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, true, "C flag inverted by subtract implementation");
}

#[test]
fn test_suba_immediate_zero_result() {
    // Test SUBA resulting in zero
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().a = 0x42; // Initial A = 66
    
    // Setup: SUBA #$42 instruction (subtract same value)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x80); // SUBA immediate
    memory_bus.borrow_mut().write(0xC801, 0x42); // Subtract same value
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00, "A should be 0x42 - 0x42 = 0x00");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set (result = 0)");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_subb_immediate_basic() {
    // Test SUBB #$11 - Subtract immediate from B
    let mut cpu = create_test_cpu();
    
    // Initial state
    cpu.registers_mut().b = 0x44; // Initial B = 68
    
    // Setup: SUBB #$11 instruction
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xC0); // SUBB immediate opcode
    memory_bus.borrow_mut().write(0xC801, 0x11); // Subtract 17
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x33, "B should be 0x44 - 0x11 = 0x33");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.c, false, "C flag inverted by subtract implementation");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_add_sub_comprehensive_flags() {
    // Test various flag combinations with ADD/SUB
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Test 1: ADD negative result
    cpu.registers_mut().a = 0x70; // 112
    memory_bus.borrow_mut().write(0xC800, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(0xC801, 0x20); // Add 32 = 144 = 0x90 (negative)
    cpu.registers_mut().pc = 0xC800;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x90, "A should be 0x90");
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    
    // Test 2: SUB overflow: negative - positive = positive (should set V)
    cpu.registers_mut().b = 0x80; // -128 in signed
    memory_bus.borrow_mut().write(0xC810, 0xC0); // SUBB immediate
    memory_bus.borrow_mut().write(0xC811, 0x01); // Subtract 1
    cpu.registers_mut().pc = 0xC810;
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().b, 0x7F, "B should be 0x7F");
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set (overflow)");
}