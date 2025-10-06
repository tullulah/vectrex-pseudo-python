use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::Cpu6809;

const RAM_START: u16 = 0xC800;

use super::setup_cpu_with_ram;

#[test]
fn test_adda_immediate_0x8b() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA #0x42
    // PC: 0x0000, A: 0x10, esperamos A: 0x52
    cpu.memory_bus_mut().write(RAM_START, 0x8B); // ADDA immediate
    cpu.memory_bus_mut().write(RAM_START + 1, 0x42); // operando

    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().pc = RAM_START;

    let cycles = cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x52);
    assert_eq!(cpu.registers().pc, RAM_START + 2);
    assert_eq!(cycles, 2);
    assert!(!cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
    assert!(!cpu.registers().cc.c);
    assert!(!cpu.registers().cc.v);
}

#[test]
fn test_adda_direct_0x9b() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM page
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM page
                                   // ADDA <0x50
    cpu.memory_bus_mut().write(RAM_START, 0x9B); // ADDA direct
    cpu.memory_bus_mut().write(RAM_START + 1, 0x50); // dirección directa
    cpu.memory_bus_mut().write(RAM_START + 0x50, 0x25); // valor en memoria

    cpu.registers_mut().a = 0x30;
    cpu.registers_mut().pc = RAM_START;

    let cycles = cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x55);
    assert_eq!(cpu.registers().pc, RAM_START + 2);
    assert_eq!(cycles, 4);
    assert!(!cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
}

#[test]
fn test_adda_indexed_0xab() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA ,X (usar direcciones que no colisionen debido al módulo 1024 de RAM)
    cpu.memory_bus_mut().write(RAM_START, 0xAB); // ADDA indexed
    cpu.memory_bus_mut().write(RAM_START + 1, 0x84); // postbyte para ,X
    cpu.memory_bus_mut().write(RAM_START + 0x100, 0x33); // valor en X (cambié de 0x1000 a 0x0100)

    cpu.registers_mut().a = 0x20;
    cpu.registers_mut().x = RAM_START + 0x100; // cambié de 0x1000 a 0x0100
    cpu.registers_mut().pc = RAM_START;

    let cycles = cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x53);
    assert_eq!(cpu.registers().pc, RAM_START + 2);
    assert_eq!(cycles, 4);
}

#[test]
fn test_adda_extended_0xbb() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA $CA00 (extended addressing apunta a RAM_START + 0x200)
    cpu.memory_bus_mut().write(RAM_START, 0xBB); // ADDA extended
    cpu.memory_bus_mut().write(RAM_START + 1, 0xCA); // dirección alta (0xCA)
    cpu.memory_bus_mut().write(RAM_START + 2, 0x00); // dirección baja (0x00)
    cpu.memory_bus_mut().write(RAM_START + 0x200, 0x44); // valor en 0xCA00

    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().pc = RAM_START;

    let cycles = cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x55);
    assert_eq!(cpu.registers().pc, RAM_START + 3);
    assert_eq!(cycles, 5);
}

#[test]
fn test_adda_zero_flag_0x8b() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA #0x00 cuando A=0x00 → Z=1
    cpu.memory_bus_mut().write(RAM_START, 0x8B);
    cpu.memory_bus_mut().write(RAM_START + 1, 0x00);

    cpu.registers_mut().a = 0x00;
    cpu.registers_mut().pc = RAM_START;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x00);
    assert!(cpu.registers().cc.z);
    assert!(!cpu.registers().cc.n);
}

#[test]
fn test_adda_negative_flag_0x8b() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA #0x90 cuando A=0x10 → resultado 0xA0 (negativo)
    cpu.memory_bus_mut().write(RAM_START, 0x8B);
    cpu.memory_bus_mut().write(RAM_START + 1, 0x90);

    cpu.registers_mut().a = 0x10;
    cpu.registers_mut().pc = RAM_START;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0xA0);
    assert!(cpu.registers().cc.n);
    assert!(!cpu.registers().cc.z);
}

#[test]
fn test_adda_carry_flag_0x8b() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // ADDA #0xFF cuando A=0x02 → resultado 0x01 con carry
    cpu.memory_bus_mut().write(RAM_START, 0x8B);
    cpu.memory_bus_mut().write(RAM_START + 1, 0xFF);

    cpu.registers_mut().a = 0x02;
    cpu.registers_mut().pc = RAM_START;

    cpu.execute_instruction(false, false).unwrap();

    assert_eq!(cpu.registers().a, 0x01);
    assert!(cpu.registers().cc.c);
    assert!(!cpu.registers().cc.z);
}
