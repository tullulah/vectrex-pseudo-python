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
fn test_print_str_d_jump_sequence() {
    println!("=== TEST ESPECÍFICO PRINT_STR_D JUMP ===");
    
    let mut cpu = create_cpu();
    
    // Paso 1: Llegar hasta el primer JSR Print_Str_d
    let mut print_str_d_reached = false;
    let mut print_str_d_pc: u16 = 0;
    let mut moveto_d_7f_calls = 0;
    let mut delay_1_calls = 0;
    let mut jmp_print_str_executed = false;
    let mut print_str_reached = false;
    
    for step in 0..50000 {
        let current_pc = cpu.pc;
        
        // Detectar entrada a Print_Str_d
        if current_pc == 0xF37A { // Print_Str_d según BIOS
            if !print_str_d_reached {
                println!("📍 LLEGADA a Print_Str_d en paso {}", step);
                print_str_d_reached = true;
                print_str_d_pc = current_pc;
            }
        }
        
        // Tras entrar a Print_Str_d, rastrear secuencia
        if print_str_d_reached && !jmp_print_str_executed {
            // Detectar JSR Moveto_d_7F (primera instrucción de Print_Str_d)
            if current_pc > 0xF37A && current_pc < 0xF380 {
                let opcode = cpu.bus.read8(current_pc);
                if opcode == 0xAD { // JSR extended
                    let target = ((cpu.bus.read8(current_pc + 1) as u16) << 8) | 
                                 (cpu.bus.read8(current_pc + 2) as u16);
                    println!("🎯 JSR desde Print_Str_d a ${:04X} (paso {})", target, step);
                    if target == 0xF2FC { // Moveto_d_7F
                        moveto_d_7f_calls += 1;
                    }
                }
            }
            
            // Detectar JSR Delay_1
            if current_pc > 0xF37C && current_pc < 0xF380 {
                let opcode = cpu.bus.read8(current_pc);
                if opcode == 0xAD { // JSR extended  
                    let target = ((cpu.bus.read8(current_pc + 1) as u16) << 8) | 
                                 (cpu.bus.read8(current_pc + 2) as u16);
                    println!("⏱️  JSR Delay_1 a ${:04X} (paso {})", target, step);
                    delay_1_calls += 1;
                }
            }
            
            // Detectar JMP Print_Str (crítico)
            if current_pc > 0xF37E && current_pc < 0xF383 {
                let opcode = cpu.bus.read8(current_pc);
                if opcode == 0x7E { // JMP extended
                    let target = ((cpu.bus.read8(current_pc + 1) as u16) << 8) | 
                                 (cpu.bus.read8(current_pc + 2) as u16);
                    println!("🚀 JMP Print_Str a ${:04X} (paso {})", target, step);
                    jmp_print_str_executed = true;
                    
                    if target == 0xF495 {
                        println!("✅ JMP dirigido correctamente a Print_Str (F495)");
                    } else {
                        println!("❌ JMP dirigido incorrectamente a ${:04X} (esperado F495)", target);
                    }
                }
            }
        }
        
        // Detectar llegada a Print_Str (F495)
        if current_pc == 0xF495 {
            if !print_str_reached {
                println!("🎉 LLEGADA a Print_Str (F495) en paso {}", step);
                print_str_reached = true;
            }
        }
        
        // Detectar llegada directa a Print_Str_hwyx (problema)
        if current_pc == 0xF373 && print_str_d_reached && !print_str_reached {
            println!("⚠️  SALTO DIRECTO a Print_Str_hwyx (F373) sin pasar por Print_Str (F495)");
            println!("   - Moveto_d_7F calls: {}", moveto_d_7f_calls);
            println!("   - Delay_1 calls: {}", delay_1_calls);
            println!("   - JMP Print_Str ejecutado: {}", jmp_print_str_executed);
            break;
        }
        
        // Detectar llegada a F4EB (el loop problemático)
        if current_pc == 0xF4EB {
            println!("🎯 LLEGADA al loop F4EB (DECB/BNE dentro de Print_Str)");
            break;
        }
        
        cpu.step();
        
        // Si llegamos a Print_Str, podemos parar aquí
        if print_str_reached {
            println!("✅ Secuencia completa: Print_Str_d → Print_Str alcanzada");
            break;
        }
    }
    
    println!("\n=== RESUMEN JUMP SEQUENCE ===");
    println!("Print_Str_d alcanzado: {}", print_str_d_reached);
    println!("Moveto_d_7F calls: {}", moveto_d_7f_calls);
    println!("Delay_1 calls: {}", delay_1_calls);
    println!("JMP Print_Str ejecutado: {}", jmp_print_str_executed);
    println!("Print_Str (F495) alcanzado: {}", print_str_reached);
}