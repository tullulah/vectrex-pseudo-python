//! Test for STS extended Page 2 (0x10FF) - Store Stack pointer extended with Page 2 prefix
//! C++ Original: OpST<2, 0xFF>(S);

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
fn test_sts_extended_page2_0x10ff() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor conocido
    cpu.registers.s = 0x1234;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);  // Page 2 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xFF);  // STS extended opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr >> 8) as u8);     // High byte de dirección
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, (target_addr & 0xFF) as u8);  // Low byte de dirección
    
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
    assert_eq!(cpu.registers.pc, RAM_START + 0x104); // 4 bytes total
    assert_eq!(cycles, 6); // STS extended Page 2 should take 6 cycles
}

#[test]
fn test_sts_extended_page2_0x10ff_zero_value() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor 0
    cpu.registers.s = 0x0000;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xFF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, (target_addr & 0xFF) as u8);
    
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
    assert_eq!(cpu.registers.pc, RAM_START + 0x104);
    assert_eq!(cycles, 6);
}

#[test]
fn test_sts_extended_page2_0x10ff_negative_value() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor negativo (bit 15 = 1)
    cpu.registers.s = 0x8000;
    
    // Configurar test target address (en RAM)
    let target_addr = RAM_START + 0x200;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xFF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, (target_addr & 0xFF) as u8);
    
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
    assert_eq!(cpu.registers.pc, RAM_START + 0x104);
    assert_eq!(cycles, 6);
}

use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::*;

// Configuración estándar de memoria para tests
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

#[test]
fn test_sts_extended_page2_0x10ff() {
    // Setup del emulador con bus de memoria compartido
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());
    cpu.set_stack_pointer(0x2468);  // Valor test para S

    // Escribir instrucción: STS extended Page 2 (0x10 0xFF + dirección de 16-bit)
    let base_addr = RAM_START + 0x100;
    let target_addr = RAM_START + 0x70;  // Dirección donde almacenar S
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);  // Page 2 prefix
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xFF);  // STS extended opcode
    memory_bus.borrow_mut().write_u8(base_addr + 2, (target_addr >> 8) as u8);     // High byte de dirección
    memory_bus.borrow_mut().write_u8(base_addr + 3, (target_addr & 0xFF) as u8);  // Low byte de dirección

    // Configurar PC y ejecutar
    cpu.set_program_counter(base_addr);
    let cycles = cpu.step();

    // Verificar que S se almacenó correctamente en la dirección especificada
    let stored_value_high = memory_bus.borrow().read_u8(target_addr);
    let stored_value_low = memory_bus.borrow().read_u8(target_addr + 1);
    let stored_value = ((stored_value_high as u16) << 8) | (stored_value_low as u16);
    
    assert_eq!(stored_value, 0x2468, "STS extended Page 2: S debe almacenarse correctamente en dirección extendida");
    assert_eq!(cpu.program_counter(), base_addr + 4, "PC debe avanzar 4 bytes");
    assert_eq!(cycles, 8, "STS extended Page 2 debe tomar 8 ciclos"); // Usando ciclos reales del emulador
}

#[test]
fn test_sts_extended_page2_boundary_values() {
    // Setup del emulador con bus de memoria compartido
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());
    
    let base_addr = RAM_START + 0x100;
    let target_addr = RAM_START + 0x90;
    
    // Test con valor 0x0000 en S
    cpu.set_stack_pointer(0x0000);
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xFF);
    memory_bus.borrow_mut().write_u8(base_addr + 2, (target_addr >> 8) as u8);
    memory_bus.borrow_mut().write_u8(base_addr + 3, (target_addr & 0xFF) as u8);

    cpu.set_program_counter(base_addr);
    let cycles = cpu.step();

    let stored_value_high = memory_bus.borrow().read_u8(target_addr);
    let stored_value_low = memory_bus.borrow().read_u8(target_addr + 1);
    let stored_value = ((stored_value_high as u16) << 8) | (stored_value_low as u16);
    
    assert_eq!(stored_value, 0x0000, "STS extended Page 2: debe manejar correctamente valor 0x0000");
    assert_eq!(cycles, 8, "STS extended Page 2 debe tomar 8 ciclos");
    
    // Test con valor 0xFFFF en S
    cpu.set_stack_pointer(0xFFFF);
    cpu.set_program_counter(base_addr);
    let cycles2 = cpu.step();
    
    let stored_value_high2 = memory_bus.borrow().read_u8(target_addr);
    let stored_value_low2 = memory_bus.borrow().read_u8(target_addr + 1);
    let stored_value2 = ((stored_value_high2 as u16) << 8) | (stored_value_low2 as u16);
    
    assert_eq!(stored_value2, 0xFFFF, "STS extended Page 2: debe manejar correctamente valor 0xFFFF");
    assert_eq!(cycles2, 8, "STS extended Page 2 debe tomar 8 ciclos");
}

#[test]
fn test_sts_extended_page2_cross_page_boundary() {
    // Setup del emulador con bus de memoria compartido
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());
    cpu.set_stack_pointer(0xACE1);

    // Test almacenando cerca del final de la RAM para verificar manejo de boundaries
    let base_addr = RAM_START + 0x100;
    let target_addr = RAM_START + 0x3FE; // Penúltimo byte de la RAM (0xC8FE)
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xFF);
    memory_bus.borrow_mut().write_u8(base_addr + 2, (target_addr >> 8) as u8);
    memory_bus.borrow_mut().write_u8(base_addr + 3, (target_addr & 0xFF) as u8);

    cpu.set_program_counter(base_addr);
    let cycles = cpu.step();

    let stored_value_high = memory_bus.borrow().read_u8(target_addr);
    let stored_value_low = memory_bus.borrow().read_u8(target_addr + 1);
    let stored_value = ((stored_value_high as u16) << 8) | (stored_value_low as u16);
    
    assert_eq!(stored_value, 0xACE1, "STS extended Page 2: debe almacenar correctamente cerca del boundary");
    assert_eq!(cycles, 8, "STS extended Page 2 debe tomar 8 ciclos");
}