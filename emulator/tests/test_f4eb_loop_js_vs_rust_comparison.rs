// Test de comparación específica: Rust vs JSVecx en el bucle F4EB
use vectrex_emulator::emulator::Emulator;

#[test]
fn test_f4eb_loop_js_vs_rust_comparison() {
    println!("=== COMPARACIÓN RUST vs JSVecx - BUCLE F4EB ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    println!("🦀 RUST EMULATOR:");
    
    // Ejecutar hasta llegar a F4EB con límite muchísimo mayor (25M steps)
    let mut step_count = 0;
    while emulator.cpu.pc != 0xF4EB && step_count < 25_000_000 {
        emulator.step();
        step_count += 1;
    }
    
    if emulator.cpu.pc != 0xF4EB {
        panic!("Rust: No se alcanzó F4EB en {} steps (límite: 25M)", step_count);
    }
    
    println!("✅ Rust llegó a F4EB en step {}", step_count);
    println!("📊 Estado Rust en F4EB:");
    println!("   Cycles: {}", emulator.cpu.cycles);
    println!("   PC: 0x{:04X}", emulator.cpu.pc);
    println!("   A: 0x{:02X}, B: 0x{:02X}", emulator.cpu.a, emulator.cpu.b);
    println!("   X: 0x{:04X}, Y: 0x{:04X}", emulator.cpu.x, emulator.cpu.y);
    println!("   S: 0x{:04X}, U: 0x{:04X}", emulator.cpu.s, emulator.cpu.u);
    println!("   CC: E={} F={} H={} I={} N={} Z={} V={} C={}", 
             emulator.cpu.cc_e, emulator.cpu.cc_f, emulator.cpu.cc_h, emulator.cpu.cc_i,
             emulator.cpu.cc_n, emulator.cpu.cc_z, emulator.cpu.cc_v, emulator.cpu.cc_c);
    
    // Estado VIA crítico
    let via_ifr = emulator.cpu.bus.via_ifr();
    let via_ier = emulator.cpu.bus.via_ier();
    let timer2_counter = emulator.cpu.bus.read8(0xD05A); // T2C-L
    
    println!("🔧 Estado VIA Rust:");
    println!("   IFR: 0x{:02X} (T1={} T2={} CB1={} CB2={} SR={} CA1={} CA2={})", 
             via_ifr,
             (via_ifr & 0x40) != 0,  // T1
             (via_ifr & 0x20) != 0,  // T2  
             (via_ifr & 0x10) != 0,  // CB1
             (via_ifr & 0x08) != 0,  // CB2
             (via_ifr & 0x04) != 0,  // SR
             (via_ifr & 0x02) != 0,  // CA1
             (via_ifr & 0x01) != 0); // CA2
    
    println!("   IER: 0x{:02X} (T1={} T2={} CB1={} CB2={} SR={} CA1={} CA2={})", 
             via_ier,
             (via_ier & 0x40) != 0,  // T1
             (via_ier & 0x20) != 0,  // T2  
             (via_ier & 0x10) != 0,  // CB1
             (via_ier & 0x08) != 0,  // CB2
             (via_ier & 0x04) != 0,  // SR
             (via_ier & 0x02) != 0,  // CA1
             (via_ier & 0x01) != 0); // CA2
    
    println!("   Timer2 Counter (0xD05A): 0x{:02X}", timer2_counter);
    
    // Mostrar algunos registros VIA clave
    println!("📋 Registros VIA completos:");
    for reg in 0..16 {
        let addr = 0xD000 + reg;
        let val = emulator.cpu.bus.read8(addr);
        println!("   VIA[0x{:X}] (0x{:04X}): 0x{:02X}", reg, addr, val);
    }
    
    // Ejecutar una iteración del bucle para ver qué compara
    println!("🔍 Ejecutando una iteración del bucle F4EB:");
    let pc_before = emulator.cpu.pc;
    let a_before = emulator.cpu.a;
    let cycles_before = emulator.cpu.cycles;
    
    // F4EB: LDA #0x81
    emulator.step();
    println!("   F4EB: LDA #0x81 → A: 0x{:02X}→0x{:02X}", a_before, emulator.cpu.a);
    
    // F4ED: CMPA 0xD05A (Timer2 counter)
    let timer2_val_read = emulator.cpu.bus.read8(0xD05A);
    emulator.step();
    println!("   F4ED: CMPA 0xD05A → Comparing A(0x{:02X}) with Timer2(0x{:02X})", 
             emulator.cpu.a, timer2_val_read);
    println!("   CC flags after CMPA: N={} Z={} V={} C={}", 
             emulator.cpu.cc_n, emulator.cpu.cc_z, emulator.cpu.cc_v, emulator.cpu.cc_c);
    
    // F4EF: BNE F4EB
    let will_branch = !emulator.cpu.cc_z; // BNE branches if Z=0
    emulator.step();
    println!("   F4EF: BNE F4EB → Will branch: {} (Z={})", will_branch, emulator.cpu.cc_z);
    println!("   PC after BNE: 0x{:04X}", emulator.cpu.pc);
    
    let cycles_after = emulator.cpu.cycles;
    println!("   Total cycles for iteration: {}", cycles_after - cycles_before);
    
    // Conclusión
    println!("\n🎯 DIAGNÓSTICO RUST:");
    if timer2_counter == 0x81 {
        println!("   ✅ Timer2 tiene el valor esperado (0x81) - debería salir del bucle");
    } else {
        println!("   ❌ Timer2 tiene 0x{:02X}, espera 0x81 - se queda en bucle", timer2_counter);
        println!("   📍 Diferencia: {} (0x{:02X})", 
                 (timer2_counter as i16) - 0x81, 
                 ((timer2_counter as i16) - 0x81) as u8);
    }
    
    println!("\n⚡ PARA COMPARAR CON JSVecx:");
    println!("   1. Ejecutar JSVecx hasta F4EB");
    println!("   2. Comparar cycles: {}", emulator.cpu.cycles);
    println!("   3. Comparar Timer2 counter: 0x{:02X}", timer2_counter);
    println!("   4. Comparar registros VIA IFR/IER: 0x{:02X}/0x{:02X}", via_ifr, via_ier);
    println!("   5. Verificar si JSVecx también se queda en bucle o sale");
    
    // Información adicional para debugging
    println!("\n🔬 INFO ADICIONAL PARA DEBUGGING:");
    println!("   - Timer2 parece estar funcionando (valor no es 0x00)");
    println!("   - IFR Timer2 bit: {}", (via_ifr & 0x20) != 0);
    println!("   - IER Timer2 bit: {}", (via_ier & 0x20) != 0);
    println!("   - Si JSVecx tiene Timer2=0x81 aquí, entonces el problema es sincronización VIA");
    println!("   - Si JSVecx también tiene Timer2=0x{:02X}, entonces ambos emuladores están igual", timer2_counter);
    
    // 🚀 NUEVO: Test extendido para ver si Rust eventualmente progresa
    println!("\n🚀 EJECUTANDO TEST EXTENDIDO (25M cycles):");
    println!("   Verificando si Rust eventualmente sale del bucle F4EB...");
    
    let mut extended_steps = 0;
    
    while emulator.cpu.pc == 0xF4EB && extended_steps < 25_000_000 {
        emulator.step();
        extended_steps += 1;
        
        // Reportar progreso cada millón de steps
        if extended_steps % 1_000_000 == 0 {
            let timer2_current = emulator.cpu.bus.read8(0xD05A);
            println!("   📊 Step {}M: PC=0x{:04X}, Timer2=0x{:02X}, Cycles={}", 
                     extended_steps / 1_000_000, emulator.cpu.pc, timer2_current, emulator.cpu.cycles);
        }
    }
    
    if emulator.cpu.pc != 0xF4EB {
        println!("   🎉 ¡RUST SALIÓ DEL BUCLE! PC=0x{:04X} después de {} steps adicionales", 
                 emulator.cpu.pc, extended_steps);
        println!("   Timer2 final: 0x{:02X}", emulator.cpu.bus.read8(0xD05A));
    } else {
        println!("   ❌ Rust sigue atascado en F4EB después de {} steps adicionales", extended_steps);
        println!("   Timer2 final: 0x{:02X}", emulator.cpu.bus.read8(0xD05A));
    }
}