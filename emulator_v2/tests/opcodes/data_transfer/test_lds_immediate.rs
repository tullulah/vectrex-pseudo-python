// Test for LDS immediate opcode (0x10CE)
// C++ Original: case 0xCE: OpLD<1, 0xCE>(S); - LDS immediate

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
fn test_lds_immediate_0x10ce() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - S register con valor conocido
    let original_s = cpu.registers.s;
    let new_stack_value = 0xD000u16;
    
    // Escribir opcode Page 1 (0x10) y LDS immediate (0xCE) en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xCE); // LDS immediate opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (new_stack_value >> 8) as u8); // Immediate value high
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, (new_stack_value & 0xFF) as u8); // Immediate value low
    
    // Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que S cambió al nuevo valor
    assert_eq!(cpu.registers.s, new_stack_value, "S register should contain new stack pointer value");
    assert_ne!(cpu.registers.s, original_s, "S register should have changed from original value");
    
    // Verificar flags - LDS debe actualizar N y Z
    assert_eq!(cpu.registers.cc.z, false, "Zero flag should be clear for non-zero value");
    assert_eq!(cpu.registers.cc.n, true, "Negative flag should be set for negative value (bit 15 = 1)");
    
    // Verificar que PC avanzó correctamente (4 bytes: 0x10 + 0xCE + value_high + value_low)
    assert_eq!(cpu.registers.pc, RAM_START + 0x104, "PC should advance by 4 bytes");
    
    // Verificar cycles (4 según tabla de timing MC6809 para immediate 16-bit)
    assert_eq!(cycles, 4, "LDS immediate should take 4 cycles");
}

