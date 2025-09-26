// Test for BNE (Branch if Not Equal) opcode 0x26  
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
fn test_bne_taken_zero_flag_clear_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - limpiar Zero flag para que BNE sea tomado
    emulator.cpu_mut().condition_codes_mut().set_zero(false);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BNE +15 (0x26 0x0F)
    memory.borrow_mut().write(pc_start, 0x26).unwrap();      // BNE opcode
    memory.borrow_mut().write(pc_start + 1, 0x0F).unwrap();  // offset +15
    
    // Ejecutar instrucción BNE
    let cycles = emulator.step().unwrap();
    
    // Verificar que el salto fue tomado
    let expected_pc = pc_start + 2 + 15;
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BNE should jump when Zero flag is clear");
    
    // Verificar cycles (BNE tomado toma 3 cycles)
    assert_eq!(cycles, 3, "BNE taken should take 3 cycles");
}

#[test]
fn test_bne_not_taken_zero_flag_set_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - establecer Zero flag para que BNE NO sea tomado
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BNE +15 (0x26 0x0F)
    memory.borrow_mut().write(pc_start, 0x26).unwrap();      // BNE opcode
    memory.borrow_mut().write(pc_start + 1, 0x0F).unwrap();  // offset +15
    
    // Ejecutar instrucción BNE
    let cycles = emulator.step().unwrap();
    
    // Verificar que el salto NO fue tomado
    let expected_pc = pc_start + 2;  // Solo avanza por el tamaño de la instrucción
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BNE should not jump when Zero flag is set");
    
    // Verificar cycles (BNE no tomado toma 2 cycles)
    assert_eq!(cycles, 2, "BNE not taken should take 2 cycles");
}

#[test]
fn test_bne_backward_jump_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - limpiar Zero flag y salto hacia atrás
    emulator.cpu_mut().condition_codes_mut().set_zero(false);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BNE -8 (0x26 0xF8, donde 0xF8 = -8 en complemento a 2)
    memory.borrow_mut().write(pc_start, 0x26).unwrap();      // BNE opcode
    memory.borrow_mut().write(pc_start + 1, 0xF8).unwrap();  // offset -8
    
    // Ejecutar instrucción BNE
    let cycles = emulator.step().unwrap();
    
    // Verificar salto hacia atrás
    let expected_pc = pc_start + 2 - 8; // pc_start - 6
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BNE should jump backward when Zero flag is clear");
    
    // Verificar cycles
    assert_eq!(cycles, 3, "BNE taken should take 3 cycles");
}

#[test]
fn test_bne_opposite_behavior_to_beq_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    let pc_start = RAM_START + 0x100;
    
    // Test 1: Zero flag = false -> BNE taken, BEQ not taken
    emulator.cpu_mut().condition_codes_mut().set_zero(false);
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BNE +5
    memory.borrow_mut().write(pc_start, 0x26).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x05).unwrap();
    
    let cycles1 = emulator.step().unwrap();
    let pc_after_bne = emulator.cpu().program_counter();
    
    assert_eq!(pc_after_bne, pc_start + 2 + 5, "BNE should be taken when Zero=false");
    assert_eq!(cycles1, 3, "BNE taken should take 3 cycles");
    
    // Test 2: Zero flag = true -> BNE not taken, BEQ taken
    emulator.cpu_mut().condition_codes_mut().set_zero(true);
    emulator.cpu_mut().set_program_counter(pc_start);
    
    let cycles2 = emulator.step().unwrap();
    let pc_after_bne_not_taken = emulator.cpu().program_counter();
    
    assert_eq!(pc_after_bne_not_taken, pc_start + 2, "BNE should not be taken when Zero=true");
    assert_eq!(cycles2, 2, "BNE not taken should take 2 cycles");
}

#[test]
fn test_bne_boundary_offsets_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup para saltos tomados
    emulator.cpu_mut().condition_codes_mut().set_zero(false);
    
    // Test offset máximo positivo (+127 = 0x7F)
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x26).unwrap();      // BNE opcode
    memory.borrow_mut().write(pc_start + 1, 0x7F).unwrap();  // offset +127
    
    let cycles = emulator.step().unwrap();
    
    let expected_pc = pc_start + 2 + 127;
    assert_eq!(emulator.cpu().program_counter(), expected_pc, "BNE should handle maximum positive offset");
    assert_eq!(cycles, 3, "BNE taken should take 3 cycles");
    
    // Test offset máximo negativo (-128 = 0x80)
    emulator.cpu_mut().set_program_counter(pc_start);
    
    memory.borrow_mut().write(pc_start, 0x26).unwrap();      // BNE opcode
    memory.borrow_mut().write(pc_start + 1, 0x80).unwrap();  // offset -128
    
    let cycles2 = emulator.step().unwrap();
    
    let expected_pc2 = pc_start + 2 - 128;
    assert_eq!(emulator.cpu().program_counter(), expected_pc2, "BNE should handle maximum negative offset");
    assert_eq!(cycles2, 3, "BNE taken should take 3 cycles");
}

#[test]
fn test_bne_condition_preservation_0x26() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial con múltiples flags establecidos
    emulator.cpu_mut().condition_codes_mut().set_zero(false);  // Para que BNE sea tomado
    emulator.cpu_mut().condition_codes_mut().set_negative(true);
    emulator.cpu_mut().condition_codes_mut().set_carry(true);
    emulator.cpu_mut().condition_codes_mut().set_overflow(true);
    
    let pc_start = RAM_START + 0x100;
    emulator.cpu_mut().set_program_counter(pc_start);
    
    // BNE +1 (0x26 0x01)
    memory.borrow_mut().write(pc_start, 0x26).unwrap();
    memory.borrow_mut().write(pc_start + 1, 0x01).unwrap();
    
    // Ejecutar BNE
    emulator.step().unwrap();
    
    // Verificar que BNE NO modifica condition codes
    assert_eq!(emulator.cpu().condition_codes().zero(), false, "BNE should not modify Zero flag");
    assert_eq!(emulator.cpu().condition_codes().negative(), true, "BNE should not modify Negative flag");
    assert_eq!(emulator.cpu().condition_codes().carry(), true, "BNE should not modify Carry flag");
    assert_eq!(emulator.cpu().condition_codes().overflow(), true, "BNE should not modify Overflow flag");
}