//! Test for STY direct (0x109F) - Store Y register direct
//! C++ Original: OpST<1, 0x9F>(Y);

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
fn test_sty_direct_0x109f() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - Y register con valor a almacenar
    let test_value: u16 = 0xABCD;
    cpu.registers.y = test_value;
    
    // Configurar DP register para apuntar al área de RAM 
    cpu.registers.dp = 0xC8; // DP = 0xC8 para que DP*256 = 0xC800 (RAM_START)
    
    let direct_addr: u8 = 0x90; // Dirección directa donde almacenar
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x9F); // STY direct opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, direct_addr); // Direct address
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar que el valor se almacenó correctamente en memoria
    let target_addr = (cpu.registers.dp as u16) << 8 | direct_addr as u16;
    let stored_high = cpu.memory_bus().borrow().read(target_addr);
    let stored_low = cpu.memory_bus().borrow().read(target_addr + 1);
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, test_value);
    
    // 5. Verificar flags - STY debe actualizar N y Z 
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, true); // Negative flag for 0xABCD (bit 15 = 1)
    assert_eq!(cpu.registers.cc.v, false);
    
    // 6. Verificar PC y cycles
    assert_eq!(cpu.registers.pc, RAM_START + 0x103); // 3 bytes total
    assert_eq!(cycles, 6); // STY direct should take 6 cycles according to table
}