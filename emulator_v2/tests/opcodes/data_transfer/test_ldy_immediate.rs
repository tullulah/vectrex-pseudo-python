//! Test for LDY immediate (0x108E) - Load Y register immediate
//! C++ Original: OpLD<1, 0x8E>(Y);

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
fn test_ldy_immediate_0x108e() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - Y register con valor conocido
    cpu.registers.y = 0x0000;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x8E); // LDY immediate opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x12); // High byte
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x34); // Low byte
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - registros, flags, memoria, cycles
    assert_eq!(cpu.registers.y, 0x1234);
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x104); // 4 bytes total
    assert_eq!(cycles, 4); // LDY immediate should take 4 cycles according to table
}