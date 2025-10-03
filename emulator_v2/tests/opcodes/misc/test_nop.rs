use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_emulator() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Conectar RAM para tests
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    cpu
}

#[test]
fn test_nop_0x12() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial - guardar estado original
    let original_a = 0x42;
    let original_b = 0x69;
    let original_x = 0x1234;
    let original_y = 0x5678;
    let original_u = 0x9ABC;
    let original_s = 0xDEF0;
    let original_dp = 0x12;
    
    cpu.registers_mut().a = original_a;
    cpu.registers_mut().b = original_b;
    cpu.registers_mut().x = original_x;
    cpu.registers_mut().y = original_y;
    cpu.registers_mut().u = original_u;
    cpu.registers_mut().s = original_s;
    cpu.registers_mut().dp = original_dp;
    
    // Configurar condition codes iniciales
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.v = true;
    
    let original_cc = cpu.registers().cc;
    
    // Escribir instrucción NOP: 0x12
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0x12); // NOP
    
    // Configurar PC y ejecutar
    let original_pc = RAM_START + 0x100;
    cpu.registers_mut().pc = original_pc;
    
    let cycles_before = cpu.get_cycles();
    cpu.execute_instruction(false, false);
    let cycles_after = cpu.get_cycles();
    
    // Verificar que NOP no cambió ningún registro
    assert_eq!(cpu.registers().a, original_a, "A register should not change");
    assert_eq!(cpu.registers().b, original_b, "B register should not change");
    assert_eq!(cpu.registers().x, original_x, "X register should not change");
    assert_eq!(cpu.registers().y, original_y, "Y register should not change");
    assert_eq!(cpu.registers().u, original_u, "U register should not change");
    assert_eq!(cpu.registers().s, original_s, "S register should not change");
    assert_eq!(cpu.registers().dp, original_dp, "DP register should not change");
    
    // Verificar que los condition codes no cambiaron
    assert_eq!(cpu.registers().cc.c, original_cc.c, "Carry flag should not change");
    assert_eq!(cpu.registers().cc.z, original_cc.z, "Zero flag should not change");
    assert_eq!(cpu.registers().cc.n, original_cc.n, "Negative flag should not change");
    assert_eq!(cpu.registers().cc.v, original_cc.v, "Overflow flag should not change");
    
    // Verificar que PC avanzó correctamente (NOP es 1 byte)
    assert_eq!(cpu.registers().pc, original_pc + 1, "PC should advance by 1 byte for NOP");
    
    // Verificar que se consumieron cycles (NOP debe tomar 2 cycles según M6809)
    let cycles_consumed = cycles_after - cycles_before;
    assert_eq!(cycles_consumed, 2, "NOP should consume 2 cycles");
}

#[test]
fn test_nop_sequence_0x12() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Escribir secuencia de NOPs para verificar que funcionan consecutivamente
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0x12); // NOP
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x12); // NOP
    memory_bus.borrow_mut().write(RAM_START + 0x102, 0x12); // NOP
    memory_bus.borrow_mut().write(RAM_START + 0x103, 0x86); // LDA #immediate (para cambio visible)
    memory_bus.borrow_mut().write(RAM_START + 0x104, 0x99); // Immediate value
    
    // Configurar PC inicial
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.registers_mut().a = 0x00; // A inicial
    
    // Ejecutar primera NOP
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, RAM_START + 0x101, "PC should be at second NOP");
    assert_eq!(cpu.registers().a, 0x00, "A should not change after first NOP");
    
    // Ejecutar segunda NOP
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102, "PC should be at third NOP");
    assert_eq!(cpu.registers().a, 0x00, "A should not change after second NOP");
    
    // Ejecutar tercera NOP
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, RAM_START + 0x103, "PC should be at LDA instruction");
    assert_eq!(cpu.registers().a, 0x00, "A should not change after third NOP");
    
    // Ejecutar LDA para confirmar que el emulador sigue funcionando normalmente
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers().pc, RAM_START + 0x105, "PC should advance past LDA");
    assert_eq!(cpu.registers().a, 0x99, "A should contain loaded value after LDA");
}

#[test]
fn test_nop_memory_unchanged_0x12() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Colocar algunos valores en memoria
    memory_bus.borrow_mut().write(RAM_START + 0x200, 0xAA);
    memory_bus.borrow_mut().write(RAM_START + 0x201, 0xBB);
    memory_bus.borrow_mut().write(RAM_START + 0x202, 0xCC);
    
    // Escribir NOP
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0x12);
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que la memoria no cambió
    assert_eq!(memory_bus.borrow().read(RAM_START + 0x200), 0xAA, "Memory should not change");
    assert_eq!(memory_bus.borrow().read(RAM_START + 0x201), 0xBB, "Memory should not change");
    assert_eq!(memory_bus.borrow().read(RAM_START + 0x202), 0xCC, "Memory should not change");
}