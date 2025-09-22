//! Test de la nueva arquitectura Emulator
use vectrex_emulator::Emulator;

#[test]
fn test_nueva_arquitectura_basica() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { 
        Ok(b) => b, 
        Err(e) => { 
            eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); 
            return; 
        } 
    };
    
    // Crear emulador con nueva arquitectura
    let mut emulator = Emulator::new();
    
    // Cargar BIOS
    assert!(emulator.load_bios(&bios), "Failed to load BIOS");
    
    // Reset
    emulator.reset();
    
    // Verificar estado inicial
    let debug_state = emulator.debug_state();
    println!("[NEW_ARCH] Estado inicial:");
    println!("  PC: {:04X}", debug_state.cpu_pc);
    println!("  Cycles: {}", debug_state.total_cycles);
    println!("  Frames: {}", debug_state.total_frames);
    
    // Ejecutar algunos pasos
    let mut steps = 0;
    for i in 0..100 {
        if !emulator.step() {
            println!("[NEW_ARCH] CPU halt en step {}", i);
            break;
        }
        steps = i + 1;
    }
    
    // Verificar que se ejecutó
    let final_state = emulator.debug_state();
    println!("[NEW_ARCH] Después de {} steps:", steps);
    println!("  PC: {:04X}", final_state.cpu_pc);
    println!("  Cycles: {}", final_state.total_cycles);
    println!("  Instructions: {}", emulator.stats.instructions_executed);
    
    // Verificaciones básicas
    assert!(final_state.cpu_pc != 0, "PC should have changed");
    assert!(emulator.stats.instructions_executed > 0, "Should have executed instructions");
    assert!(final_state.total_cycles > 0, "Should have consumed cycles");
    
    // Probar drenaje de segmentos
    let segments = emulator.drain_vector_segments();
    println!("[NEW_ARCH] Segmentos drenados: {}", segments.len());
    
    println!("[NEW_ARCH] ✅ Nueva arquitectura funciona correctamente!");
}

#[test]
fn test_comparacion_arquitecturas() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { 
        Ok(b) => b, 
        Err(e) => { 
            eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); 
            return; 
        } 
    };
    
    // Test con CPU viejo
    let mut cpu_old = vectrex_emulator::CPU::default();
    cpu_old.load_bios(&bios);
    cpu_old.reset();
    
    // Test con Emulator nuevo
    let mut emulator_new = Emulator::new();
    emulator_new.load_bios(&bios);
    emulator_new.reset();
    
    // Ejecutar la misma cantidad de pasos en ambos
    let steps = 50;
    
    for _ in 0..steps {
        cpu_old.step();
        emulator_new.step();
    }
    
    // Comparar estados finales
    let debug_state = emulator_new.debug_state();
    
    println!("[COMPARISON]");
    println!("  CPU old PC: {:04X} | Emulator new PC: {:04X}", cpu_old.pc, debug_state.cpu_pc);
    println!("  CPU old A:  {:02X}   | Emulator new A:  {:02X}", cpu_old.a, debug_state.cpu_a);
    println!("  CPU old B:  {:02X}   | Emulator new B:  {:02X}", cpu_old.b, debug_state.cpu_b);
    
    // Deberían ser iguales (misma lógica subyacente)
    assert_eq!(cpu_old.pc, debug_state.cpu_pc, "PC should match");
    assert_eq!(cpu_old.a, debug_state.cpu_a, "Register A should match");
    assert_eq!(cpu_old.b, debug_state.cpu_b, "Register B should match");
    
    println!("[COMPARISON] ✅ Ambas arquitecturas producen resultados idénticos!");
}