use vectrex_emulator::Emulator;

fn main() {
    println!("🔍 TEST SIMPLE: Investigando por qué Y-axis siempre es 0");
    
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    
    match std::fs::read(bios_path) {
        Ok(bios_data) => {
            emulator.cpu.bus.load_block(0xE000, &bios_data, false); // BIOS en ROM
            println!("✅ BIOS cargada desde: {}", bios_path);
        },
        Err(e) => {
            println!("❌ Error cargando BIOS: {}", e);
            return;
        }
    }
    
    println!("\n=== TEST MANUAL: Escribiendo directamente a VIA ===");
    
    // 1. Habilitar MUX con selector Y-axis (Port B = 0x00)
    println!("1. Habilitando MUX para Y-axis (Port B = 0x00)...");
    emulator.cpu.test_write8(0xD000, 0x00); // VIA Port B = 0x00 (MUX enabled, selector 0=Y-axis)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // 2. Escribir valor Y a Port A  
    println!("2. Escribiendo Y=100 a Port A...");
    emulator.cpu.test_write8(0xD001, 100); // VIA Port A = 100 (valor Y)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // 3. Cambiar MUX a brightness pero mantener habilitado
    println!("3. Cambiando MUX a brightness (Port B = 0x04)...");
    emulator.cpu.test_write8(0xD000, 0x04); // VIA Port B = 0x04 (MUX enabled, selector 2=brightness)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // 4. Escribir brightness
    println!("4. Escribiendo brightness=255...");
    emulator.cpu.test_write8(0xD001, 255); // VIA Port A = 255 (brightness máxima)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // 5. Cambiar MUX de vuelta a Y-axis
    println!("5. Cambiando MUX de vuelta a Y-axis...");
    emulator.cpu.test_write8(0xD000, 0x00); // VIA Port B = 0x00 (MUX enabled, selector 0=Y-axis)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // 6. Escribir valor diferente para generar movimiento
    println!("6. Escribiendo Y=200 para crear movimiento...");
    emulator.cpu.test_write8(0xD001, 200); // VIA Port A = 200 (nuevo valor Y)
    emulator.step(); // ⭐ PROCESAR LA ESCRITURA
    
    // Drenar los eventos VIA para ver qué se registró
    let via_writes = emulator.cpu.drain_via_writes();
    println!("\n📋 Eventos VIA registrados ({} eventos):", via_writes.len());
    for (i, event) in via_writes.iter().enumerate() {
        println!("  {}: {:?}", i + 1, event);
    }
    
    // Ver si se generaron vectores
    let vectors = emulator.drain_vector_segments();
    println!("\n🎯 Vectores generados ({} vectores):", vectors.len());
    if vectors.is_empty() {
        println!("  ❌ No se generaron vectores");
    } else {
        for (i, segment) in vectors.iter().take(5).enumerate() {
            println!("  {}: ({:.1},{:.1}) -> ({:.1},{:.1}) intensidad={} frame={}", 
                i + 1, segment.x0, segment.y0, segment.x1, segment.y1, segment.intensity, segment.frame);
        }
        if vectors.len() > 5 {
            println!("  ... y {} vectores más", vectors.len() - 5);
        }
    }
    
    println!("\n=== COMPARACIÓN CON BIOS ===");
    println!("Ahora vamos a ejecutar la BIOS normal y ver la diferencia...");
    
    // Reset y ejecutar BIOS normalmente por unos ciclos
    emulator = Emulator::new();
    emulator.cpu.bus.load_block(0xE000, &std::fs::read(bios_path).unwrap(), false);
    
    // Ejecutar unos pocos ciclos de BIOS
    for _ in 0..1000 {
        emulator.step();
    }
    
    let bios_via_writes = emulator.cpu.drain_via_writes();
    println!("\n📋 Eventos VIA de BIOS real ({} eventos):", bios_via_writes.len());
    
    // Mostrar solo los primeros 10 para no abrumar
    for (i, event) in bios_via_writes.iter().take(10).enumerate() {
        println!("  {}: {:?}", i + 1, event);
    }
    if bios_via_writes.len() > 10 {
        println!("  ... y {} eventos más", bios_via_writes.len() - 10);
    }
    
    let bios_vectors = emulator.drain_vector_segments();
    println!("\n🎯 Vectores de BIOS ({} vectores):", bios_vectors.len());
    if bios_vectors.is_empty() {
        println!("  ❌ BIOS no generó vectores en 1000 ciclos");
    } else {
        for (i, segment) in bios_vectors.iter().take(5).enumerate() {
            println!("  {}: ({:.1},{:.1}) -> ({:.1},{:.1}) intensidad={} frame={}", 
                i + 1, segment.x0, segment.y0, segment.x1, segment.y1, segment.intensity, segment.frame);
        }
        if bios_vectors.len() > 5 {
            println!("  ... y {} vectores más", bios_vectors.len() - 5);
        }
    }
}