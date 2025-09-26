// Test for STA (Store A register) opcode 0x97 (direct mode)
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
fn test_sta_direct_basic_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x42);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8); // DP = 0xC8 para direccionamiento directo
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0x50; // Offset dentro de la página direct
    let target_address = (emulator.cpu().register_dp() as u16) << 8 | target_offset as u16;
    
    // STA $50 (0x97 0x50) - direccionamiento directo
    memory.borrow_mut().write(pc_start, 0x97).unwrap();        // STA direct
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap(); // offset directo
    
    // Ejecutar instrucción STA
    let cycles = emulator.step().unwrap();
    
    // Verificar que el valor de A se escribió en memoria
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x42, 
               "STA should store A (0x42) at address 0x{:04X}", target_address);
    
    // Verificar que A no cambió
    assert_eq!(emulator.cpu().register_a(), 0x42, "STA should not modify register A");
    
    // Verificar condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "STA with 0x42 should not set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "STA with 0x42 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "STA always clears Overflow flag");
    
    // Verificar cycles (STA direct toma 4 cycles según MC6809)
    assert_eq!(cycles, 4, "STA direct should take 4 cycles");
    
    // Verificar que PC avanzó 2 bytes
    assert_eq!(emulator.cpu().program_counter(), pc_start + 2, "STA direct should advance PC by 2");
}

#[test]
fn test_sta_direct_zero_flag_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - A con valor cero
    emulator.cpu_mut().set_register_a(0x00);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0x60;
    let target_address = (emulator.cpu().register_dp() as u16) << 8 | target_offset as u16;
    
    // STA $60 (0x97 0x60)
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x00, 
               "STA should store 0x00 at target address");
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "STA with A=0x00 should set Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), false, "STA with A=0x00 should not set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "STA always clears Overflow flag");
}

#[test]
fn test_sta_direct_negative_flag_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - A con valor negativo
    emulator.cpu_mut().set_register_a(0x80);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0x70;
    let target_address = (emulator.cpu().register_dp() as u16) << 8 | target_offset as u16;
    
    // STA $70 (0x97 0x70)
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x80, 
               "STA should store 0x80 at target address");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "STA with A=0x80 should set Negative flag");
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "STA with A=0x80 should not set Zero flag");
}

#[test]
fn test_sta_direct_overflow_always_clear_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Overflow flag inicialmente
    emulator.cpu_mut().set_register_a(0x55);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0x80;
    
    // STA $80 (0x97 0x80) - STA siempre limpia V flag
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().condition_codes().overflow(), false, "STA should always clear Overflow flag");
}

#[test]
fn test_sta_direct_carry_preservation_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Carry flag
    emulator.cpu_mut().set_register_a(0xAA);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0x90;
    
    // STA $90 (0x97 0x90) - STA NO afecta Carry flag
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "STA should not modify Carry flag");
}

#[test]
fn test_sta_direct_different_dp_values_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Test con diferentes valores de DP
    let test_cases = [
        (RAM_START >> 8, 0x10), // DP = 0xC8, offset = 0x10 -> 0xC810
        (RAM_START >> 8, 0x20), // DP = 0xC8, offset = 0x20 -> 0xC820
        (RAM_START >> 8, 0xFF), // DP = 0xC8, offset = 0xFF -> 0xC8FF
    ];
    
    let pc_start = RAM_START + 0x100;
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    
    for (dp_value, offset) in test_cases.iter() {
        let test_value = 0x30 + offset; // Valor único para cada test
        emulator.cpu_mut().set_register_a(test_value);
        emulator.cpu_mut().set_register_dp(*dp_value);
        emulator.cpu_mut().set_program_counter(pc_start);
        
        let expected_address = (*dp_value as u16) << 8 | (*offset as u16);
        memory.borrow_mut().write(pc_start + 1, *offset).unwrap();
        
        emulator.step().unwrap();
        
        assert_eq!(memory.borrow().read(expected_address).unwrap(), test_value, 
                   "STA should store 0x{:02X} at address 0x{:04X} (DP=0x{:02X}, offset=0x{:02X})", 
                   test_value, expected_address, dp_value, offset);
    }
}

#[test]
fn test_sta_direct_preserve_other_registers_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer todos los registros
    emulator.cpu_mut().set_register_a(0x11);
    emulator.cpu_mut().set_register_b(0x22);
    emulator.cpu_mut().set_register_x(0x3344);
    emulator.cpu_mut().set_register_y(0x5566);
    emulator.cpu_mut().set_register_u(0x7788);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    
    let initial_a = emulator.cpu().register_a();
    let initial_b = emulator.cpu().register_b();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // STA $A0
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0xA0).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar que todos los registros permanecen iguales
    assert_eq!(emulator.cpu().register_a(), initial_a, "STA should not modify register A");
    assert_eq!(emulator.cpu().register_b(), initial_b, "STA should not modify register B");
    assert_eq!(emulator.cpu().register_x(), initial_x, "STA should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "STA should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "STA should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "STA should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "STA should not modify register DP");
}

#[test]
fn test_sta_direct_overwrite_memory_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    let target_offset = 0xB0;
    let target_address = (emulator.cpu().register_dp() as u16) << 8 | target_offset as u16;
    
    // Escribir valor inicial en memoria
    memory.borrow_mut().write(target_address, 0xFF).unwrap();
    
    // Verificar valor inicial
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0xFF, "Initial memory value should be 0xFF");
    
    // Preparar STA para sobrescribir
    emulator.cpu_mut().set_register_a(0x33);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // STA $B0
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar sobrescritura
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x33, 
               "STA should overwrite memory location with A value (0x33)");
    assert_ne!(memory.borrow().read(target_address).unwrap(), 0xFF, 
               "Memory should no longer contain initial value");
}

#[test]
fn test_sta_direct_sequential_stores_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    
    let pc_start = RAM_START + 0x100;
    
    // Escribir secuencia: diferentes valores de A y diferentes direcciones
    let test_sequence = [
        (0x10, 0x01), // A=0x10, store at offset 0x01
        (0x20, 0x02), // A=0x20, store at offset 0x02
        (0x30, 0x03), // A=0x30, store at offset 0x03
        (0x00, 0x04), // A=0x00, store at offset 0x04 (test zero)
        (0xFF, 0x05), // A=0xFF, store at offset 0x05 (test negative)
    ];
    
    for (i, (a_val, offset)) in test_sequence.iter().enumerate() {
        let instruction_address = pc_start + (i * 2) as u16;
        memory.borrow_mut().write(instruction_address, 0x97).unwrap();        // STA direct
        memory.borrow_mut().write(instruction_address + 1, *offset).unwrap(); // offset
    }
    
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Ejecutar secuencia
    for (i, (a_val, offset)) in test_sequence.iter().enumerate() {
        emulator.cpu_mut().set_register_a(*a_val);
        
        let cycles = emulator.step().unwrap();
        
        let target_address = (emulator.cpu().register_dp() as u16) << 8 | (*offset as u16);
        assert_eq!(memory.borrow().read(target_address).unwrap(), *a_val, 
                   "STA step {} should store 0x{:02X} at address 0x{:04X}", i + 1, a_val, target_address);
        assert_eq!(cycles, 4, "Each STA should take 4 cycles");
        assert_eq!(emulator.cpu().condition_codes().overflow(), false, 
                   "Each STA should clear overflow flag");
        
        // Verificar flags específicos
        match a_val {
            0x00 => assert_eq!(emulator.cpu().condition_codes().zero(), true, "STA with A=0x00 should set Zero flag"),
            0xFF => assert_eq!(emulator.cpu().condition_codes().negative(), true, "STA with A=0xFF should set Negative flag"),
            _ => {
                let exp_neg = *a_val & 0x80 != 0;
                assert_eq!(emulator.cpu().condition_codes().negative(), exp_neg, 
                           "STA with A=0x{:02X} negative flag should be {}", a_val, exp_neg);
                assert_eq!(emulator.cpu().condition_codes().zero(), false, "STA with A non-zero should clear Zero flag");
            }
        }
    }
}

#[test]
fn test_sta_direct_read_back_verify_0x97() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x96);
    emulator.cpu_mut().set_register_dp(RAM_START >> 8);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let target_offset = 0xC0;
    let target_address = (emulator.cpu().register_dp() as u16) << 8 | target_offset as u16;
    
    // STA $C0
    memory.borrow_mut().write(pc_start, 0x97).unwrap();
    memory.borrow_mut().write(pc_start + 1, target_offset).unwrap();
    
    emulator.step().unwrap();
    
    // Verificar escritura
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x96, 
               "Memory should contain stored value");
    
    // Cambiar A y verificar que la memoria no se afecta
    emulator.cpu_mut().set_register_a(0x69);
    assert_eq!(memory.borrow().read(target_address).unwrap(), 0x96, 
               "Memory should retain stored value even after A changes");
    assert_ne!(emulator.cpu().register_a(), memory.borrow().read(target_address).unwrap(), 
               "Register A and stored memory value should be independent");
}