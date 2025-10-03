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
fn debug_step_by_step() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // Setup exactamente como el test que falla
    memory_bus.borrow_mut().write(0x0000, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(0x0001, 0x84); // postbyte para ,X
    memory_bus.borrow_mut().write(0x1000, 0x33); // valor en X
    
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().pc = 0x0000;
    
    println!("SETUP VERIFICATION:");
    println!("Memory[0x0000]: 0x{:02X} (ADDA opcode)", memory_bus.borrow().read(0x0000));
    println!("Memory[0x0001]: 0x{:02X} (postbyte)", memory_bus.borrow().read(0x0001));
    println!("Memory[0x1000]: 0x{:02X} (data)", memory_bus.borrow().read(0x1000));
    println!("A: 0x{:02X}", cpu.registers().a);
    println!("X: 0x{:04X}", cpu.registers().x);
    println!("PC: 0x{:04X}", cpu.registers().pc);
    
    // Simulemos manualmente lo que debería pasar:
    println!("\nMANUAL SIMULATION:");
    // 1. Lee opcode desde PC (0x0000)
    let opcode = memory_bus.borrow().read(0x0000);
    println!("1. Read opcode from PC=0x0000: 0x{:02X}", opcode);
    
    // 2. PC avanza a 0x0001
    // 3. read_indexed_ea() debería leer postbyte desde PC=0x0001
    let postbyte = memory_bus.borrow().read(0x0001);
    println!("2. Read postbyte from PC=0x0001: 0x{:02X}", postbyte);
    
    // 4. Calcula EA usando register_select
    let register_bits = (postbyte >> 5) & 0x03;
    println!("3. Register bits: {:02b} (X register)", register_bits);
    let ea = cpu.registers().x; // Para ,X mode (0x04)
    println!("4. Calculated EA: 0x{:04X}", ea);
    
    // 5. Lee valor desde EA
    let operand = memory_bus.borrow().read(ea);
    println!("5. Read operand from EA=0x{:04X}: 0x{:02X}", ea, operand);
    
    // 6. Suma A + operand
    let result = cpu.registers().a.wrapping_add(operand);
    println!("6. A + operand: 0x{:02X} + 0x{:02X} = 0x{:02X}", cpu.registers().a, operand, result);
    
    println!("\nNOW RUNNING REAL INSTRUCTION:");
    let cycles = cpu.execute_instruction(false, false);
    
    println!("AFTER REAL EXECUTION:");
    println!("A: 0x{:02X} (expected: 0x{:02X})", cpu.registers().a, result);
    println!("PC: 0x{:04X}", cpu.registers().pc);
    println!("Cycles: {}", cycles);
}