#[cfg(test)]
mod test_decb_bne_bug {
    use vectrex_emulator::cpu6809::CPU;

    #[test]
    fn test_decb_bne_loop_termination() {
        println!("🧪 Testing DECB + BNE loop termination bug");
        
        let mut cpu = CPU::default();
        
        // Configurar B con un valor pequeño para prueba rápida
        cpu.b = 3;
        cpu.pc = 0x1000;
        
        // Escribir programa: DECB, BNE $1000 (bucle infinito hasta B=0)
        cpu.bus.mem[0x1000] = 0x5A;  // DECB
        cpu.bus.mem[0x1001] = 0x26;  // BNE
        cpu.bus.mem[0x1002] = 0xFC;  // offset -4 (para saltar a 0x1000)
        
        println!("Estado inicial: B={:02X}, Z={}", cpu.b, cpu.cc_z);
        
        // Ejecutar bucle hasta que termine o limite de seguridad
        let mut iterations = 0;
        let max_iterations = 10;
        
        while iterations < max_iterations {
            let old_b = cpu.b;
            let old_pc = cpu.pc;
            
            // Ejecutar una instrucción
            cpu.step();
            iterations += 1;
            
            println!("Iteración {}: PC {:04X}, B {:02X} -> {:02X}, Z={}", 
                     iterations, old_pc, old_b, cpu.b, cpu.cc_z);
            
            // Si PC cambió y no es el bucle, salimos
            if cpu.pc != 0x1000 && cpu.pc != 0x1001 && cpu.pc != 0x1003 {
                println!("✅ Bucle terminado - PC saltó a {:04X}", cpu.pc);
                return;
            }
        }
        
        println!("❌ Test terminado por límite de iteraciones");
        println!("   Estado final: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
    }

    #[test]
    fn test_decb_bne_real_bios_scenario() {
        println!("🧪 Testing DECB + BNE con escenario real BIOS");
        
        let mut cpu = CPU::default();
        
        // Simular el valor de B que vemos en el trace (pero más pequeño para test rápido)
        cpu.b = 5;  // Valor pequeño para test
        cpu.pc = 0x1000;
        
        // Escribir programa: DECB, BNE $1000 (bucle como en BIOS)
        cpu.bus.mem[0x1000] = 0x5A;  // DECB
        cpu.bus.mem[0x1001] = 0x26;  // BNE
        cpu.bus.mem[0x1002] = 0xFC;  // offset -4 (para saltar a 0x1000)
        
        println!("Estado inicial: B={:02X} ({}), Z={}", cpu.b, cpu.b, cpu.cc_z);
        
        let mut iterations = 0;
        let max_iterations = 20;  // Límite de seguridad
        
        while iterations < max_iterations {
            let old_b = cpu.b;
            let old_pc = cpu.pc;
            
            // Ejecutar una instrucción
            cpu.step();
            iterations += 1;
            
            println!("Iteración {}: PC {:04X} -> {:04X}, B {:02X} -> {:02X}, Z={}", 
                     iterations, old_pc, cpu.pc, old_b, cpu.b, cpu.cc_z);
            
            // Si PC no está en el bucle, significa que salió correctamente
            if cpu.pc != 0x1000 && cpu.pc != 0x1001 && cpu.pc != 0x1003 {
                println!("✅ Bucle terminado correctamente después de {} iteraciones", iterations);
                println!("   Estado final: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
                return;
            }
            
            // Si B llega a 0 pero sigue en bucle, hay un problema
            if cpu.b == 0 && (cpu.pc == 0x1000 || cpu.pc == 0x1001) {
                println!("❌ PROBLEMA: B=0 pero el bucle no terminó");
                println!("   Estado: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
                panic!("BNE no está funcionando correctamente con Z flag");
            }
        }
        
        println!("❌ Bucle no terminó después de {} iteraciones", max_iterations);
        println!("   Estado final: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
        panic!("Posible bucle infinito - DECB o BNE mal implementados");
    }
}