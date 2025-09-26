// Test for STY (Store Index Register Y) - Indexed mode
// Opcode: 0x10AF
// Page: 1 (two-byte opcode: 0x10 0xAF)
// Cycles: 6+
// Flags: N, Z

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
fn test_sty_indexed_0x10af() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial: Y = 0x1234
    cpu.registers.y = 0x1234;
    
    // Setup: X = dirección válida en RAM 
    cpu.registers.x = RAM_START + 0x50;
    
    // Escribir opcode en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xAF); // STY opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x84); // Post-byte: ,X (no offset)
    
    // Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que Y se almacenó en la dirección correcta (X)
    let stored_high = cpu.memory_bus().borrow().read(cpu.registers.x);
    let stored_low = cpu.memory_bus().borrow().read(cpu.registers.x + 1);
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    assert_eq!(stored_value, 0x1234);
    
    // Verificar flags: N=0 (valor positivo), Z=0 (valor no es cero)
    assert_eq!(cpu.registers.cc.z, false); // Z=0
    assert_eq!(cpu.registers.cc.n, false); // N=0
    
    // Verificar PC (debe avanzar 3 bytes: page + opcode + post-byte)
    assert_eq!(cpu.registers.pc, RAM_START + 0x103);
    
    // Verificar cycles (6 para STY indexed Page 1 - no es Page 2)
    assert_eq!(cycles, 6);
}