//! Test de diagnóstico para debug de opcodes
//! 
//! Verificar comportamiento real vs esperado

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Conectar RAM para tests - mismo setup que tests reales
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    Cpu6809::new(memory_bus)
}

#[test]
fn debug_anda_indexed_0xa4() {
    // Test ANDA indexed diagnostics - verificar paso a paso lo que pasa
    let mut cpu = create_test_cpu();
    
    // Setup inicial exacto del test que falla
    cpu.registers_mut().a = 0xFF;       // A = 0xFF (11111111)
    cpu.registers_mut().x = 0xC900;     // X apunta a memoria
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x0F); // Valor en memoria = 0x0F
    memory_bus.borrow_mut().write(0xC800, 0xA4); // ANDA indexed
    memory_bus.borrow_mut().write(0xC801, 0x84); // postbyte ,X
    
    cpu.registers_mut().pc = 0xC800;
    
    // DIAGNÓSTICO: Capturar estado antes
    println!("ANTES:");
    println!("  A = 0x{:02X}", cpu.registers().a);
    println!("  X = 0x{:04X}", cpu.registers().x);
    println!("  PC = 0x{:04X}", cpu.registers().pc);
    println!("  Memoria[0xC900] = 0x{:02X}", memory_bus.borrow().read(0xC900));
    println!("  Opcode = 0x{:02X}", memory_bus.borrow().read(0xC800));
    println!("  Postbyte = 0x{:02X}", memory_bus.borrow().read(0xC801));
    
    // Ejecutar instrucción
    let cycles = cpu.execute_instruction(false, false);
    
    // DIAGNÓSTICO: Capturar estado después  
    println!("DESPUÉS:");
    println!("  A = 0x{:02X}", cpu.registers().a);
    println!("  PC = 0x{:04X}", cpu.registers().pc);
    println!("  Cycles = {}", cycles);
    println!("  N = {}, Z = {}, V = {}", 
             cpu.registers().cc.n, cpu.registers().cc.z, cpu.registers().cc.v);
    
    // Verificar qué dirección efectiva se calculó leyendo memoria en diferentes ubicaciones
    for addr in [0xC900, 0xC904, 0x04, 0x0004] {
        println!("  Memoria[0x{:04X}] = 0x{:02X}", addr, memory_bus.borrow().read(addr));
    }
    
    // Resultado esperado: 0xFF AND 0x0F = 0x0F
    println!("ESPERADO: 0xFF AND 0x0F = 0x0F");
    println!("OBTENIDO: 0xFF AND ??? = 0x{:02X}", cpu.registers().a);
}

#[test]
fn debug_stx_indexed_0xaf() {
    // Test STX indexed diagnostics
    let mut cpu = create_test_cpu();
    
    // Setup del test que falla
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = 0xCA00;     // Y como puntero base
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0xAF); // STX indexed
    memory_bus.borrow_mut().write(0xC801, 0xA4); // postbyte ,Y
    
    cpu.registers_mut().pc = 0xC800;
    
    // DIAGNÓSTICO: Estado antes
    println!("STX ANTES:");
    println!("  X = 0x{:04X}", cpu.registers().x);
    println!("  Y = 0x{:04X}", cpu.registers().y);  
    println!("  PC = 0x{:04X}", cpu.registers().pc);
    println!("  Postbyte = 0x{:02X}", memory_bus.borrow().read(0xC801));
    
    // Ejecutar
    let cycles = cpu.execute_instruction(false, false);
    
    // DIAGNÓSTICO: Estado después
    println!("STX DESPUÉS:");
    println!("  PC = 0x{:04X}", cpu.registers().pc);
    println!("  Cycles = {}", cycles);
    
    // Verificar dónde se escribió X
    for addr in [0xCA00, 0xCA01, 0x24, 0x0024, 0xC824] {
        let high = memory_bus.borrow().read(addr);
        let low = memory_bus.borrow().read(addr + 1);
        println!("  Memoria[0x{:04X}:0x{:04X}] = 0x{:02X}:{:02X}", 
                 addr, addr + 1, high, low);
    }
    
    println!("ESPERADO: X (0x5678) escrito en Y (0xCA00)");
}

#[test] 
fn debug_postbyte_decoding() {
    // Test específico para entender postbyte decoding
    let mut cpu = create_test_cpu();
    
    // Configurar diferentes postbytes y ver direcciones efectivas
    let postbytes = [
        (0x84, ",X"),
        (0xA4, ",Y"),
        (0xC4, ",U"),
        (0xE4, ",S"),
    ];
    
    cpu.registers_mut().x = 0x1000;
    cpu.registers_mut().y = 0x2000;
    cpu.registers_mut().u = 0x3000;
    cpu.registers_mut().s = 0x4000;
    
    for (postbyte, desc) in postbytes {
        let memory_bus = cpu.memory_bus().clone();
        memory_bus.borrow_mut().write(0xC800, 0xA4); // ANDA indexed como ejemplo
        memory_bus.borrow_mut().write(0xC801, postbyte);
        memory_bus.borrow_mut().write(0x1000, 0xAA); // Valor en X
        memory_bus.borrow_mut().write(0x2000, 0xBB); // Valor en Y  
        memory_bus.borrow_mut().write(0x3000, 0xCC); // Valor en U
        memory_bus.borrow_mut().write(0x4000, 0xDD); // Valor en S
        
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().a = 0xFF;
        
        cpu.execute_instruction(false, false);
        
        println!("Postbyte 0x{:02X} ({}): A = 0x{:02X}", 
                 postbyte, desc, cpu.registers().a);
    }
}