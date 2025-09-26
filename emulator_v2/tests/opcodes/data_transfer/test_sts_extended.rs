//! Test for STS extended (0xFF) - Store Stack pointer extended
//! C++ Original: OpST<0xFF>(S);

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

// CONFIGURACIÓN OBLIGATORIA en todos los tests de opcodes:
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
fn test_sts_extended_0xff() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor conocido
    cpu.registers.s = 0x1234;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0xFF); // STS extended opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, (target_addr >> 8) as u8);     // High byte de dirección
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr & 0xFF) as u8);  // Low byte de dirección
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - registros, flags, memoria, cycles
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x12);
    assert_eq!(stored_value_low, 0x34);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x103); // 3 bytes total
    assert_eq!(cycles, 6); // STS extended should take 6 cycles
}

#[test]
fn test_sts_extended_0xff_zero_value() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor 0
    cpu.registers.s = 0x0000;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0xFF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr & 0xFF) as u8);
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x00);
    assert_eq!(stored_value_low, 0x00);
    assert_eq!(cpu.registers.cc.z, true);  // Z flag debe estar en 1 cuando el resultado es 0
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x103);
    assert_eq!(cycles, 6);
}

#[test]
fn test_sts_extended_0xff_negative_value() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor negativo (bit 15 = 1)
    cpu.registers.s = 0x8000;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0xFF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr & 0xFF) as u8);
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x80);
    assert_eq!(stored_value_low, 0x00);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, true);  // N flag debe estar en 1 cuando bit 15 = 1
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x103);
    assert_eq!(cycles, 6);
}