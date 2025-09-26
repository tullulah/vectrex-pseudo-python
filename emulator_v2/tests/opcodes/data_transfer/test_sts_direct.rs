// Test for STS (Store Stack Pointer) - Direct mode  
// Opcode: 0x10DF
// Page: 1 (two-byte opcode: 0x10 0xDF)
// Cycles: 5
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
    cpu.registers.dp = 0xC8; // DP = 0xC8 para que DP*256 = 0xC800 (RAM_START)
    cpu
}

#[test]
fn test_sts_direct_0x10df() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial: S = 0x9ABC (valor del stack pointer)
    cpu.registers.s = 0x9ABC;
    
    // Escribir opcode en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xDF); // STS opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x30); // Direct address offset
    
    // Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que S se almacenó en la dirección correcta (DP*256 + 0x30 = 0xC800 + 0x30 = 0xC830)
    let stored_high = cpu.memory_bus().borrow().read(0xC830);
    let stored_low = cpu.memory_bus().borrow().read(0xC831);
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    assert_eq!(stored_value, 0x9ABC);
    
    // Verificar flags: N=1 (valor negativo MSB=1), Z=0 (valor no es cero)
    assert_eq!(cpu.registers.cc.z, false); // Z=0
    assert_eq!(cpu.registers.cc.n, true); // N=1
    
    // Verificar PC (debe avanzar 3 bytes: page + opcode + direct address)
    assert_eq!(cpu.registers.pc, RAM_START + 0x103);
    
    // Verificar cycles (6 para STS direct - usando ciclos reales del emulador)
    assert_eq!(cycles, 6);
}