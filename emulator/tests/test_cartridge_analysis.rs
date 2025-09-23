use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn create_cpu() -> CPU {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path)
        .expect("No se pudo cargar la BIOS. Verificar ruta.");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y resetear
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.reset();
    
    cpu
}

#[test]
fn test_cartridge_detection_analysis() {
    let mut cpu = create_cpu();
    let max_instructions = 50_000;
    let mut instruction_count = 0;
    
    println!("=== ANÁLISIS DE DETECCIÓN DE CARTUCHO ===");
    println!("PC inicial: {:04X}", cpu.pc);
    
    // Monitorear puntos críticos del flujo de detección de cartucho
    let mut cartridge_check_started = false;
    let mut copyright_check_complete = false;
    let mut c839_final_value = None;
    
    loop {
        let pc_before = cpu.pc;
        
        // Detectar inicio de verificación de cartucho (F084)
        if pc_before == 0xF084 && !cartridge_check_started {
            cartridge_check_started = true;
            println!("🔍 INICIO verificación cartucho en PC=F084");
            
            // Examinar estado inicial para verificación
            let u_reg = cpu.u;
            let x_reg = cpu.x;
            let b_reg = cpu.b;
            println!("   U={:04X} (dirección a verificar)", u_reg);
            println!("   X={:04X} (Copyright_Str)", x_reg);
            println!("   B={:02X} (contador)", b_reg);
            
            // Mostrar primeros bytes del cartucho (debería ser $0000)
            for i in 0..8 {
                let byte_val = cpu.bus.read8(i);
                println!("   ${:04X}: {:02X}", i, byte_val);
            }
        }
        
        // Detectar cuando se decide la ruta (F092 = cartucho malo, F097 = cartucho bueno)
        match pc_before {
            0xF092 => {
                println!("🚫 CARTUCHO MALO detectado - tomando ruta Minestorm");
                println!("   Próximo: LDU #$E000 (Minestorm)");
            },
            0xF097 => {
                println!("✅ CARTUCHO BUENO detectado - usando cartucho");
                println!("   Continuando verificación...");
            },
            0xF09E => {
                if !copyright_check_complete {
                    copyright_check_complete = true;
                    println!("📋 VERIFICACIÓN COMPLETA - configurando Vec_Run_Index");
                    
                    // Capturar estado de $C839 (dirección copyright)
                    let c839_addr = 0xC839;
                    let c839_lo = cpu.bus.read8(c839_addr);
                    let c839_hi = cpu.bus.read8(c839_addr + 1);
                    let c839_value = ((c839_hi as u16) << 8) | (c839_lo as u16);
                    c839_final_value = Some(c839_value);
                    
                    println!("   $C839 = {:04X} (dirección copyright)", c839_value);
                    if c839_value == 0x0000 {
                        println!("   → SIN CARTUCHO: usará copyright interno");
                    } else {
                        println!("   → CON CARTUCHO: usará copyright del cartucho");
                    }
                }
            },
            _ => {}
        }
        
        // Ejecutar instrucción
        cpu.step();
        instruction_count += 1;
        
        let pc_after = cpu.pc;
        
        // Detectar llegada a F373 (Print_Str) para terminar
        if pc_after == 0xF373 {
            println!("🎯 LLEGADA a Print_Str (F373)");
            break;
        }
        
        // Límites de seguridad
        if instruction_count >= max_instructions {
            println!("⚠️ Límite de instrucciones alcanzado");
            break;
        }
    }
    
    println!("\n=== RESUMEN DETECCIÓN CARTUCHO ===");
    println!("Verificación iniciada: {}", cartridge_check_started);
    println!("Verificación completa: {}", copyright_check_complete);
    
    if let Some(c839_val) = c839_final_value {
        println!("$C839 final: {:04X}", c839_val);
        
        // Explicar el impacto en Print_Str
        if c839_val == 0x0000 {
            println!("PREDICCIÓN: Print_Str_d usará copyright interno → F4EB EJECUTADO");
        } else {
            println!("PREDICCIÓN: Print_Str_d usará copyright cartucho → F4EB POSIBLE");
        }
    }
    
    println!("Total instrucciones: {}", instruction_count);
    
    // Verificar estado final de memoria C839
    let final_c839_lo = cpu.bus.read8(0xC839);
    let final_c839_hi = cpu.bus.read8(0xC83A);
    let final_c839 = ((final_c839_hi as u16) << 8) | (final_c839_lo as u16);
    println!("$C839 al final del test: {:04X}", final_c839);
}