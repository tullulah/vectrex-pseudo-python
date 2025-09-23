// Debug: ¿Por qué la BIOS no llega a Set_Refresh?
// Investigar el flujo desde el reset hasta encontrar dónde se queda

use vectrex_emulator::Emulator;

#[test]
fn test_set_refresh_path_debug() {
    println!("=== DEBUGGING PATH TO SET_REFRESH ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    let mut step_count = 0;
    let max_steps = 100000; // Incrementar límite
    
    let mut last_pc_sequence = std::collections::VecDeque::new();
    let max_sequence_len = 20;
    
    println!("Starting from PC={:04X}", emulator.cpu.pc);
    
    while step_count < max_steps {
        let current_pc = emulator.cpu.pc;
        
        // Detectar Set_Refresh (F1A2-F1AF)
        if current_pc >= 0xF1A2 && current_pc <= 0xF1AF {
            println!("🎯 REACHED Set_Refresh at PC={:04X} after {} steps!", current_pc, step_count);
            break;
        }
        
        // Detectar bucles infinitos
        last_pc_sequence.push_back(current_pc);
        if last_pc_sequence.len() > max_sequence_len {
            last_pc_sequence.pop_front();
        }
        
        if last_pc_sequence.len() == max_sequence_len {
            let is_loop = last_pc_sequence.iter().all(|&pc| pc == current_pc);
            if is_loop {
                println!("⚠️ Detected infinite loop at PC={:04X} after {} steps", current_pc, step_count);
                println!("Recent PC sequence: {:?}", last_pc_sequence);
                break;
            }
        }
        
        // Log puntos importantes
        match current_pc {
            0xF000 => println!("Step {}: Reset vector start at F000", step_count),
            0xF06B => println!("Step {}: Cold_Start at F06B", step_count),
            0xF1B0 => println!("Step {}: Init_VIA_Timer at F1B0", step_count),
            0xF1A2 => println!("Step {}: Set_Refresh at F1A2", step_count),
            0xF19E => println!("Step {}: Wait_Recal at F19E", step_count),
            0xF4EB => println!("Step {}: Timer1 delay area at F4EB", step_count),
            _ => {}
        }
        
        // Log cada 10k steps para mostrar progreso
        if step_count % 10000 == 0 && step_count > 0 {
            println!("Step {}: PC={:04X}", step_count, current_pc);
        }
        
        emulator.step();
        step_count += 1;
    }
    
    if step_count >= max_steps {
        println!("⚠️ Reached max steps without reaching Set_Refresh");
        println!("Final PC: {:04X}", emulator.cpu.pc);
        println!("Recent PC sequence: {:?}", last_pc_sequence);
    }
    
    println!("=== Set_Refresh Path Debug Complete ===");
}