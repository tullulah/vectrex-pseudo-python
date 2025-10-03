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
fn test_anda_indexed_0xa4() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0xFF; // A = 0xFF
    cpu.registers_mut().x = RAM_START + 0x200;
    
    // Colocar valor en memoria para AND operation
    memory_bus.borrow_mut().write(RAM_START + 0x200, 0x0F); // Valor en memoria = 0x0F
    
    // Debug: Verificar el setup
    println!("Test setup:");
    println!("A register: 0x{:02X}", cpu.registers().a);
    println!("X register: 0x{:04X}", cpu.registers().x);
    println!("Memory at 0x{:04X}: 0x{:02X}", RAM_START + 0x200, memory_bus.borrow().read(RAM_START + 0x200));
    
    // Escribir instrucción ANDA indexed: 0xA4 + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xA4); // ANDA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x84); // ,X (postbyte para X register sin offset)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    
    println!("Before execution:");
    println!("PC: 0x{:04X}", cpu.registers().pc);
    println!("Instruction at PC: 0x{:02X}", memory_bus.borrow().read(RAM_START + 0x100));
    println!("Postbyte at PC+1: 0x{:02X} (should be 0x84 for ,X)", memory_bus.borrow().read(RAM_START + 0x101));
    
    cpu.execute_instruction(false, false);
    
    println!("After execution:");
    println!("A register: 0x{:02X}", cpu.registers().a);
    println!("Expected: 0x0F (0xFF AND 0x0F)");
    
    // Verificar resultado: A = 0xFF AND 0x0F = 0x0F
    assert_eq!(cpu.registers().a, 0x0F, "A should be result of AND operation");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for AND operation");
}

#[test]
fn test_eora_indexed_0xa8() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0xAA; // A = 0xAA (10101010)
    cpu.registers_mut().y = RAM_START + 0x250;
    
    // Colocar valor en memoria para EOR operation
    memory_bus.borrow_mut().write(RAM_START + 0x250, 0x55); // Valor en memoria = 0x55 (01010101)
    
    // Escribir instrucción EORA indexed: 0xA8 + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xA8); // EORA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0xA4); // ,Y (postbyte para Y register)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar resultado: A = 0xAA EOR 0x55 = 0xFF
    assert_eq!(cpu.registers().a, 0xFF, "A should be result of EOR operation");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for EOR operation");
}

#[test]
fn test_oraa_indexed_0xaa() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0x0F; // A = 0x0F (00001111)
    cpu.registers_mut().u = RAM_START + 0x300;
    
    // Colocar valor en memoria para OR operation
    memory_bus.borrow_mut().write(RAM_START + 0x300, 0xF0); // Valor en memoria = 0xF0 (11110000)
    
    // Escribir instrucción ORAA indexed: 0xAA + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xAA); // ORAA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0xC4); // ,U (postbyte para U register)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar resultado: A = 0x0F OR 0xF0 = 0xFF
    assert_eq!(cpu.registers().a, 0xFF, "A should be result of OR operation");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for OR operation");
}

#[test]
fn test_suba_indexed_0xa0() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0x50; // A = 0x50
    cpu.registers_mut().x = RAM_START + 0x400;
    
    // Colocar valor en memoria para SUB operation
    memory_bus.borrow_mut().write(RAM_START + 0x400, 0x30); // Valor en memoria = 0x30
    
    // Escribir instrucción SUBA indexed: 0xA0 + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xA0); // SUBA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x84); // ,X (postbyte para X register)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar resultado: A = 0x50 - 0x30 = 0x20
    assert_eq!(cpu.registers().a, 0x20, "A should be result of SUB operation");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no borrow)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for normal subtraction");
}

#[test]
fn test_adda_indexed_0xab() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0x7F; // A = 0x7F (max positive)
    cpu.registers_mut().x = RAM_START + 0x250; // Usar dirección que funciona en otros tests
    
    // Colocar valor en memoria para ADD operation que causará overflow
    memory_bus.borrow_mut().write(RAM_START + 0x250, 0x01); // Valor en memoria = 0x01
    
    // Escribir instrucción ADDA indexed: 0xAB + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x84); // ,X (postbyte para X register)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar resultado: A = 0x7F + 0x01 = 0x80 (overflow)
    assert_eq!(cpu.registers().a, 0x80, "A should be result of ADD operation");
    
    // Verificar flags - overflow de positivo a negativo
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative result");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero result");
    assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (no carry out)");
    assert_eq!(cpu.registers().cc.v, true, "V flag should be set for overflow");
}

#[test]
fn test_adda_indexed_zero_result_0xab() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup para resultado cero
    cpu.registers_mut().a = 0xFF; // A = 0xFF (-1)
    cpu.registers_mut().x = RAM_START + 0x260; // Usar dirección diferente para evitar conflictos
    
    // Colocar valor en memoria
    memory_bus.borrow_mut().write(RAM_START + 0x260, 0x01); // Valor = 0x01
    
    // Escribir instrucción ADDA indexed: 0xAB + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x84); // ,X
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar resultado: A = 0xFF + 0x01 = 0x00 (con carry)
    assert_eq!(cpu.registers().a, 0x00, "A should be zero");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for zero");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set for zero result");
    assert_eq!(cpu.registers().cc.c, true, "C flag should be set for carry out");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}