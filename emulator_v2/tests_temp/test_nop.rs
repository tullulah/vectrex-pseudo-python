// Test for NOP (No Operation) opcode 0x12
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
fn test_nop_basic_0x12() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Capturar estado inicial de todos los registros
    let initial_a = emulator.cpu().register_a();
    let initial_b = emulator.cpu().register_b();
    let initial_x = emulator.cpu().register_x();
    let initial_y = emulator.cpu().register_y();
    let initial_u = emulator.cpu().register_u();
    let initial_s = emulator.cpu().register_s();
    let initial_dp = emulator.cpu().register_dp();
    let initial_cc = emulator.cpu().condition_codes().value();
    
    // NOP (0x12)
    memory.borrow_mut().write(pc_start, 0x12).unwrap();
    
    // Ejecutar instrucción NOP
    let cycles = emulator.step().unwrap();
    
    // Verificar que PC avanzó en 1 byte (tamaño de NOP)
    assert_eq!(emulator.cpu().program_counter(), pc_start + 1, "NOP should advance PC by 1");
    
    // Verificar que NOP NO modifica ningún registro
    assert_eq!(emulator.cpu().register_a(), initial_a, "NOP should not modify register A");
    assert_eq!(emulator.cpu().register_b(), initial_b, "NOP should not modify register B");
    assert_eq!(emulator.cpu().register_x(), initial_x, "NOP should not modify register X");
    assert_eq!(emulator.cpu().register_y(), initial_y, "NOP should not modify register Y");
    assert_eq!(emulator.cpu().register_u(), initial_u, "NOP should not modify register U");
    assert_eq!(emulator.cpu().register_s(), initial_s, "NOP should not modify register S");
    assert_eq!(emulator.cpu().register_dp(), initial_dp, "NOP should not modify register DP");
    
    // Verificar que NOP NO modifica condition codes
    assert_eq!(emulator.cpu().condition_codes().value(), initial_cc, "NOP should not modify condition codes");
    
    // Verificar cycles (NOP toma 2 cycles según MC6809)
    assert_eq!(cycles, 2, "NOP should take 2 cycles");
}

#[test]
fn test_nop_sequence_0x12() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - secuencia de múltiples NOPs
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Escribir secuencia de NOPs
    for i in 0..5 {
        memory.borrow_mut().write(pc_start + i, 0x12).unwrap();
    }
    
    let initial_pc = pc_start;
    let mut expected_pc = initial_pc;
    let mut total_cycles = 0;
    
    // Ejecutar 5 NOPs secuenciales
    for i in 0..5 {
        let cycles = emulator.step().unwrap();
        expected_pc += 1;
        total_cycles += cycles;
        
        assert_eq!(emulator.cpu().program_counter(), expected_pc, 
                   "NOP #{} should advance PC to expected position", i + 1);
        assert_eq!(cycles, 2, "Each NOP should take 2 cycles");
    }
    
    assert_eq!(total_cycles, 10, "5 NOPs should take total of 10 cycles");
    assert_eq!(emulator.cpu().program_counter(), initial_pc + 5, "PC should advance by 5 after 5 NOPs");
}

#[test]
fn test_nop_with_preset_conditions_0x12() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer varios registros y flags
    emulator.cpu_mut().set_register_a(0xAA);
    emulator.cpu_mut().set_register_b(0xBB);
    emulator.cpu_mut().set_register_x(0x1122);
    emulator.cpu_mut().set_register_y(0x3344);
    emulator.cpu_mut().set_register_u(0x5566);
    emulator.cpu_mut().set_register_dp(0x77);
    
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    emulator.cpu_mut().condition_codes_mut().set_negative(true);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // NOP (0x12)
    memory.borrow_mut().write(pc_start, 0x12).unwrap();
    
    // Ejecutar NOP
    let cycles = emulator.step().unwrap();
    
    // Verificar que todos los valores establecidos se mantienen sin cambios
    assert_eq!(emulator.cpu().register_a(), 0xAA, "NOP should preserve register A");
    assert_eq!(emulator.cpu().register_b(), 0xBB, "NOP should preserve register B");
    assert_eq!(emulator.cpu().register_x(), 0x1122, "NOP should preserve register X");
    assert_eq!(emulator.cpu().register_y(), 0x3344, "NOP should preserve register Y");
    assert_eq!(emulator.cpu().register_u(), 0x5566, "NOP should preserve register U");
    assert_eq!(emulator.cpu().register_dp(), 0x77, "NOP should preserve register DP");
    
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "NOP should preserve Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "NOP should preserve Negative flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "NOP should preserve Carry flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "NOP should preserve Overflow flag");
    
    assert_eq!(cycles, 2, "NOP should take 2 cycles");
}

#[test]
fn test_nop_memory_preservation_0x12() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - escribir datos en memoria
    let test_addresses = [RAM_START, RAM_START + 0x50, RAM_START + 0x100, RAM_START + 0x200];
    let test_values = [0x11, 0x22, 0x33, 0x44];
    
    for (addr, val) in test_addresses.iter().zip(test_values.iter()) {
        memory.borrow_mut().write(*addr, *val).unwrap();
    }
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start + 1); // Evitar sobrescribir el NOP
    
    // NOP en una ubicación que no interfiera con los datos de test
    memory.borrow_mut().write(pc_start + 1, 0x12).unwrap();
    
    // Ejecutar NOP
    emulator.step().unwrap();
    
    // Verificar que la memoria no se modificó
    for (addr, expected_val) in test_addresses.iter().zip(test_values.iter()) {
        let actual_val = memory.borrow().read(*addr).unwrap();
        assert_eq!(actual_val, *expected_val, 
                   "NOP should not modify memory at address 0x{:04X}", addr);
    }
}

#[test]
fn test_nop_timing_consistency_0x12() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // Escribir múltiples NOPs para test de timing
    for i in 0..10 {
        memory.borrow_mut().write(pc_start + i, 0x12).unwrap();
    }
    
    // Ejecutar 10 NOPs y verificar timing consistente
    for i in 0..10 {
        let cycles = emulator.step().unwrap();
        assert_eq!(cycles, 2, "NOP #{} should consistently take 2 cycles", i + 1);
    }
}