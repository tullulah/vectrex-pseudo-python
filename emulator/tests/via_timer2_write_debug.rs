#[cfg(test)]
mod test {
    use vectrex_emulator::CPU;
    use std::fs;

    fn load_bios() -> Option<Vec<u8>> {
        let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
    }

    #[test]
    fn trace_via_timer2_writes() {
        println!("=== TRACE: VIA Timer2 Write Operations ===");
        
        let bios = match load_bios() { 
            Some(b)=>b, 
            None=>{ println!("❌ No se pudo cargar BIOS"); return; }
        };
        
        let mut cpu = CPU::with_pc(0xF000);
        cpu.load_bios(&bios);
        
        let max_steps = 3000;
        let mut step_count = 0;
        
        // Almacenar valores previos para detectar cambios
        let mut prev_ifr = cpu.bus.via.raw_ifr();
        
        println!("🚀 Initial state - IFR=${:02X}", prev_ifr);
        
        while step_count < max_steps {
            let old_pc = cpu.pc;
            
            // Capturar accesos a memoria VIA antes del step
            let via_range_access = old_pc >= 0xD000 && old_pc <= 0xD00F;
            
            cpu.step();
            step_count += 1;
            
            // Monitorear cambios en IFR después del step
            let curr_ifr = cpu.bus.via.raw_ifr();
            
            // Detectar cambios en IFR
            if curr_ifr != prev_ifr {
                println!("🚨 IFR CHANGE: Step {} PC=${:04X} IFR=${:02X} -> ${:02X}", 
                       step_count, old_pc, prev_ifr, curr_ifr);
            }
            
            // Log cuando estamos en el bucle TST $0D
            if old_pc == 0xF19E {
                let d009_value = cpu.bus.read8(0xD009); // Timer2 high byte directo
                println!("📊 TST Loop: Step {} PC=${:04X} [D009]=${:02X} IFR=${:02X}", 
                       step_count, old_pc, d009_value, curr_ifr);
                
                // Si llevamos mucho tiempo en este bucle, terminar
                if step_count > 1500 {
                    println!("⚠️  BREAKING: TST loop demasiado largo");
                    break;
                }
            }
            
            // Detectar accesos a rangos VIA importantes
            if via_range_access {
                println!("🏷️  VIA ACCESS: Step {} PC=${:04X} (VIA range)", step_count, old_pc);
            }
            
            // Detectar escribidas directas a Timer2 registers
            if old_pc >= 0xD008 && old_pc <= 0xD009 {
                println!("🎯 TIMER2 REG ACCESS: Step {} PC=${:04X}", step_count, old_pc);
            }
            
            // Detectar escribidas a Timer2 indirectamente (ej. STA $D009)
            if old_pc < 0xD000 {  // No estamos en VIA directamente
                // Leer la instrucción para ver si es STA a VIA
                let instr = cpu.bus.read8(old_pc);
                if instr == 0x97 || instr == 0xB7 {  // STA directo o extendido
                    let next_byte = cpu.bus.read8(old_pc + 1);
                    if next_byte == 0x08 || next_byte == 0x09 {  // Timer2 registers
                        println!("🎯 STA to Timer2: Step {} PC=${:04X} STA ${:02X}", 
                               step_count, old_pc, next_byte);
                    }
                    if instr == 0xB7 {  // Extended addressing
                        let addr_hi = next_byte;
                        let addr_lo = cpu.bus.read8(old_pc + 2);
                        let target_addr = (addr_hi as u16) << 8 | addr_lo as u16;
                        if target_addr == 0xD008 || target_addr == 0xD009 {
                            println!("🎯 STA to Timer2 Extended: Step {} PC=${:04X} STA ${:04X}", 
                                   step_count, old_pc, target_addr);
                        }
                    }
                }
            }
            
            // Actualizar valores previos
            prev_ifr = curr_ifr;
        }
        
        println!("🏁 Final state - IFR=${:02X}", prev_ifr);
        println!("Steps executed: {}", step_count);
    }
}