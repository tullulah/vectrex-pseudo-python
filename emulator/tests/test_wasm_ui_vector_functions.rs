

#[test]
fn test_wasm_ui_vector_functions() {
    println!("=== TEST FUNCIONES WASM QUE USA LA UI (núcleo Emulator) ===");

    // Cargar BIOS real (igual que hace la UI)
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_bytes = std::fs::read(bios_path)
        .expect("Failed to read BIOS file");
    println!("BIOS cargada: {} bytes", bios_bytes.len());

    // Instanciar Emulator directamente
    let mut emu = vectrex_emulator::Emulator::new();
    assert!(emu.load_bios(&bios_bytes));
    emu.reset();

    // Ejecutar unos pocos frames como la UI (step_multiple = instrucciones)
    // Ejecutar 20 frames, cada uno con 7 millones de instrucciones (total 140M)
    for frame in 0..20 {
        println!("\n--- Frame {} ---", frame);
        let _ = emu.step_multiple(7_000_000); // 7M instrucciones por frame

        // Obtener los vectores del integrador (simulando la función WASM)
        let segments = emu.cpu.integrator.take_segments();
        println!("integrator_segments: {} segmentos", segments.len());

        // Mostrar los primeros 10 vectores
        for (i, seg) in segments.iter().take(10).enumerate() {
            println!("    {}: ({:.1},{:.1}) → ({:.1},{:.1}) intensidad={}", i, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
        }
        if segments.len() > 10 {
            println!("  ... y {} más", segments.len() - 10);
        }
    }
}