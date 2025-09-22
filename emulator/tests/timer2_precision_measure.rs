// Test para medir exactamente cuántos decrementos de Timer2 por paso

#[cfg(test)]
mod test {
    use vectrex_emulator::CPU;
    use std::fs;

    fn load_bios() -> Option<Vec<u8>> {
        let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
    }

    #[test]
    fn measure_timer2_precision() {
        let bios = match load_bios() { 
            Some(b)=>b, 
            None=>{println!("❌ BIOS no encontrado"); return;} 
        };
        
        let mut cpu = CPU::with_pc(0xF000);
        cpu.load_bios(&bios);
        
        // Ejecutar hasta llegar al loop TST
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > 2000 { break; }
            cpu.step();
            if cpu.pc == 0xF19E { break; }
        }
        
        if cpu.pc != 0xF19E {
            println!("❌ No se alcanzó TST loop en 2000 pasos");
            return;
        }
        
        println!("🎯 En TST loop después de {} pasos", steps);
        
        // Medir exactamente 5 pasos del loop TST
        for i in 1..=5 {
            let t2_before_low = cpu.bus.read8(0xD008);
            let t2_before_high = cpu.bus.read8(0xD009);
            let t2_before = (t2_before_high as u16) << 8 | (t2_before_low as u16);
            
            let cycles_before = cpu.bus.total_cycles();
            let pc_before = cpu.pc;
            
            cpu.step(); // Una instrucción TST 
            
            let cycles_after = cpu.bus.total_cycles();
            let pc_after = cpu.pc;
            let cycles_consumed = cycles_after - cycles_before;
            
            let t2_after_low = cpu.bus.read8(0xD008);
            let t2_after_high = cpu.bus.read8(0xD009);
            let t2_after = (t2_after_high as u16) << 8 | (t2_after_low as u16);
            
            let t2_difference = t2_before - t2_after;
            
            println!("Step {}: PC ${:04X}→${:04X} cycles={} T2: ${:04X}→${:04X} diff={}", 
                     i, pc_before, pc_after, cycles_consumed, t2_before, t2_after, t2_difference);
            
            // Verificar la relación 1:1 entre ciclos de CPU y decrementos de Timer2
            if cycles_consumed != 0 && t2_difference as u64 != cycles_consumed {
                println!("⚠️  DISCREPANCIA: {} ciclos CPU ≠ {} decrementos Timer2", 
                         cycles_consumed, t2_difference);
            }
        }
        
        println!("\n💡 Expectativa: TST direct = 6 ciclos CPU = 6 decrementos Timer2");
        println!("💡 Si Timer2 decrementa correcto: diferencia = ciclos exactos");
        println!("💡 Si Timer2 demasiado rápido: diferencia > ciclos");
    }
}