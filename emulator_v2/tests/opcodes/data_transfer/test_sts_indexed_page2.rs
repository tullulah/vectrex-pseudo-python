//! Test for STS indexed Page 2 (0x10EF) - Store Stack pointer indexed with Page 2 prefix
//! C++ Original: OpST<2, 0xEF>(S);

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
fn test_sts_indexed_page2_0x10ef_no_offset() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register y X register con valores conocidos
    cpu.registers.s = 0x1234;
    cpu.registers.x = RAM_START + 0x200; // X apunta a zona de RAM
    
    let target_addr = cpu.registers.x; // Sin offset
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);  // Page 2 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xEF);  // STS indexed opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x84); // ,X indexed mode (no offset)
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - S debe almacenarse en dirección indexada
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x12);
    assert_eq!(stored_value_low, 0x34);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x103); // 3 bytes total
    assert_eq!(cycles, 6); // STS indexed Page 2 should take 6 cycles
}

#[test]
fn test_sts_indexed_page2_0x10ef_8bit_offset() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register y Y register con valores conocidos
    cpu.registers.s = 0x5678;
    cpu.registers.y = RAM_START + 0x100; // Y como base
    
    let offset = 8i8; // Offset de 8-bit 
    let target_addr = (cpu.registers.y as i32 + offset as i32) as u16;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);  // Page 2 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xEF);  // STS indexed opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0xA8); // 8-bit offset mode con Y
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x08); // Offset +8
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - S debe almacenarse en Y+8
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x56);
    assert_eq!(stored_value_low, 0x78);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x104); // 4 bytes total
    assert_eq!(cycles, 7); // STS indexed Page 2 con 8-bit offset should take 7 cycles
}

#[test]
fn test_sts_indexed_page2_0x10ef_16bit_offset() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register y X register con valores conocidos
    cpu.registers.s = 0x9ABC;
    cpu.registers.x = RAM_START; // X como base
    
    let offset = 0x0050i16; // Offset de 16-bit
    let target_addr = (cpu.registers.x as i32 + offset as i32) as u16;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);  // Page 2 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xEF);  // STS indexed opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x89); // 16-bit offset mode con X
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x00); // High byte offset
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x104, 0x50); // Low byte offset
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - S debe almacenarse en X+0x0050
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    assert_eq!(stored_value_high, 0x9A);
    assert_eq!(stored_value_low, 0xBC);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, true);  // 0x9ABC es negativo (bit 15 = 1)
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x105); // 5 bytes total
    assert_eq!(cycles, 8); // STS indexed Page 2 con 16-bit offset should take 8 cycles
}