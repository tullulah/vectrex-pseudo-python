use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn create_cpu() -> CPU {
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path)
        .expect("No se pudo cargar la BIOS. Verificar ruta.");
    
    let mut cpu = CPU::default();
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.reset();
    cpu
}

#[test]
fn test_copyright_flow_analysis() {
    let mut cpu = create_cpu();
    let max_instructions = 50_000;
    let mut instruction_count = 0;
    
    println!("=== ANÁLISIS FLUJO COPYRIGHT/HIGH SCORE ===");
    
    // Puntos críticos a monitorear
    let mut copyright_display_reached = false;
    let mut high_score_check_reached = false;
    let mut print_str_d_calls = 0;
    let mut print_str_calls = 0;
    let mut print_str_hwyx_calls = 0;
    
    loop {
        let pc_before = cpu.pc;
        
        // Detectar puntos críticos del flujo
        match pc_before {
            0xF151 => {
                if !copyright_display_reached {
                    copyright_display_reached = true;
                    println!("🎨 DISPLAY COPYRIGHT - Línea 156 del BIOS");
                    
                    // Examinar estado del copyright
                    let c839_lo = cpu.bus.read8(0xC839);
                    let c839_hi = cpu.bus.read8(0xC83A);
                    let c839_value = ((c839_hi as u16) << 8) | (c839_lo as u16);
                    
                    println!("   $C839 (copyright ptr): {:04X}", c839_value);
                    println!("   D register: {:04X}", ((cpu.a as u16) << 8) | (cpu.b as u16));
                    println!("   Próximo: LDU $C839, JSR Print_Str_d");
                }
            },
            0xF159 => {
                if !high_score_check_reached {
                    high_score_check_reached = true;
                    println!("🏆 HIGH SCORE CHECK - Línea 159 del BIOS");
                    
                    let c83b_value = cpu.bus.read8(0xC83B);
                    println!("   $C83B (high score flag): {:02X}", c83b_value);
                    println!("   A register: {:02X}", cpu.a);
                    
                    if c83b_value != 0 {
                        println!("   → SALTARÁ high score display (BNE LF0D2)");
                    } else {
                        println!("   → MOSTRARÁ high score (continúa a Print_Str_d)");
                    }
                }
            },
            0xF383 => {
                print_str_d_calls += 1;
                println!("📄 LLAMADA #{} a Print_Str_d (F383)", print_str_d_calls);
                println!("   U register: {:04X}", cpu.u);
                println!("   Esta función hace JMP Print_Str → F495 → contiene F4EB");
            },
            0xF373 => {
                print_str_hwyx_calls += 1;
                if print_str_hwyx_calls <= 3 {
                    println!("📄 LLAMADA #{} a Print_Str_hwyx (F373)", print_str_hwyx_calls);
                    println!("   Esta función NO contiene F4EB");
                }
            },
            0xF495 => {
                print_str_calls += 1;
                if print_str_calls <= 3 {
                    println!("🎯 LLAMADA #{} a Print_Str (F495) - AQUÍ ESTÁ F4EB", print_str_calls);
                }
            },
            _ => {}
        }
        
        // Ejecutar instrucción
        cpu.step();
        instruction_count += 1;
        
        let pc_after = cpu.pc;
        
        // Terminar cuando lleguemos a alguna función print
        if matches!(pc_after, 0xF373 | 0xF495) && instruction_count > 1000 {
            println!("🏁 LLEGADA a función print: {:04X}", pc_after);
            break;
        }
        
        // Límite de seguridad
        if instruction_count >= max_instructions {
            println!("⚠️ Límite de instrucciones alcanzado");
            break;
        }
    }
    
    println!("\n=== RESUMEN FLUJO COPYRIGHT ===");
    println!("Copyright display alcanzado: {}", copyright_display_reached);
    println!("High score check alcanzado: {}", high_score_check_reached);
    println!("Llamadas a Print_Str_d (F383): {}", print_str_d_calls);
    println!("Llamadas a Print_Str (F495): {}", print_str_calls);
    println!("Llamadas a Print_Str_hwyx (F373): {}", print_str_hwyx_calls);
    
    println!("\n=== ESTADO FINAL MEMORIA ===");
    let final_c839 = ((cpu.bus.read8(0xC83A) as u16) << 8) | (cpu.bus.read8(0xC839) as u16);
    let final_c83b = cpu.bus.read8(0xC83B);
    println!("$C839 (copyright ptr): {:04X}", final_c839);
    println!("$C83B (high score flag): {:02X}", final_c83b);
    
    println!("Total instrucciones: {}", instruction_count);
    
    // Predicción basada en llamadas
    if print_str_calls > 0 {
        println!("\n🔍 PREDICCIÓN: Este emulador EJECUTARÁ F4EB (usa Print_Str)");
    } else if print_str_hwyx_calls > 0 {
        println!("\n🔍 PREDICCIÓN: Este emulador NO ejecutará F4EB (usa solo Print_Str_hwyx)");
    }
}