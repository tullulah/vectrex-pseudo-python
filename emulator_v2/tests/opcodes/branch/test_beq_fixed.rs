// Test for BEQ (Branch if Equal) opcode 0x27
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
fn test_beq_taken_0x27() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - configurar zero flag activo
    cpu.registers.pc = 0xC800;
    cpu.registers.cc.z = true; // Condición para que BEQ salte
    
    // Escribir BEQ +10
    cpu.memory_bus().borrow_mut().write(0xC800, 0x27); // BEQ opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x0A); // offset +10
    
    // Ejecutar instrucción BEQ
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que saltó: PC + 2 + 10 = 0xC80C
    let expected_pc = 0xC800 + 2 + 10;
    assert_eq!(cpu.registers.pc, expected_pc, "BEQ should branch when zero flag is set");
    assert_eq!(cycles, 3, "BEQ taken should take 3 cycles");
}

#[test]
fn test_beq_not_taken_0x27() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - zero flag inactivo
    cpu.registers.pc = 0xC800;
    cpu.registers.cc.z = false; // Condición para que BEQ NO salte
    
    // Escribir BEQ +10
    cpu.memory_bus().borrow_mut().write(0xC800, 0x27); // BEQ opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0x0A); // offset +10
    
    // Ejecutar instrucción BEQ
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que NO saltó: PC + 2 = 0xC802
    let expected_pc = 0xC800 + 2;
    assert_eq!(cpu.registers.pc, expected_pc, "BEQ should not branch when zero flag is clear");
    // Nota: BEQ always takes 3 cycles in this implementation (consistent with branch opcodes)
    assert_eq!(cycles, 3, "BEQ should take 3 cycles");
}

#[test]
fn test_beq_boundary_conditions_0x27() {
    let mut cpu = create_test_cpu();
    
    // Test salto máximo positivo con zero flag set
    cpu.registers.pc = 0xC800;
    cpu.registers.cc.z = true;
    
    cpu.memory_bus().borrow_mut().write(0xC800, 0x27); // BEQ
    cpu.memory_bus().borrow_mut().write(0xC801, 0x7F); // +127
    
    let cycles = cpu.execute_instruction(false, false);
    
    let expected_pc = 0xC800 + 2 + 127;
    assert_eq!(cpu.registers.pc, expected_pc);
    assert_eq!(cycles, 3);
    
    // Test salto máximo negativo con zero flag set
    cpu.registers.pc = 0xC900;
    cpu.registers.cc.z = true;
    
    cpu.memory_bus().borrow_mut().write(0xC900, 0x27); // BEQ
    cpu.memory_bus().borrow_mut().write(0xC901, 0x80); // -128
    
    let cycles2 = cpu.execute_instruction(false, false);
    
    let expected_pc2 = 0xC900 + 2 - 128;
    assert_eq!(cpu.registers.pc, expected_pc2);
    assert_eq!(cycles2, 3);
}