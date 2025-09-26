// Test for BEQ (Branch if Equal) opcode 0x27
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
fn test_beq_taken_zero_flag_set_0x27() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Zero flag para que BEQ sea tomado
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BEQ +10 (0x27 0x0A)
    memory.borrow_mut().write(pc_start, 0x27).unwrap();      // BEQ opcode
    memory.borrow_mut().write(pc_start + 1, 0x0A).unwrap();  // offset +10
    
    // Ejecutar instrucción BEQ
    let cycles = emulator.step().unwrap();
    
    // Verificar que el salto fue tomado
    let expected_pc = pc_start + 2 + 10;
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BEQ should jump when Zero flag is set");
    
    // Verificar cycles (BEQ tomado toma 3 cycles)
    assert_eq!(cycles, 3, "BEQ taken should take 3 cycles");
}

#[test]
fn test_beq_not_taken_zero_flag_clear_0x27() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - limpiar Zero flag para que BEQ NO sea tomado
    emulator.cpu_mut().condition_codes_mut().set_zero(false);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BEQ +10 (0x27 0x0A)
    memory.borrow_mut().write(pc_start, 0x27).unwrap();      // BEQ opcode
    memory.borrow_mut().write(pc_start + 1, 0x0A).unwrap();  // offset +10
    
    // Ejecutar instrucción BEQ
    let cycles = emulator.step().unwrap();
    
    // Verificar que el salto NO fue tomado - PC debe apuntar a la siguiente instrucción
    let expected_pc = pc_start + 2;  // Solo avanza por el tamaño de la instrucción
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BEQ should not jump when Zero flag is clear");
    
    // Verificar cycles (BEQ no tomado toma 2 cycles)
    assert_eq!(cycles, 2, "BEQ not taken should take 2 cycles");
}

#[test]
fn test_beq_backward_jump_0x27() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Zero flag y salto hacia atrás
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BEQ -5 (0x27 0xFB, donde 0xFB = -5 en complemento a 2)
    memory.borrow_mut().write(pc_start, 0x27).unwrap();      // BEQ opcode
    memory.borrow_mut().write(pc_start + 1, 0xFB).unwrap();  // offset -5
    
    // Ejecutar instrucción BEQ
    let cycles = emulator.step().unwrap();
    
    // Verificar salto hacia atrás
    let expected_pc = pc_start + 2 - 5; // pc_start - 3
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BEQ should jump backward when Zero flag is set");
    
    // Verificar cycles
    assert_eq!(cycles, 3, "BEQ taken should take 3 cycles");
}

#[test] 
fn test_beq_condition_code_preservation_0x27() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer múltiples flags
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    emulator.cpu_mut().condition_codes_mut().set_negative(true);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BEQ +0 (0x27 0x00)
    memory.borrow_mut().write(pc_start, 0x27).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x00).unwrap();
    
    // Ejecutar BEQ
    emulator.step().unwrap();
    
    // Verificar que BEQ NO modifica condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), true, "BEQ should not modify Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "BEQ should not modify Negative flag");  
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "BEQ should not modify Carry flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "BEQ should not modify Overflow flag");
}

#[test]
fn test_beq_edge_case_zero_offset_0x27() {
    let (mut emulator, memory) = setup_emulator();
    
    // Test con Zero flag set y offset 0 (apunta a instrucción siguiente)
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BEQ +0 (0x27 0x00)
    memory.borrow_mut().write(pc_start, 0x27).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x00).unwrap();
    
    let cycles = emulator.step().unwrap();
    
    // PC debe apuntar a la instrucción inmediatamente después de BEQ
    let expected_pc = pc_start + 2 + 0;
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BEQ with zero offset should point to next instruction");
    assert_eq!(cycles, 3, "BEQ taken should take 3 cycles even with zero offset");
}