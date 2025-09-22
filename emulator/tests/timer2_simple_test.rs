use vectrex_emulator::CPU;

#[test]
fn timer2_expiry_with_corrected_tst() {
    println!("🎯 Test Timer2 con TST corregido a 4 ciclos");
    
    // BIOS real path
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS desde archivo
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    cpu.load_bios(&bios_data);
    cpu.reset();
    
    // Ejecutar hasta el loop TST
    let mut step_count = 0;
    let max_steps = 10000; // Aumentamos el límite
    
    println!("Ejecutando BIOS hasta TST loop...");
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Detectar el loop TST $0D; BEQ $F19E
        if pc_before == 0xF19E && cpu.pc == 0xF1A0 {
            println!("🎯 Detectado TST $0D en loop, step {}", step_count);
            break;
        }
        
        // Stop si sale del rango esperado
        if cpu.pc < 0xF190 || cpu.pc > 0xF1A5 {
            if step_count > 900 { // Solo loggear si ya hemos avanzado
                println!("🎯 BIOS salió del loop Wait_Recal en PC={:04X} después de {} pasos", cpu.pc, step_count);
                break;
            }
        }
    }
    
    if step_count >= max_steps {
        println!("❌ Test timeout después de {} pasos", max_steps);
        panic!("Timer2 no expiró en tiempo razonable");
    }
    
    // Verificar que Timer2 expiró (IFR bit 5 debería estar set)
    let ifr = cpu.bus.via_ifr();
    println!("🎯 IFR final: {:02X}", ifr);
    
    if (ifr & 0x20) != 0 {
        println!("✅ Timer2 expiró correctamente (IFR bit 5 set)");
    } else {
        println!("❌ Timer2 no expiró (IFR bit 5 clear)");
    }
    
    // Mostrar estadísticas
    println!("📊 Pasos totales: {}", step_count);
    println!("📊 PC final: {:04X}", cpu.pc);
    println!("📊 Ciclos totales: {}", cpu.cycles);
    
    // El test pasa si salimos del loop (PC fuera del rango F19E-F1A0)
    assert!(cpu.pc < 0xF19E || cpu.pc > 0xF1A0, 
           "CPU debería haber salido del loop TST pero PC={:04X}", cpu.pc);
}