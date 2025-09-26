//! Test for LDY indexed (0x10AE) - Load Y register indexed
//! C++ Original: OpLD<1, 0xAE>(Y);

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
fn test_ldy_indexed_0x10ae() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - Y register con valor conocido y configurar X register
    cpu.registers.y = 0x0000;
    cpu.registers.x = RAM_START + 0x200; // Índice base en área de RAM válida
    
    let test_value: u16 = 0x1234; // Valor simple para debugging
    let target_addr = cpu.registers.x; // Sin offset para simplificar
    
    // Escribir el valor en la dirección indexada
    cpu.memory_bus().borrow_mut().write(target_addr, (test_value >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(target_addr + 1, (test_value & 0xFF) as u8);
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xAE); // LDY indexed opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x84); // Indexed mode: ,X (basic indexing sin offset)
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - registros, flags, memoria, cycles
    assert_eq!(cpu.registers.y, test_value);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, false); // Negative flag for 0x1234 (bit 15 = 0)
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x103); // 3 bytes total
    assert_eq!(cycles, 6); // LDY indexed actually takes 6 cycles for basic ,X mode
}