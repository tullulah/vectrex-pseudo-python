// Test para determinar el rango real de coordenadas que genera el emulador
// Verificar qué valores min/max produce self.pos en screen.rs durante renderizado

use vectrex_emulator_v2::Emulator;
use vectrex_emulator_v2::core::engine_types::{RenderContext, Input, AudioContext};
use std::path::PathBuf;

const BIOS_PATH: &str = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";

#[test]
fn test_coordinate_range_during_title_screen() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("ide/frontend/dist/bios.bin");
    
    let bios_path = if path.exists() {
        path.to_str().unwrap()
    } else {
        BIOS_PATH
    };

    println!("📂 BIOS path: {}", bios_path);

    let mut emulator = Emulator::new();
    
    // Inicializar con BIOS (esto sí mapea la memoria correctamente)
    emulator.init(bios_path);
    emulator.reset();
    
    println!("🚀 Ejecutando hasta primera pantalla de título...");
    
    // Ejecutar hasta que tengamos vectores dibujados (título COPYRIGHT)
    let mut total_cycles = 0;
    let max_instructions = 15000; // ~2 segundos worth de instrucciones
    
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut vector_count = 0;
    
    let input = Input::new();
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // CPU cycles per audio sample
    
    for _ in 0..max_instructions {
        let cycles = emulator.execute_instruction(&input, &mut render_context, &mut audio_context).unwrap_or(0);
        total_cycles += cycles;
        
        // Verificar si hay vectores nuevos en render_context
        if !render_context.lines.is_empty() {
            for line in &render_context.lines {
                vector_count += 1;
                
                // Actualizar rangos
                min_x = min_x.min(line.p0.x).min(line.p1.x);
                max_x = max_x.max(line.p0.x).max(line.p1.x);
                min_y = min_y.min(line.p0.y).min(line.p1.y);
                max_y = max_y.max(line.p0.y).max(line.p1.y);
                
                // Imprimir primeros 20 vectores para ver valores reales
                if vector_count <= 20 {
                    println!("Vector #{}: ({:.2}, {:.2}) → ({:.2}, {:.2})", 
                        vector_count, line.p0.x, line.p0.y, line.p1.x, line.p1.y);
                }
                
                // Imprimir estadísticas cada 50 vectores
                if vector_count % 50 == 0 {
                    let temp_center_x = (min_x + max_x) / 2.0;
                    let temp_center_y = (min_y + max_y) / 2.0;
                    println!("  [{}v] Centro temporal: ({:.2}, {:.2}), Rango X: {:.2} a {:.2}", 
                        vector_count, temp_center_x, temp_center_y, min_x, max_x);
                }
            }
            
            // Limpiar render context para próxima iteración
            render_context.lines.clear();
            
            if vector_count >= 200 {  // Aumentado a 200 para ver más allá del marco inicial
                break; // Suficientes vectores para estadística
            }
        }
    }
    
    println!("\n📊 ESTADÍSTICAS DE COORDENADAS:");
    println!("Total vectores analizados: {}", vector_count);
    println!("Rango X: {:.2} a {:.2} (delta: {:.2})", min_x, max_x, max_x - min_x);
    println!("Rango Y: {:.2} a {:.2} (delta: {:.2})", min_y, max_y, max_y - min_y);
    println!("Centro aproximado X: {:.2}", (min_x + max_x) / 2.0);
    println!("Centro aproximado Y: {:.2}", (min_y + max_y) / 2.0);
    
    println!("\n🔍 COMPARACIÓN CON SISTEMAS CONOCIDOS:");
    println!("Vectrex DAC: -127 a +127 (256 units)");
    println!("JSVecx: 0 a 33000 (X), 0 a 41000 (Y)");
    println!("Vectrexy: Coordenadas float acumuladas con LINE_DRAW_SCALE=0.85");
    
    // Verificar si las coordenadas parecen estar en alguno de estos rangos
    let in_dac_range = min_x >= -128.0 && max_x <= 128.0;
    let in_jsvecx_range = min_x >= 0.0 && max_x <= 34000.0 && min_y >= 0.0 && max_y <= 42000.0;
    
    println!("\n✓ En rango DAC (-127 a +127): {}", in_dac_range);
    println!("✓ En rango JSVecx (0-33000): {}", in_jsvecx_range);
    
    if !in_dac_range && !in_jsvecx_range {
        println!("⚠️  Las coordenadas NO están en ningún rango estándar conocido.");
        println!("    Este es el sistema de coordenadas propietario de Vectrexy.");
        println!("    Nuestro HTML DEBE ajustarse a este rango, NO asumir -127 a +127.");
    }
    
    assert!(vector_count > 0, "Debería haber dibujado al menos 1 vector");
}
