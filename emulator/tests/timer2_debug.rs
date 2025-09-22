#[cfg(test)]
mod test {
    use vectrex_emulator::CPU;
    use std::fs;

    fn load_bios() -> Option<Vec<u8>> {
        let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
    }

    #[test]
    fn trace_timer2_behavior() {
        // Configurar variables de environment para tracing
        std::env::set_var("VIA_T2_TRACE", "1");
        std::env::set_var("IRQ_TRACE", "1");
        
        println!("=== TRACE: Timer2 Configuration & Behavior ===");
        
        let bios = match load_bios() { 
            Some(b)=>b, 
            None => { 
                eprintln!("[SKIP] BIOS real no encontrada"); 
                return; 
            } 
        };
        
        let mut cpu = CPU::with_pc(0xF000);
        cpu.load_bios(&bios);
        
        let mut step_count = 0;
        let max_steps = 5000;  // Aumentar para capturar Set_Refresh
        let writes_to_d008: Vec<(usize, u16, u8)> = Vec::new();
        let writes_to_d009: Vec<(usize, u16, u8)> = Vec::new();
        
        while step_count < max_steps {
            let old_pc = cpu.pc;
            let via_ifr_before = cpu.bus.via.raw_ifr();
            
            // Ejecutar una instrucción
            cpu.step();
            step_count += 1;
            
            let via_ifr_after = cpu.bus.via.raw_ifr();
            
            // Detectar escrituras a Timer2 indirectamente observando cambios en la VIA
            // En el primer test anterior vimos muchas escrituras a 0xD000, pero ninguna a D008/D009
            // Esto sugiere que o nunca llegamos a Set_Refresh, o hay un mapeo diferente
            
            // Log actividad del PC que nos pueda indicar si llegamos a Set_Refresh
            if old_pc == 0xF1A2 {  // Set_Refresh routine (exact address from VECTREX.I)
                println!("🎯 PC AT Set_Refresh: Step {} PC=${:04X}", step_count, old_pc);
            }
            
            if old_pc >= 0xF1A0 && old_pc <= 0xF1B0 {  // Set_Refresh range (ampliado)
                println!("📍 IN Set_Refresh RANGE: Step {} PC=${:04X} IFR=${:02X}", 
                       step_count, old_pc, via_ifr_after);
                
                // Leer el valor de Vec_Rfrsh durante Set_Refresh
                let rfrsh_lo = cpu.bus.read8(0xC83D);
                let rfrsh_hi = cpu.bus.read8(0xC83E);
                let rfrsh_val = (rfrsh_hi as u16) << 8 | rfrsh_lo as u16;
                println!("🔧 Vec_Rfrsh value: ${:04X} (lo=${:02X} hi=${:02X})", 
                       rfrsh_val, rfrsh_lo, rfrsh_hi);
            }
            
            // Detectar cambios en IFR flags del Timer2 (bit 5 = 0x20)
            let ifr_t2_before = via_ifr_before & 0x20;
            let ifr_t2_after = via_ifr_after & 0x20;
            if ifr_t2_before != ifr_t2_after {
                println!("🔥 T2 IFR CHANGE: Step {} PC=${:04X} IFR T2 bit: {} -> {} (full IFR: ${:02X} -> ${:02X})",
                       step_count, old_pc, ifr_t2_before != 0, ifr_t2_after != 0, via_ifr_before, via_ifr_after);
            }
            
            // Detectar el loop infinito específico
            if cpu.pc == 0xF19E {
                let ifr_val = cpu.bus.via.raw_ifr();
                println!("🔄 AT LOOP ADDRESS: Step {} PC=${:04X} IFR=${:02X} T2_bit={}", 
                       step_count, cpu.pc, ifr_val, (ifr_val & 0x20) != 0);
                
                // Salir del loop si ya detectamos el problema
                if step_count > 2000 {
                    println!("⚠️  BREAKING: Loop detected for too long");
                    break;
                }
            }
            
            // Logging detallado cuando PC está en rutinas de Timer o Init
            if (old_pc >= 0xF180 && old_pc <= 0xF1B0) || (old_pc >= 0xF000 && old_pc <= 0xF050) {
                println!("📍 BIOS ROUTINE: Step {} PC=${:04X} IFR=${:02X}", 
                       step_count, old_pc, via_ifr_after);
            }
        }
        
        println!("\nFinal IFR: ${:02X}", cpu.bus.via.raw_ifr());
        println!("Final PC: ${:04X}", cpu.pc);
        
        if writes_to_d008.len() > 0 {
            println!("Timer2 Low writes: {:?}", writes_to_d008);
        }
        if writes_to_d009.len() > 0 {
            println!("Timer2 High writes: {:?}", writes_to_d009);
        }
    }
}