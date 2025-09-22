#[cfg(test)]
mod test_opcode_capture {
    use vectrex_emulator::cpu6809::CPU;
    use std::fs;

    #[test]
    fn test_bios_opcode_capture() {
        println!("=== RUST EMULATOR OPCODE CAPTURE ===");
        
        // Cargar BIOS real
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
        let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
        println!("BIOS loaded: {} bytes", bios_data.len());
        
        // Crear CPU
        let mut cpu = CPU::default();
        
        // Cargar BIOS usando el método oficial
        cpu.load_bios(&bios_data);
        
        // Reset del CPU (lee reset vector de 0xFFFE-0xFFFF)
        cpu.reset();
        
        test_bios_opcode_capture_steps(&mut cpu, 20);
    }

    fn test_bios_opcode_capture_steps(cpu: &mut CPU, steps: usize) {
        println!("Initial PC: 0x{:04X}", cpu.pc);
        
        // Verificar reset vector
        let reset_vector_lo = cpu.bus.mem[0xFFFE];
        let reset_vector_hi = cpu.bus.mem[0xFFFF];
        let reset_vector = ((reset_vector_hi as u16) << 8) | (reset_vector_lo as u16);
        println!("Reset vector at 0xFFFE-0xFFFF: 0x{:04X}", reset_vector);
        
        // Mostrar primeros bytes de ROM
        println!("First 8 bytes of ROM:");
        for i in 0..8 {
            let addr = 0xE000 + i;
            let byte = cpu.bus.mem[addr];
            println!("  0x{:04X}: 0x{:02X}", addr, byte);
        }
        
        // Capturar secuencia de opcodes
        println!("\n=== RUST OPCODE SEQUENCE ({} steps) ===", steps);
        println!("┌──────┬──────┬────────┬────┬────┬──────┬──────┬──────┬──────┬────┬────┐");
        println!("│ Step │  PC  │ Opcode │ A  │ B  │  X   │  Y   │  S   │  U   │ DP │ CC │");
        println!("├──────┼──────┼────────┼────┼────┼──────┼──────┼──────┼──────┼────┼────┤");
        
        for step in 0..steps {
            let pc = cpu.pc;
            
            // PC es u16, siempre válido
            
            let opcode = cpu.bus.mem[pc as usize];
            
            println!(
                "│ {:4} │ {:04X} │   0x{:02X}   │ {:02X} │ {:02X} │ {:04X} │ {:04X} │ {:04X} │ {:04X} │ {:02X} │ {:02X} │",
                step,
                pc,
                opcode,
                cpu.a,
                cpu.b,
                cpu.x,
                cpu.y,
                cpu.s,
                cpu.u,
                cpu.dp,
                // Reconstruir CC desde flags individuales
                (if cpu.cc_e { 0x80 } else { 0 }) |
                (if cpu.cc_f { 0x40 } else { 0 }) |
                (if cpu.cc_h { 0x20 } else { 0 }) |
                (if cpu.cc_i { 0x10 } else { 0 }) |
                (if cpu.cc_n { 0x08 } else { 0 }) |
                (if cpu.cc_z { 0x04 } else { 0 }) |
                (if cpu.cc_v { 0x02 } else { 0 }) |
                (if cpu.cc_c { 0x01 } else { 0 })
            );
            
            // Ejecutar un paso
            let old_pc = cpu.pc;
            cpu.step();
            
            // Detectar bucles infinitos simples
            if step > 2 && cpu.pc == old_pc {
                println!("│      │      │ ⚠ PC unchanged, possible infinite loop │");
                break;
            }
            
            // Verificar si se está ejecutando fuera del rango esperado
            if cpu.pc < 0xE000 && cpu.pc > 0x1000 {
                println!("│      │      │ ⚠ PC outside expected range: 0x{:04X} │", cpu.pc);
            }
        }
        
        println!("└──────┴──────┴────────┴────┴────┴──────┴──────┴──────┴──────┴────┴────┘");
        println!("\n=== RUST EMULATOR CAPTURE COMPLETE ===");
    }
}