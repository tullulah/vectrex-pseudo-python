//! Test diagnóstico para investigar por qué se detiene la CPU

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_bios_cpu_diagnostic() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Habilitar trace para debugging
    cpu.trace = true;
    
    // Usar el método correcto para cargar BIOS
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar reset vector desde BIOS (via bus)
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("🔍 Test diagnóstico CPU");
    println!("Reset vector: 0x{:04X}", reset_vector);
    println!("PC inicial: 0x{:04X}", cpu.pc);
    
    // Verificar que la BIOS se cargó correctamente
    println!("BIOS bytes en reset vector:");
    println!("  0xFFFE: 0x{:02X}", cpu.bus.read8(0xFFFE));
    println!("  0xFFFF: 0x{:02X}", cpu.bus.read8(0xFFFF));
    println!("  PC start (0x{:04X}): 0x{:02X}", reset_vector, cpu.bus.read8(reset_vector));
    
    // Intentar algunos pasos con trace habilitado
    for step in 1..=10 {
        println!("\n--- Step {} ---", step);
        println!("Pre-step: PC=0x{:04X}", cpu.pc);
        
        let step_result = cpu.step();
        println!("Step result: {}", step_result);
        println!("Post-step: PC=0x{:04X}", cpu.pc);
        
        if !step_result {
            println!("❌ CPU se detuvo en step {}", step);
            break;
        }
    }
}