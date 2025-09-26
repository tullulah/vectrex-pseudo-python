// Test for INCB (Increment B register) opcode 0x5C
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
fn test_incb_basic_increment_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_b(0x33);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // INCB (0x5C)
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    // Ejecutar instrucción INCB
    let cycles = emulator.step().unwrap();
    
    // Verificar que B se incrementó
    assert_eq!(emulator.cpu().register_b(), 0x34, "INCB should increment register B from 0x33 to 0x34");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "INCB result 0x34 should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "INCB result 0x34 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "INCB 0x33->0x34 should not set Overflow flag");
    
    // Verificar cycles (INCB toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "INCB should take 2 cycles");
    
    // Verificar que PC avanzó
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "INCB should advance PC by 1");
}

#[test]
fn test_incb_zero_flag_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - incrementar 0xFF para obtener 0x00 (overflow a 0)
    emulator.cpu_mut().set_register_b(0xFF);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // INCB (0x5C)
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    // Ejecutar INCB
    let cycles = emulator.step().unwrap();
    
    // Verificar que B se convirtió en 0x00
    assert_eq!(emulator.cpu().register_b(), 0x00, "INCB should wrap 0xFF to 0x00");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "INCB result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "INCB result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "INCB 0xFF->0x00 should not set Overflow flag");
    
    assert_eq!(cycles, 2, "INCB should take 2 cycles");
}

#[test]
fn test_incb_negative_flag_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - incrementar 0x7E para obtener 0x7F (positivo)
    emulator.cpu_mut().set_register_b(0x7E);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x7F, "INCB should increment 0x7E to 0x7F");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "INCB result 0x7F should not set Negative flag");
    
    // Test con resultado negativo - incrementar 0x7F para obtener 0x80 (negativo)
    emulator.cpu_mut().set_register_b(0x7F);
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x80, "INCB should increment 0x7F to 0x80");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "INCB result 0x80 should set Negative flag");
}

#[test]
fn test_incb_overflow_flag_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - incrementar 0x7F (127) para obtener 0x80 (-128)
    // Esto es overflow en aritmética con signo: +127 + 1 = -128
    emulator.cpu_mut().set_register_b(0x7F);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x80, "INCB should increment 0x7F to 0x80");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "INCB 0x7F->0x80 should set Overflow flag (signed overflow)");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "INCB result 0x80 should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "INCB result 0x80 should not set Zero flag");
}

#[test]
fn test_incb_preserve_other_registers_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los otros registros
    emulator.cpu_mut().set_register_a(0x10);
    emulator.cpu_mut().set_register_b(0x20);
    emulator.cpu_mut().set_register_x(0x3040);
    emulator.cpu_mut().set_register_y(0x5060);
    emulator.cpu_mut().set_register_u(0x7080);
    emulator.cpu_mut().set_register_dp(0x90);
    
    let initial_a = emulator.cpu().register_a();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    // Ejecutar INCB
    emulator.step().unwrap();
    
    // Verificar que solo B cambió
    assert_eq!(emulator.cpu().register_b(), 0x21, "INCB should increment register B");
    assert_eq!(emulator.cpu().register_a(), initial_a, "INCB should not modify register A");
    assert_eq!(emulator.cpu().register_x(), initial_x, "INCB should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "INCB should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "INCB should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "INCB should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "INCB should not modify register DP");
}

#[test]
fn test_incb_carry_flag_preservation_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Carry flag
    emulator.cpu_mut().set_register_b(0x60);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    // Ejecutar INCB
    emulator.step().unwrap();
    
    // Verificar que INCB NO modifica Carry flag (INC no afecta Carry en 6809)
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "INCB should not modify Carry flag");
    assert_eq!(emulator.cpu().register_b(), 0x61, "INCB should increment B normally");
}

#[test]
fn test_incb_vs_inca_independence_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer ambos registros A y B
    emulator.cpu_mut().set_register_a(0x10);
    emulator.cpu_mut().set_register_b(0x20);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5C).unwrap(); // INCB
    
    // Ejecutar INCB
    emulator.step().unwrap();
    
    // Verificar que solo B cambió, A no fue afectado
    assert_eq!(emulator.cpu().register_a(), 0x10, "INCB should not affect register A");
    assert_eq!(emulator.cpu().register_b(), 0x21, "INCB should increment register B only");
}

#[test]
fn test_incb_boundary_conditions_0x5C() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x5C).unwrap();
    
    // Test casos límite
    let test_cases = [
        (0x00, 0x01, false, false, false), // 0 -> 1
        (0x7E, 0x7F, false, false, false), // último positivo
        (0x7F, 0x80, true, true, false),   // overflow a negativo
        (0x80, 0x81, false, true, false),  // negativo -> más negativo
        (0xFE, 0xFF, false, true, false),  // -2 -> -1
        (0xFF, 0x00, false, false, true),  // -1 -> 0 (wrap)
    ];
    
    for (initial, expected, exp_overflow, exp_negative, exp_zero) in test_cases.iter() {
        emulator.cpu_mut().set_register_b(*initial);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_b(), *expected, 
                   "INCB 0x{:02X} should result in 0x{:02X}", initial, expected);
        assert_eq!(emulator.cpu().condition_codes().overflow(), *exp_overflow,
                   "INCB 0x{:02X} overflow flag should be {}", initial, exp_overflow);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "INCB 0x{:02X} negative flag should be {}", initial, exp_negative);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "INCB 0x{:02X} zero flag should be {}", initial, exp_zero);
    }
}