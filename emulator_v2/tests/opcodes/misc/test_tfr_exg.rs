use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

// Configuración estándar de memoria para tests
const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    cpu
}

#[test]
fn test_tfr_a_to_b_0x1f() {
    let mut cpu = setup_cpu();
    
    // Setup inicial - A=0x42, B=0x00
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x00;
    
    // TFR A,B - opcode 0x1F, postbyte 0x89 (A=8, B=9)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x1F);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x89);
    
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.step();
    
    // Verificar - B debe tener valor de A
    assert_eq!(cpu.registers().a, 0x42);
    assert_eq!(cpu.registers().b, 0x42);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102);
}

#[test]
fn test_tfr_x_to_d_0x1f() {
    let mut cpu = setup_cpu();
    
    // Setup inicial - X=0x1234, D=0x0000
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().d = 0x0000;
    
    // TFR X,D - opcode 0x1F, postbyte 0x10 (X=1, D=0)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x1F);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x10);
    
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.step();
    
    // Verificar - D debe tener valor de X
    assert_eq!(cpu.registers().x, 0x1234);
    assert_eq!(cpu.registers().d, 0x1234);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102);
}

#[test]
fn test_exg_a_b_0x1e() {
    let mut cpu = setup_cpu();
    
    // Setup inicial - A=0x42, B=0x33
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().b = 0x33;
    
    // EXG A,B - opcode 0x1E, postbyte 0x89 (A=8, B=9)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x1E);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x89);
    
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.step();
    
    // Verificar - valores intercambiados
    assert_eq!(cpu.registers().a, 0x33);
    assert_eq!(cpu.registers().b, 0x42);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102);
}

#[test]
fn test_exg_x_y_0x1e() {
    let mut cpu = setup_cpu();
    
    // Setup inicial - X=0x1234, Y=0x5678
    cpu.registers_mut().x = 0x1234;
    cpu.registers_mut().y = 0x5678;
    
    // EXG X,Y - opcode 0x1E, postbyte 0x12 (X=1, Y=2)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x1E);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x12);
    
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.step();
    
    // Verificar - valores intercambiados
    assert_eq!(cpu.registers().x, 0x5678);
    assert_eq!(cpu.registers().y, 0x1234);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102);
}

#[test]
fn test_tfr_d_to_x_0x1f() {
    let mut cpu = setup_cpu();
    
    // Setup inicial - D=0xABCD, X=0x0000
    cpu.registers_mut().d = 0xABCD;
    cpu.registers_mut().x = 0x0000;
    
    // TFR D,X - opcode 0x1F, postbyte 0x01 (D=0, X=1)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x1F);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x01);
    
    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.step();
    
    // Verificar - X debe tener valor de D
    assert_eq!(cpu.registers().d, 0xABCD);
    assert_eq!(cpu.registers().x, 0xABCD);
    assert_eq!(cpu.registers().pc, RAM_START + 0x102);
}