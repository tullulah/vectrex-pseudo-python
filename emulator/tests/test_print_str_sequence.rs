use vectrex_emulator::Emulator;
use std::fs;

#[test]
fn test_bios_flow_from_start() {
    // Test para trazar el flujo completo desde F000 hasta encontrar dónde se desvía
    let mut emu = Emulator::new();
    emu.cpu.trace = true;
    emu.cpu.trace_enabled = true;
    
    let bios_data = fs::read(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin")
        .expect("Failed to read BIOS file");
    assert!(emu.load_bios(&bios_data), "Failed to load BIOS");
    
    println!("=== Traza completa desde F000 ===");
    println!("PC inicial: {:04X}", emu.cpu.pc);
    
    // Buscar puntos clave
    let key_addresses = vec![
        (0xF18B, "Init_OS"),
        (0xF0A4, "Bucle principal"),
        (0xF0C1, "JSR Print_Str_d"), 
        (0xF37A, "Print_Str_d"),
        (0xF495, "Print_Str"),
        (0xF0D2, "LDU Vec_Run_Index"),
        (0xF542, "Clear_C8_RAM"),
        (0xF545, "Clear_x_256"),
        (0xF548, "Clear_x_d"),
    ];
    
    let mut found_addresses = Vec::new();
    let mut step_count = 0;
    let max_steps = 1000;
    
    while step_count < max_steps {
        let pc_before = emu.cpu.pc;
        
        // Verificar si estamos en una dirección clave
        for (addr, name) in &key_addresses {
            if pc_before == *addr {
                println!("\n*** LLEGÓ A {} ({:04X}) en step {} ***", name, addr, step_count);
                found_addresses.push((step_count, *addr, name.to_string()));
                
                // Estado del sistema
                println!("Registros: A={:02X} B={:02X} X={:04X} Y={:04X} U={:04X} S={:04X}", 
                    emu.cpu.a, emu.cpu.b, emu.cpu.x, emu.cpu.y, emu.cpu.u, emu.cpu.s);
                
                // Si estamos en el bucle principal, mostrar variables relevantes
                if *addr == 0xF0A4 {
                    let vec_music_flag = emu.cpu.test_read8(0xC856);
                    let vec_loop_count = (emu.cpu.test_read8(0xC835) as u16) << 8 | 
                                         emu.cpu.test_read8(0xC836) as u16;
                    println!("Vec_Music_Flag: {:02X}, Vec_Loop_Count: {:04X}", 
                        vec_music_flag, vec_loop_count);
                }
                
                // Si estamos en F0D2, verificar Vec_Run_Index
                if *addr == 0xF0D2 {
                    let vec_run_index = (emu.cpu.test_read8(0xC839) as u16) << 8 | 
                                        emu.cpu.test_read8(0xC83A) as u16;
                    println!("Vec_Run_Index: {:04X}", vec_run_index);
                }
            }
        }
        
        if !emu.step() {
            println!("Emulator stopped at step {} PC={:04X}", step_count, emu.cpu.pc);
            break;
        }
        
        step_count += 1;
        
        // Mostrar progreso cada 50 pasos
        if step_count % 50 == 0 {
            println!("Step {}: PC={:04X}", step_count, emu.cpu.pc);
        }
        
        // Detectar bucles infinitos
        if step_count > 100 {
            let recent_pcs: Vec<_> = (0..10).map(|_| {
                let old_pc = emu.cpu.pc;
                if !emu.step() { return 0; }
                step_count += 1;
                old_pc
            }).collect();
            
            // Si las últimas 10 instrucciones repiten un patrón
            let first_pc = recent_pcs[0];
            if recent_pcs.iter().all(|&pc| pc == first_pc) && first_pc != 0 {
                println!("\n!!! BUCLE INFINITO DETECTADO en {:04X} !!!", first_pc);
                break;
            }
        }
    }
    
    println!("\n=== RESUMEN ===");
    println!("Pasos totales: {}", step_count);
    println!("PC final: {:04X}", emu.cpu.pc);
    
    println!("\nDirecciones clave encontradas:");
    for (step, addr, name) in &found_addresses {
        println!("  Step {}: {:04X} ({})", step, addr, name);
    }
    
    // Verificar si llegamos a los puntos esperados
    let reached_init_os = found_addresses.iter().any(|(_, addr, _)| *addr == 0xF18B);
    let reached_main_loop = found_addresses.iter().any(|(_, addr, _)| *addr == 0xF0A4);
    let reached_print_str_d = found_addresses.iter().any(|(_, addr, _)| *addr == 0xF37A);
    
    println!("\nAnálisis:");
    println!("- Llegó a Init_OS (F18B): {}", reached_init_os);
    println!("- Llegó al bucle principal (F0A4): {}", reached_main_loop);
    println!("- Llegó a Print_Str_d (F37A): {}", reached_print_str_d);
    
    if !reached_main_loop {
        println!("⚠️  PROBLEMA: Nunca llegó al bucle principal del BIOS");
    }
    if !reached_print_str_d {
        println!("⚠️  PROBLEMA: Nunca llamó a Print_Str_d");
    }
}

#[test]
fn test_direct_to_print_str() {
    // Test que va directamente a Print_Str para depurar la secuencia
    let mut emu = Emulator::new();
    emu.cpu.trace = true;
    
    let bios_data = fs::read(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin")
        .expect("Failed to read BIOS file");
    assert!(emu.load_bios(&bios_data), "Failed to load BIOS");
    
    // Setup manual para Print_Str
    emu.cpu.dp = 0xD0; // DP = D0 para hardware I/O
    emu.cpu.u = 0xF5D9; // Copyright string pointer (conocido de la documentación)
    emu.cpu.pc = 0xF495; // Ir directamente a Print_Str
    
    // Simular dirección de retorno en stack (manualmente)
    let return_addr = 0xF0C4; // Dirección en el bucle principal
    emu.cpu.s = emu.cpu.s.wrapping_sub(2);
    emu.cpu.test_write8(emu.cpu.s, (return_addr >> 8) as u8);
    emu.cpu.test_write8(emu.cpu.s + 1, return_addr as u8);
    
    println!("=== Test Direct Print_Str ===");
    println!("PC: {:04X} (Print_Str)", emu.cpu.pc);
    println!("Stack: {:04X} (return: {:04X})", emu.cpu.s, return_addr);
    println!("U: {:04X} (string pointer)", emu.cpu.u);
    
    // Ejecutar las primeras 20 instrucciones
    for step in 0..20 {
        let pc_before = emu.cpu.pc;
        
        if !emu.step() {
            println!("Emulator stopped at step {}", step);
            break;
        }
        
        println!("Step {}: {:04X} -> {:04X} A={:02X} B={:02X} U={:04X}", 
            step + 1, pc_before, emu.cpu.pc, emu.cpu.a, emu.cpu.b, emu.cpu.u);
        
        // Si salimos de Print_Str, verificar a dónde fuimos
        if emu.cpu.pc < 0xF495 || emu.cpu.pc > 0xF520 {
            println!("Salió de Print_Str a PC={:04X}", emu.cpu.pc);
            
            if emu.cpu.pc == return_addr {
                println!("✓ Regresó correctamente al llamador");
            } else if emu.cpu.pc == 0xF354 {
                println!("→ Fue a Reset0Ref");
            } else if emu.cpu.pc == 0xF35B {
                println!("→ Fue a Reset_Pen");
            }
            break;
        }
        
        // Detectar bucle
        if step > 5 && emu.cpu.pc == pc_before {
            println!("!!! Bucle detectado en PC={:04X}", emu.cpu.pc);
            break;
        }
    }
}