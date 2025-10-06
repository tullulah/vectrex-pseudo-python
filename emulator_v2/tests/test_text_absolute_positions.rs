// Test para verificar posiciones absolutas del texto "MINE STORM"
// Objetivo: Ver si las letras están todas desplazadas hacia la izquierda
// comparado con dónde deberían estar

use vectrex_emulator_v2::core::emulator::Emulator;
use vectrex_emulator_v2::engine_types::{Input, RenderContext, AudioContext};

#[test]
fn test_minestorm_text_absolute_positions() {
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();
    
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Ejecutar hasta que aparezca "MINE STORM"
    for _ in 0..300_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }
    
    println!("\n=== MINE STORM TEXT POSITION ANALYSIS ===");
    println!("Total lines: {}", render_context.lines.len());
    
    // Buscar filas de texto en la región esperada para el título
    // "MINE STORM" debería estar cerca de Y=30 (zona superior-media)
    
    let mut title_lines: Vec<(f32, f32, f32, f32)> = Vec::new(); // (y, x_min, x_max, x_avg)
    
    // Agrupar líneas horizontales en la región del título (Y entre 15 y 40)
    for line in &render_context.lines {
        let y_center = (line.p0.y + line.p1.y) / 2.0;
        let dx = (line.p1.x - line.p0.x).abs();
        let dy = (line.p1.y - line.p0.y).abs();
        
        // Solo líneas horizontales en región del título
        if y_center > 15.0 && y_center < 40.0 && dy < 2.0 && dx > 0.5 {
            title_lines.push((
                y_center,
                line.p0.x.min(line.p1.x),
                line.p0.x.max(line.p1.x),
                (line.p0.x + line.p1.x) / 2.0
            ));
        }
    }
    
    println!("Found {} line segments in title region (Y: 15-40)", title_lines.len());
    
    // Calcular bounding box del título
    if title_lines.len() > 0 {
        let overall_min_x = title_lines.iter().map(|l| l.1).fold(f32::INFINITY, f32::min);
        let overall_max_x = title_lines.iter().map(|l| l.2).fold(f32::NEG_INFINITY, f32::max);
        let overall_avg_x = title_lines.iter().map(|l| l.3).sum::<f32>() / title_lines.len() as f32;
        
        println!("\n📊 TITLE BOUNDING BOX:");
        println!("  Min X: {:.2}", overall_min_x);
        println!("  Max X: {:.2}", overall_max_x);
        println!("  Width: {:.2}", overall_max_x - overall_min_x);
        println!("  Avg X: {:.2}", overall_avg_x);
        println!("  Center offset from origin: {:.2}", overall_avg_x);
        
        // El texto "MINE STORM" debería estar CENTRADO (avg_x ≈ 0)
        // Si está significativamente negativo, está desplazado a la izquierda
        
        if overall_avg_x < -10.0 {
            println!("\n⚠️  WARNING: Title is shifted LEFT by {:.1} units", -overall_avg_x);
            println!("   Expected: Text should be centered around X=0");
            println!("   Actual: Text is centered around X={:.1}", overall_avg_x);
        } else if overall_avg_x > 10.0 {
            println!("\n⚠️  WARNING: Title is shifted RIGHT by {:.1} units", overall_avg_x);
        } else {
            println!("\n✅ Title is properly centered");
        }
        
        // Analizar distribución Y
        let mut y_positions: Vec<f32> = title_lines.iter().map(|l| l.0).collect();
        y_positions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        y_positions.dedup_by(|a, b| (*a - *b).abs() < 1.0);
        
        println!("\n📏 Y POSITIONS ({} distinct rows):", y_positions.len());
        for (i, y) in y_positions.iter().enumerate().take(10) {
            println!("  Row {}: Y={:.1}", i, y);
        }
        
    } else {
        println!("⚠️  No title lines found!");
    }
    
    // Análisis adicional: copyright text (debería estar en Y≈-100)
    let mut copyright_lines: Vec<(f32, f32, f32, f32)> = Vec::new();
    
    for line in &render_context.lines {
        let y_center = (line.p0.y + line.p1.y) / 2.0;
        let dx = (line.p1.x - line.p0.x).abs();
        let dy = (line.p1.y - line.p0.y).abs();
        
        if y_center < -90.0 && y_center > -115.0 && dy < 2.0 && dx > 0.5 {
            copyright_lines.push((
                y_center,
                line.p0.x.min(line.p1.x),
                line.p0.x.max(line.p1.x),
                (line.p0.x + line.p1.x) / 2.0
            ));
        }
    }
    
    if copyright_lines.len() > 0 {
        let cr_avg_x = copyright_lines.iter().map(|l| l.3).sum::<f32>() / copyright_lines.len() as f32;
        
        println!("\n📊 COPYRIGHT TEXT (Y: -90 to -115):");
        println!("  Line count: {}", copyright_lines.len());
        println!("  Avg X: {:.2}", cr_avg_x);
        println!("  Center offset: {:.2}", cr_avg_x);
        
        if cr_avg_x < -10.0 {
            println!("  ⚠️  Shifted LEFT by {:.1} units", -cr_avg_x);
        }
    }
}
