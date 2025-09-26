// Test for BRA (Branch Always) opcode 0x20
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
    cpu.registers.s = STACK_START; // Stack pointer al final de RAM
    cpu
}

#[test]
fn test_bra_forward_0x20() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - PC en RAM
    cpu.registers.pc = RAM_START + 0x100;
    
    // Escribir opcode BRA y desplazamiento +10
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x20); // BRA opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x0A); // offset +10
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar resultados - BRA debe saltar 10 bytes adelante desde PC después de fetch
    let expected_pc = RAM_START + 0x102 + 0x0A; // PC + 2 (tamaño instrucción) + offset
    assert_eq!(cpu.registers.pc, expected_pc);
    assert_eq!(cycles, 3); // BRA toma 3 cycles
}

#[test]
fn test_bra_backward_0x20() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - PC en RAM  
    cpu.registers.pc = RAM_START + 0x200;
    
    // Escribir opcode BRA y desplazamiento -50 (0xCE en complemento a 2)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x200, 0x20); // BRA opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x201, 0xCE); // offset -50
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar resultados - BRA debe saltar 50 bytes atrás
    let expected_pc = RAM_START + 0x202 - 50; // PC + 2 - 50
    assert_eq!(cpu.registers.pc, expected_pc);
    assert_eq!(cycles, 3); // BRA toma 3 cycles
}

#[test]  
fn test_bra_zero_offset_0x20() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = RAM_START + 0x150;
    
    // Escribir opcode BRA con offset 0 (loop infinito)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x150, 0x20); // BRA opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x151, 0x00); // offset 0
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar - PC debe quedar en la misma posición formando loop
    let expected_pc = RAM_START + 0x152; // PC + 2 + 0 = siguiente instrucción
    assert_eq!(cpu.registers.pc, expected_pc);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bra_boundary_conditions_0x20() {
    let mut cpu = create_test_cpu();
    
    // Test salto máximo positivo (+127)
    cpu.registers.pc = RAM_START;
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x20);     // BRA
    cpu.memory_bus().borrow_mut().write(RAM_START + 1, 0x7F); // +127
    
    let cycles = cpu.execute_instruction(false, false);
    
    let expected_pc = RAM_START + 2 + 127;
    assert_eq!(cpu.registers.pc, expected_pc);
    assert_eq!(cycles, 3);
    
    // Test salto máximo negativo (-128)
    cpu.registers.pc = RAM_START + 0x300;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x300, 0x20);     // BRA  
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x301, 0x80);     // -128
    
    let cycles2 = cpu.execute_instruction(false, false);
    
    let expected_pc2 = RAM_START + 0x302 - 128;
    assert_eq!(cpu.registers.pc, expected_pc2);
    assert_eq!(cycles2, 3);
}

#[test]
fn test_bra_backward_jump_0x20() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - PC más adelante para poder saltar hacia atrás
    cpu.registers.pc = 0xC900;
    
    // Escribir BRA -50 (0xCE en complemento a 2)
    cpu.memory_bus().borrow_mut().write(0xC900, 0x20); // BRA opcode
    cpu.memory_bus().borrow_mut().write(0xC901, 0xCE); // offset -50
    
    // Ejecutar instrucción BRA
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar salto hacia atrás: PC + 2 - 50 = 0xC902 - 50 = 0xC8D0
    let expected_pc = 0xC900 + 2 - 50;
    assert_eq!(cpu.registers.pc, expected_pc, "BRA should jump backward");
    assert_eq!(cycles, 3, "BRA should take 3 cycles");
}