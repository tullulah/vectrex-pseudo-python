//! Test super detallado para debuggear el problema de lectura

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_debug_read_issue() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Usar el método correcto para cargar BIOS
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    println!("🔍 Debug lectura detallada");
    
    // Verificar acceso directo a memoria
    println!("Acceso directo cpu.mem:");
    for addr in 0xF000..=0xF003 {
        let byte = cpu.mem[addr as usize];
        println!("  cpu.mem[0x{:04X}] = 0x{:02X}", addr, byte);
    }
    
    // Verificar acceso via bus.read8()
    println!("Acceso via bus.read8():");
    for addr in 0xF000..=0xF003 {
        let byte = cpu.bus.read8(addr);
        println!("  bus.read8(0x{:04X}) = 0x{:02X}", addr, byte);
    }
    
    // Configurar PC y hacer una lectura manual como en el código
    cpu.pc = 0xF001; // Después de leer el opcode 0x10
    
    println!("PC configurado en 0x{:04X}", cpu.pc);
    
    // Simular la lectura que hace el código en línea 1725
    let bop = cpu.bus.read8(cpu.pc);
    println!("bus.read8(PC=0x{:04X}) = 0x{:02X}", cpu.pc, bop);
    
    // Verificar que debería ser 0xCE
    assert_eq!(bop, 0xCE, "Debería leer 0xCE en 0xF001");
    
    println!("✅ Lectura correcta");
}