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
fn test_print_str_internal_execution() {
    println!("=== TEST EJECUCIÓN INTERNA PRINT_STR ===");
    
    let mut cpu = create_cpu();
    
    // Llegar hasta Print_Str (F495)
    let mut print_str_reached = false;
    let mut inside_print_str = false;
    let mut f4eb_reached = false;
    let mut print_str_exit = false;
    
    for step in 0..50000 {
        let current_pc = cpu.pc;
        
        // Detectar entrada a Print_Str (F495)
        if current_pc == 0xF495 && !print_str_reached {
            println!("🎉 ENTRADA a Print_Str (F495) en paso {}", step);
            print_str_reached = true;
            inside_print_str = true;
        }
        
        // Si estamos dentro de Print_Str, rastrear cada instrucción
        if inside_print_str && !print_str_exit {
            // Imprimir todas las instrucciones dentro del rango de Print_Str
            if current_pc >= 0xF495 && current_pc < 0xF500 {
                let opcode = cpu.bus.read8(current_pc);
                println!("📍 ${:04X}: {:02X} (paso {})", current_pc, opcode, step);
                
                // Detectar llegada específica a F4EB (el loop problemático)
                if current_pc == 0xF4EB {
                    if !f4eb_reached {
                        println!("🎯 LLEGADA AL LOOP F4EB (DECB/BNE) en paso {}", step);
                        f4eb_reached = true;
                        
                        // Mostrar estado del registro B (contador del loop)
                        println!("   Registro B (contador): {:02X}", cpu.b);
                        println!("   Registro A: {:02X}", cpu.a);
                        println!("   Flags: Z={} N={}", 
                                cpu.cc_z, cpu.cc_n);
                    }
                }
                
                // Detectar si hay salto fuera de Print_Str
                if opcode == 0x39 { // RTS
                    println!("🚪 RTS - SALIDA de Print_Str en ${:04X} (paso {})", current_pc, step);
                    print_str_exit = true;
                    break;
                }
                
                // Detectar saltos fuera del rango de Print_Str
                if opcode == 0x7E { // JMP extended
                    let target = ((cpu.bus.read8(current_pc + 1) as u16) << 8) | 
                                 (cpu.bus.read8(current_pc + 2) as u16);
                    println!("🚀 JMP a ${:04X} desde Print_Str (paso {})", target, step);
                    if target < 0xF495 || target >= 0xF500 {
                        println!("   ⚠️  SALTO FUERA del rango Print_Str");
                        inside_print_str = false;
                        break;
                    }
                }
                
                // Detectar BSR/JSR que podrían llevarnos fuera
                if opcode == 0x8D || opcode == 0xAD { // BSR/JSR
                    let target = if opcode == 0x8D {
                        // BSR - relative
                        let offset = cpu.bus.read8(current_pc + 1) as i8;
                        ((current_pc as i32) + 2 + (offset as i32)) as u16
                    } else {
                        // JSR - extended
                        ((cpu.bus.read8(current_pc + 1) as u16) << 8) | 
                        (cpu.bus.read8(current_pc + 2) as u16)
                    };
                    println!("📞 BSR/JSR a ${:04X} desde Print_Str (paso {})", target, step);
                }
            } else if print_str_reached {
                // Estamos fuera del rango de Print_Str
                println!("🚪 SALIDA de Print_Str - ahora en ${:04X} (paso {})", current_pc, step);
                inside_print_str = false;
                break;
            }
        }
        
        // Verificar si llegamos a Print_Str_hwyx sin pasar por F4EB
        if current_pc == 0xF373 && print_str_reached && !f4eb_reached {
            println!("⚠️  LLEGADA DIRECTA a Print_Str_hwyx sin pasar por F4EB");
            println!("   Print_Str fue alcanzado: {}", print_str_reached);
            println!("   F4EB fue alcanzado: {}", f4eb_reached);
            break;
        }
        
        cpu.step();
        
        // Salir si alcanzamos F4EB o ejecutamos demasiados pasos dentro de Print_Str
        if f4eb_reached || step > 20000 {
            break;
        }
    }
    
    println!("\n=== RESUMEN EJECUCIÓN PRINT_STR ===");
    println!("Print_Str (F495) alcanzado: {}", print_str_reached);
    println!("Loop F4EB alcanzado: {}", f4eb_reached);
    println!("Print_Str exit detectado: {}", print_str_exit);
}