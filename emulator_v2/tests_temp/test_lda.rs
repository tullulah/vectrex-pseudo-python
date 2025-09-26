// Test for LDA (Load A register) opcode 0x86 (immediate mode)
// Siguiendo las reglas de copilot-instructions.md

use crate::core::emulator::Emulator;
use crate::memory_device::MemoryDevice;
use crate::ram_device::RamDevice;
use std::rc::Rc;
use std::cell::RefCell;

// CONFIGURACIÓN OBLIGATORIA según copilot-instructions.md
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

fn setup_emulator() -> (Emulator, Rc<RefCell<dyn MemoryDevice>>) {
    let mut emulator = Emulator::new();
    let memory: Rc<RefCell<dyn MemoryDevice>> = Rc::new(RefCell::new(RamDevice::new()));
    emulator.memory().add_device(RAM_START, memory.clone()).unwrap();
    emulator.cpu_mut().set_stack_pointer(STACK_START);
    (emulator, memory)
}

#[test]
fn test_lda_immediate_basic_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - A con valor previo
    emulator.cpu_mut().set_register_a(0x99);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$42 (0x86 0x42)
    memory.borrow_mut().write(pc_start, 0x86).unwrap();     // LDA immediate
    memory.borrow_mut().write(pc_start + 1, 0x42).unwrap(); // valor a cargar
    
    // Ejecutar instrucción LDA
    let cycles = emulator.step().unwrap();
    
    // Verificar que A = 0x42
    assert_eq!(emulator.cpu().register_a(), 0x42, "LDA should load 0x42 into register A");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "LDA result 0x42 should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "LDA result 0x42 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "LDA always clears Overflow flag");
    
    // Verificar cycles (LDA immediate toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "LDA immediate should take 2 cycles");
    
    // Verificar que PC avanzó 2 bytes
    assert_eq!(emulator.cpu().program_counter(), pc_start + 2, "LDA immediate should advance PC by 2");
}

#[test]
fn test_lda_immediate_zero_flag_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0xFF);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$00 (0x86 0x00)
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x00).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x00, "LDA should load 0x00 into register A");
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "LDA result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "LDA result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "LDA always clears Overflow flag");
}

#[test]
fn test_lda_immediate_negative_flag_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x00);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$80 (0x86 0x80) - bit 7 = 1, negativo
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x80).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x80, "LDA should load 0x80 into register A");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "LDA result 0x80 should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "LDA result 0x80 should not set Zero flag");
}

#[test]
fn test_lda_immediate_overflow_always_clear_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Overflow flag inicialmente
    emulator.cpu_mut().set_register_a(0x11);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$77 (0x86 0x77) - LDA siempre limpia V flag
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x77).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x77, "LDA should load 0x77 into register A");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "LDA should always clear Overflow flag");
}

#[test]
fn test_lda_immediate_carry_preservation_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Carry flag
    emulator.cpu_mut().set_register_a(0x22);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$55 (0x86 0x55) - LDA NO afecta Carry flag
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x55).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x55, "LDA should load 0x55 into register A");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "LDA should not modify Carry flag");
}

#[test]
fn test_lda_immediate_preserve_other_registers_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los otros registros
    emulator.cpu_mut().set_register_a(0x11);
    emulator.cpu_mut().set_register_b(0x22);
    emulator.cpu_mut().set_register_x(0x3344);
    emulator.cpu_mut().set_register_y(0x5566);
    emulator.cpu_mut().set_register_u(0x7788);
    emulator.cpu_mut().set_register_dp(0x99);
    
    let initial_b = emulator.cpu().register_b();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$AB
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0xAB).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar que solo A cambió
    assert_eq!(emulator.cpu().register_a(), 0xAB, "LDA should load 0xAB into register A");
    assert_eq!(emulator.cpu().register_b(), initial_b, "LDA should not modify register B");
    assert_eq!(emulator.cpu().register_x(), initial_x, "LDA should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "LDA should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "LDA should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "LDA should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "LDA should not modify register DP");
}

#[test]
fn test_lda_immediate_boundary_values_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    
    // Test valores límite importantes
    let test_cases = [
        // (value_to_load, zero_flag, negative_flag)
        (0x00, true, false),   // Mínimo (zero)
        (0x01, false, false),  // Primer positivo
        (0x7F, false, false),  // Máximo positivo con signo
        (0x80, false, true),   // Mínimo negativo con signo
        (0xFE, false, true),   // Penúltimo valor (negativo)
        (0xFF, false, true),   // Máximo (negativo)
    ];
    
    for (value, exp_zero, exp_negative) in test_cases.iter() {
        emulator.cpu_mut().set_register_a(0x00); // Reset A
        emulator.cpu_mut().set_program_counter(pc_start);
        memory.borrow_mut().write(pc_start + 1, *value).unwrap();
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *value, 
                   "LDA should load 0x{:02X} into register A", value);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "LDA 0x{:02X} zero flag should be {}", value, exp_zero);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "LDA 0x{:02X} negative flag should be {}", value, exp_negative);
        assert_eq!(emulator.cpu().condition_codes().overflow(), false,
                   "LDA should always clear overflow flag");
    }
}

#[test]
fn test_lda_immediate_sequential_loads_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir secuencia: LDA #$10, LDA #$20, LDA #$FF, LDA #$00
    let values = [0x10, 0x20, 0xFF, 0x00];
    for (i, value) in values.iter().enumerate() {
        memory.borrow_mut().write(pc_start + (i * 2) as u16, 0x86).unwrap();     // LDA immediate
        memory.borrow_mut().write(pc_start + (i * 2) as u16 + 1, *value).unwrap(); // valor
    }
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Ejecutar secuencia
    for (i, expected_value) in values.iter().enumerate() {
        let cycles = emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *expected_value, 
                   "LDA step {} should load 0x{:02X}", i + 1, expected_value);
        assert_eq!(cycles, 2, "Each LDA should take 2 cycles");
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, 
                   "Each LDA should clear overflow flag");
        
        // Verificar flags específicos
        match expected_value {
            0x00 => assert_eq!(emulator.cpu().condition_codes().zero(), true, "LDA 0x00 should set Zero flag"),
            0xFF => assert_eq!(emulator.cpu().condition_codes().negative(), true, "LDA 0xFF should set Negative flag"),
            _ => {
                assert_eq!(emulator.cpu().condition_codes().zero(), false, "LDA non-zero should clear Zero flag");
                let exp_neg = *expected_value & 0x80 != 0;
                assert_eq!(emulator.cpu().condition_codes().negative(), exp_neg, 
                           "LDA 0x{:02X} negative flag should be {}", expected_value, exp_neg);
            }
        }
    }
}

#[test]
fn test_lda_immediate_overwrite_previous_value_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - A con valor conocido
    emulator.cpu_mut().set_register_a(0xAA);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$55 - cargar valor completamente diferente
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x55).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x55, "LDA should completely overwrite previous value");
    assert_ne!(emulator.cpu().register_a(), 0xAA, "Previous value should be completely replaced");
}

#[test]
fn test_lda_immediate_vs_ldb_independence_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer ambos registros A y B
    emulator.cpu_mut().set_register_a(0x11);
    emulator.cpu_mut().set_register_b(0x22);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // LDA #$33
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x33).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar que solo A cambió, B no fue afectado
    assert_eq!(emulator.cpu().register_a(), 0x33, "LDA should load into register A only");
    assert_eq!(emulator.cpu().register_b(), 0x22, "LDA should not affect register B");
}

#[test]
fn test_lda_immediate_flag_transitions_0x86() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x86).unwrap();
    
    // Test transiciones específicas de flags
    
    // 1. Establecer flags, luego cargar valor que los limpia
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    emulator.cpu_mut().condition_codes_mut().set_negative(true);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    emulator.cpu_mut().set_program_counter(pc_start);
    memory.borrow_mut().write(pc_start + 1, 0x01).unwrap(); // Positivo, no cero
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x01, "LDA should load 0x01");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "Loading 0x01 should clear Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "Loading 0x01 should clear Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "LDA should clear Overflow flag");
    
    // 2. Cargar cero para establecer Zero flag
    emulator.cpu_mut().set_program_counter(pc_start);
    memory.borrow_mut().write(pc_start + 1, 0x00).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "Loading 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "Loading 0x00 should not set Negative flag");
}