// Test suite for ADC (Add with Carry) and SBC (Subtract with Carry) opcodes
// Siguiendo patrón 1:1 con Vectrexy C++ - tests estrictos

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

/// Test ADCA immediate (0x89) - Add with carry immediate
#[test]
fn test_adca_immediate_0x89_basic() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x10, carry=0, operand=0x20 -> A=0x30, carry=0
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x89);     // ADCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x20); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x30);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.v, false);
}

/// Test ADCA immediate with carry set
#[test]
fn test_adca_immediate_with_carry() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x10, carry=1, operand=0x20 -> A=0x31, carry=0
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x89);     // ADCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x20); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x31);
    assert_eq!(cpu.registers().cc.c, false);
}

/// Test ADCA immediate with overflow
#[test]
fn test_adca_immediate_carry_overflow() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0xFF, carry=1, operand=0x01 -> A=0x01, carry=1
    cpu.registers_mut().a = 0xFF;
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x89);     // ADCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x01); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x01);
    assert_eq!(cpu.registers().cc.c, true);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.n, false);
}

/// Test SBCA immediate (0x82) - Subtract with carry immediate
#[test]  
fn test_sbca_immediate_0x82_basic() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x30, carry=0, operand=0x20 -> A=0x10, carry=0
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x82);     // SBCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x20); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x10);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.n, false);
    assert_eq!(cpu.registers().cc.v, false);
}

/// Test SBCA immediate with carry set
#[test]
fn test_sbca_immediate_with_carry() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x30, carry=1, operand=0x20 -> A=0x0F, carry=0
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x82);     // SBCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x20); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x0F);
    assert_eq!(cpu.registers().cc.c, false);
}

/// Test SBCA immediate with underflow
#[test]
fn test_sbca_immediate_underflow() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x10, carry=1, operand=0x20 -> A=0xEF, carry=1
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x82);     // SBCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x20); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xEF);
    assert_eq!(cpu.registers().cc.c, true);
    assert_eq!(cpu.registers().cc.n, true);
}

/// Test SBCA direct (0x92) - Subtract with carry direct
#[test]
fn test_sbca_direct_0x92_basic() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x50, carry=0, [RAM+100]=0x30 -> A=0x20, carry=0
    cpu.registers_mut().a = 0x50;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().dp = (RAM_START >> 8) as u8; // Direct page matches RAM
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START + 100, 0x30);    // Data at direct address
    memory_bus.borrow_mut().write(RAM_START, 0x92);          // SBCA direct
    memory_bus.borrow_mut().write(RAM_START + 1, (RAM_START + 100) as u8); // Direct address
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x20);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.n, false);
}

/// Test ADCA direct (0x99) - Add with carry direct
#[test] 
fn test_adca_direct_0x99_basic() {
    let mut cpu = create_test_cpu();
    
    // Setup: A=0x20, carry=0, [RAM+100]=0x30 -> A=0x50, carry=0
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().dp = (RAM_START >> 8) as u8; // Direct page matches RAM
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START + 100, 0x30);    // Data at direct address  
    memory_bus.borrow_mut().write(RAM_START, 0x99);          // ADCA direct
    memory_bus.borrow_mut().write(RAM_START + 1, (RAM_START + 100) as u8); // Direct address
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x50);
    assert_eq!(cpu.registers().cc.c, false);
    assert_eq!(cpu.registers().cc.z, false);
    assert_eq!(cpu.registers().cc.n, false);
}

/// Test comprehensive flag behavior for ADC/SBC
#[test]
fn test_adc_sbc_flags_comprehensive() {
    let mut cpu = create_test_cpu();
    
    // Test 1: ADCA zero result  
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().pc = RAM_START;
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(RAM_START, 0x89);     // ADCA immediate
    memory_bus.borrow_mut().write(RAM_START + 1, 0x00); // operand
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert_eq!(cpu.registers().cc.z, true);
    assert_eq!(cpu.registers().cc.n, false);
    
    // Reset for Test 2: SBCA negative result
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().cc.c = false;
    cpu.registers_mut().pc = RAM_START + 2;
    
    memory_bus.borrow_mut().write(RAM_START + 2, 0x82);  // SBCA immediate
    memory_bus.borrow_mut().write(RAM_START + 3, 0x30);  // operand (larger than A)
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xF0);
    assert_eq!(cpu.registers().cc.c, true);   // Borrow occurred
    assert_eq!(cpu.registers().cc.n, true);   // Negative result
    assert_eq!(cpu.registers().cc.z, false);
}