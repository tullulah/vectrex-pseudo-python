// Test for DECA (Decrement A register) opcode 0x4A
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
fn test_deca_basic_decrement_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x42);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // DECA (0x4A)
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Ejecutar instrucción DECA
    let cycles = emulator.step().unwrap();
    
    // Verificar que A se decrementó
    assert_eq!(emulator.cpu().register_a(), 0x41, "DECA should decrement register A from 0x42 to 0x41");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECA result 0x41 should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECA result 0x41 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECA 0x42->0x41 should not set Overflow flag");
    
    // Verificar cycles (DECA toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "DECA should take 2 cycles");
    
    // Verificar que PC avanzó
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "DECA should advance PC by 1");
}

#[test]
fn test_deca_zero_flag_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x01 para obtener 0x00
    emulator.cpu_mut().set_register_a(0x01);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // DECA (0x4A)
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Ejecutar DECA
    let cycles = emulator.step().unwrap();
    
    // Verificar que A se convirtió en 0x00
    assert_eq!(emulator.cpu().register_a(), 0x00, "DECA should decrement 0x01 to 0x00");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "DECA result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECA result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECA 0x01->0x00 should not set Overflow flag");
    
    assert_eq!(cycles, 2, "DECA should take 2 cycles");
}

#[test]
fn test_deca_underflow_wrap_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x00 para obtener 0xFF (underflow)
    emulator.cpu_mut().set_register_a(0x00);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0xFF, "DECA should wrap 0x00 to 0xFF");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "DECA result 0xFF should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECA result 0xFF should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECA 0x00->0xFF should not set Overflow flag");
}

#[test]
fn test_deca_overflow_flag_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x80 (-128) para obtener 0x7F (+127)
    // Esto es overflow en aritmética con signo: -128 - 1 = +127
    emulator.cpu_mut().set_register_a(0x80);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x7F, "DECA should decrement 0x80 to 0x7F");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "DECA 0x80->0x7F should set Overflow flag (signed overflow)");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECA result 0x7F should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECA result 0x7F should not set Zero flag");
}

#[test]
fn test_deca_negative_flag_transitions_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Test transición de positivo a positivo
    emulator.cpu_mut().set_register_a(0x02);
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x01, "DECA should decrement 0x02 to 0x01");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECA result 0x01 should not set Negative flag");
    
    // Test transición de negativo a negativo
    emulator.cpu_mut().set_register_a(0x81); // -127
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x80, "DECA should decrement 0x81 to 0x80");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "DECA result 0x80 should set Negative flag");
}

#[test]
fn test_deca_preserve_other_registers_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los otros registros
    emulator.cpu_mut().set_register_a(0x10);
    emulator.cpu_mut().set_register_b(0x20);
    emulator.cpu_mut().set_register_x(0x3040);
    emulator.cpu_mut().set_register_y(0x5060);
    emulator.cpu_mut().set_register_u(0x7080);
    emulator.cpu_mut().set_register_dp(0x90);
    
    let initial_b = emulator.cpu().register_b();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Ejecutar DECA
    emulator.step().unwrap();
    
    // Verificar que solo A cambió
    assert_eq!(emulator.cpu().register_a(), 0x0F, "DECA should decrement register A");
    assert_eq!(emulator.cpu().register_b(), initial_b, "DECA should not modify register B");
    assert_eq!(emulator.cpu().register_x(), initial_x, "DECA should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "DECA should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "DECA should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "DECA should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "DECA should not modify register DP");
}

#[test]
fn test_deca_carry_flag_preservation_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Carry flag
    emulator.cpu_mut().set_register_a(0x50);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Ejecutar DECA
    emulator.step().unwrap();
    
    // Verificar que DECA NO modifica Carry flag (DEC no afecta Carry en 6809)
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "DECA should not modify Carry flag");
    assert_eq!(emulator.cpu().register_a(), 0x4F, "DECA should decrement A normally");
}

#[test]
fn test_deca_boundary_conditions_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x4A).unwrap();
    
    // Test casos límite importantes
    let test_cases = [
        (0x01, 0x00, false, false, true),  // 1 -> 0 (zero)
        (0x00, 0xFF, false, true, false),  // 0 -> 255 (wrap to negative)
        (0x80, 0x7F, true, false, false),  // -128 -> +127 (overflow)
        (0x81, 0x80, false, true, false),  // -127 -> -128 (stays negative)
        (0x7F, 0x7E, false, false, false), // +127 -> +126 (stays positive)
        (0xFF, 0xFE, false, true, false),  // -1 -> -2 (stays negative)
    ];
    
    for (initial, expected, exp_overflow, exp_negative, exp_zero) in test_cases.iter() {
        emulator.cpu_mut().set_register_a(*initial);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *expected, 
                   "DECA 0x{:02X} should result in 0x{:02X}", initial, expected);
        assert_eq!(emulator.cpu().condition_codes().overflow(), *exp_overflow,
                   "DECA 0x{:02X} overflow flag should be {}", initial, exp_overflow);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "DECA 0x{:02X} negative flag should be {}", initial, exp_negative);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "DECA 0x{:02X} zero flag should be {}", initial, exp_zero);
    }
}

#[test]
fn test_deca_sequential_decrements_0x4A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - empezar en 0x03 y decrementar hasta wrap
    emulator.cpu_mut().set_register_a(0x03);
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir secuencia de DECAs
    for i in 0..6 {
        memory.borrow_mut().write(pc_start + i, 0x4A).unwrap();
    }
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Ejecutar secuencia: 0x03 -> 0x02 -> 0x01 -> 0x00 -> 0xFF -> 0xFE
    let expected_values = [0x02, 0x01, 0x00, 0xFF, 0xFE, 0xFD];
    let expected_zero_flags = [false, false, true, false, false, false];
    let expected_negative_flags = [false, false, false, true, true, true];
    
    for (i, (expected_val, expected_zero, expected_neg)) in 
        expected_values.iter()
                      .zip(expected_zero_flags.iter())
                      .zip(expected_negative_flags.iter())
                      .enumerate() {
        
        let cycles = emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *expected_val, 
                   "DECA step {} should result in 0x{:02X}", i + 1, expected_val);
        assert_eq!(emulator.cpu().condition_codes().zero(), *expected_zero,
                   "DECA step {} Zero flag should be {}", i + 1, expected_zero);
        assert_eq!(emulator.cpu().condition_codes().negative(), *expected_neg,
                   "DECA step {} Negative flag should be {}", i + 1, expected_neg);
        assert_eq!(cycles, 2, "Each DECA should take 2 cycles");
    }
}