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
    
    // Ejecutar suficientes instrucciones para capturar el inicio completo, detectar salida del bucle de música
    // y avanzar hasta el área de Minestorm (0xF800-0xFFFF)
    let mut total_instructions = 0;
    let max_instructions = 25_000_000;  // 25 millones para ver si llega a Mine Storm
    let mut stuck_counter = 0;
    let mut pc_history = std::collections::VecDeque::new();
    let mut music_loop_detected = false;
    let mut music_loop_pc: Option<u16> = None;
    let mut music_loop_count = 0;
    let mut music_loop_exit_at: Option<u64> = None;
    let mut minestorm_entered = false;
    let mut minestorm_pc: Option<u16> = None;
    let mut vector_draw_detected = false;

    // El rango típico del bucle de espera de música es F4EB-F4EF
    while total_instructions < max_instructions {
        let step_ok = cpu.step();
        if step_ok {
            total_instructions += 1;
            pc_history.push_back(cpu.pc);
            if pc_history.len() > 100 {
                pc_history.pop_front();
            }

            // Detectar bucle de música (F4EB-F4EF)
            if (0xF4EB..=0xF4EF).contains(&cpu.pc) {
                if !music_loop_detected {
                    music_loop_detected = true;
                    music_loop_pc = Some(cpu.pc);
                    println!("[INFO] Entrando en bucle de espera de música en PC={:04X} (instrucción #{})", cpu.pc, total_instructions);
                }
                music_loop_count += 1;
            } else if music_loop_detected && music_loop_exit_at.is_none() {
                // Salida del bucle de música detectada
                music_loop_exit_at = Some(total_instructions);
                println!("[INFO] BIOS ha salido del bucle de espera de música tras {} instrucciones.", music_loop_count);
                println!("[INFO] PC actual tras salir del bucle: {:04X}", cpu.pc);
                // No romper: seguir ejecutando para ver si llega a Minestorm
            }

            // Detectar entrada en área de Minestorm (0xF800-0xFFFF)
            if !minestorm_entered && (0xF800..=0xFFFF).contains(&cpu.pc) {
                minestorm_entered = true;
                minestorm_pc = Some(cpu.pc);
                println!("[INFO] BIOS ha saltado al área de Minestorm en PC={:04X} (instrucción #{})", cpu.pc, total_instructions);
            }

            // Detectar llamada a rutinas de dibujo de vectores
            if !vector_draw_detected {
                match cpu.pc {
                    0xF2A4 | 0xF2A6 | 0xF2A8 | 0xF2AA => {
                        vector_draw_detected = true;
                        println!("[INFO] Rutina de dibujo de vectores ejecutada en PC={:04X} (instrucción #{})", cpu.pc, total_instructions);
                    }
                    _ => {}
                }
            }

            // Progreso y atasco (como antes)
            if total_instructions % 2000 == 0 {
                let current_trace_len = cpu.trace_buf.len();
                println!("Instrucciones ejecutadas: {}, Trace entries: {}, PC actual: {:04X}", 
                         total_instructions, current_trace_len, cpu.pc);
                if pc_history.len() >= 100 {
                    let unique_pcs: std::collections::HashSet<_> = pc_history.iter().collect();
                    if unique_pcs.len() <= 5 {
                        stuck_counter += 1;
                        println!("⚠️  DETECCIÓN DE ATASCO: bucle de {} PCs únicos (contador: {})", 
                                unique_pcs.len(), stuck_counter);
                        // Volcado de punteros y cadena si estamos en el bucle de F4EB
                        if (0xF4EB..=0xF4EF).contains(&cpu.pc) {
                            // Vec_Str_Ptr está en $C839/C83A
                            let vec_str_ptr = ((cpu.bus.read8(0xC839) as u16) << 8) | (cpu.bus.read8(0xC83A) as u16);
                            println!("[DEBUG] Vec_Str_Ptr: ${:04X}", vec_str_ptr);
                            // Volcar 32 bytes desde ese puntero
                            let mut bytes = vec![];
                            for offset in 0..32u16 {
                                let addr = vec_str_ptr.wrapping_add(offset);
                                let b = cpu.bus.read8(addr);
                                bytes.push(b);
                            }
                            println!("[DEBUG] Cadena en Vec_Str_Ptr (hex): {:02X?}", bytes);
                            // Mostrar como ASCII hasta $80 o fin
                            let mut ascii = String::new();
                            for &b in &bytes {
                                if b == 0x80 { break; }
                                if b >= 32 && b < 127 {
                                    ascii.push(b as char);
                                } else {
                                    ascii.push('.');
                                }
                            }
                            println!("[DEBUG] Cadena en Vec_Str_Ptr (ascii): {}", ascii);
                        }
                        if stuck_counter >= 10 {
                            println!("🚨 EMULADOR COMPLETAMENTE ATASCADO");
                            println!("PCs del bucle: {:?}", 
                                   unique_pcs.iter().map(|pc| format!("{:04X}", pc)).collect::<Vec<_>>());
                            break;
                        }
                    } else {
                        stuck_counter = 0;
                    }
                }
                let recent_entries = &cpu.trace_buf[current_trace_len.saturating_sub(10)..];
                let pc_pattern: Vec<u16> = recent_entries.iter().map(|e| e.pc).collect();
                println!("Patrón de PC reciente: {:04X?}", pc_pattern);
            }

            // Si ya entró en Minestorm y ejecutó dibujo vectorial, se puede terminar
            if minestorm_entered && vector_draw_detected {
                // Verificar si llegó a generar segmentos reales (fuera del bucle de copyright/música)
                if cpu.integrator.segments.len() > 0 {
                    println!("[INFO] BIOS ha llegado a Minestorm y ejecutado dibujo vectorial. Test finaliza.");
                    // minestorm_reached = true; // variable no existe, eliminar
                    if let Some(entry) = cpu.trace_buf.get((total_instructions.min(cpu.trace_buf.len() as u64)) as usize - 1) {
                        minestorm_pc = Some(entry.pc);
                    }
                    break;
                }
            }
        
            // ANÁLISIS DE FLAGS Z y BRANCHES críticos para copyright detection
            if total_instructions % 10000 == 0 && total_instructions > 10000 {
                // Reconstruir valor CC desde los flags booleanos
                let mut cc_byte = 0u8;
                if cpu.cc_z { cc_byte |= 0x04; }
                if cpu.cc_n { cc_byte |= 0x08; }
                if cpu.cc_c { cc_byte |= 0x01; }
                if cpu.cc_v { cc_byte |= 0x02; }
                if cpu.cc_h { cc_byte |= 0x20; }
                if cpu.cc_f { cc_byte |= 0x40; }
                if cpu.cc_e { cc_byte |= 0x80; }
                if cpu.cc_i { cc_byte |= 0x10; }
                
                println!("[DEBUG] PC={:04X} Z={} CC={:02X}", cpu.pc, cpu.cc_z, cc_byte);
                
                // Detectar branches críticos de copyright
                if cpu.pc >= 0xF180 && cpu.pc <= 0xF1C0 {
                    let opcode = cpu.bus.read8(cpu.pc);
                    match opcode {
                        0x26 => println!("[BRANCH] BNE en {:04X}, Z={}, saltará: {}", cpu.pc, cpu.cc_z, !cpu.cc_z),
                        0x27 => println!("[BRANCH] BEQ en {:04X}, Z={}, saltará: {}", cpu.pc, cpu.cc_z, cpu.cc_z),
                        _ => {}
                    }
                }
                
                // ANÁLISIS DE Vec_Str_Ptr y memoria de copyright
                // Vec_Str_Ptr está en $C839/C83A - vamos a leer ese puntero y volcar su contenido
                let vec_str_ptr_low = cpu.bus.read8(0xC83A);
                let vec_str_ptr_high = cpu.bus.read8(0xC839);
                let vec_str_ptr = ((vec_str_ptr_high as u16) << 8) | (vec_str_ptr_low as u16);
                
                println!("[DEBUG] Vec_Str_Ptr: ${:04X}", vec_str_ptr);
                
                // Volcar contenido de la dirección apuntada por Vec_Str_Ptr
                if vec_str_ptr != 0 {
                    print!("[DEBUG] Contenido en Vec_Str_Ptr: ");
                    for i in 0..32 {
                        let byte_val = cpu.bus.read8(vec_str_ptr + i);
                        print!("{:02X} ", byte_val);
                    }
                    println!();
                    
                    print!("[DEBUG] ASCII: ");
                    for i in 0..32 {
                        let byte_val = cpu.bus.read8(vec_str_ptr + i);
                        if byte_val >= 32 && byte_val <= 126 {
                            print!("{}", byte_val as char);
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                    
                    // También volcar memoria alrededor de $C800-$C900 (área típica de datos de cartucho)
                    println!("[DEBUG] Memoria C800-C81F:");
                    for row in 0..2 {
                        print!("[DEBUG] C{:03X}: ", 0x800 + row * 16);
                        for col in 0..16 {
                            let addr = 0xC800 + row * 16 + col;
                            let byte_val = cpu.bus.read8(addr);
                            print!("{:02X} ", byte_val);
                        }
                        println!();
                    }
                } else {
                    println!("[DEBUG] Vec_Str_Ptr es NULL (0x0000) - problema detectado!");
                }
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
            "flags_post": entry.flags_post,
            "cycles": entry.cycles,
            "illegal": entry.illegal
        })
    }).collect();
    
    serde_json::to_string_pretty(&trace_array).unwrap_or_else(|_| "[]".to_string())
}