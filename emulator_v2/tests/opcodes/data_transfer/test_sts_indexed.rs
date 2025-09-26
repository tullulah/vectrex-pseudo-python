//! Test for STS indexed (0xEF) - Store S stack pointer indexed
//! C++ Original: OpST<0, 0xEF>(S);

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
fn test_sts_indexed_0xef() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor conocido y X como índice
    cpu.registers.s = 0x3456;
    cpu.registers.x = RAM_START + 0x0020;  // Offset para modo indexed dentro de RAM
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0xEF); // STS indexed opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x84); // ,X indexed mode (no offset)
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar que S se almacenó correctamente en memoria[X]
    let target_addr = cpu.registers.x;
    let stored_value_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_value_low = cpu.memory_bus().borrow().read(target_addr + 1);
    let stored_value = ((stored_value_high as u16) << 8) | (stored_value_low as u16);
    
    assert_eq!(stored_value, 0x3456);
    assert_eq!(cpu.registers.pc, RAM_START + 0x102); // 2 bytes total
    assert_eq!(cycles, 6); // STS indexed should take 6 cycles
}