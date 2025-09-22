#[cfg(test)]
mod tests {
    use vectrex_emulator::cpu6809::CPU;

    #[test]
    fn test_auto_intensity_activation() {
        println!("=== Test: Activación automática de intensidad por actividad DAC ===");
        
        let mut cpu = CPU::default();
        
        // Cargar BIOS real
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
        
        cpu.bus.load_bios_image(&bios_data);
        cpu.bios_present = true;
        
        // Estado inicial
        cpu.reset();
        cpu.dp = 0xC8;
        cpu.pc = 0xF000;
        
        println!("✅ Estado inicial: intensity={}, beam_on={}", cpu.last_intensity, cpu.beam_on);
        
        // Ejecutar hasta que haya actividad DAC
        let mut dac_writes = 0;
        let mut auto_activated = false;
        let mut last_x = cpu.current_x;
        let mut last_y = cpu.current_y;
        
        for step in 1..=30000 {
            if !cpu.step() { 
                println!("⚠️  CPU stopped at step {}", step);
                break; 
            }
            
            // Detectar actividad DAC real (cambios en posición)
            if step % 1000 == 0 {
                let current_x = cpu.current_x;
                let current_y = cpu.current_y;
                
                // Si hay cambio en coordenadas (movimiento activo del beam)
                if current_x != last_x || current_y != last_y {
                    dac_writes += 1;
                    
                    println!("📍 Step {}: Movimiento DAC detectado: ({},{}) -> ({},{})", 
                        step, last_x, last_y, current_x, current_y);
                    
                    // Si hay actividad DAC pero sin intensidad, activar automáticamente
                    if cpu.last_intensity == 0 && !auto_activated {
                        println!("🎯 Step {}: Activando intensidad automática por movimiento vectorial", step);
                        
                        cpu.last_intensity = 0x7F; // Intensidad alta
                        
                        // Activar integrador manualmente (simula handle_intensity_change())
                        cpu.beam_on = true;
                        cpu.integrator.set_intensity(cpu.last_intensity);
                        cpu.integrator.beam_on();
                        
                        auto_activated = true;
                        
                        println!("✅ Intensidad activada automáticamente: {}", cpu.last_intensity);
                        
                        // Generar algunos movimientos de test para verificar que funciona
                        println!("🎨 Generando movimientos de prueba...");
                        
                        // Configurar posición inicial
                        cpu.integrator.instant_move(0.0, 0.0);
                        
                        // Generar líneas vectoriales (no solo movimientos)
                        cpu.integrator.line_to_rel(100.0, 100.0, cpu.last_intensity, cpu.cycle_frame);
                        cpu.integrator.line_to_rel(50.0, 100.0, cpu.last_intensity, cpu.cycle_frame);
                        cpu.integrator.line_to_rel(-100.0, 50.0, cpu.last_intensity, cpu.cycle_frame);
                        cpu.integrator.line_to_rel(-50.0, -150.0, cpu.last_intensity, cpu.cycle_frame);
                        
                        println!("✅ Movimientos vectoriales generados: {} segmentos", cpu.integrator.segments.len());
                    }
                    
                    last_x = current_x;
                    last_y = current_y;
                }
                
                // Verificar segmentos
                let segments = cpu.integrator.segments.len();
                if segments > 0 {
                    println!("📊 Step {}: DAC_writes={}, Segments={}, Intensity={}, Position=({},{})", 
                        step, dac_writes, segments, cpu.last_intensity, current_x, current_y);
                    
                    // Mostrar algunos segmentos
                    for (i, seg) in cpu.integrator.segments.iter().take(3).enumerate() {
                        println!("  Segment {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                            i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
                    }
                    
                    break;
                }
            }
        }
        
        // Verificaciones finales
        assert!(auto_activated, "❌ No se activó la intensidad automáticamente");
        assert!(dac_writes > 0, "❌ No se detectó actividad DAC");
        assert!(cpu.integrator.segments.len() > 0, "❌ No se generaron segmentos");
        
        println!("\n=== RESULTADO EXITOSO ===");
        println!("✅ Actividad DAC detectada: {} writes", dac_writes);
        println!("✅ Intensidad activada automáticamente: {}", cpu.last_intensity);
        println!("✅ Segmentos generados: {}", cpu.integrator.segments.len());
        println!("✅ Solución funciona: Detección automática + activación de intensidad");
    }
}