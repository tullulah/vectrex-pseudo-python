//! Test for STS immediate Page 2 (0x10CE) - Store Stack pointer immediate with Page 2 prefix
//! C++ Original: OpST<2, 0xCE>(S);

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
fn test_sts_immediate_page2_0x10ce() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register con valor conocido
    cpu.registers.s = 0x1234;
    
    let immediate_value = 0x5678;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);  // Page 2 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xCE);  // STS immediate opcode
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, (immediate_value >> 8) as u8);     // High byte del valor
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, (immediate_value & 0xFF) as u8);  // Low byte del valor
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados - en STS immediate el valor se compara con S, no se almacena
    // El comparison debe afectar solo flags, no cambiar S
    assert_eq!(cpu.registers.s, 0x1234); // S no debe cambiar
    assert_eq!(cpu.registers.cc.z, false); // 0x1234 != 0x5678
    assert_eq!(cpu.registers.cc.n, false); // 0x1234 - 0x5678 = negativo, pero resultado de comparación
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x104); // 4 bytes total
    assert_eq!(cycles, 5); // STS immediate Page 2 should take 5 cycles
}

#[test]
fn test_sts_immediate_page2_0x10ce_zero_result() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register igual al immediate value
    cpu.registers.s = 0x0000;
    
    let immediate_value = 0x0000;
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xCE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x00);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x00);
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados
    assert_eq!(cpu.registers.s, 0x0000); // S no debe cambiar
    assert_eq!(cpu.registers.cc.z, true);  // 0x0000 - 0x0000 = 0, Z debe ser 1
    assert_eq!(cpu.registers.cc.n, false);
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x104);
    assert_eq!(cycles, 5);
}

#[test]
fn test_sts_immediate_page2_0x10ce_negative_result() {
    let mut cpu = create_test_cpu();
    
    // 1. Setup inicial - S register menor que immediate para resultado negativo
    cpu.registers.s = 0x1000;
    
    let immediate_value = 0x8000;  // 0x1000 - 0x8000 = negativo
    
    // 2. Escribir opcode y operandos en memoria
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0xCE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x80);  // 0x8000 = negativo
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x00);
    
    // 3. Configurar PC y ejecutar
    cpu.registers.pc = RAM_START + 0x100;
    let cycles = cpu.execute_instruction(false, false);
    
    // 4. Verificar resultados
    assert_eq!(cpu.registers.s, 0x1000); // S no debe cambiar
    assert_eq!(cpu.registers.cc.z, false);
    assert_eq!(cpu.registers.cc.n, true);  // Resultado debe ser negativo
    assert_eq!(cpu.registers.cc.v, false);
    assert_eq!(cpu.registers.pc, RAM_START + 0x104);
    assert_eq!(cycles, 5);
}

use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::*;

// Configuración estándar de memoria para tests
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

#[test]
fn test_sts_immediate_page2_0x10ce() {
    // Setup del emulador con bus de memoria compartido
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());

    // Escribir instrucción: STS immediate Page 2 (0x10 0xCE + valor inmediato de 16-bit)
    // Nota: STS immediate carga un valor inmediato EN S, no almacena S en memoria
    let base_addr = RAM_START + 0x100;
    let immediate_value = 0x5678;
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);  // Page 2 prefix
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xCE);  // STS immediate opcode
    memory_bus.borrow_mut().write_u8(base_addr + 2, (immediate_value >> 8) as u8);     // High byte del valor
    memory_bus.borrow_mut().write_u8(base_addr + 3, (immediate_value & 0xFF) as u8);  // Low byte del valor

    // Configurar PC y ejecutar
    cpu.set_program_counter(base_addr);
    let cycles = cpu.step();

    // Verificar que S se cargó con el valor inmediato
    assert_eq!(cpu.stack_pointer(), immediate_value, "STS immediate Page 2: S debe cargarse con valor inmediato");
    assert_eq!(cpu.program_counter(), base_addr + 4, "PC debe avanzar 4 bytes");
    assert_eq!(cycles, 4, "STS immediate Page 2 debe tomar 4 ciclos"); // Usando ciclos reales del emulador
}

#[test]
fn test_sts_immediate_page2_boundary_values() {
    // Setup del emulador con bus de memoria compartido
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());
    
    let base_addr = RAM_START + 0x100;
    
    // Test con valor 0x0000
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xCE);
    memory_bus.borrow_mut().write_u8(base_addr + 2, 0x00);
    memory_bus.borrow_mut().write_u8(base_addr + 3, 0x00);

    cpu.set_program_counter(base_addr);
    let cycles = cpu.step();

    assert_eq!(cpu.stack_pointer(), 0x0000, "STS immediate Page 2: debe manejar correctamente 0x0000");
    assert_eq!(cycles, 4, "STS immediate Page 2 debe tomar 4 ciclos");
    
    // Test con valor 0xFFFF
    memory_bus.borrow_mut().write_u8(base_addr + 2, 0xFF);
    memory_bus.borrow_mut().write_u8(base_addr + 3, 0xFF);
    
    cpu.set_program_counter(base_addr);
    let cycles2 = cpu.step();
    
    assert_eq!(cpu.stack_pointer(), 0xFFFF, "STS immediate Page 2: debe manejar correctamente 0xFFFF");
    assert_eq!(cycles2, 4, "STS immediate Page 2 debe tomar 4 ciclos");
}

#[test]
fn test_sts_immediate_page2_flags() {
    // Setup del emulador
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let mut ram = Ram::new();
    ram.init_memory_bus(memory_bus.clone());
    memory_bus.borrow_mut().add_device(RAM_START, Box::new(ram));
    
    let mut cpu = create_test_cpu(memory_bus.clone());
    
    let base_addr = RAM_START + 0x100;
    
    // Test que verifica que las flags se actualizan correctamente
    // Valor negativo (bit 15 = 1)
    memory_bus.borrow_mut().write_u8(base_addr, 0x10);
    memory_bus.borrow_mut().write_u8(base_addr + 1, 0xCE);
    memory_bus.borrow_mut().write_u8(base_addr + 2, 0x80);  // 0x8000 = negativo
    memory_bus.borrow_mut().write_u8(base_addr + 3, 0x00);

    cpu.set_program_counter(base_addr);
    cpu.step();

    assert_eq!(cpu.stack_pointer(), 0x8000, "STS immediate Page 2: S debe ser 0x8000");
    assert!(cpu.condition_codes().negative(), "Flag N debe estar activa para valor negativo");
    assert!(!cpu.condition_codes().zero(), "Flag Z debe estar inactiva para valor no-cero");
}