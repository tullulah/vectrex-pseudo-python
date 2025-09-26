// Test for INCA (Increment A) opcode 0x4C  
// Port directo desde Vectrexy siguiendo copilot-instructions.md

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

// CONFIGURACIÓN OBLIGATORIA según copilot-instructions.md
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers.sp = STACK_START; // Stack pointer al final de RAM
    cpu
}

#[test]
fn test_inca_basic_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0x42; // Valor inicial en A
    
    // Escribir opcode INCA
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C); // INCA opcode
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar resultados
    assert_eq!(cpu.registers.a, 0x43); // A debe incrementarse
    assert_eq!(cpu.registers.pc, RAM_START + 0x101); // PC avanza 1 byte
    assert_eq!(cycles, 2); // INCA toma 2 cycles
    
    // Verificar flags - ningún flag especial debería estar activo
    assert!(!cpu.condition_codes.zero);
    assert!(!cpu.condition_codes.negative);
    assert!(!cpu.condition_codes.overflow);
}

#[test]
fn test_inca_zero_flag_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - A = 0xFF para que incremente a 0x00
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0xFF;
    
    // Escribir opcode INCA
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C);
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar resultados
    assert_eq!(cpu.registers.a, 0x00); // A debe ser 0 después del overflow
    assert_eq!(cycles, 2);
    
    // Verificar flags
    assert!(cpu.condition_codes.zero);    // Zero flag debe estar activo
    assert!(!cpu.condition_codes.negative); // No negativo
    // Carry no se afecta por INC según MC6809
}

#[test]
fn test_inca_negative_flag_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - A = 0x7E para que incremente a 0x7F (positivo)
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0x7E;
    
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C);
    let cycles1 = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0x7F);
    assert!(!cpu.condition_codes.negative); // 0x7F es positivo
    assert_eq!(cycles1, 2);
    
    // Ahora incrementar de 0x7F a 0x80 (negativo)
    cpu.registers.pc = RAM_START + 0x100; // Reset PC
    let cycles2 = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers.a, 0x80);
    assert!(cpu.condition_codes.negative); // 0x80 es negativo en complemento a 2
    assert_eq!(cycles2, 2);
}

#[test]
fn test_inca_overflow_flag_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - A = 0x7F (máximo positivo)
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0x7F;
    
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C);
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar overflow de positivo (0x7F) a negativo (0x80)
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cycles, 2);
    
    // Verificar flags
    assert!(cpu.condition_codes.overflow); // Overflow de +127 a -128
    assert!(cpu.condition_codes.negative); // Resultado es negativo
    assert!(!cpu.condition_codes.zero);    // No es cero
}

#[test] 
fn test_inca_preserves_other_registers_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - configurar todos los registros
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0x33;
    cpu.registers.b = 0x44;
    cpu.registers.x = 0x1234;
    cpu.registers.y = 0x5678;
    cpu.registers.dp = 0x12;
    
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C);
    
    // Ejecutar INCA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que solo A cambió
    assert_eq!(cpu.registers.a, 0x34); // Solo A debe cambiar
    assert_eq!(cpu.registers.b, 0x44); // B sin cambios
    assert_eq!(cpu.registers.x, 0x1234); // X sin cambios  
    assert_eq!(cpu.registers.y, 0x5678); // Y sin cambios
    assert_eq!(cpu.registers.dp, 0x12);  // DP sin cambios
    assert_eq!(cycles, 2);
}

#[test]
fn test_inca_multiple_increments_0x4c() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = RAM_START + 0x100;
    cpu.registers.a = 0xFD; // -3 en complemento a 2
    
    // Escribir múltiples INCA consecutivos
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x4C); // INCA 1
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x4C); // INCA 2  
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x4C); // INCA 3
    
    // Ejecutar primer INCA: 0xFD -> 0xFE
    let cycles1 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.a, 0xFE);
    assert!(cpu.condition_codes.negative);
    assert!(!cpu.condition_codes.zero);
    assert_eq!(cycles1, 2);
    
    // Ejecutar segundo INCA: 0xFE -> 0xFF
    let cycles2 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.a, 0xFF);
    assert!(cpu.condition_codes.negative);
    assert!(!cpu.condition_codes.zero);
    assert_eq!(cycles2, 2);
    
    // Ejecutar tercer INCA: 0xFF -> 0x00 (wrap around)
    let cycles3 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.a, 0x00);
    assert!(!cpu.condition_codes.negative);
    assert!(cpu.condition_codes.zero);
    assert_eq!(cycles3, 2);
}