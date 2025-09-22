use vectrex_emulator::Emulator;
use std::path::Path;

#[test]
fn test_cartridge_detection_routine() {
    let mut emulator = Emulator::new();
    
    // Cargar BIOS real
    let bios_path = Path::new(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin");
    if !bios_path.exists() {
        panic!("BIOS no encontrada en: {:?}", bios_path);
    }
    
    let bios_data = std::fs::read(bios_path).unwrap();
    let success = emulator.load_bios(&bios_data);
    if !success {
        panic!("No se pudo cargar la BIOS");
    }
    
    println!("=== ANÁLISIS DE DETECCIÓN DE CARTUCHOS ===");
    
    // Activar trace para ver todas las instrucciones
    emulator.cpu.trace = true;
    
    let mut found_ldu_0000 = false;
    let found_ldx_copyright = false;
    let mut found_ldu_e000 = false;
    let mut in_detection_routine = false;
    
    // Ejecutar hasta encontrar la rutina de detección de cartuchos
    for step in 0..10000 {
        let debug_before = emulator.debug_state();
        let pc_before = debug_before.cpu_pc;
        
        // Detectar cuando entramos en la rutina de detección (LDU #$0000)
        if pc_before >= 0xF080 && pc_before <= 0xF0A0 {
            in_detection_routine = true;
            
            if pc_before == 0xF085 || pc_before == 0xF086 {  // LDU #$0000 está cerca de F084
                found_ldu_0000 = true;
                println!("🎯 ENCONTRADO: LDU #$0000 en PC=0x{:04X}", pc_before);
                println!("   U-register antes: 0x{:04X}", debug_before.cpu_x); // Note: debug_state doesn't expose U, using X as proxy
            }
        }
        
        emulator.step();
        let debug_after = emulator.debug_state();
        let pc_after = debug_after.cpu_pc;
        
        // Detectar LDU #$E000 (salto a Mine Storm)
        if in_detection_routine && pc_after != pc_before {
            let pc_diff = if pc_after > pc_before { 
                pc_after - pc_before 
            } else { 
                pc_before - pc_after 
            };
            
            // Si hay un salto grande, podría ser el BRA hacia Mine Storm
            if pc_diff > 10 {
                println!("🔄 SALTO DETECTADO: 0x{:04X} → 0x{:04X} (diff: {})", 
                        pc_before, pc_after, pc_diff);
                        
                // Verificar si saltamos hacia 0xE000 (Mine Storm)
                if pc_after >= 0xE000 && pc_after < 0xF000 {
                    found_ldu_e000 = true;
                    println!("✅ ¡ÉXITO! Saltó a Mine Storm: 0x{:04X}", pc_after);
                    break;
                }
            }
        }
        
        // Si nos quedamos mucho tiempo en la rutina sin saltar, es problemático
        if in_detection_routine && step > 5000 {
            println!("❌ PROBLEMA: Llevamos mucho tiempo en la rutina de detección");
            println!("   PC actual: 0x{:04X}", pc_after);
            break;
        }
        
        // Si llegamos a Print_Str (F495), algo está mal
        if pc_after == 0xF495 {
            println!("❌ PROBLEMA: Llegamos a Print_Str sin saltar a Mine Storm");
            println!("   La detección de cartuchos falló");
            break;
        }
    }
    
    println!("\n=== RESUMEN DE DETECCIÓN ===");
    println!("LDU #$0000 encontrado: {}", found_ldu_0000);
    println!("LDX Copyright encontrado: {}", found_ldx_copyright);
    println!("LDU #$E000 (Mine Storm) encontrado: {}", found_ldu_e000);
    
    if found_ldu_e000 {
        println!("✅ La rutina de detección funciona correctamente");
    } else {
        println!("❌ La rutina de detección tiene problemas");
        println!("   Verificar:");
        println!("   1. ¿Se ejecuta LDU #$0000?");
        println!("   2. ¿La comparación con Copyright_Str falla correctamente?");
        println!("   3. ¿Se ejecuta LDU #$E000?");
        println!("   4. ¿El salto a Mine Storm funciona?");
    }
    
    // Verificar el contenido de la dirección 0x0000
    println!("\n=== VERIFICACIÓN MEMORIA 0x0000 ===");
    for addr in 0x0000..0x0010 {
        let val = emulator.cpu.bus.read8(addr);
        println!("0x{:04X}: 0x{:02X} ({})", addr, val, val as char);
    }
}