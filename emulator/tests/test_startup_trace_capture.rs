use vectrex_emulator::cpu6809::CPU;
use std::collections::HashMap;
use std::fs;

/// Test para capturar y analizar trace data del bucle de inicio BIOS
/// Este test genera output JSON que puede ser analizado para entender el problema de la diagonal
#[test]
fn test_capture_startup_trace() {
    println!("=== CAPTURA DE TRACE STARTUP BIOS ===");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path)
        .expect("No se pudo cargar la BIOS. Verificar ruta.");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS usando el método correcto
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Configurar reset vector desde BIOS
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    // Habilitar trace con límite muy amplio para capturar progreso real
    cpu.trace_enabled = true;
    cpu.trace_limit = 100_000;  // Aumentar para capturar más progreso
    cpu.trace_buf.clear();
    
    println!("BIOS cargada, PC inicial: {:04X}, iniciando captura de trace...", cpu.pc);
    
    // Ejecutar suficientes instrucciones para capturar el inicio completo y VER DÓNDE SE QUEDA ATASCADO
    let mut total_instructions = 0;
    let max_instructions = 25_000_000;  // 25 millones para ver si llega a Mine Storm
    let mut stuck_counter = 0;
    let mut pc_history = std::collections::VecDeque::new();
    
    // Ejecutar hasta que encontremos el verdadero punto donde se atasca
    while total_instructions < max_instructions {
        let step_ok = cpu.step();
        if step_ok {
            total_instructions += 1;
            
            // Mantener historial de PCs para detectar bucles largos
            pc_history.push_back(cpu.pc);
            if pc_history.len() > 100 {
                pc_history.pop_front();
            }
            
            // Verificar progreso cada 2000 instrucciones
            if total_instructions % 2000 == 0 {
                let current_trace_len = cpu.trace_buf.len();
                println!("Instrucciones ejecutadas: {}, Trace entries: {}, PC actual: {:04X}", 
                         total_instructions, current_trace_len, cpu.pc);
                
                // Detectar si se queda atascado en un bucle muy corto (real atasco)
                if pc_history.len() >= 100 {
                    let unique_pcs: std::collections::HashSet<_> = pc_history.iter().collect();
                    
                    if unique_pcs.len() <= 5 {  // Bucle muy corto = posible atasco real
                        stuck_counter += 1;
                        println!("⚠️  DETECCIÓN DE ATASCO: bucle de {} PCs únicos (contador: {})", 
                                unique_pcs.len(), stuck_counter);
                        
                        if stuck_counter >= 10 {
                            println!("🚨 EMULADOR COMPLETAMENTE ATASCADO");
                            println!("PCs del bucle: {:?}", 
                                   unique_pcs.iter().map(|pc| format!("{:04X}", pc)).collect::<Vec<_>>());
                            break;
                        }
                    } else {
                        stuck_counter = 0;  // Reset si hay progreso
                    }
                }
                
                // NO salir automáticamente en F4EB-F4EF - es solo un delay
                let recent_entries = &cpu.trace_buf[current_trace_len.saturating_sub(10)..];
                let pc_pattern: Vec<u16> = recent_entries.iter().map(|e| e.pc).collect();
                println!("Patrón de PC reciente: {:04X?}", pc_pattern);
            }
        } else {
            println!("Error en step() en PC {:04X}", cpu.pc);
            break;
        }
    }
    
    println!("\n=== ANÁLISIS DE TRACE CAPTURADO ===");
    println!("Total instrucciones ejecutadas: {}", total_instructions);
    println!("Entradas de trace capturadas: {}", cpu.trace_buf.len());
    
    // Análisis de PCs más visitados
    let mut pc_counts: HashMap<u16, usize> = HashMap::new();
    for entry in &cpu.trace_buf {
        *pc_counts.entry(entry.pc).or_insert(0) += 1;
    }
    
    let mut hot_pcs: Vec<_> = pc_counts.iter().collect();
    hot_pcs.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    println!("\nTOP 20 PCs MÁS VISITADOS (posibles bucles):");
    for (pc, count) in hot_pcs.iter().take(20) {
        println!("{:04X}: {} veces", pc, count);
    }
    
    // Buscar específicamente el rango de delay F4EB-F4EF
    let delay_entries: Vec<_> = cpu.trace_buf.iter()
        .enumerate()
        .filter(|(_, entry)| entry.pc >= 0xF4EB && entry.pc <= 0xF4EF)
        .collect();
    
    println!("\n=== ANÁLISIS BUCLE F4EB-F4EF ===");
    println!("Entradas en rango F4EB-F4EF: {}", delay_entries.len());
    
    if !delay_entries.is_empty() {
        println!("PRIMERAS 20 entradas del bucle:");
        for (idx, (trace_idx, entry)) in delay_entries.iter().take(20).enumerate() {
            let op_str = entry.op_str.as_deref().unwrap_or("UNK");
            println!("{}: trace[{}] {:04X}: {} Y={:04X}", 
                     idx, trace_idx, entry.pc, op_str, entry.y);
        }
        
        if delay_entries.len() > 20 {
            println!("...");
            println!("ÚLTIMAS 10 entradas del bucle:");
            for (idx, (trace_idx, entry)) in delay_entries.iter().rev().take(10).rev().enumerate() {
                let relative_idx = delay_entries.len() - 10 + idx;
                let op_str = entry.op_str.as_deref().unwrap_or("UNK");
                println!("{}: trace[{}] {:04X}: {} Y={:04X}", 
                         relative_idx, trace_idx, entry.pc, op_str, entry.y);
            }
        }
    }
    
    // Mostrar primeras 50 entradas del trace para ver la secuencia de inicio
    println!("\n=== PRIMERAS 50 ENTRADAS (INICIO BIOS) ===");
    for (i, entry) in cpu.trace_buf.iter().take(50).enumerate() {
        let op_str = entry.op_str.as_deref().unwrap_or("UNK");
        println!("{:3}: {:04X}: {:8} A={:02X} B={:02X} X={:04X} Y={:04X}", 
                 i, entry.pc, op_str, entry.a, entry.b, entry.x, entry.y);
    }
    
    // Mostrar últimas 20 entradas
    println!("\n=== ÚLTIMAS 20 ENTRADAS ===");
    let start_idx = cpu.trace_buf.len().saturating_sub(20);
    for (i, entry) in cpu.trace_buf.iter().skip(start_idx).enumerate() {
        let trace_idx = start_idx + i;
        let op_str = entry.op_str.as_deref().unwrap_or("UNK");
        println!("{:4}: {:04X}: {:8} A={:02X} B={:02X} X={:04X} Y={:04X}", 
                 trace_idx, entry.pc, op_str, entry.a, entry.b, entry.x, entry.y);
    }
    
    // Generar JSON para análisis externo
    let trace_json = generate_trace_json(&cpu.trace_buf);
    fs::write("startup_trace.json", trace_json)
        .expect("Error escribiendo startup_trace.json");
    
    println!("\n=== TRACE EXPORTADO ===");
    println!("Trace completo guardado en startup_trace.json");
    println!("Usar tools/analyze_trace.py para análisis detallado");
    
    // Verificar si encontramos generación de vectores
    let vector_calls: Vec<_> = cpu.trace_buf.iter()
        .enumerate()
        .filter(|(_, entry)| {
            // Buscar llamadas típicas de drawing en BIOS
            match entry.pc {
                0xF2A4 | 0xF2A6 | 0xF2A8 | 0xF2AA => true, // Draw vector calls
                _ => false
            }
        })
        .collect();
    
    println!("\n=== LLAMADAS VECTORIALES DETECTADAS ===");
    println!("Llamadas BIOS de vectores encontradas: {}", vector_calls.len());
    for (idx, (trace_idx, entry)) in vector_calls.iter().take(5).enumerate() {
        let op_str = entry.op_str.as_deref().unwrap_or("UNK");
        println!("{}: trace[{}] {:04X}: {} A={:02X} B={:02X}", 
                 idx, trace_idx, entry.pc, op_str, entry.a, entry.b);
    }
    
    // Análisis específico del problema: buscar generación de diagonal
    println!("\n=== ANÁLISIS DIAGONAL ===");
    
    // Buscar operaciones que podrían generar coordenadas (254,254) a (-30,-30)
    let suspicious_coords: Vec<_> = cpu.trace_buf.iter()
        .enumerate()
        .filter(|(_, entry)| {
            // Buscar valores sospechosos en registros
            entry.a == 254 || entry.b == 254 || entry.x == 254 || entry.y == 254 ||
            entry.a == 30 || entry.b == 30 || entry.x == 30 || entry.y == 30
        })
        .collect();
    
    println!("Entradas con coordenadas sospechosas (254 o 30): {}", suspicious_coords.len());
    for (idx, (trace_idx, entry)) in suspicious_coords.iter().take(10).enumerate() {
        let op_str = entry.op_str.as_deref().unwrap_or("UNK");
        println!("{}: trace[{}] {:04X}: {} A={:02X} B={:02X} X={:04X} Y={:04X}", 
                 idx, trace_idx, entry.pc, op_str, entry.a, entry.b, entry.x, entry.y);
    }
}

fn generate_trace_json(trace_buf: &[vectrex_emulator::cpu6809::TraceEntry]) -> String {
    use serde_json::json;
    
    let trace_array: Vec<_> = trace_buf.iter().map(|entry| {
        json!({
            "pc": entry.pc,
            "op": entry.opcode,
            "sub": entry.sub,
            "m": entry.op_str,
            "hex": format!("{:02X}", entry.opcode),
            "a": entry.a,
            "b": entry.b,
            "x": entry.x,
            "y": entry.y,
            "u": entry.u,
            "s": entry.s,
            "dp": entry.dp,
            "operand": null,
            "repeat": entry.loop_count,
            "flags": entry.flags,
            "cycles": entry.cycles,
            "illegal": entry.illegal
        })
    }).collect();
    
    serde_json::to_string_pretty(&trace_array).unwrap_or_else(|_| "[]".to_string())
}