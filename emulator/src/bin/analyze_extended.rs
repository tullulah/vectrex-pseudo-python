use std::fs;

fn main() {
    println!("=== ANÁLISIS DEL ARCHIVO DE COMPARACIÓN EXTENDIDA ===");
    
    let filename = "../emulator_comparison_10000_steps.txt";
    match fs::read_to_string(filename) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            
            println!("Total líneas en el archivo: {}", lines.len());
            
            // Analizar operaciones CLR indexed
            let mut clr_operations = 0;
            
            for line in &lines {
                if line.contains("CLR indexed (0x6F) detected") {
                    clr_operations += 1;
                    
                    // Solo mostrar los primeros 10 para referencia
                    if clr_operations <= 10 {
                        println!("CLR #{}: {}", clr_operations, line.trim());
                    }
                }
            }
            
            println!("\n=== RESUMEN OPERACIONES CLR INDEXED ===");
            println!("Total operaciones CLR indexed: {}", clr_operations);
            
            // Analizar generación de vectores
            let vector_lines: Vec<&str> = lines.iter()
                .filter(|&line| line.contains("📐 Vector:"))
                .copied()
                .collect();
            
            println!("\n=== ANÁLISIS GENERACIÓN DE VECTORES ===");
            println!("Total vectores generados: {}", vector_lines.len());
            
            if vector_lines.len() > 0 {
                println!("✅ El emulador SÍ está generando vectores");
                println!("Ejemplos de vectores generados:");
                for (i, vector_line) in vector_lines.iter().take(5).enumerate() {
                    println!("  Vector {}: {}", i + 1, vector_line.trim());
                }
                
                if vector_lines.len() > 5 {
                    println!("  ... y {} vectores más", vector_lines.len() - 5);
                }
            } else {
                println!("❌ No se detectaron vectores generados");
            }
            
            // Verificar intensidad de vectores
            let non_zero_intensity: Vec<&str> = vector_lines.iter()
                .filter(|&line| !line.contains("intensity=0"))
                .copied()
                .collect();
            
            println!("\nVectores con intensidad > 0: {}", non_zero_intensity.len());
            if non_zero_intensity.len() > 0 {
                println!("✅ Hay vectores visibles (intensidad > 0)");
                for (i, line) in non_zero_intensity.iter().take(3).enumerate() {
                    println!("  Vector visible {}: {}", i + 1, line.trim());
                }
            }
            
            // Analizar frames BIOS
            let bios_frame_lines: Vec<&str> = lines.iter()
                .filter(|&line| line.contains("BIOS Frame completed"))
                .copied()
                .collect();
            
            println!("\n=== ANÁLISIS FRAMES BIOS ===");
            println!("Total frames BIOS completados: {}", bios_frame_lines.len());
            
            if bios_frame_lines.len() > 0 {
                println!("✅ El emulador SÍ está completando frames BIOS");
                for (i, frame_line) in bios_frame_lines.iter().take(3).enumerate() {
                    println!("  Frame {}: {}", i + 1, frame_line.trim());
                }
            }
            
            println!("\n=== CONCLUSIÓN ===");
            if clr_operations > 0 && vector_lines.len() > 0 && bios_frame_lines.len() > 0 {
                println!("✅ El emulador backend está funcionando correctamente:");
                println!("   - {} operaciones CLR indexed detectadas", clr_operations);
                println!("   - {} vectores generados", vector_lines.len());
                println!("   - {} frames BIOS completados", bios_frame_lines.len());
                println!("   - {} vectores con intensidad visible", non_zero_intensity.len());
                println!();
                println!("📋 El problema está en la UI o en la comunicación backend-frontend");
                println!("📋 El backend RUST está trabajando bien, pero algo no llega a la UI");
            } else {
                println!("⚠️  Hay problemas en el backend:");
                if clr_operations == 0 {
                    println!("   - No se detectaron operaciones CLR indexed");
                }
                if vector_lines.len() == 0 {
                    println!("   - No se están generando vectores");
                }
                if bios_frame_lines.len() == 0 {
                    println!("   - No se completan frames BIOS");
                }
            }
        }
        Err(e) => {
            println!("Error leyendo archivo: {}", e);
        }
    }
}