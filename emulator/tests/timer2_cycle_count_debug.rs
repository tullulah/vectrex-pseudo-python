// Test para verificar cuántos ciclos exactos procesa Timer2 por instrucción TST

#[cfg(test)]
mod test {
    use vectrex_emulator::CPU;
    use std::fs;

    fn load_bios() -> Option<Vec<u8>> {
        let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
    }

    #[test]
    fn test_timer2_cycle_accuracy() {
        let bios = match load_bios() { 
            Some(b)=>b, 
            None=>{println!("❌ BIOS no encontrado"); return;} 
        };
        
        let mut cpu = CPU::with_pc(0xF000);
        cpu.load_bios(&bios);
        
        // Ejecutar hasta llegar al loop TST en F19E
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > 2000 { 
                panic!("No se alcanzó el loop TST después de 2000 pasos"); 
            }
            cpu.step();
            if cpu.pc == 0xF19E {
                break;
            }
        }
        
        println!("🎯 Llegamos al loop TST en step {}", steps);
        
        // Capturar estado inicial del Timer2
        let initial_t2_low = cpu.bus.read8(0xD008);
        let initial_t2_high = cpu.bus.read8(0xD009);
        let initial_t2 = (initial_t2_high as u16) << 8 | (initial_t2_low as u16);
        
        println!("📊 Timer2 inicial: ${:04X} ({} decimal)", initial_t2, initial_t2);
        
        // Ejecutar exactamente 10 instrucciones TST y medir Timer2
        for i in 1..=10 {
            let pc_before = cpu.pc;
            let total_cycles_before = cpu.bus.total_cycles();
            
            cpu.step(); // Ejecutar una instrucción
            
            let pc_after = cpu.pc;
            let total_cycles_after = cpu.bus.total_cycles();
            let cycles_consumed = total_cycles_after - total_cycles_before;
            
            // Leer Timer2 después
            let t2_low = cpu.bus.read8(0xD008);
            let t2_high = cpu.bus.read8(0xD009);
            let t2_current = (t2_high as u16) << 8 | (t2_low as u16);
            let t2_decreased = initial_t2 - t2_current;
            
            println!("Step {}: PC ${:04X}→${:04X} cycles={} T2=${:04X} decreased={}", 
                     i, pc_before, pc_after, cycles_consumed, t2_current, t2_decreased);
            
            // Verificar que estamos en el loop correcto
            if pc_before != 0xF19E && pc_before != 0xF1A0 {
                println!("⚠️  Salimos del loop TST en step {}", i);
                break;
            }
        }
        
        // Calcular la relación ciclos reales vs Timer2
        let final_t2_low = cpu.bus.read8(0xD008);
        let final_t2_high = cpu.bus.read8(0xD009);
        let final_t2 = (final_t2_high as u16) << 8 | (final_t2_low as u16);
        let total_t2_decreased = initial_t2 - final_t2;
        
        println!("🧮 Resumen:");
        println!("  Timer2 inicial: {} → final: {} = {} decrementos", initial_t2, final_t2, total_t2_decreased);
        println!("  Total ciclos de CPU esperados: ~40 (10 instrucciones × 4 ciclos cada una)");
        println!("  Timer2 decrementos observados: {}", total_t2_decreased);
        
        if total_t2_decreased > 100 {
            println!("❌ ERROR: Timer2 está decrementando demasiado rápido ({} vs ~40 esperados)", total_t2_decreased);
        } else {
            println!("✅ Timer2 parece estar decrementando a velocidad correcta");
        }
    }
}