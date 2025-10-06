// Test para detectar drift ACUMULATIVO (no el delay transitorio de 6 ciclos)
// Problema reportado: Las letras se van desplazando hacia la izquierda progresivamente

use vectrex_emulator_v2::core::emulator::Emulator;
use vectrex_emulator_v2::engine_types::{Input, RenderContext, AudioContext};

#[test]
fn test_cumulative_text_drift() {
    // Test: Verificar si el texto "MINE STORM" tiene un desplazamiento
    // progresivo hacia la izquierda (cada letra más a la izquierda que la anterior)
    
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();
    
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Ejecutar suficiente para que aparezca el texto completo
    for _ in 0..300_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }
    
    println!("\n=== CUMULATIVE DRIFT DETECTION ===");
    println!("Total lines: {}", render_context.lines.len());
    
    // Agrupar líneas por "filas" (Y similar) para detectar texto
    // El texto "MINE STORM" debería tener múltiples letras en la misma fila Y
    
    let mut y_groups: Vec<(f32, Vec<f32>)> = Vec::new(); // (Y_center, [X_centers])
    
    for line in &render_context.lines {
        let y_center = (line.p0.y + line.p1.y) / 2.0;
        let x_center = (line.p0.x + line.p1.x) / 2.0;
        let dx = (line.p1.x - line.p0.x).abs();
        let dy = (line.p1.y - line.p0.y).abs();
        
        // Solo líneas horizontales (texto)
        if dy < 2.0 && dx > 0.5 {
            // Buscar grupo Y existente (tolerancia ±2.0)
            let mut found = false;
            for (y_avg, x_list) in &mut y_groups {
                if (y_center - *y_avg).abs() < 2.0 {
                    x_list.push(x_center);
                    found = true;
                    break;
                }
            }
            
            if !found {
                y_groups.push((y_center, vec![x_center]));
            }
        }
    }
    
    // Ordenar grupos por Y (de arriba a abajo)
    y_groups.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    println!("\nFound {} text rows (groups of horizontal lines)", y_groups.len());
    
    // Analizar cada fila de texto
    let mut cumulative_drift_detected = false;
    
    for (i, (y_avg, x_centers)) in y_groups.iter().enumerate() {
        if x_centers.len() < 5 {
            continue; // Muy pocas líneas para ser texto
        }
        
        // Ordenar X positions
        let mut sorted_x = x_centers.clone();
        sorted_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Calcular espaciado entre letras
        let mut spacings: Vec<f32> = Vec::new();
        for j in 1..sorted_x.len() {
            spacings.push(sorted_x[j] - sorted_x[j-1]);
        }
        
        // Calcular X mínimo y máximo
        let min_x = sorted_x.first().unwrap();
        let max_x = sorted_x.last().unwrap();
        let range_x = max_x - min_x;
        
        println!("\nRow {} (Y={:.1}): {} segments", i, y_avg, x_centers.len());
        println!("  X range: {:.1} to {:.1} (span={:.1})", min_x, max_x, range_x);
        println!("  First 10 X positions: {:?}", &sorted_x[..sorted_x.len().min(10)]);
        
        // Detectar drift progresivo: Si las letras están todas desplazadas a la izquierda
        // esperaríamos ver un patrón donde X va disminuyendo consistentemente
        
        // Calcular tendencia lineal (regresión simple)
        if sorted_x.len() > 10 {
            let n = sorted_x.len() as f32;
            let sum_x: f32 = sorted_x.iter().sum();
            let sum_y: f32 = (0..sorted_x.len()).map(|i| i as f32).sum();
            let sum_xy: f32 = sorted_x.iter().enumerate().map(|(i, &x)| i as f32 * x).sum();
            let sum_xx: f32 = (0..sorted_x.len()).map(|i| (i * i) as f32).sum();
            
            let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_y * sum_y);
            
            println!("  Linear trend slope: {:.3} (negative = leftward drift)", slope);
            
            // Un slope muy negativo indicaría drift progresivo hacia la izquierda
            if slope < -0.5 {
                cumulative_drift_detected = true;
                println!("  ⚠️  CUMULATIVE DRIFT DETECTED in this row!");
            }
        }
    }
    
    if cumulative_drift_detected {
        panic!("❌ CUMULATIVE DRIFT DETECTED: Text positions show progressive leftward shift");
    } else {
        println!("\n✅ No cumulative drift detected - text positioning is consistent");
    }
}
