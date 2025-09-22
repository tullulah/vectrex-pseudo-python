#[cfg(test)]
mod tests {
    use vectrex_emulator::cpu6809::CPU;

    #[test]
    fn test_bios_intensity_calls_detection() {
        println!("=== Test: Detección de llamadas Intensity_7F en BIOS ===");
        
        // Cargar BIOS real
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
        
        let mut cpu = CPU::default();
        
        // Cargar BIOS y configurar
        cpu.bus.load_bios_image(&bios_data);
        cpu.bios_present = true;
        
        // NO auto drain para acumular segmentos
        cpu.integrator_auto_drain = false;
        
        // Estado inicial
        cpu.reset();
        cpu.dp = 0xC8;
        cpu.pc = 0xF000;
        
        println!("✅ Estado inicial: intensity={}, beam_on={}", cpu.last_intensity, cpu.beam_on);
        
        // Buscar llamadas JSR a Intensity_7F (dirección 0xF2A9 según BIOS)
        let mut intensity_calls = 0;
        let mut last_mux_selector = 0;
        
        for step in 1..=50000 {
            if !cpu.step() { 
                println!("⚠️  CPU stopped at step {}", step);
                break; 
            }
            
            // Detectar llamadas JSR a rutinas de intensidad
            if cpu.pc >= 0xF29D && cpu.pc <= 0xF2AB {
                intensity_calls += 1;
                println!("📞 Step {}: ¡Llamada BIOS a rutina intensidad en PC=0x{:04X}!", step, cpu.pc);
                println!("  Contexto: A=0x{:02X}, MUX enabled={}, selector={}, port_a=0x{:02X}", 
                    cpu.a, cpu.mux_enabled, cpu.mux_selector, cpu.port_a_value);
            }
            
            // Detectar cambios en MUX selector (especialmente selector=2 para brightness)
            if cpu.mux_selector != last_mux_selector {
                println!("🔄 Step {}: Cambio MUX selector: {} -> {} (enabled={}, port_a=0x{:02X})", 
                    step, last_mux_selector, cpu.mux_selector, cpu.mux_enabled, cpu.port_a_value);
                last_mux_selector = cpu.mux_selector;
            }
            
            // Verificar segmentos cada 10k pasos
            if step % 10000 == 0 {
                let segments = cpu.integrator.segments.len();
                println!("📊 Step {}: PC=0x{:04X}, Segments={}, Intensity={}, Calls={}", 
                    step, cpu.pc, segments, cpu.last_intensity, intensity_calls);
                
                if segments > 0 {
                    println!("🎉 Segmentos generados! Total: {}", segments);
                    break;
                }
            }
        }
        
        // Reporte final
        println!("\n=== ANÁLISIS DE LLAMADAS BIOS ===");
        println!("✅ Llamadas a rutinas Intensity_*: {}", intensity_calls);
        println!("✅ Intensidad final: {}", cpu.last_intensity);
        println!("✅ Segmentos generados: {}", cpu.integrator.segments.len());
        
        if intensity_calls > 0 {
            println!("🎯 ¡CONFIRMADO! La BIOS SÍ llama rutinas Intensity_*");
        } else {
            println!("🔍 La BIOS no llamó rutinas Intensity_* en esta ejecución");
            println!("   (pero la auto-activación funcionó como fallback)");
        }
    }
}