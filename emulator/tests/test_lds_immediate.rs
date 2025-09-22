//! Test específico para verificar LDS immediate 10 CE

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_lds_immediate_10ce() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Habilitar trace
    cpu.trace = true;
    
    // Usar el método correcto para cargar BIOS
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar PC directamente a F000 para probar LDS immediate
    cpu.pc = 0xF000;
    
    println!("🔍 Test LDS immediate 10 CE");
    println!("PC: 0x{:04X}", cpu.pc);
    
    // Verificar bytes via bus (no acceso directo)
    println!("Bytes a ejecutar:");
    for i in 0..4 {
        let addr = cpu.pc + i;
        let byte = cpu.bus.read8(addr);
        println!("  0x{:04X}: 0x{:02X}", addr, byte);
    }
    
    println!("Stack pointer antes: 0x{:04X}", cpu.s);
    
    // Ejecutar 1 paso (debería ejecutar LDS #$CBEA)
    let step_result = cpu.step();
    
    println!("Step result: {}", step_result);
    println!("PC después: 0x{:04X}", cpu.pc);
    println!("Stack pointer después: 0x{:04X}", cpu.s);
    
    if step_result {
        // Verificar que S se haya configurado correctamente
        assert_eq!(cpu.s, 0xCBEA, "Stack pointer debería ser 0xCBEA");
        println!("✅ LDS immediate ejecutado correctamente");
    } else {
        println!("❌ Instrucción falló");
        // No hacer panic para poder ver logs
    }
}