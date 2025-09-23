// Debug: Investigar exactamente qué valor se carga en Timer2 durante Set_Refresh
// y cuántos ciclos realmente transcurren antes de llegar a Wait_Recal

use vectrex_emulator::Emulator;

#[test]
fn test_timer2_bios_timing_analysis() {
    println!("=== TIMER2 BIOS TIMING ANALYSIS ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    let mut step_count = 0;
    let mut set_refresh_detected = false;
    let mut timer2_configured = false;
    let mut timer2_config_cycle = 0;
    let mut timer2_value = 0u16;
    let mut timer2_via_write_detected = false;
    let mut timer2_via_write_cycle = 0;
    
    println!("Buscando configuración de Timer2...");
    
    while step_count < 50000 {
        let current_pc = emulator.cpu.pc;
        
        // Detectar cuando estamos en Set_Refresh
        if current_pc >= 0xF1A2 && current_pc <= 0xF1AF && !set_refresh_detected {
            set_refresh_detected = true;
            println!("🎯 Set_Refresh detectado en step {}, PC={:04X}", step_count, current_pc);
        }
        
        // Detectar escritura a Timer2 (registro VIA 0x09)
        if set_refresh_detected && !timer2_configured {
            // Leer los refresh values en RAM usando bus
            let refresh_lo = emulator.cpu.bus.read8(0xC83D);
            let refresh_hi = emulator.cpu.bus.read8(0xC83E);
            
            if refresh_lo != 0 || refresh_hi != 0 {
                timer2_configured = true;
                timer2_config_cycle = step_count;
                timer2_value = ((refresh_hi as u16) << 8) | (refresh_lo as u16);
                println!("⚡ Timer2 configurado en step {}:", step_count);
                println!("   Refresh value: {:04X} ({} decimal)", timer2_value, timer2_value);
                println!("   IER: {:02X}", emulator.cpu.bus.via_ier());
                println!("   IFR: {:02X}", emulator.cpu.bus.via_ifr());
            }
        }
        
        // Detectar Wait_Recal
        if current_pc == 0xF19E {
            println!("🔄 Wait_Recal alcanzado en step {}", step_count);
            if timer2_configured {
                let cycles_since_config = step_count - timer2_config_cycle;
                println!("   Ciclos desde config Timer2: {}", cycles_since_config);
                println!("   Timer2 debería expirar en: {} ciclos", timer2_value);
                println!("   ¿Expired? {}", cycles_since_config >= timer2_value as u64);
            }
            println!("   IER: {:02X}", emulator.cpu.bus.via_ier());
            println!("   IFR: {:02X}", emulator.cpu.bus.via_ifr());
            println!("   A-reg: {:02X} (esperando bit 0x20 en IFR)", emulator.cpu.a);
            break;
        }
        
        emulator.step();
        step_count += 1;
    }
    
    if !timer2_configured {
        println!("⚠️ Timer2 nunca se configuró en {} steps", step_count);
    }
    
    println!("=== Timer2 BIOS Timing Analysis Complete ===");
}