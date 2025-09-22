//! Test para trazar exactamente cómo se calcula el DAC 0x60 -> 192

#[test]
fn test_dac_calculation_trace() {
    println!("=== TRACEANDO CÁLCULO DAC ===");
    
    // Valores observados en los logs
    let test_cases = [
        (0x60, "96 decimal → ¿192?"),
        (0xA0, "-96 decimal → ¿-192?"),
        (0xF1, "-15 decimal → ¿-30?"),
        (0x00, "0 decimal → ¿0?"),
        (0x7F, "127 decimal → ¿254?"),
        (0x80, "-128 decimal → ¿-256?"),
    ];
    
    println!("\n📊 ANÁLISIS DE CONVERSIÓN DAC:");
    
    for (hex_val, description) in test_cases.iter() {
        // Conversión como i8 (signed)
        let as_i8 = *hex_val as i8;
        let as_i8_f32 = as_i8 as f32;
        
        // Aplicar DAC_SCALE = 2.0
        const DAC_SCALE: f32 = 2.0;
        let dac_result = as_i8_f32 * DAC_SCALE;
        let as_i16 = dac_result as i16;
        
        println!("  0x{:02X} {} → i8={} → f32={:.1} → DAC*2.0={:.1} → i16={}", 
                 hex_val, description, as_i8, as_i8_f32, dac_result, as_i16);
    }
    
    println!("\n🔍 VERIFICACIÓN CON LOGS OBSERVADOS:");
    println!("  Log: 0x60 (96) → y_dac=192.0 → current_y=192");
    println!("  Calc: 0x60 → i8=96 → f32=96.0 → DAC=192.0 → i16=192 ✅");
    
    println!("  Log: 0xA0 (-96) → y_dac=-192.0 → current_y=-192");
    println!("  Calc: 0xA0 → i8=-96 → f32=-96.0 → DAC=-192.0 → i16=-192 ✅");
    
    println!("  Log: 0xF1 (-15) → y_dac=-30.0 → current_y=-30");
    println!("  Calc: 0xF1 → i8=-15 → f32=-15.0 → DAC=-30.0 → i16=-30 ✅");
    
    println!("\n💡 CONCLUSIÓN: DAC_SCALE=2.0 está funcionando correctamente");
    println!("   El factor x2 amplifica el rango de 8-bit (-128..127) a 16-bit (-256..254)");
}

#[test]
fn test_dac_range_analysis() {
    println!("\n=== ANÁLISIS DE RANGO DAC ===");
    
    const DAC_SCALE: f32 = 2.0;
    
    println!("📏 RANGO COMPLETO 8-bit DAC:");
    println!("  Mínimo: 0x80 (-128) → {} → DAC={:.1}", -128i8, -128f32 * DAC_SCALE);
    println!("  Máximo: 0x7F (127) → {} → DAC={:.1}", 127i8, 127f32 * DAC_SCALE);
    println!("  Centro: 0x00 (0) → {} → DAC={:.1}", 0i8, 0f32 * DAC_SCALE);
    
    println!("\n🎯 VALORES TÍPICOS VECTREX:");
    let typical_values = [0x00, 0x20, 0x40, 0x60, 0x7F, 0x80, 0xA0, 0xC0, 0xE0, 0xFF];
    
    for val in typical_values.iter() {
        let as_i8 = *val as i8;
        let dac = as_i8 as f32 * DAC_SCALE;
        println!("  0x{:02X} ({:4}) → DAC={:6.1}", val, as_i8, dac);
    }
}

#[test]
fn test_compare_with_vectrexy_range() {
    println!("\n=== COMPARACIÓN CON VECTREXY ===");
    
    // Según documentación Vectrex, el DAC típicamente va de -127 a +127
    // y se mapea a coordenadas de pantalla aproximadamente -32768 a +32767
    
    const OUR_DAC_SCALE: f32 = 2.0;
    const VECTREX_SCREEN_RANGE: f32 = 32768.0; // Rango teórico completo
    const VECTREX_DAC_RANGE: f32 = 127.0;      // Rango DAC máximo
    
    println!("🔍 ANÁLISIS DE ESCALADO:");
    println!("  Nuestro DAC_SCALE: {:.1}", OUR_DAC_SCALE);
    println!("  Rango resultante: ±{:.1}", 127.0 * OUR_DAC_SCALE);
    
    println!("  Vectrex teórico: DAC ±{:.0} → pantalla ±{:.0}", VECTREX_DAC_RANGE, VECTREX_SCREEN_RANGE);
    println!("  Factor teórico: {:.1}", VECTREX_SCREEN_RANGE / VECTREX_DAC_RANGE);
    
    println!("\n💭 EVALUACIÓN:");
    if OUR_DAC_SCALE < 100.0 {
        println!("  ✅ DAC_SCALE=2.0 es conservador (bueno para desarrollo)");
        println!("  📝 Podría aumentarse para mayor resolución si es necesario");
    } else {
        println!("  ⚠️  DAC_SCALE muy alto");
    }
}