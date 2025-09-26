// Test for INCA (Increment A register) opcode 0x4C
// Port directo desde Vectrexy siguiendo copilot-instructions.md

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

// CONFIGURACIÓN OBLIGATORIA según copilot-instructions.md
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers.s = STACK_START; // Stack pointer al final de RAM
    cpu
}

#[test]
fn test_inca_basic_increment_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x42;
    
    // Escribir INCA
    cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
    
    // Ejecutar instrucción INCA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que A se incrementó
    assert_eq!(cpu.registers.a, 0x43, "INCA should increment A from 0x42 to 0x43");
    assert_eq!(cpu.registers.pc, 0xC801, "PC should advance by 1");
    assert_eq!(cycles, 2, "INCA should take 2 cycles");
    
    // Verificar condition codes
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
    assert!(!cpu.registers.cc.n, "Negative flag should be clear");
    assert!(!cpu.registers.cc.v, "Overflow flag should be clear");
}

#[test]
fn test_inca_zero_flag_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - incrementar 0xFF para obtener 0x00 (overflow a 0)
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0xFF;
    
    // Escribir INCA
    cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
    
    // Ejecutar INCA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que A se convirtió en 0x00
    assert_eq!(cpu.registers.a, 0x00, "INCA should wrap 0xFF to 0x00");
    assert_eq!(cycles, 2, "INCA should take 2 cycles");
    
    // Verificar condition codes
    assert!(cpu.registers.cc.z, "Zero flag should be set");
    assert!(!cpu.registers.cc.n, "Negative flag should be clear");
    assert!(!cpu.registers.cc.v, "Overflow flag should be clear");
}

#[test]
fn test_inca_negative_flag_0x4c() { 
    let mut cpu = create_test_cpu();
    
    // Test con resultado negativo - incrementar 0x7F para obtener 0x80 (negativo)
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x7F;
    
    cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0x80, "INCA should increment 0x7F to 0x80");
    assert_eq!(cycles, 2);
    assert!(cpu.registers.cc.n, "Negative flag should be set for 0x80");
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
}

#[test]
fn test_inca_overflow_flag_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - incrementar 0x7F (127) para obtener 0x80 (-128)
    // Esto es overflow en aritmética con signo: +127 + 1 = -128
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x7F;
    
    cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0x80, "INCA should increment 0x7F to 0x80");
    assert_eq!(cycles, 2);
    assert!(cpu.registers.cc.v, "Overflow flag should be set (signed overflow)");
    assert!(cpu.registers.cc.n, "Negative flag should be set");
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
}

#[test]
fn test_inca_preserve_other_registers_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - establecer todos los otros registros
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x10;
    cpu.registers.b = 0x20;
    cpu.registers.x = 0x3040;
    cpu.registers.y = 0x5060;
    cpu.registers.u = 0x7080;
    cpu.registers.dp = 0x90;
    
    let initial_b = cpu.registers.b;
    let initial_x = cpu.registers.x;
    let initial_y = cpu.registers.y;
    let initial_u = cpu.registers.u;
    let initial_s = cpu.registers.s;
    let initial_dp = cpu.registers.dp;
    
    cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
    
    // Ejecutar INCA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que solo A cambió
    assert_eq!(cpu.registers.a, 0x11, "INCA should increment register A");
    assert_eq!(cpu.registers.b, initial_b, "INCA should not modify register B");
    assert_eq!(cpu.registers.x, initial_x, "INCA should not modify register X");
    assert_eq!(cpu.registers.y, initial_y, "INCA should not modify register Y");
    assert_eq!(cpu.registers.u, initial_u, "INCA should not modify register U");
    assert_eq!(cpu.registers.s, initial_s, "INCA should not modify register S");
    assert_eq!(cpu.registers.dp, initial_dp, "INCA should not modify register DP");
    assert_eq!(cycles, 2);
}