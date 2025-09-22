use vectrex_emulator::Emulator;
use std::fs;

#[test]
fn test_clear_x_d_specific_behavior() {
    let mut emu = Emulator::new();
    emu.cpu.trace = true;
    
    // Cargar BIOS
    let bios_data = fs::read(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin")
        .expect("Failed to read BIOS file");
    assert!(emu.load_bios(&bios_data), "Failed to load BIOS");
    
    println!("=== Análisis específico de Clear_x_d ===");
    
    // Ejecutar hasta llegar a Clear_x_d (F548)
    let mut step = 0;
    loop {
        let pc = emu.cpu.pc;
        if pc == 0xF548 || step > 1000 {
            break;
        }
        emu.step();
        step += 1;
    }
    
    println!("Llegamos a Clear_x_d en step {}, PC={:04X}", step, emu.cpu.pc);
    
    // Examinar el comportamiento de Clear_x_d paso a paso
    let mut clear_steps = 0;
    let max_clear_steps = 200; // Límite para evitar bucle infinito en test
    
    while emu.cpu.pc >= 0xF548 && emu.cpu.pc <= 0xF54F && clear_steps < max_clear_steps {
        let pc = emu.cpu.pc;
        let a = emu.cpu.a;
        let b = emu.cpu.b;
        let d = ((a as u16) << 8) | (b as u16);
        let cc_n = emu.cpu.cc_n;
        let cc_z = emu.cpu.cc_z;
        
        println!("Clear_x_d step {}: PC={:04X} A={:02X} B={:02X} D={:04X} N={} Z={}", 
                 clear_steps, pc, a, b, d, cc_n, cc_z);
                 
        // Si estamos en la instrucción BPL
        if pc == 0xF54D {
            println!("  -> BPL check: D={:04X} negative_flag={} zero_flag={} (should branch if N=false)", 
                     d, cc_n, cc_z);
        }
        
        emu.step();
        clear_steps += 1;
        
        // Si D llega a 0, deberíamos salir
        if d == 0 {
            println!("  -> D llegó a 0, deberíamos salir del bucle");
        }
    }
    
    println!("=== Final del análisis Clear_x_d ===");
    println!("Clear_x_d steps: {}", clear_steps);
    println!("PC final: {:04X}", emu.cpu.pc);
    let final_a = emu.cpu.a;
    let final_b = emu.cpu.b;
    let final_d = ((final_a as u16) << 8) | (final_b as u16);
    println!("Registros finales: A={:02X} B={:02X} D={:04X}", 
             final_a, final_b, final_d);
             
    if clear_steps >= max_clear_steps {
        println!("⚠️  PROBLEMA: Clear_x_d no terminó en {} pasos", max_clear_steps);
    } else {
        println!("✅ Clear_x_d terminó correctamente");
    }
}