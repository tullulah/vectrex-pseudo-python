// Test for COMB (Complement B register) opcode 0x53
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
fn test_comb_basic_complement_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_b(0x33); // 00110011
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // COMB (0x53)
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    // Ejecutar instrucción COMB
    let cycles = emulator.step().unwrap();
    
    // Verificar que B = ~0x33 = 0xCC (11001100)
    assert_eq!(emulator.cpu().register_b(), 0xCC, "COMB should complement 0x33 to 0xCC");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "COMB result 0xCC should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "COMB result 0xCC should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "COMB always clears Overflow flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "COMB always sets Carry flag");
    
    // Verificar cycles (COMB toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "COMB should take 2 cycles");
    
    // Verificar que PC avanzó 1 byte
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "COMB should advance PC by 1");
}

#[test]
fn test_comb_zero_flag_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - complementar 0xFF para obtener 0x00
    emulator.cpu_mut().set_register_b(0xFF);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // COMB (0x53)
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x00, "COMB should complement 0xFF to 0x00");
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "COMB result 0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "COMB result 0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "COMB always sets Carry flag");
}

#[test]
fn test_comb_negative_flag_transitions_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    // Test complemento de valores positivos -> negativos
    emulator.cpu_mut().set_register_b(0x3F); // 00111111
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0xC0, "COMB should complement 0x3F to 0xC0");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "COMB result 0xC0 should set Negative flag");
    
    // Test complemento de valores negativos -> positivos
    emulator.cpu_mut().set_register_b(0x81); // 10000001
    emulator.cpu_mut().set_program_counter(pc_start);
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().register_b(), 0x7E, "COMB should complement 0x81 to 0x7E");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "COMB result 0x7E should not set Negative flag");
}

#[test]
fn test_comb_always_sets_carry_clears_overflow_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    // Test diferentes valores iniciales para verificar que C=1, V=0 siempre
    let test_values = [0x00, 0x01, 0x7F, 0x80, 0xFF, 0xAA, 0x55];
    
    for initial_val in test_values.iter() {
        // Setup inicial - establecer V=1, C=0 para probar que COMB los cambia
        emulator.cpu_mut().set_register_b(*initial_val);
        emulator.cpu_mut().condition_codes_mut().set_carry(false);
        emulator.cpu_mut().condition_codes_mut().set_overflow(true);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        let expected = !initial_val;
        assert_eq!(emulator.cpu().register_b(), expected, 
                   "COMB should complement 0x{:02X} to 0x{:02X}", initial_val, expected);
        assert_eq!(emulator.cpu().condition_codes().carry(), true, 
                   "COMB should always set Carry flag regardless of input 0x{:02X}", initial_val);
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, 
                   "COMB should always clear Overflow flag regardless of input 0x{:02X}", initial_val);
    }
}

#[test]
fn test_comb_bit_patterns_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
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
        emulator.cpu_mut().set_register_b(*input);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_b(), *expected, 
                   "COMB 0x{:02X} should result in 0x{:02X}", input, expected);
        assert_eq!(emulator.cpu().condition_codes().zero(), *exp_zero,
                   "COMB 0x{:02X} zero flag should be {}", input, exp_zero);
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "COMB 0x{:02X} negative flag should be {}", input, exp_negative);
    }
}

#[test]
fn test_comb_preserve_other_registers_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los otros registros
    emulator.cpu_mut().set_register_a(0x11);
    emulator.cpu_mut().set_register_b(0x44);
    emulator.cpu_mut().set_register_x(0x5566);
    emulator.cpu_mut().set_register_y(0x7788);
    emulator.cpu_mut().set_register_u(0x9900);
    emulator.cpu_mut().set_register_dp(0xBB);
    
    let initial_a = emulator.cpu().register_a();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar que solo B cambió
    assert_eq!(emulator.cpu().register_b(), 0xBB, "COMB should complement 0x44 to 0xBB");
    assert_eq!(emulator.cpu().register_a(), initial_a, "COMB should not modify register A");
    assert_eq!(emulator.cpu().register_x(), initial_x, "COMB should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "COMB should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "COMB should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "COMB should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "COMB should not modify register DP");
}

#[test]
fn test_comb_double_complement_identity_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - verificar que COMB es su propia inversa
    let original_value = 0x96;
    emulator.cpu_mut().set_register_b(original_value);
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir dos instrucciones COMB consecutivas
    memory.borrow_mut().write(pc_start, 0x53).unwrap();     // Primera COMB
    memory.borrow_mut().write(pc_start + 1, 0x53).unwrap(); // Segunda COMB
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Primera complementación: 0x96 -> 0x69
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_b(), 0x69, "First COMB should complement 0x96 to 0x69");
    
    // Segunda complementación: 0x69 -> 0x96 (vuelta al original)
    emulator.step().unwrap();
    assert_eq!(emulator.cpu().register_b(), original_value, "Double COMB should restore original value");
    
    // Verificar flags de la segunda operación
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "Second COMB should set Carry flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "Second COMB should clear Overflow flag");
}

#[test]
fn test_comb_vs_coma_independence_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer ambos registros A y B
    emulator.cpu_mut().set_register_a(0x0F);
    emulator.cpu_mut().set_register_b(0xF0);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x53).unwrap(); // COMB
    
    emulator.step().unwrap();
    
    // Verificar que solo B cambió, A no fue afectado
    assert_eq!(emulator.cpu().register_b(), 0x0F, "COMB should complement register B to 0x0F");
    assert_eq!(emulator.cpu().register_a(), 0x0F, "COMB should not affect register A");
}

#[test]
fn test_comb_comprehensive_bit_operations_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x53).unwrap();
    
    // Test operaciones de complemento en todos los patrones bit significativos
    let comprehensive_tests = [
        // Casos límite
        (0x00, 0xFF, true, true),    // Min -> Max (zero -> negative)
        (0xFF, 0x00, true, false),   // Max -> Min (negative -> zero)
        (0x80, 0x7F, false, false),  // MSB set -> MSB clear
        (0x7F, 0x80, true, false),   // MSB clear -> MSB set
        (0x01, 0xFE, true, false),   // LSB set -> LSB clear (negative result)
        (0xFE, 0x01, false, false),  // LSB clear -> LSB set
        
        // Patrones de nibbles
        (0xF0, 0x0F, false, false),  // High nibble -> Low nibble
        (0x0F, 0xF0, true, false),   // Low nibble -> High nibble (negative)
        
        // Patrones alternados
        (0xAA, 0x55, false, false),  // 10101010 -> 01010101
        (0x55, 0xAA, true, false),   // 01010101 -> 10101010 (negative)
    ];
    
    for (input, expected, sets_carry, exp_negative) in comprehensive_tests.iter() {
        emulator.cpu_mut().set_register_b(*input);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_b(), *expected, 
                   "COMB 0x{:02X} should result in 0x{:02X}", input, expected);
        assert_eq!(emulator.cpu().condition_codes().carry(), *sets_carry, 
                   "COMB always sets carry flag");
        assert_eq!(emulator.cpu().condition_codes().negative(), *exp_negative,
                   "COMB 0x{:02X} negative flag should be {}", input, exp_negative);
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, 
                   "COMB always clears overflow flag");
        
        let exp_zero = *expected == 0x00;
        assert_eq!(emulator.cpu().condition_codes().zero(), exp_zero,
                   "COMB 0x{:02X} zero flag should be {}", input, exp_zero);
    }
}

#[test]
fn test_comb_sequential_alternating_patterns_0x53() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir secuencia de COMB
    for i in 0..5 {
        memory.borrow_mut().write(pc_start + i, 0x53).unwrap();
    }
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Secuencia que alterna entre patrones
    let sequence = [0x12, 0xED, 0x34, 0xCB, 0x56]; // Cada uno es complemento del anterior
    let expected = [0xED, 0x12, 0xCB, 0x34, 0xA9];
    
    for (i, (initial, expected_result)) in sequence.iter().zip(expected.iter()).enumerate() {
        emulator.cpu_mut().set_register_b(*initial);
        
        let cycles = emulator.step().unwrap();
        
        assert_eq!(emulator.cpu().register_b(), *expected_result, 
                   "COMB step {} should complement 0x{:02X} to 0x{:02X}", i + 1, initial, expected_result);
        assert_eq!(cycles, 2, "Each COMB should take 2 cycles");
        assert_eq!(emulator.cpu().condition_codes().carry(), true, "Each COMB should set Carry flag");
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, "Each COMB should clear Overflow flag");
    }
}