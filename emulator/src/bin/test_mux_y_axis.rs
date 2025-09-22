use std::fs;
use vectrex_emulator::emulator::Emulator;

fn main() {
    println!("=== TEST MANUAL DEL MUX Y-AXIS ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    
    // Inicializar emulador
    let mut emulator = Emulator::new();
    
    // Cargar BIOS
    match fs::read(bios_path) {
        Ok(bios_data) => {
            emulator.load_bios(&bios_data);
            println!("✅ BIOS cargada correctamente");
        }
        Err(e) => {
            eprintln!("❌ Error cargando BIOS: {}", e);
            return;
        }
    }
    
    // Cargar un cartucho simple
    let cart_data = vec![
        0x16, 0xF0, 0x00,  // Reset vector apunta a 0xF000
        0x00, 0x00,        // Padding
    ];
    emulator.load_cartridge(&cart_data);
    
    emulator.reset();
    
    println!("\n=== TEST MANUAL: Escribiendo directamente a VIA para generar vector con Y ===");
    
    // Simular escrituras directas a VIA usando test_write8 que es la API pública
    
    // 1. Habilitar MUX con selector Y-axis (Port B = 0x00)
    println!("1. Habilitando MUX para Y-axis...");
    emulator.cpu.test_write8(0xD000, 0x00); // VIA Port B = 0x00 (MUX enabled, selector 0=Y-axis)
    
    // 2. Escribir valor Y a Port A  
    println!("2. Escribiendo Y=100 a Port A...");
    emulator.cpu.test_write8(0xD001, 100); // VIA Port A = 100 (valor Y)
    
    // 3. Cambiar MUX a brightness pero mantener habilitado
    println!("3. Cambiando MUX a brightness...");
    emulator.cpu.test_write8(0xD000, 0x04); // VIA Port B = 0x04 (MUX enabled, selector 2=brightness)
    
    // 4. Escribir brightness
    println!("4. Escribiendo brightness=255...");
    emulator.cpu.test_write8(0xD001, 255); // VIA Port A = 255 (brightness máxima)
    
    // 5. Cambiar MUX a selector 0 para X (realmente es Y según nuestro código)
    println!("5. Cambiando MUX de vuelta a Y-axis para test...");
    emulator.cpu.test_write8(0xD000, 0x00); // VIA Port B = 0x00 (MUX enabled, selector 0=Y-axis)
    
    // 6. Escribir valor diferente para generar movimiento
    println!("6. Escribiendo Y=200 para crear movimiento...");
    emulator.cpu.test_write8(0xD001, 200); // VIA Port A = 200 (nuevo valor Y)
    
    // 8. Obtener vectores generados
    let vectors = emulator.drain_vector_segments();
    
    println!("\n=== RESULTADOS ===");
    println!("Vectores generados: {}", vectors.len());
    
    if vectors.is_empty() {
        println!("❌ No se generaron vectores");
    } else {
        println!("✅ Vectores generados:");
        for (i, vector) in vectors.iter().enumerate() {
            println!("  Vector {}: start=({:.1},{:.1}), end=({:.1},{:.1}), intensity={}", 
                i + 1, vector.x0, vector.y0, vector.x1, vector.y1, vector.intensity);
        }
    }
    
    // Debug del estado actual
    let _debug_state = emulator.debug_state();
    println!("\n=== ESTADO DEBUG ===");
    println!("current_x: {}", emulator.cpu.current_x);
    println!("current_y: {}", emulator.cpu.current_y);
    println!("last_intensity: {}", emulator.cpu.last_intensity);
    println!("mux_enabled: {}", emulator.cpu.mux_enabled);
    println!("mux_selector: {}", emulator.cpu.mux_selector);
    println!("port_a_value: 0x{:02X}", emulator.cpu.port_a_value);
    println!("port_b_value: 0x{:02X}", emulator.cpu.port_b_value);
    
    // Test manual de integrator
    println!("\n=== TEST MANUAL DEL INTEGRATOR ===");
    println!("Posición inicial integrator: ({:.1}, {:.1})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Los vectores se generan automáticamente cuando el integrator detecta movimiento
    // Ya no necesitamos llamar line_to manualmente
    
    let manual_vectors = emulator.drain_vector_segments();
    println!("Vectores del test manual: {}", manual_vectors.len());
    
    if !manual_vectors.is_empty() {
        for (i, vector) in manual_vectors.iter().enumerate() {
            println!("  Manual Vector {}: start=({:.1},{:.1}), end=({:.1},{:.1}), intensity={}", 
                i + 1, vector.x0, vector.y0, vector.x1, vector.y1, vector.intensity);
        }
    }
}