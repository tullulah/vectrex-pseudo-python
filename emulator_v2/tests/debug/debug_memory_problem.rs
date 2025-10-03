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
fn debug_memory_problem() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    println!("Testing memory writes and reads...");
    
    // Prueba de escritura y lectura simple
    println!("\n1. Writing to different addresses:");
    memory_bus.borrow_mut().write(0x0000, 0xAB);
    memory_bus.borrow_mut().write(0x0001, 0x84);
    memory_bus.borrow_mut().write(0x1000, 0x33);
    
    println!("Wrote 0xAB to 0x0000");
    println!("Wrote 0x84 to 0x0001");
    println!("Wrote 0x33 to 0x1000");
    
    println!("\n2. Reading back:");
    let val_0000 = memory_bus.borrow().read(0x0000);
    let val_0001 = memory_bus.borrow().read(0x0001);
    let val_1000 = memory_bus.borrow().read(0x1000);
    
    println!("Read from 0x0000: 0x{:02X} (expected: 0xAB)", val_0000);
    println!("Read from 0x0001: 0x{:02X} (expected: 0x84)", val_0001);
    println!("Read from 0x1000: 0x{:02X} (expected: 0x33)", val_1000);
    
    // Verificación adicional: leer desde múltiples direcciones para ver el patrón
    println!("\n3. Testing address mapping:");
    for addr in [0x0000, 0x0001, 0x0002, 0x1000, 0x1001, 0x2000].iter() {
        let val = memory_bus.borrow().read(*addr);
        println!("Address 0x{:04X}: 0x{:02X}", addr, val);
    }
    
    // Test escritura secuencial
    println!("\n4. Sequential write test:");
    for i in 0..10 {
        let addr = 0x2000 + i;
        let val = 0x10 + i as u8;
        memory_bus.borrow_mut().write(addr, val);
        let read_back = memory_bus.borrow().read(addr);
        println!("Wrote 0x{:02X} to 0x{:04X}, read back: 0x{:02X}", val, addr, read_back);
    }
}