use vectrex_emulator::Emulator;
use std::fs;

#[test]
fn test_clear_x_d_final_countdown() {
    let mut emu = Emulator::new();
    emu.cpu.trace = true;
    
    // Cargar BIOS
    let bios_data = fs::read(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin")
        .expect("Failed to read BIOS file");
    assert!(emu.load_bios(&bios_data), "Failed to load BIOS");
    
    println!("=== Test específico: ¿Qué pasa cuando D llega a 0? ===");
    
    // Ejecutar hasta llegar a Clear_x_d
    let mut step = 0;
    loop {
        let pc = emu.cpu.pc;
        if pc == 0xF548 || step > 1000 {
            break;
        }
        emu.step();
        step += 1;
    }
    
    // Simular directamente Clear_x_d con D=3 para ver los últimos pasos
    println!("Configurando registros para simular los últimos pasos:");
    emu.cpu.a = 0x00;
    emu.cpu.b = 0x03;  // Empezar con D=3 para ver qué pasa
    emu.cpu.pc = 0xF548;
    
    let mut clear_steps = 0;
    let max_clear_steps = 20; // Solo unos pocos pasos
    
    while emu.cpu.pc >= 0xF548 && emu.cpu.pc <= 0xF54F && clear_steps < max_clear_steps {
        let pc = emu.cpu.pc;
        let a = emu.cpu.a;
        let b = emu.cpu.b;
        let d = ((a as u16) << 8) | (b as u16);
        let cc_n = emu.cpu.cc_n;
        let cc_z = emu.cpu.cc_z;
        
        println!("Step {}: PC={:04X} A={:02X} B={:02X} D={:04X} N={} Z={}", 
                 clear_steps, pc, a, b, d, cc_n, cc_z);
                 
        if pc == 0xF54D {
            println!("  -> BPL: D={:04X} N={} (branch si N=false, continue si N=true)", d, cc_n);
            if d == 0 {
                println!("  -> D=0: BPL debería NO saltar (0 no es positivo)");
            }
        }
        
        if pc == 0xF54F {
            println!("  -> RTS: ¡Saliendo de Clear_x_d!");
            break;
        }
        
        emu.step();
        clear_steps += 1;
        
        // Si D llega a 0, ver exactamente qué pasa
        let new_d = ((emu.cpu.a as u16) << 8) | (emu.cpu.b as u16);
        if new_d == 0 {
            println!("  -> ¡D llegó a 0! Registros actuales: A={:02X} B={:02X} N={} Z={}", 
                     emu.cpu.a, emu.cpu.b, emu.cpu.cc_n, emu.cpu.cc_z);
        }
    }
    
    println!("=== Resultado final ===");
    println!("PC final: {:04X}", emu.cpu.pc);
    let final_d = ((emu.cpu.a as u16) << 8) | (emu.cpu.b as u16);
    println!("D final: {:04X}", final_d);
    
    if emu.cpu.pc == 0xF54F {
        println!("✅ Clear_x_d terminó correctamente con RTS");
    } else if emu.cpu.pc > 0xF54F {
        println!("✅ Clear_x_d salió del rango (probablemente terminó)");
    } else {
        println!("⚠️  Clear_x_d sigue en bucle");
    }
}