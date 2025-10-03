use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;

// Configuración estándar para tests
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
fn test_stx_direct_0x9f() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial - cargar X con valor conocido
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().dp = 0xC8; // DP = 0xC8 para que direct apunte a RAM
    
    // Escribir instrucción STX direct: 0x9F + dirección direct
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0x9F); // STX direct
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x50); // Dirección direct (DP:50 = C8:50 = C850)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que X se almacenó correctamente en memoria (big-endian)
    let stored_high = memory_bus.borrow().read(0xC850);
    let stored_low = memory_bus.borrow().read(0xC851);
    assert_eq!(stored_high, 0x12, "High byte of X should be stored");
    assert_eq!(stored_low, 0x34, "Low byte of X should be stored");
    
    // Verificar flags - STX afecta N, Z, V
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for positive value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear for non-zero value");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear for store operation");
}

#[test]
fn test_stx_indexed_0xaf() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = RAM_START + 0x200; // Y como base para indexed
    
    // Escribir instrucción STX indexed: 0xAF + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xAF); // STX indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0xA4); // ,Y (postbyte para Y register, no offset)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que X se almacenó en la dirección apuntada por Y
    let target_addr = RAM_START + 0x200;
    let stored_high = memory_bus.borrow().read(target_addr);
    let stored_low = memory_bus.borrow().read(target_addr + 1);
    assert_eq!(stored_high, 0x56, "High byte of X should be stored at Y address");
    assert_eq!(stored_low, 0x78, "Low byte of X should be stored at Y+1 address");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_stx_extended_0xbf() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().x = 0x9ABC;
    
    // Escribir instrucción STX extended: 0xBF + dirección 16-bit
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xBF); // STX extended
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0xC8); // High byte of target address
    memory_bus.borrow_mut().write(RAM_START + 0x102, 0x60); // Low byte of target address (0xC860)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que X se almacenó en 0xC860
    let stored_high = memory_bus.borrow().read(0xC860);
    let stored_low = memory_bus.borrow().read(0xC861);
    assert_eq!(stored_high, 0x9A, "High byte of X should be stored");
    assert_eq!(stored_low, 0xBC, "Low byte of X should be stored");
    
    // Verificar flags - valor negativo (bit 15 set)
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_std_direct_0xdd() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial - cargar D (A:B) con valor conocido
    cpu.registers_mut().a = 0xDE;
    cpu.registers_mut().b = 0xAD;
    cpu.registers_mut().dp = 0xC8;
    
    // Escribir instrucción STD direct: 0xDD + dirección direct
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xDD); // STD direct
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x70); // Dirección direct (C8:70 = C870)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que D se almacenó correctamente
    let stored_high = memory_bus.borrow().read(0xC870); // A (high byte)
    let stored_low = memory_bus.borrow().read(0xC871);  // B (low byte)
    assert_eq!(stored_high, 0xDE, "A should be stored as high byte");
    assert_eq!(stored_low, 0xAD, "B should be stored as low byte");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_stu_direct_0xdf() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().u = 0x1111;
    cpu.registers_mut().dp = 0xC8;
    
    // Escribir instrucción STU direct: 0xDF + dirección direct
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xDF); // STU direct
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x80); // Dirección direct (C8:80 = C880)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que U se almacenó correctamente
    let stored_high = memory_bus.borrow().read(0xC880);
    let stored_low = memory_bus.borrow().read(0xC881);
    assert_eq!(stored_high, 0x11, "High byte of U should be stored");
    assert_eq!(stored_low, 0x11, "Low byte of U should be stored");
    
    // Verificar flags
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_std_indexed_0xed() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().b = 0x00;
    cpu.registers_mut().x = RAM_START + 0x300;
    
    // Escribir instrucción STD indexed: 0xED + postbyte
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xED); // STD indexed
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0x84); // ,X (postbyte para X register)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que D=0x0000 se almacenó
    let target_addr = RAM_START + 0x300;
    let stored_high = memory_bus.borrow().read(target_addr);
    let stored_low = memory_bus.borrow().read(target_addr + 1);
    assert_eq!(stored_high, 0x00, "A (high byte) should be stored");
    assert_eq!(stored_low, 0x00, "B (low byte) should be stored");
    
    // Verificar flags - valor cero
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear for zero");
    assert_eq!(cpu.registers().cc.z, true, "Z flag should be set for zero value");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}

#[test]
fn test_stu_extended_0xff() {
    let mut cpu = setup_emulator();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup inicial
    cpu.registers_mut().u = 0x8000; // Valor negativo para test de flag N
    
    // Escribir instrucción STU extended: 0xFF + dirección 16-bit
    memory_bus.borrow_mut().write(RAM_START + 0x100, 0xFF); // STU extended
    memory_bus.borrow_mut().write(RAM_START + 0x101, 0xC8); // High byte of target
    memory_bus.borrow_mut().write(RAM_START + 0x102, 0x90); // Low byte of target (0xC890)
    
    // Configurar PC y ejecutar
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    
    // Verificar que U se almacenó en 0xC890
    let stored_high = memory_bus.borrow().read(0xC890);
    let stored_low = memory_bus.borrow().read(0xC891);
    assert_eq!(stored_high, 0x80, "High byte of U should be stored");
    assert_eq!(stored_low, 0x00, "Low byte of U should be stored");
    
    // Verificar flags - valor negativo
    assert_eq!(cpu.registers().cc.n, true, "N flag should be set for negative value");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear");
}