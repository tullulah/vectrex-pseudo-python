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
fn test_adda_immediate_0x8b() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA #0x42
    // PC: 0x0000, A: 0x10, esperamos A: 0x52
    memory_bus.borrow_mut().write(0x0000, 0x8B); // ADDA immediate
    memory_bus.borrow_mut().write(0x0001, 0x42); // operando
    
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().pc = 0x0000;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x52);
    assert_eq!(cpu.registers().pc, 0x0002);
    assert_eq!(cycles, 2);
    assert!(!cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
    assert!(!cpu.registers().cc.c);
    assert!(!cpu.registers().cc.v);
}

#[test]
fn test_adda_direct_0x9b() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA <0x50
    memory_bus.borrow_mut().write(0x0000, 0x9B); // ADDA direct
    memory_bus.borrow_mut().write(0x0001, 0x50); // dirección directa
    memory_bus.borrow_mut().write(0x0050, 0x25); // valor en memoria
    
    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().pc = 0x0000;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55);
    assert_eq!(cpu.registers().pc, 0x0002);
    assert_eq!(cycles, 4);
    assert!(!cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
}

#[test]
fn test_adda_indexed_0xab() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA ,X (usar direcciones que no colisionen debido al módulo 1024 de RAM)
    memory_bus.borrow_mut().write(0x0000, 0xAB); // ADDA indexed
    memory_bus.borrow_mut().write(0x0001, 0x84); // postbyte para ,X
    memory_bus.borrow_mut().write(0x0100, 0x33); // valor en X (cambié de 0x1000 a 0x0100)
    
    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().x = 0x0100; // cambié de 0x1000 a 0x0100
    cpu.registers_mut().pc = 0x0000;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x53);
    assert_eq!(cpu.registers().pc, 0x0002);
    assert_eq!(cycles, 4);
}

#[test]
fn test_adda_extended_0xbb() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA $0200 (usar dirección que no colisione con el módulo 1024)
    memory_bus.borrow_mut().write(0x0000, 0xBB); // ADDA extended
    memory_bus.borrow_mut().write(0x0001, 0x02); // dirección alta
    memory_bus.borrow_mut().write(0x0002, 0x00); // dirección baja
    memory_bus.borrow_mut().write(0x0200, 0x44); // valor en memoria (cambié de 0x2000 a 0x0200)
    
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().pc = 0x0000;
    
    let cycles = cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x55);
    assert_eq!(cpu.registers().pc, 0x0003);
    assert_eq!(cycles, 5);
}

#[test]
fn test_adda_zero_flag_0x8b() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA #0x00 cuando A=0x00 → Z=1
    memory_bus.borrow_mut().write(0x0000, 0x8B);
    memory_bus.borrow_mut().write(0x0001, 0x00);
    
    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().pc = 0x0000;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
}

#[test]
fn test_adda_negative_flag_0x8b() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA #0x90 cuando A=0x10 → resultado 0xA0 (negativo)
    memory_bus.borrow_mut().write(0x0000, 0x8B);
    memory_bus.borrow_mut().write(0x0001, 0x90);
    
    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().pc = 0x0000;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0xA0);
    assert!(cpu.registers().cc.n);
    assert!(!cpu.registers().cc.z);
}

#[test]
fn test_adda_carry_flag_0x8b() {
    let mut cpu = create_test_cpu();
    let memory_bus = cpu.memory_bus().clone();
    
    // ADDA #0xFF cuando A=0x02 → resultado 0x01 con carry
    memory_bus.borrow_mut().write(0x0000, 0x8B);
    memory_bus.borrow_mut().write(0x0001, 0xFF);
    
    cpu.registers_mut().a = 0x02;
    cpu.registers_mut().pc = 0x0000;
    
    cpu.execute_instruction(false, false);
    
    assert_eq!(cpu.registers().a, 0x01);
    assert!(cpu.registers().cc.c);
    assert!(!cpu.registers().cc.z);
}