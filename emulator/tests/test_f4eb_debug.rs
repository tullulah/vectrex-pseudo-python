use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn test_f4eb_entry_debug() {
    let mut cpu = create_cpu();
    let max_instructions = 10_000_000;
    let mut instruction_count = 0;
    let mut in_f4eb_loop = false;
    let mut f4eb_entry_count = 0;
    
    println!("=== DEBUG: Buscando entrada a F4EB ===");
    
    loop {
        let pc_before = cpu.pc;
        let b_before = cpu.b;
        let cc_before = (cpu.cc_c as u8) | ((cpu.cc_v as u8) << 1) | ((cpu.cc_z as u8) << 2) | ((cpu.cc_n as u8) << 3);
        
        // Ejecutar una instrucción
        cpu.step();
        instruction_count += 1;
        
        let pc_after = cpu.pc;
        let b_after = cpu.b;
        let cc_after = (cpu.cc_c as u8) | ((cpu.cc_v as u8) << 1) | ((cpu.cc_z as u8) << 2) | ((cpu.cc_n as u8) << 3);
        
        // Detectar entrada a F4EB desde fuera del bucle
        if pc_after == 0xF4EB && pc_before != 0xF4EF {
            f4eb_entry_count += 1;
            println!("🚨 ENTRADA #{} a F4EB desde PC={:04X} en instrucción {}", 
                     f4eb_entry_count, pc_before, instruction_count);
            println!("   B_antes={:02X} B_después={:02X} CC_antes={:02X} CC_después={:02X}", 
                     b_before, b_after, cc_before, cc_after);
            
            // Mostrar contexto de las últimas instrucciones
            if f4eb_entry_count == 1 {
                println!("   🔍 PRIMERA ENTRADA - Analizar contexto:");
                println!("   PC anterior: {:04X}", pc_before);
                
                // Leer instrucción en PC anterior
                let opcode = cpu.bus.read8(pc_before);
                println!("   Opcode en {:04X}: {:02X}", pc_before, opcode);
                
                // Si es un branch, mostrar destino
                if matches!(opcode, 0x20..=0x2F) {
                    let offset = cpu.bus.read8(pc_before + 1) as i8;
                    let target = (pc_before as i32 + 2 + offset as i32) as u16;
                    println!("   Branch target: {:04X} (offset: {:+})", target, offset);
                }
            }
            
            in_f4eb_loop = true;
        }
        
        // Detectar salida de F4EB (si ocurre)
        if in_f4eb_loop && (pc_after < 0xF4EB || pc_after > 0xF4EF) {
            println!("✅ SALIDA de F4EB hacia {:04X} en instrucción {} (B={:02X})", 
                     pc_after, instruction_count, b_after);
            in_f4eb_loop = false;
        }
        
        // En el bucle F4EB, monitorear B cada cierto número de ciclos
        if in_f4eb_loop && pc_after == 0xF4EE && instruction_count % 100 == 0 {
            println!("📊 F4EB loop check: instrucción {} B={:02X} CC={:02X}", 
                     instruction_count, b_after, cc_after);
        }
        
        // Límite de seguridad y detección de atasco
        if instruction_count >= max_instructions {
            println!("⚠️  LÍMITE ALCANZADO: {} instrucciones ejecutadas", max_instructions);
            if in_f4eb_loop {
                println!("💥 CONFIRMADO: Atascado en bucle F4EB-F4EF");
                println!("   B final: {:02X}", b_after);
                println!("   CC final: {:02X}", cc_after);
            }
            break;
        }
        
        // Detección de progreso - si llegamos a copyright o Minestorm
        if matches!(pc_after, 0xF373 | 0xF49F) {
            println!("🎯 ÉXITO: Llegamos a funciones de progreso en {:04X} tras {} instrucciones", 
                     pc_after, instruction_count);
            break;
        }
        
        // Detección temprana de atasco en F4EB
        if in_f4eb_loop && instruction_count > 1_000_000 && f4eb_entry_count > 0 {
            // Contar cuánto tiempo llevamos en F4EB
            let f4eb_duration = instruction_count - 
                (instruction_count - 1_000_000); // Aproximación
            if f4eb_duration > 500_000 {
                println!("💥 ATASCO DETECTADO: Más de 500k instrucciones en F4EB");
                println!("   B actual: {:02X}", b_after);
                println!("   CC actual: {:02X}", cc_after);
                
                // Mostrar análisis de valores de B recientes
                println!("🔍 Analizando patrón de B en las próximas 10 iteraciones:");
                for i in 0..10 {
                    let b_before_iter = cpu.b;
                    cpu.step(); // F4EB: LDA #$81
                    cpu.step(); // F4ED: NOP
                    cpu.step(); // F4EE: DECB  
                    let b_after_iter = cpu.b;
                    println!("   Iter {}: B {:02X} -> {:02X}", i+1, b_before_iter, b_after_iter);
                    cpu.step(); // F4EF: BNE
                }
                break;
            }
        }
    }
    
    println!("=== RESUMEN ===");
    println!("Instrucciones ejecutadas: {}", instruction_count);
    println!("Entradas a F4EB: {}", f4eb_entry_count);
    println!("Estado final: {}", if matches!(cpu.pc, 0xF373 | 0xF49F) { "ÉXITO" } else { "ATASCADO" });
}

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