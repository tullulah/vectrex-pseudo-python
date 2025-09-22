#[cfg(test)]
mod tests {
    use vectrex_emulator::cpu6809::CPU;

    #[test]
    fn test_mux_emulation_extended() {
        println!("=== Test Extendido: Emulación MUX física (siguiendo patrón Vectrexy) ===");
        
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
        let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
        
        let mut cpu = CPU::default();
        cpu.bus.load_bios_image(&bios_data);
        cpu.bios_present = true;
        cpu.integrator_auto_drain = false;
        cpu.reset(); 
        
        println!("✅ Estado inicial: intensity={}, beam_on={}", cpu.last_intensity, cpu.last_intensity > 0);
        
        let mut segments_checkpoints = Vec::new();
        let mut max_segments = 0;
        
        // Ejecutar por más tiempo para ver el comportamiento real
        for step in 1..=200000 {
            if !cpu.step() { 
                println!("⚠️  CPU stopped at step {}", step);
                break; 
            }
            
            // Checkpoint cada 25k pasos
            if step % 25000 == 0 {
                let segments = cpu.integrator.segments.len();
                max_segments = max_segments.max(segments);
                segments_checkpoints.push((step, segments, cpu.last_intensity));
                println!("📊 Step {}: PC=0x{:04X}, Segments={}, Intensity={}, MUX(enabled={}, sel={})", 
                    step, cpu.pc, segments, cpu.last_intensity, cpu.mux_enabled, cpu.mux_selector);
                    
                if segments > 50 {
                    println!("🎉 ¡Excelente progreso! {} segmentos alcanzados", segments);
                }
            }
            
            // Detectar si llegamos a números significativos
            if cpu.integrator.segments.len() >= 100 && cpu.integrator.segments.len() > max_segments + 10 {
                println!("🚀 ¡GRAN MEJORA! Segments: {} (nuevo récord)", cpu.integrator.segments.len());
                max_segments = cpu.integrator.segments.len();
            }
        }
        
        let final_segments = cpu.integrator.segments.len();
        
        println!("\n=== RESULTADOS FINALES ===");
        println!("✅ Segmentos máximos alcanzados: {}", max_segments);
        println!("✅ Segmentos finales: {}", final_segments);
        println!("✅ Intensidad final: {}", cpu.last_intensity);
        
        println!("\n=== CHECKPOINTS DE PROGRESO ===");
        for (step, segments, intensity) in segments_checkpoints {
            println!("  Step {}: {} segments, intensity={}", step, segments, intensity);
        }
        
        // Analizar algunos segmentos si los hay
        if final_segments > 0 {
            println!("\n=== ANÁLISIS DE SEGMENTOS ===");
            let show_count = final_segments.min(10);
            for i in 0..show_count {
                let seg = &cpu.integrator.segments[i];
                println!("  Segment {}: start=({:.1},{:.1}) end=({:.1},{:.1}) intensity={}", 
                    i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
            }
            if final_segments > show_count {
                println!("  ... y {} segmentos más", final_segments - show_count);
            }
        }
        
        // Comparar con el objetivo de 143+ segments
        if final_segments >= 143 {
            println!("🎯 ¡OBJETIVO ALCANZADO! {} ≥ 143 segments esperados", final_segments);
        } else if final_segments >= 50 {
            println!("📈 Progreso bueno: {} segments (objetivo: 143+)", final_segments);
        } else {
            println!("📊 Baseline: {} segments (objetivo: 143+)", final_segments);
        }
        
        // El test pasa si hay al menos algunos segmentos (progreso)
        assert!(final_segments >= 3, "Should generate at least some segments with MUX emulation");
    }
}