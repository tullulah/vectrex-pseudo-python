// Test for DECB (Decrement B register) opcode 0x5A
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
fn test_decb_basic_decrement_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_b(0x55);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // DECB (0x5A)
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    // Ejecutar instrucción DECB
    let cycles = emulator.step().unwrap();
    
    // Verificar que B se decrementó
    assert_eq!(emulator.cpu().register_b(), 0x54, "DECB should decrement register B from 0x55 to 0x54");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECB result 0x54 should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECB result 0x54 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECB 0x55->0x54 should not set Overflow flag");
    
    // Verificar cycles (DECB toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "DECB should take 2 cycles");
    
    // Verificar que PC avanzó
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "DECB should advance PC by 1");
}

#[test]
fn test_decb_zero_flag_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x01 para obtener 0x00
    emulator.cpu_mut().set_register_b(0x01);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // DECB (0x5A)
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    // Ejecutar DECB
    let cycles = emulator.step().unwrap();
    
    // Verificar que B se convirtió en 0x00
    assert_eq!(emulator.cpu().register_b(), 0x00, "DECB should decrement 0x01 to 0x00");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "DECB result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECB result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECB 0x01->0x00 should not set Overflow flag");
    
    assert_eq!(cycles, 2, "DECB should take 2 cycles");
}

#[test]
fn test_decb_underflow_wrap_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x00 para obtener 0xFF (underflow)
    emulator.cpu_mut().set_register_b(0x00);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0xFF, "DECB should wrap 0x00 to 0xFF");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "DECB result 0xFF should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECB result 0xFF should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "DECB 0x00->0xFF should not set Overflow flag");
}

#[test]
fn test_decb_overflow_flag_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - decrementar 0x80 (-128) para obtener 0x7F (+127)
    // Esto es overflow en aritmética con signo: -128 - 1 = +127
    emulator.cpu_mut().set_register_b(0x80);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x7F, "DECB should decrement 0x80 to 0x7F");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "DECB 0x80->0x7F should set Overflow flag (signed overflow)");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "DECB result 0x7F should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "DECB result 0x7F should not set Zero flag");
}

#[test]
fn test_decb_preserve_other_registers_0x5A() {
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
    
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    // Ejecutar DECB
    emulator.step().unwrap();
    
    // Verificar que solo B cambió
    assert_eq!(emulator.cpu().register_b(), 0x1F, "DECB should decrement register B");
    assert_eq!(emulator.cpu().register_a(), initial_a, "DECB should not modify register A");
    assert_eq!(emulator.cpu().register_x(), initial_x, "DECB should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "DECB should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "DECB should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "DECB should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "DECB should not modify register DP");
}

#[test]
fn test_decb_carry_flag_preservation_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Carry flag
    emulator.cpu_mut().set_register_b(0x60);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
    // Ejecutar DECB
    emulator.step().unwrap();
    
    // Verificar que DECB NO modifica Carry flag (DEC no afecta Carry en 6809)
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "DECB should not modify Carry flag");
    assert_eq!(emulator.cpu().register_b(), 0x5F, "DECB should decrement B normally");
}

#[test]
fn test_decb_vs_deca_independence_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer ambos registros A y B
    emulator.cpu_mut().set_register_a(0x15);
    emulator.cpu_mut().set_register_b(0x25);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x5A).unwrap(); // DECB
    
    // Ejecutar DECB
    emulator.step().unwrap();
    
    // Verificar que solo B cambió, A no fue afectado
    assert_eq!(emulator.cpu().register_a(), 0x15, "DECB should not affect register A");
    assert_eq!(emulator.cpu().register_b(), 0x24, "DECB should decrement register B only");
}

#[test]
fn test_decb_boundary_conditions_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();
    
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
        emulator.cpu_mut().set_register_b(*initial);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_b(), *expected, 
                   "DECB 0x{:02X} should result in 0x{:02X}", initial, expected);
        assert_eq!(emulator.cpu().condition_codes().overflow(), *exp_overflow,
                   "DECB 0x{:02X} overflow flag should be {}", initial, exp_overflow);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "DECB 0x{:02X} negative flag should be {}", initial, exp_negative);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "DECB 0x{:02X} zero flag should be {}", initial, exp_zero);
    }
}

#[test]
fn test_decb_mixed_with_incb_operations_0x5A() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - verificar que DECB e INCB son operaciones inversas
    emulator.cpu_mut().set_register_b(0x50);
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir DECB seguido de INCB
    memory.borrow_mut().write(pc_start, 0x5A).unwrap();     // DECB
    memory.borrow_mut().write(pc_start + 1, 0x5C).unwrap(); // INCB
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Ejecutar DECB
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_b(), 0x4F, "DECB should decrement to 0x4F");
    
    // Ejecutar INCB
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_b(), 0x50, "INCB should restore to original value 0x50");
}