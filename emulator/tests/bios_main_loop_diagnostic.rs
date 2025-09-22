use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let data = std::fs::read(path).expect("BIOS real requerida para test");
    assert_eq!(data.len(), 8192, "BIOS size inesperado");
    for (i, b) in data.iter().enumerate() { 
        let addr = 0xE000 + i as u16; 
        cpu.mem[addr as usize] = *b; 
        cpu.bus.mem[addr as usize] = *b; 
    }
    cpu.bios_present = true;
}

#[test]
fn bios_main_loop_diagnostic() {
    println!("=== BIOS Main Loop Diagnostic ===");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS real
    load_real_bios(&mut cpu);
    println!("BIOS loaded");
    
    cpu.reset();
    
    println!("Initial state after reset:");
    println!("PC: {:04X}, SP: {:04X}, DP: {:02X}", cpu.pc, cpu.s, cpu.dp);
    
    // Ejecutar más tiempo para encontrar el bucle principal
    let mut main_loop_candidates = Vec::new();
    let mut pc_frequency = std::collections::HashMap::new();
    let mut vector_list_accesses = Vec::new();
    let mut dac_writes = Vec::new();
    
    for step in 0..20000 {
        let old_pc = cpu.pc;
        
        // Monitorear accesos a memoria que podrían ser listas de vectores (rango típico C800-CFFF)
        let checking_vector_area = cpu.pc >= 0xC800 && cpu.pc <= 0xCFFF;
        
        // Monitorear escrituras al DAC (VIA Port A/B pueden controlar el DAC)
        let old_via_porta = cpu.bus.via.read(0x01);
        let old_via_portb = cpu.bus.via.read(0x00);
        
        cpu.step();
        
        // Detectar cambios en puertos VIA (posibles señales DAC)
        let new_via_porta = cpu.bus.via.read(0x01);
        let new_via_portb = cpu.bus.via.read(0x00);
        
        if new_via_porta != old_via_porta || new_via_portb != old_via_portb {
            dac_writes.push((step, old_pc, old_via_porta, new_via_porta, old_via_portb, new_via_portb));
        }
        
        // Contar frecuencia de PCs para detectar bucles
        *pc_frequency.entry(cpu.pc).or_insert(0) += 1;
        
        // Detectar posibles accesos a listas de vectores
        if checking_vector_area {
            vector_list_accesses.push((step, old_pc, cpu.pc));
        }
        
        // Cada 2000 pasos, reportar estado
        if step % 2000 == 0 && step > 0 {
            println!("Step {}: PC={:04X}, Cycles={}, VIA_T1={:04X}, VIA_T2={:04X}, PortA={:02X}, PortB={:02X}", 
                step, cpu.pc, cpu.cycles,
                cpu.bus.via.read(0x04) as u16 | ((cpu.bus.via.read(0x05) as u16) << 8),
                cpu.bus.via.read(0x08) as u16 | ((cpu.bus.via.read(0x09) as u16) << 8),
                cpu.bus.via.read(0x01), cpu.bus.via.read(0x00)
            );
        }
        
        // Detectar si estamos en un bucle estable
        if step > 5000 {
            if let Some((&most_frequent_pc, &count)) = pc_frequency.iter().max_by_key(|(_, &count)| count) {
                if count > 200 && !main_loop_candidates.contains(&most_frequent_pc) {
                    main_loop_candidates.push(most_frequent_pc);
                    println!("*** Potential main loop detected at PC {:04X} (visited {} times) ***", most_frequent_pc, count);
                }
            }
        }
        
        // Salir si detectamos que estamos en el bucle principal estable
        if step > 10000 && main_loop_candidates.len() > 0 {
            println!("=== Main loop analysis after {} steps ===", step);
            break;
        }
    }
    
    println!("\n=== Final Analysis ===");
    println!("PC: {:04X}, Cycles: {}", cpu.pc, cpu.cycles);
    
    // Top 10 PCs más visitados
    let mut pc_vec: Vec<_> = pc_frequency.iter().collect();
    pc_vec.sort_by_key(|(_, &count)| std::cmp::Reverse(count));
    
    println!("\nTop 10 most visited PCs (potential loop locations):");
    for (i, (&pc, &count)) in pc_vec.iter().take(10).enumerate() {
        println!("  {}: {:04X} (visited {} times)", i+1, pc, count);
    }
    
    println!("\nVector list area accesses (C800-CFFF): {}", vector_list_accesses.len());
    for (step, from_pc, to_pc) in vector_list_accesses.iter().take(10) {
        println!("  Step {}: {:04X} -> {:04X}", step, from_pc, to_pc);
    }
    
    println!("\nDAC writes (VIA Port changes): {}", dac_writes.len());
    for (step, pc, old_a, new_a, old_b, new_b) in dac_writes.iter().take(10) {
        println!("  Step {} (PC {:04X}): PortA {:02X}->{:02X}, PortB {:02X}->{:02X}", step, pc, old_a, new_a, old_b, new_b);
    }
    
    println!("\n=== BIOS Calls Summary ===");
    for (i, call) in cpu.bios_calls.iter().enumerate() {
        println!("  {}: {}", i+1, call);
        if i >= 20 { println!("  ... and {} more", cpu.bios_calls.len() - 20); break; }
    }
    
    // Análisis específico de Draw_VL
    println!("\n=== Draw_VL Analysis ===");
    let draw_vl_calls = cpu.bios_calls.iter().filter(|call| call.contains("Draw_VL")).count();
    println!("Draw_VL calls detected: {}", draw_vl_calls);
    
    if draw_vl_calls == 0 {
        println!("*** No Draw_VL calls detected - this may explain why no vectors are being drawn ***");
        println!("*** The BIOS may be waiting for a cartridge or user input ***");
    }
}