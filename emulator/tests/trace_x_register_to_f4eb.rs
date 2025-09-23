// Test para rastrear el valor del registro X hasta llegar a F4EB
// Objetivo: Descubrir por qué X=0xCBE6 en lugar de X=0xCB81

use vectrex_emulator::emulator::Emulator;

#[test]
fn trace_x_register_to_f4eb() {
    println!("=== TRACE REGISTRO X HASTA F4EB ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);

    let mut step_count = 0;
    let max_steps = 2_000_000; // Aumentar límite para llegar a F4EB
    let mut x_changes = Vec::new(); // Historial de cambios de X
    let mut last_x = emulator.cpu.x;
    
    // Rastrear cambios de X hasta llegar a F4EB
    while step_count < max_steps {
        let pc = emulator.cpu.pc;
        let current_x = emulator.cpu.x;
        
        // Detectar cambios en X
        if current_x != last_x {
            let opcode = emulator.cpu.bus.read8(pc);
            x_changes.push((step_count, pc, opcode, last_x, current_x));
            
            // Solo imprimir los primeros cambios para no saturar
            if x_changes.len() <= 30 {
                println!("   Step {}: PC=0x{:04X} Op=0x{:02X} X: 0x{:04X} → 0x{:04X}", 
                        step_count, pc, opcode, last_x, current_x);
            } else if x_changes.len() == 31 {
                println!("   ... (limitando salida, continuando hasta F4EB)");
            }
            
            last_x = current_x;
        }
        
        // ¿Llegamos a F4EB?
        if pc == 0xF4EB {
            println!("\n🎯 LLEGAMOS A F4EB!");
            println!("   Paso: {}", step_count);
            println!("   X = 0x{:04X}", current_x);
            println!("   Byte bajo X = 0x{:02X}", current_x & 0xFF);
            println!("   ¿Es 0x81? {}", if (current_x & 0xFF) == 0x81 { "SÍ ✅" } else { "NO ❌" });
            
            // Analizar últimos cambios de X
            println!("\n📊 Últimos 5 cambios de X:");
            let start_idx = if x_changes.len() > 5 { x_changes.len() - 5 } else { 0 };
            for i in start_idx..x_changes.len() {
                let (step, pc, op, old_x, new_x) = x_changes[i];
                println!("   {} Step {}: PC=0x{:04X} Op=0x{:02X} X: 0x{:04X} → 0x{:04X}", 
                        if i == x_changes.len() - 1 { "👉" } else { "  " },
                        step, pc, op, old_x, new_x);
            }
            
            break;
        }
        
        // Avanzar
        emulator.step();
        step_count += 1;
    }
    
    if step_count >= max_steps {
        println!("\n⚠️  Límite de pasos alcanzado sin llegar a F4EB");
        println!("   X final = 0x{:04X}", emulator.cpu.x);
    }
    
    println!("\n📈 Resumen de cambios de X:");
    println!("   Total cambios registrados: {}", x_changes.len());
    if !x_changes.is_empty() {
        let (_, _, _, initial_x, _) = x_changes[0];
        let (_, _, _, _, final_x) = x_changes[x_changes.len() - 1];
        println!("   X inicial: 0x{:04X}", initial_x);
        println!("   X final: 0x{:04X}", final_x);
        println!("   Diferencia: 0x{:04X}", final_x.wrapping_sub(initial_x));
    }
}