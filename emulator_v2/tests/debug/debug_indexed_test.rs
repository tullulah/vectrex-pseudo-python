use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Conectar RAM para tests
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    Cpu6809::new(memory_bus)
}

#[test]
fn debug_indexed_addressing() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Configurar memoria inicial para debug
    memory_bus.borrow_mut().write(0x1000, 0x33); // valor que esperamos leer
    
    // Configurar registros
    cpu.registers_mut().x = 0x1000;
    
    // Test directo de lectura
    let direct_read = cpu.read8(0x1000);
    println!("Direct read from 0x1000: 0x{:02X}", direct_read);
    
    // Test manual de indexed EA calculation
    memory_bus.borrow_mut().write(0x0000, 0x84); // postbyte para ,X
    cpu.registers_mut().pc = 0x0000;
    
    // Simulamos lo que hace read_indexed_ea manualmente
    let postbyte = cpu.read8(0x0000);
    println!("Postbyte read: 0x{:02X}", postbyte);
    
    // Simulamos register_select
    let register_bits = (postbyte >> 5) & 0x03;
    println!("Register bits: {:02b} (should be 00 for X)", register_bits);
    
    let x_value = cpu.registers().x;
    println!("X register value: 0x{:04X}", x_value);
    
    // Intentamos leer desde la dirección calculada
    let memory_value = cpu.read8(x_value);
    println!("Memory at X address: 0x{:02X} (should be 0x33)", memory_value);
}

#[test]
fn debug_full_adda_indexed() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup exactamente como el test que falla
    memory_bus.borrow_mut().write(0x0000, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(0x0001, 0x84); // postbyte para ,X
    memory_bus.borrow_mut().write(0x1000, 0x33); // valor en X
    
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().pc = 0x0000;
    
    println!("BEFORE EXECUTION:");
    println!("A: 0x{:02X}", cpu.registers().a);
    println!("X: 0x{:04X}", cpu.registers().x);
    println!("PC: 0x{:04X}", cpu.registers().pc);
    println!("Memory[0x1000]: 0x{:02X}", memory_bus.borrow().read(0x1000));
    
    let cycles = cpu.execute_instruction(false, false);
    
    println!("AFTER EXECUTION:");
    println!("A: 0x{:02X} (expected: 0x53)", cpu.registers().a);
    println!("PC: 0x{:04X}", cpu.registers().pc);
    println!("Cycles: {}", cycles);
    
    // Test si realmente se sumó algo 
    if cpu.registers().a == 0x20 {
        println!("ERROR: A no cambió, parece que se sumó 0");
    } else if cpu.registers().a == 0x53 {
        println!("SUCCESS: A tiene el valor correcto");
    } else {
        println!("UNEXPECTED: A tiene un valor inesperado");
    }
}