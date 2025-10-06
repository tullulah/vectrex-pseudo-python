// Test para verificar el ORDEN de operaciones en LDD ,U++
// 
// CRITICO: La BIOS usa LDD ,U++ extensivamente en Print_Str_hwyx
// Necesitamos verificar que el orden sea CORRECTO:
// 
// ORDEN ESPERADO (según documentación 6809):
// 1. Leer valor de memoria en dirección apuntada por U
// 2. Incrementar U en 2
// 3. Cargar valor leído en D
//
// NO debe ser:
// 1. Incrementar U en 2
// 2. Leer de la nueva dirección

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_ldd_indexed_post_increment_order() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    // Setup de datos en memoria
    // Pondremos dos valores diferentes para detectar si lee antes o después del incremento
    let data_addr = RAM_START + 0x100;
    
    // Valor en dirección U inicial (debería leer ESTE)
    unsafe { &mut *memory.get() }.write(data_addr, 0x12);      // High byte
    unsafe { &mut *memory.get() }.write(data_addr + 1, 0x34);  // Low byte
    
    // Valor en dirección U+2 (NO debería leer este)
    unsafe { &mut *memory.get() }.write(data_addr + 2, 0xAB);  // High byte diferente
    unsafe { &mut *memory.get() }.write(data_addr + 3, 0xCD);  // Low byte diferente
    
    // Inicializar U apuntando a data_addr
    cpu.registers_mut().u = data_addr;
    println!("🔧 Setup:");
    println!("   U inicial = 0x{:04X}", data_addr);
    println!("   Memoria[0x{:04X}] = 0x12 0x34", data_addr);
    println!("   Memoria[0x{:04X}] = 0xAB 0xCD", data_addr + 2);
    
    // Escribir instrucción LDD ,U++ (opcode 0xEC 0xC1)
    // 0xEC = LDD indexed
    // 0xC1 = postbyte para ,U++ (bit 7=1, bits 6-5=11 (U), bits 0-3=0001 (post-increment by 2))
    let code_addr = RAM_START + 0x200;
    unsafe { &mut *memory.get() }.write(code_addr, 0xEC);       // LDD indexed
    unsafe { &mut *memory.get() }.write(code_addr + 1, 0xC1);   // ,U++ postbyte
    
    cpu.registers_mut().pc = code_addr;
    println!("\n📝 Instrucción:");
    println!("   PC = 0x{:04X}: LDD ,U++ (0xEC 0xC1)", code_addr);
    
    // Estado ANTES de ejecutar
    let u_before = cpu.registers().u;
    let d_before = cpu.registers().d();
    println!("\n⏮️  ANTES:");
    println!("   U = 0x{:04X}", u_before);
    println!("   D = 0x{:04X}", d_before);
    
    // Ejecutar instrucción
    cpu.execute_instruction(false, false).unwrap();
    
    // Estado DESPUÉS de ejecutar
    let u_after = cpu.registers().u;
    let d_after = cpu.registers().d();
    println!("\n⏭️  DESPUÉS:");
    println!("   U = 0x{:04X} (debería ser 0x{:04X})", u_after, data_addr + 2);
    println!("   D = 0x{:04X}", d_after);
    
    // VERIFICACIÓN CRÍTICA:
    // Si lee ANTES del incremento → D = 0x1234 ✅ CORRECTO
    // Si lee DESPUÉS del incremento → D = 0xABCD ❌ INCORRECTO
    
    println!("\n🔍 VERIFICACIÓN:");
    if d_after == 0x1234 {
        println!("   ✅ D = 0x1234 → Lee ANTES del incremento (CORRECTO)");
    } else if d_after == 0xABCD {
        println!("   ❌ D = 0xABCD → Lee DESPUÉS del incremento (INCORRECTO)");
    } else {
        println!("   ⚠️  D = 0x{:04X} → Valor inesperado", d_after);
    }
    
    assert_eq!(u_after, data_addr + 2, "U debería incrementarse en 2");
    assert_eq!(d_after, 0x1234, 
        "D debería contener el valor ANTES del incremento (0x1234), no después (0xABCD)");
}

#[test]
fn test_std_indexed_post_increment_order() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    
    // Setup: queremos escribir 0x5678 usando STD ,U++
    let dest_addr = RAM_START + 0x100;
    
    // Inicializar U y D
    cpu.registers_mut().u = dest_addr;
    cpu.registers_mut().set_d(0x5678);
    
    println!("🔧 Setup:");
    println!("   U inicial = 0x{:04X}", dest_addr);
    println!("   D = 0x5678");
    
    // Escribir instrucción STD ,U++ (opcode 0xED 0xC1)
    let code_addr = RAM_START + 0x200;
    unsafe { &mut *memory.get() }.write(code_addr, 0xED);       // STD indexed
    unsafe { &mut *memory.get() }.write(code_addr + 1, 0xC1);   // ,U++ postbyte
    
    cpu.registers_mut().pc = code_addr;
    println!("\n📝 Instrucción:");
    println!("   PC = 0x{:04X}: STD ,U++ (0xED 0xC1)", code_addr);
    
    // Ejecutar
    cpu.execute_instruction(false, false).unwrap();
    
    // Verificar que escribió en la dirección ORIGINAL, no en U+2
    let mem = unsafe { &*memory.get() };
    let value_at_original = ((mem.read(dest_addr) as u16) << 8) | (mem.read(dest_addr + 1) as u16);
    let value_at_incremented = ((mem.read(dest_addr + 2) as u16) << 8) | (mem.read(dest_addr + 3) as u16);
    let u_after = cpu.registers().u;
    
    println!("\n⏭️  DESPUÉS:");
    println!("   U = 0x{:04X} (debería ser 0x{:04X})", u_after, dest_addr + 2);
    println!("   Memoria[0x{:04X}] = 0x{:04X}", dest_addr, value_at_original);
    println!("   Memoria[0x{:04X}] = 0x{:04X}", dest_addr + 2, value_at_incremented);
    
    println!("\n🔍 VERIFICACIÓN:");
    if value_at_original == 0x5678 {
        println!("   ✅ Escribió en dirección ORIGINAL (CORRECTO)");
    } else {
        println!("   ❌ NO escribió 0x5678 en dirección original");
    }
    
    assert_eq!(u_after, dest_addr + 2, "U debería incrementarse en 2");
    assert_eq!(value_at_original, 0x5678, 
        "STD debería escribir en la dirección ANTES del incremento");
}

