// Test para diagnosticar el bug de desplazamiento progresivo del beam
// Problema: Las líneas son rectas pero el offset se desplaza hacia la izquierda

use vectrex_emulator_v2::core::emulator::Emulator;
use vectrex_emulator_v2::engine_types::{Input, RenderContext, AudioContext};

#[test]
fn test_beam_reset_sequence() {
    // Simula la secuencia: dibujar línea → Reset0Ref → dibujar otra línea
    // Verifica si el beam realmente vuelve al origen entre líneas
    
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();
    
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Ejecutar hasta que la BIOS dibuje algo
    for _ in 0..100_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }
    
    println!("\n=== BEAM POSITIONING DIAGNOSTIC ===");
    println!("Lines generated: {}", render_context.lines.len());
    
    // Analizar las primeras 50 líneas para ver el patrón de offset
    println!("\n=== First 50 lines - checking for progressive X offset ===");
    
    let mut prev_x_center = 0.0f32;
    let mut x_drift_detected = false;
    
    for (i, line) in render_context.lines.iter().take(50).enumerate() {
        let x_center = (line.p0.x + line.p1.x) / 2.0;
        let y_center = (line.p0.y + line.p1.y) / 2.0;
        let dx = line.p1.x - line.p0.x;
        let dy = line.p1.y - line.p0.y;
        let length = (dx * dx + dy * dy).sqrt();
        
        // Detectar si líneas horizontales están desplazándose en X
        if dy.abs() < 0.1 && length > 1.0 {
            // Línea horizontal
            if i > 0 {
                let x_diff = x_center - prev_x_center;
                if x_diff.abs() > 0.5 {
                    println!(
                        "Line {}: X_center={:.2} (diff from prev: {:.2}) Y={:.2} len={:.1}",
                        i, x_center, x_diff, y_center, length
                    );
                    if x_diff < -0.5 {
                        x_drift_detected = true;
                    }
                }
            }
            prev_x_center = x_center;
        }
    }
    
    if x_drift_detected {
        println!("\n⚠️  DRIFT DETECTED: Horizontal lines are shifting left progressively");
        println!("This suggests velocity_x is not resetting properly between moves");
    } else {
        println!("\n✅ No systematic drift detected");
    }
}

#[test]
fn test_velocity_delay_impact() {
    // Test específico: verificar que velocity_x con delay=6 no cause drift
    
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();
    
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Ejecutar suficientes ciclos para ver el efecto del delay
    for _ in 0..200_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }
    
    println!("\n=== VELOCITY DELAY IMPACT ANALYSIS ===");
    
    // Buscar secuencias de líneas que deberían estar alineadas verticalmente
    // (mismo X pero diferentes Y - caracteres del texto "MINE STORM")
    let mut vertical_line_x_positions: Vec<f32> = Vec::new();
    
    for line in &render_context.lines {
        let dx = (line.p1.x - line.p0.x).abs();
        let dy = (line.p1.y - line.p0.y).abs();
        
        // Líneas mayormente verticales
        if dy > dx * 3.0 && dy > 2.0 {
            vertical_line_x_positions.push(line.p0.x);
        }
    }
    
    println!("Vertical lines found: {}", vertical_line_x_positions.len());
    
    if vertical_line_x_positions.len() > 10 {
        // Analizar desviación estándar de posiciones X
        let mean_x: f32 = vertical_line_x_positions.iter().sum::<f32>() / vertical_line_x_positions.len() as f32;
        let variance: f32 = vertical_line_x_positions.iter()
            .map(|x| (x - mean_x).powi(2))
            .sum::<f32>() / vertical_line_x_positions.len() as f32;
        let std_dev = variance.sqrt();
        
        println!("Mean X position: {:.2}", mean_x);
        println!("Standard deviation: {:.2}", std_dev);
        
        // Mostrar primeras 20 posiciones X
        println!("\nFirst 20 vertical line X positions:");
        for (i, x) in vertical_line_x_positions.iter().take(20).enumerate() {
            println!("  Line {}: X = {:.2}", i, x);
        }
        
        // Si std_dev es muy alta, indica que las líneas verticales están dispersas
        // (lo cual sería incorrecto si son parte de la misma letra)
        if std_dev > 5.0 {
            println!("\n⚠️  HIGH VARIANCE: Vertical lines are too dispersed");
            println!("Expected: Letters should have consistent X positions for vertical strokes");
        } else {
            println!("\n✅ Variance is acceptable");
        }
    }
}
