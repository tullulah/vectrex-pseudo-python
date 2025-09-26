// Test for NOP (No Operation) opcode 0x12
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
fn test_nop_basic_0x12() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial - configurar todos los registros con valores conocidos
    cpu.registers.pc = 0xC800;
    cpu.registers.a = 0x42;
    cpu.registers.b = 0x33;
    cpu.registers.x = 0x1234;
    cpu.registers.y = 0x5678;
    cpu.registers.dp = 0x99;
    cpu.registers.s = 0xCFFF;
    cpu.registers.u = 0xDEAD;
    
    // Configurar condition codes con valores conocidos
    cpu.registers.cc.z = true;
    cpu.registers.cc.n = false;
    cpu.registers.cc.v = true;
    cpu.registers.cc.c = false;
    
    // Escribir NOP
    cpu.memory_bus().borrow_mut().write(0xC800, 0x12); // NOP opcode
    
    // Ejecutar instrucción NOP
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que NADA cambió excepto PC
    assert_eq!(cpu.registers.pc, 0xC801, "PC should advance by 1");
    assert_eq!(cycles, 2, "NOP should take 2 cycles");
    
    // Verificar que todos los registros permanecen iguales
    assert_eq!(cpu.registers.a, 0x42, "A should be unchanged");
    assert_eq!(cpu.registers.b, 0x33, "B should be unchanged");
    assert_eq!(cpu.registers.x, 0x1234, "X should be unchanged");
    assert_eq!(cpu.registers.y, 0x5678, "Y should be unchanged");
    assert_eq!(cpu.registers.dp, 0x99, "DP should be unchanged");
    assert_eq!(cpu.registers.s, 0xCFFF, "S should be unchanged");
    assert_eq!(cpu.registers.u, 0xDEAD, "U should be unchanged");
    
    // Verificar que todos los condition codes permanecen iguales
    assert!(cpu.registers.cc.z, "Zero flag should be unchanged");
    assert!(!cpu.registers.cc.n, "Negative flag should be unchanged");
    assert!(cpu.registers.cc.v, "Overflow flag should be unchanged");
    assert!(!cpu.registers.cc.c, "Carry flag should be unchanged");
}

#[test]
fn test_nop_sequential_execution_0x12() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    
    // Escribir secuencia de NOPs
    cpu.memory_bus().borrow_mut().write(0xC800, 0x12); // NOP 1
    cpu.memory_bus().borrow_mut().write(0xC801, 0x12); // NOP 2
    cpu.memory_bus().borrow_mut().write(0xC802, 0x12); // NOP 3
    
    // Ejecutar primera NOP
    let cycles1 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, 0xC801);
    assert_eq!(cycles1, 2);
    
    // Ejecutar segunda NOP
    let cycles2 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, 0xC802);
    assert_eq!(cycles2, 2);
    
    // Ejecutar tercera NOP
    let cycles3 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, 0xC803);
    assert_eq!(cycles3, 2);
}

#[test]
fn test_nop_memory_unaffected_0x12() {
    let mut cpu = create_test_cpu();
    
    // Setup inicial
    cpu.registers.pc = 0xC800;
    
    // Escribir valores conocidos en memoria alrededor del NOP
    cpu.memory_bus().borrow_mut().write(0xC800, 0x12); // NOP opcode
    cpu.memory_bus().borrow_mut().write(0xC801, 0xBB); // Después del NOP
    cpu.memory_bus().borrow_mut().write(0xC802, 0xAA); // Cerca del NOP
    cpu.memory_bus().borrow_mut().write(0xC900, 0xCC); // Lejos del NOP
    
    // Ejecutar NOP
    let cycles = cpu.execute_instruction(false, false);
    
    // Verificar que la memoria no fue alterada
    assert_eq!(cpu.memory_bus().borrow().read(0xC800), 0x12, "NOP opcode should remain");
    assert_eq!(cpu.memory_bus().borrow().read(0xC802), 0xAA, "Memory near should be unchanged");
    assert_eq!(cpu.memory_bus().borrow().read(0xC801), 0xBB, "Memory after should be unchanged");
    assert_eq!(cpu.memory_bus().borrow().read(0xC900), 0xCC, "Distant memory should be unchanged");
    assert_eq!(cycles, 2);
}

#[test]
fn test_nop_at_memory_boundaries_0x12() {
    let mut cpu = create_test_cpu();
    
    // Test NOP al inicio de RAM
    cpu.registers.pc = RAM_START;
    cpu.memory_bus().borrow_mut().write(RAM_START, 0x12); // NOP
    
    let cycles1 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, RAM_START + 1);
    assert_eq!(cycles1, 2);
    
    // Test NOP cerca del final de RAM (cuidado con el stack)
    let test_addr = 0xCF00; // Lejos del stack en 0xCFFF
    cpu.registers.pc = test_addr;
    cpu.memory_bus().borrow_mut().write(test_addr, 0x12); // NOP
    
    let cycles2 = cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.pc, test_addr + 1);
    assert_eq!(cycles2, 2);
}

#[test]
fn test_nop_timing_consistency_0x12() {
    let mut cpu = create_test_cpu();
    
    // Ejecutar múltiples NOPs y verificar timing consistente
    for i in 0..10 {
        let addr = 0xC800 + i;
        cpu.registers.pc = addr;
        cpu.memory_bus().borrow_mut().write(addr, 0x12); // NOP
        
        let cycles = cpu.execute_instruction(false, false);
        assert_eq!(cycles, 2, "NOP should always take exactly 2 cycles");
        assert_eq!(cpu.registers.pc, addr + 1, "PC should always advance by 1");
    }
}

#[test]
fn test_nop_all_condition_codes_preserved_0x12() {
    let mut cpu = create_test_cpu();
    
    // Test todas las combinaciones posibles de condition codes
    let test_cases = [
        (false, false, false, false), // Todos clear
        (true, true, true, true),     // Todos set
        (true, false, true, false),   // Alternados 1
        (false, true, false, true),   // Alternados 2
    ];
    
    for (z, n, v, c) in test_cases.iter() {
        cpu.registers.pc = 0xC800;
        cpu.registers.cc.z = *z;
        cpu.registers.cc.n = *n;
        cpu.registers.cc.v = *v;
        cpu.registers.cc.c = *c;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x12); // NOP
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que los condition codes no cambiaron
        assert_eq!(cpu.registers.cc.z, *z, "Zero flag should be preserved");
        assert_eq!(cpu.registers.cc.n, *n, "Negative flag should be preserved");
        assert_eq!(cpu.registers.cc.v, *v, "Overflow flag should be preserved");
        assert_eq!(cpu.registers.cc.c, *c, "Carry flag should be preserved");
        assert_eq!(cycles, 2);
    }
}