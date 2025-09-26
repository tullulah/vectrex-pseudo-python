// Test for LDA (Load A) opcode 0x86 (immediate mode)
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
fn test_lda_immediate_basic_0x86() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x00; // Valor inicial
    
    // Escribir LDA #$42 (immediate mode)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x42); // value to load
    
    // Ejecutar instrucción LDA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que se cargó el valor
    assert_eq!(cpu.registers.a, 0x42, "LDA should load immediate value");
    assert_eq!(cpu.registers.pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "LDA immediate should take 2 cycles");
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
    assert!(!cpu.registers.cc.n, "Negative flag should be clear");
}

#[test]
fn test_lda_immediate_zero_flag_0x86() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0xFF; // Valor inicial diferente de cero
    
    // Escribir LDA #$00 (immediate mode, cargar cero)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x00); // value 0
    
    // Ejecutar instrucción LDA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que se cargó cero y se activó el flag
    assert_eq!(cpu.registers.a, 0x00, "LDA should load zero");
    assert_eq!(cpu.registers.pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "LDA immediate should take 2 cycles");
    assert!(cpu.registers.cc.z, "Zero flag should be set");
    assert!(!cpu.registers.cc.n, "Negative flag should be clear");
}

#[test]
fn test_lda_immediate_negative_flag_0x86() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x00; // Valor inicial
    
    // Escribir LDA #$80 (immediate mode, cargar valor negativo)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x80); // value -128
    
    // Ejecutar instrucción LDA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que se cargó valor negativo y se activó el flag
    assert_eq!(cpu.registers.a, 0x80, "LDA should load negative value");
    assert_eq!(cpu.registers.pc, 0xC802, "PC should advance by 2");
    assert_eq!(cycles, 2, "LDA immediate should take 2 cycles");
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
    assert!(cpu.registers.cc.n, "Negative flag should be set");
}

#[test]
fn test_lda_immediate_overflow_always_clear_0x86() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - configurar overflow flag inicialmente activo
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x00;
    cpu.registers.cc.v = true; // Overflow inicialmente activo
    
    // Escribir LDA #$7F (immediate mode)
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x7F); // value +127
    
    // Ejecutar instrucción LDA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que LDA siempre limpia el overflow flag
    assert_eq!(cpu.registers.a, 0x7F, "LDA should load positive value");
    assert_eq!(cycles, 2);
    assert!(!cpu.registers.cc.v, "LDA should always clear overflow flag");
    assert!(!cpu.registers.cc.z, "Zero flag should be clear");
    assert!(!cpu.registers.cc.n, "Negative flag should be clear");
}

#[test]
fn test_lda_immediate_preserves_other_registers_0x86() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - configurar todos los registros
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x11;
    cpu.registers.b = 0x22;
    cpu.registers.x = 0x3344;
    cpu.registers.y = 0x5566;
    cpu.registers.dp = 0x77;
    
    // Escribir LDA #$99
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x99); // new value for A
    
    // Ejecutar instrucción LDA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que solo A cambió
    assert_eq!(cpu.registers.a, 0x99, "Only A should change");
    assert_eq!(cpu.registers.b, 0x22, "B should be preserved");
    assert_eq!(cpu.registers.x, 0x3344, "X should be preserved");
    assert_eq!(cpu.registers.y, 0x5566, "Y should be preserved");
    assert_eq!(cpu.registers.dp, 0x77, "DP should be preserved");
    assert_eq!(cycles, 2);
}

#[test]
fn test_lda_immediate_boundary_values_0x86() {
    let mut cpu = create_test_cpu();
    
    // Test con valor máximo (0xFF)
    cpu.registers.pc = 0xC800;
    cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate
    cpu.memory_bus().borrow_mut().write(0xC801, 0xFF); // max value
    
    let cycles1 = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cycles1, 2);
    assert!(!cpu.registers.cc.z);
    assert!(cpu.registers.cc.n); // 0xFF es negativo
    
    // Test con valor mínimo (0x01)
    cpu.registers.pc = 0xC800; // Reset PC
    cpu.memory_bus().borrow_mut().write(0xC801, 0x01); // min positive value
    
    let cycles2 = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cycles2, 2);
    assert!(!cpu.registers.cc.z);
    assert!(!cpu.registers.cc.n); // 0x01 es positivo
}