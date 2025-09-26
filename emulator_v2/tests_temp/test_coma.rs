// Test for COMA (Complement A register) opcode 0x43
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
fn test_coma_basic_complement_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x55); // 01010101
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // COMA (0x43)
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    // Ejecutar instrucción COMA
    let cycles = emulator.step().unwrap();
    
    // Verificar que A = ~0x55 = 0xAA (10101010)
    assert_eq!(emulator.cpu().register_a(), 0xAA, "COMA should complement 0x55 to 0xAA");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "COMA result 0xAA should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "COMA result 0xAA should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "COMA always clears Overflow flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "COMA always sets Carry flag");
    
    // Verificar cycles (COMA toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "COMA should take 2 cycles");
    
    // Verificar que PC avanzó 1 byte
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "COMA should advance PC by 1");
}

#[test]
fn test_coma_zero_flag_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - complementar 0xFF para obtener 0x00
    emulator.cpu_mut().set_register_a(0xFF);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // COMA (0x43)
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x00, "COMA should complement 0xFF to 0x00");
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "COMA result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "COMA result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "COMA always sets Carry flag");
}

#[test]
fn test_coma_negative_flag_transitions_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    // Test complemento de valores positivos -> negativos
    emulator.cpu_mut().set_register_a(0x7F); // 01111111
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x80, "COMA should complement 0x7F to 0x80");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "COMA result 0x80 should set Negative flag");
    
    // Test complemento de valores negativos -> positivos
    emulator.cpu_mut().set_register_a(0x80); // 10000000
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_a(), 0x7F, "COMA should complement 0x80 to 0x7F");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "COMA result 0x7F should not set Negative flag");
}

#[test]
fn test_coma_always_sets_carry_clears_overflow_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    // Test diferentes valores iniciales para verificar que C=1, V=0 siempre
    let test_values = [0x00, 0x01, 0x7F, 0x80, 0xFF, 0xAA, 0x55];
    
    for initial_val in test_values.iter() {
        // Setup inicial - establecer V=1, C=0 para probar que COMA los cambia
        emulator.cpu_mut().set_register_a(*initial_val);
        emulator.cpu_mut().condition_codes_mut().set_carry(false);
        emulator.cpu_mut().condition_codes_mut().set_overflow(true);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        let expected = !initial_val;
        assert_eq!(emulator.cpu().register_a(), expected, 
                   "COMA should complement 0x{:02X} to 0x{:02X}", initial_val, expected);
        assert_eq!(emulator.cpu().condition_codes().carry(), true, 
                   "COMA should always set Carry flag regardless of input 0x{:02X}", initial_val);
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, 
                   "COMA should always clear Overflow flag regardless of input 0x{:02X}", initial_val);
    }
}

#[test]
fn test_coma_bit_patterns_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    // Test patrones específicos de bits
    let test_cases = [
        // (input, expected_output, zero_flag, negative_flag)
        (0x00, 0xFF, false, true),   // Todos ceros -> todos unos
        (0xFF, 0x00, true, false),   // Todos unos -> todos ceros
        (0xF0, 0x0F, false, false),  // Nibble alto -> nibble bajo
        (0x0F, 0xF0, false, true),   // Nibble bajo -> nibble alto
        (0xAA, 0x55, false, false),  // Patrón alternado
        (0x55, 0xAA, false, true),   // Patrón alternado inverso
        (0x01, 0xFE, false, true),   // Un bit -> casi todos los bits
        (0xFE, 0x01, false, false),  // Casi todos -> un bit
    ];
    
    for (input, expected, exp_zero, exp_negative) in test_cases.iter() {
        emulator.cpu_mut().set_register_a(*input);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *expected, 
                   "COMA 0x{:02X} should result in 0x{:02X}", input, expected);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "COMA 0x{:02X} zero flag should be {}", input, exp_zero);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "COMA 0x{:02X} negative flag should be {}", input, exp_negative);
    }
}

#[test]
fn test_coma_preserve_other_registers_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los otros registros
    emulator.cpu_mut().set_register_a(0x33);
    emulator.cpu_mut().set_register_b(0x44);
    emulator.cpu_mut().set_register_x(0x5566);
    emulator.cpu_mut().set_register_y(0x7788);
    emulator.cpu_mut().set_register_u(0x9900);
    emulator.cpu_mut().set_register_dp(0xAA);
    
    let initial_b = emulator.cpu().register_b();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x43).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar que solo A cambió
    assert_eq!(emulator.cpu().register_a(), 0xCC, "COMA should complement 0x33 to 0xCC");
    assert_eq!(emulator.cpu().register_b(), initial_b, "COMA should not modify register B");
    assert_eq!(emulator.cpu().register_x(), initial_x, "COMA should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "COMA should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "COMA should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "COMA should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "COMA should not modify register DP");
}

#[test]
fn test_coma_double_complement_identity_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - verificar que COMA es su propia inversa
    let original_value = 0x69;
    emulator.cpu_mut().set_register_a(original_value);
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir dos instrucciones COMA consecutivas
    memory.borrow_mut().write(pc_start, 0x43).unwrap();     // Primera COMA
    memory.borrow_mut().write(pc_start + 1, 0x43).unwrap(); // Segunda COMA
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Primera complementación: 0x69 -> 0x96
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_a(), 0x96, "First COMA should complement 0x69 to 0x96");
    
    // Segunda complementación: 0x96 -> 0x69 (vuelta al original)
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_a(), original_value, "Double COMA should restore original value");
    
    // Verificar flags de la segunda operación
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "Second COMA should set Carry flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "Second COMA should clear Overflow flag");
}

#[test]
fn test_coma_vs_comb_independence_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer ambos registros A y B
    emulator.cpu_mut().set_register_a(0xF0);
    emulator.cpu_mut().set_register_b(0x0F);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x43).unwrap(); // COMA
    
    emulator.step().unwrap();
    
    // Verificar que solo A cambió, B no fue afectado
    assert_eq!(emulator.cpu().register_a(), 0x0F, "COMA should complement register A to 0x0F");
    assert_eq!(emulator.cpu().register_b(), 0x0F, "COMA should not affect register B");
}

#[test]
fn test_coma_sequential_different_values_0x43() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir secuencia de COMA
    for i in 0..4 {
        memory.borrow_mut().write(pc_start + i, 0x43).unwrap();
    }
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Secuencia de valores para complementar
    let values = [0x12, 0x34, 0x56, 0x78];
    let expected = [0xED, 0xCB, 0xA9, 0x87];
    
    for (i, (initial, expected_result)) in values.iter().zip(expected.iter()).enumerate() {
        emulator.cpu_mut().set_register_a(*initial);
        
        let cycles = emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_a(), *expected_result, 
                   "COMA step {} should complement 0x{:02X} to 0x{:02X}", i + 1, initial, expected_result);
        assert_eq!(cycles, 2, "Each COMA should take 2 cycles");
        assert_eq!(emulator.cpu().condition_codes().carry(), true, "Each COMA should set Carry flag");
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, "Each COMA should clear Overflow flag");
    }
}