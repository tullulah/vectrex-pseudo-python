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
fn test_minestorm_progression_full() {
    println!("=== TEST PROGRESIÓN COMPLETA HACIA MINESTORM ===");
    
    let mut cpu = create_cpu();
    let max_steps = 25_000_000; // 25M steps para ver todos los loops BIOS
    
    // Marcadores de progreso críticos
    let mut copyright_reached = false;
    let mut f4eb_loop_reached = false;
    let mut f4eb_exits = 0;
    let mut print_str_exits = 0;
    let mut minestorm_reached = false;
    let mut wait_recal_loop_reached = false;
    let mut bios_init_complete = false;
    
    // Contadores VIA para debugging
    let mut via_timer1_writes = 0;
    let mut via_timer2_writes = 0;
    let mut via_interrupts = 0;
    
    let mut last_significant_pc = 0x0000;
    let mut steps_at_same_pc = 0;
    
    for step in 0..max_steps {
        let current_pc = cpu.pc;
        
        // Detectar si estamos atascados en el mismo PC
        if current_pc == last_significant_pc {
            steps_at_same_pc += 1;
            if steps_at_same_pc > 10000 {
                println!("⚠️  POSIBLE LOOP INFINITO en ${:04X} (paso {})", current_pc, step);
                println!("   VIA Timer1 writes: {}", via_timer1_writes);
                println!("   VIA Timer2 writes: {}", via_timer2_writes);
                println!("   VIA interrupts: {}", via_interrupts);
                break;
            }
        } else {
            last_significant_pc = current_pc;
            steps_at_same_pc = 0;
        }
        
        // Detectar puntos críticos
        match current_pc {
            // Copyright display
            0xF151 => {
                if !copyright_reached {
                    println!("📄 COPYRIGHT alcanzado (paso {})", step);
                    copyright_reached = true;
                }
            },
            
            // Loop F4EB (DECB/BNE en Print_Str)
            0xF4EB => {
                if !f4eb_loop_reached {
                    println!("🔄 LOOP F4EB alcanzado (paso {}) - B={:02X}", step, cpu.b);
                    f4eb_loop_reached = true;
                }
                
                // Detectar salida del loop cuando B llega a 0
                if cpu.b == 0 {
                    f4eb_exits += 1;
                    println!("   🚪 Salida #{} del loop F4EB (B=0)", f4eb_exits);
                }
            },
            
            // Salida de Print_Str (RTS)
            0xF495..=0xF4FF => {
                let opcode = cpu.bus.read8(current_pc);
                if opcode == 0x39 { // RTS
                    print_str_exits += 1;
                    println!("🚪 SALIDA #{} de Print_Str en ${:04X} (paso {})", print_str_exits, current_pc, step);
                }
            },
            
            // Wait_Recal loop (F19E - problema conocido)
            0xF19E => {
                if !wait_recal_loop_reached {
                    println!("⏳ WAIT_RECAL loop alcanzado (paso {})", step);
                    wait_recal_loop_reached = true;
                }
            },
            
            // BIOS Init_OS complete
            0xF18B => {
                if !bios_init_complete {
                    println!("🎯 BIOS Init_OS completo (paso {})", step);
                    bios_init_complete = true;
                }
            },
            
            // Minestorm entry points (búsqueda amplia)
            0x0000..=0x7FFF => {
                // Cualquier dirección en RAM podría ser Minestorm
                if current_pc != 0x0000 && !minestorm_reached {
                    println!("🎮 POSIBLE MINESTORM ENTRY en ${:04X} (paso {})", current_pc, step);
                    minestorm_reached = true;
                    
                    // Verificar si realmente es código ejecutable
                    let next_bytes = [
                        cpu.bus.read8(current_pc),
                        cpu.bus.read8(current_pc + 1),
                        cpu.bus.read8(current_pc + 2)
                    ];
                    println!("   Código: {:02X} {:02X} {:02X}", next_bytes[0], next_bytes[1], next_bytes[2]);
                    
                    // Si llegamos aquí, la BIOS completó su trabajo
                    break;
                }
            },
            
            _ => {}
        }
        
        // Monitorear escrituras VIA (críticas para timing)
        let via_base = 0xD000;
        if current_pc >= 0xF000 { // Solo en BIOS
            let opcode = cpu.bus.read8(current_pc);
            if opcode == 0x97 || opcode == 0xB7 { // STA direct/extended
                // Posible escritura a VIA
                let target = if opcode == 0x97 {
                    0xD000 | (cpu.bus.read8(current_pc + 1) as u16)
                } else {
                    ((cpu.bus.read8(current_pc + 1) as u16) << 8) | (cpu.bus.read8(current_pc + 2) as u16)
                };
                
                if target >= via_base && target < via_base + 0x20 {
                    match target - via_base {
                        0x04..=0x05 => via_timer1_writes += 1, // Timer1
                        0x06..=0x07 => via_timer2_writes += 1, // Timer2
                        _ => {}
                    }
                }
            }
        }
        
        // Contar interrupciones VIA
        let ier = cpu.bus.via_ier();
        let ifr = cpu.bus.via_ifr();
        if (ier & ifr & 0x7F) != 0 {
            via_interrupts += 1;
        }
        
        cpu.step();
        
        // Progress report cada 1M steps
        if step % 1_000_000 == 0 && step > 0 {
            println!("📊 Progreso: {} steps, PC=${:04X}, VIA T1:{} T2:{} IRQ:{}", 
                    step, current_pc, via_timer1_writes, via_timer2_writes, via_interrupts);
        }
    }
    
    println!("\n=== RESUMEN PROGRESIÓN MINESTORM ===");
    println!("Copyright alcanzado: {}", copyright_reached);
    println!("Loop F4EB alcanzado: {}", f4eb_loop_reached);
    println!("Salidas del loop F4EB: {}", f4eb_exits);
    println!("Salidas de Print_Str: {}", print_str_exits);
    println!("Wait_Recal alcanzado: {}", wait_recal_loop_reached);
    println!("BIOS Init completo: {}", bios_init_complete);
    println!("Minestorm alcanzado: {}", minestorm_reached);
    println!("VIA Timer1 writes: {}", via_timer1_writes);
    println!("VIA Timer2 writes: {}", via_timer2_writes);
    println!("VIA interrupts: {}", via_interrupts);
    println!("PC final: ${:04X}", cpu.pc);
}