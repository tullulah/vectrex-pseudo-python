#[cfg(test)]
mod tests {
    use vectrex_emulator::cpu6809::CPU;

    #[test]
    fn test_production_auto_intensity_bios() {
        println!("=== Test: Solución de producción - Auto-activación de intensidad en BIOS ===");
        
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
        
        // Ejecutar BIOS hasta que la auto-activación genere segmentos
        let mut auto_activated = false;
        
        for step in 1..=20000 {
            if !cpu.step() { 
                println!("⚠️  CPU stopped at step {}", step);
                break; 
            }
            
            // Verificar si se activó automáticamente la intensidad
            if !auto_activated && cpu.last_intensity > 0 {
                auto_activated = true;
                println!("🎯 Step {}: ¡INTENSIDAD ACTIVADA AUTOMÁTICAMENTE! intensity={}, beam_on={}", 
                    step, cpu.last_intensity, cpu.beam_on);
                println!("  Posición actual: ({}, {})", cpu.current_x, cpu.current_y);
            }
            
            // Verificar segmentos generados
            if step % 5000 == 0 {
                let segments = cpu.integrator.segments.len();
                println!("📊 Step {}: PC=0x{:04X}, Segments={}, Intensity={}, Position=({},{}) Auto={}", 
                    step, cpu.pc, segments, cpu.last_intensity, cpu.current_x, cpu.current_y, auto_activated);
                
                if segments > 0 {
                    println!("🎉 ¡ÉXITO! Segmentos generados automáticamente por la BIOS");
                    
                    // Mostrar algunos segmentos como prueba
                    for (i, seg) in cpu.integrator.segments.iter().take(3).enumerate() {
                        println!("  Segment {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                            i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
                    }
                    
                    break;
                }
            }
        }
        
        // Verificaciones finales
        assert!(auto_activated, "❌ La intensidad no se activó automáticamente");
        assert!(cpu.last_intensity > 0, "❌ La intensidad final es 0");
        assert!(cpu.integrator.segments.len() > 0, "❌ No se generaron segmentos automáticamente");
        
        println!("\n=== SOLUCIÓN IMPLEMENTADA EXITOSAMENTE ===");
        println!("✅ Auto-activación de intensidad: FUNCIONA");
        println!("✅ Intensidad final: {}", cpu.last_intensity);
        println!("✅ Segmentos generados: {}", cpu.integrator.segments.len());
        println!("✅ Frontend debería ver segmentos ahora");
        println!("\n🎯 PROBLEMA RESUELTO: 'no se generan segmentos' solucionado");
    }
}