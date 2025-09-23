// Análisis específico del bucle F4EB
use vectrex_emulator::emulator::Emulator;

#[test]
fn test_analyze_f4eb_loop() {
    println!("=== ANÁLISIS DEL BUCLE F4EB ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    // Ejecutar hasta llegar a F4EB
    let mut step_count = 0;
    let mut recent_pcs: Vec<u16> = Vec::new();
    
    while emulator.cpu.pc != 0xF4EB && step_count < 50000 {
        let current_pc = emulator.cpu.pc;
        
        // Mantener los últimos 10 PCs para contexto
        recent_pcs.push(current_pc);
        if recent_pcs.len() > 10 {
            recent_pcs.remove(0);
        }
        
        // Capturar cuando llegue al rango F4D0-F4EA para debug
        if current_pc >= 0xF4D0 && current_pc <= 0xF4EA && step_count > 0 {
            println!("🔍 PC TRACE: step {}, PC: 0x{:04X}, X: 0x{:04X}, S: 0x{:04X}", 
                     step_count, current_pc, emulator.cpu.x, emulator.cpu.s);
        }
        
        // Si llegamos a F4E1 (TFR S,X), mostrar contexto
        if current_pc == 0xF4E1 {
            println!("🚨 ABOUT TO EXECUTE TFR S,X at F4E1!");
            println!("   Recent PCs: {:?}", recent_pcs);
            println!("   Current state: X=0x{:04X}, S=0x{:04X}", emulator.cpu.x, emulator.cpu.s);
        }
        
        emulator.step();
        step_count += 1;
    }
    
    if emulator.cpu.pc != 0xF4EB {
        panic!("No se alcanzó F4EB en {} steps", step_count);
    }
    
    println!("🎯 Llegamos a F4EB en step {}", step_count);
    println!("📍 Estado al entrar en F4EB:");
    println!("   A: 0x{:02X}, B: 0x{:02X}", emulator.cpu.a, emulator.cpu.b);
    println!("   X: 0x{:04X}, Y: 0x{:04X}", emulator.cpu.x, emulator.cpu.y);
    println!("   IFR: 0x{:02X}, IER: 0x{:02X}", emulator.cpu.bus.via_ifr(), emulator.cpu.bus.via_ier());
    
    // Leer algunas instrucciones desde F4EB para entender el bucle
    println!("🔍 Código en F4EB:");
    for i in 0..10 {
        let addr = 0xF4EB + i;
        let byte = emulator.cpu.bus.read8(addr);
        println!("   F4{:02X}: 0x{:02X}", 0xEB + i, byte);
    }
    
    // Ejecutar algunas iteraciones para ver el patrón
    println!("🔄 Ejecutando 20 iteraciones del bucle:");
    for i in 0..20 {
        let pc_before = emulator.cpu.pc;
        let a_before = emulator.cpu.a;
        let ifr_before = emulator.cpu.bus.via_ifr();
        
        emulator.step();
        
        let pc_after = emulator.cpu.pc;
        let a_after = emulator.cpu.a;
        let ifr_after = emulator.cpu.bus.via_ifr();
        
        println!("   {}: PC 0x{:04X}→0x{:04X}, A 0x{:02X}→0x{:02X}, IFR 0x{:02X}→0x{:02X}", 
                 i, pc_before, pc_after, a_before, a_after, ifr_before, ifr_after);
        
        if pc_after == 0xF4EB && i > 5 {
            println!("   ⚠️  Bucle confirmado: vuelve a F4EB");
            break;
        }
    }
    
    // Analizar si está esperando algo específico
    println!("🎯 Estado VIA detallado:");
    println!("   IFR bits: T1={} T2={} CB1={} CB2={} SR={} CA1={} CA2={}", 
             (emulator.cpu.bus.via_ifr() & 0x40) != 0,  // T1
             (emulator.cpu.bus.via_ifr() & 0x20) != 0,  // T2  
             (emulator.cpu.bus.via_ifr() & 0x10) != 0,  // CB1
             (emulator.cpu.bus.via_ifr() & 0x08) != 0,  // CB2
             (emulator.cpu.bus.via_ifr() & 0x04) != 0,  // SR
             (emulator.cpu.bus.via_ifr() & 0x02) != 0,  // CA1
             (emulator.cpu.bus.via_ifr() & 0x01) != 0); // CA2
    
    println!("   IER bits: T1={} T2={} CB1={} CB2={} SR={} CA1={} CA2={}", 
             (emulator.cpu.bus.via_ier() & 0x40) != 0,  // T1
             (emulator.cpu.bus.via_ier() & 0x20) != 0,  // T2  
             (emulator.cpu.bus.via_ier() & 0x10) != 0,  // CB1
             (emulator.cpu.bus.via_ier() & 0x08) != 0,  // CB2
             (emulator.cpu.bus.via_ier() & 0x04) != 0,  // SR
             (emulator.cpu.bus.via_ier() & 0x02) != 0,  // CA1
             (emulator.cpu.bus.via_ier() & 0x01) != 0); // CA2
}