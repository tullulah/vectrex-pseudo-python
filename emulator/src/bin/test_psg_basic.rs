// Test PSG (AY-3-8912) basic functionality via VIA Port A/B control
use vectrex_emulator::*;

fn main() {
    println!("Testing PSG (AY-3-8912) via VIA Port control...");

    let mut emulator = Emulator::new();
    
    // Test básico: escribir a un registro PSG y verificar que se actualiza
    
    println!("\n1. Test: Configurar PSG Channel A frequency (440 Hz aproximadamente)");
    
    // Para escribir al PSG:
    // 1. Latch address: Port B = 0x08 (BC1=1, BDIR=0), Port A = register number
    // 2. Latch data: Port B = 0x18 (BC1=1, BDIR=1), Port A = data value
    
    // Configurar Canal A con frecuencia ~440 Hz
    // Formula: freq = clock_hz / (16 * period)
    // Para 440 Hz con clock 1.5MHz: period = 1500000 / (16 * 440) ≈ 213
    
    // Escribir registro 0 (Channel A fine tune) = 213 & 0xFF = 213
    println!("   Escribiendo registro 0 (Channel A fine): valor 213");
    emulator.cpu.test_write8(0xD001, 0x00);  // Port A = register 0
    emulator.cpu.test_write8(0xD000, 0x08);  // Port B = BC1=1, BDIR=0 (LATCH ADDRESS)
    emulator.cpu.test_write8(0xD001, 213);   // Port A = valor 213
    emulator.cpu.test_write8(0xD000, 0x18);  // Port B = BC1=1, BDIR=1 (LATCH DATA)
    
    // Escribir registro 1 (Channel A coarse tune) = (213 >> 8) = 0
    println!("   Escribiendo registro 1 (Channel A coarse): valor 0");
    emulator.cpu.test_write8(0xD001, 0x01);  // Port A = register 1
    emulator.cpu.test_write8(0xD000, 0x08);  // Port B = BC1=1, BDIR=0 (LATCH ADDRESS)
    emulator.cpu.test_write8(0xD001, 0);     // Port A = valor 0
    emulator.cpu.test_write8(0xD000, 0x18);  // Port B = BC1=1, BDIR=1 (LATCH DATA)
    
    println!("\n2. Test: Habilitar Canal A con volumen máximo");
    
    // Escribir registro 7 (Mixer): habilitar solo tone A (disable noise)
    // Bits: 0=tone A enable (0=enable), 3=noise A enable (1=disable)
    println!("   Escribiendo registro 7 (Mixer): habilitar tone A");
    emulator.cpu.test_write8(0xD001, 0x07);  // Port A = register 7
    emulator.cpu.test_write8(0xD000, 0x08);  // Port B = BC1=1, BDIR=0 (LATCH ADDRESS)
    emulator.cpu.test_write8(0xD001, 0x3E);  // Port A = 0x3E (tone A enabled, todo noise disabled)
    emulator.cpu.test_write8(0xD000, 0x18);  // Port B = BC1=1, BDIR=1 (LATCH DATA)
    
    // Escribir registro 8 (Channel A amplitude): volumen máximo
    println!("   Escribiendo registro 8 (Channel A amplitude): volumen 15");
    emulator.cpu.test_write8(0xD001, 0x08);  // Port A = register 8
    emulator.cpu.test_write8(0xD000, 0x08);  // Port B = BC1=1, BDIR=0 (LATCH ADDRESS)
    emulator.cpu.test_write8(0xD001, 15);    // Port A = valor 15 (máximo volumen)
    emulator.cpu.test_write8(0xD000, 0x18);  // Port B = BC1=1, BDIR=1 (LATCH DATA)
    
    println!("\n3. Test: Ejecutar ciclos y verificar generación de audio");
    
    // Ejecutar algunos ciclos para que el PSG genere muestras
    let cycles_to_run = 44100;  // 1 segundo de audio a 44.1kHz
    for _ in 0..cycles_to_run {
        emulator.cpu.bus.tick(1);
    }
    
    // Verificar métricas del PSG
    let psg = &emulator.cpu.bus.psg;
    println!("   PSG samples generadas: {}", psg.metric_samples);
    println!("   PSG tone toggles: {}", psg.metric_tone_toggles);
    println!("   PSG noise shifts: {}", psg.metric_noise_shifts);
    
    // Verificar que se han generado samples
    if psg.metric_samples > 0 {
        println!("   ✅ PSG está generando audio correctamente!");
    } else {
        println!("   ❌ PSG no está generando audio");
    }
    
    // Verificar que el tone está haciendo toggle
    if psg.metric_tone_toggles > 0 {
        println!("   ✅ Channel A está toggleando correctamente!");
    } else {
        println!("   ❌ Channel A no está activo");
    }
    
    println!("\n4. Test: Export audio data");
    
    // Test export de PCM
    let exported = emulator.cpu.bus.psg.drain_pcm();
    println!("   Samples exportadas: {}", exported.len());
    
    if !exported.is_empty() {
        println!("   Primera muestra: {}", exported[0]);
        println!("   Última muestra: {}", exported[exported.len()-1]);
        
        // Verificar que no todo son ceros (indicaría que hay audio)
        let non_zero_samples = exported.iter().filter(|&&x| x != 0).count();
        println!("   Samples no-cero: {}/{}", non_zero_samples, exported.len());
        
        if non_zero_samples > 0 {
            println!("   ✅ Audio PCM data contiene señal!");
        } else {
            println!("   ⚠️  Audio PCM data está en silencio");
        }
    }
    
    println!("\nTest PSG completado!");
}