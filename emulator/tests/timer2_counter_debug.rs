// Análisis específico del Timer2 Counter Low
use vectrex_emulator::emulator::Emulator;

#[test]
fn test_timer2_counter_low_behavior() {
    println!("=== ANÁLISIS TIMER2 COUNTER LOW ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    // Ejecutar hasta llegar a F4EB
    let mut step_count = 0;
    while emulator.cpu.pc != 0xF4EB && step_count < 10000 {
        emulator.step();
        step_count += 1;
    }
    
    println!("🎯 Estado VIA al llegar a F4EB:");
    println!("   IFR: 0x{:02X}", emulator.cpu.bus.via_ifr());
    println!("   IER: 0x{:02X}", emulator.cpu.bus.via_ier());
    
    // Leer directamente los registros T2C-L y T2C-H
    let t2c_low = emulator.cpu.bus.read8(0xD008);   // T2C-L
    let t2c_high = emulator.cpu.bus.read8(0xD009);  // T2C-H
    
    println!("   T2C-L (0xD008): 0x{:02X}", t2c_low);
    println!("   T2C-H (0xD009): 0x{:02X}", t2c_high);
    println!("   T2C completo: 0x{:04X}", (t2c_high as u16) << 8 | (t2c_low as u16));
    
    // Simular el patrón del bucle F4EB  
    println!("🔄 Simulando bucle F4EB:");
    for i in 0..10 {
        // LDA #$81
        let a = 0x81u8;
        
        // Leer T2C-L (como hace STX <$5A -> test A vs mem)
        let t2c_l_value = emulator.cpu.bus.read8(0xD05A);  // 0xD000 + 0x5A = 0xD05A = 0xD008+2 = T2C-L
        
        println!("   Iteración {}: A=0x{:02X}, T2C-L=0x{:02X}, A==T2C-L? {}", 
                 i, a, t2c_l_value, a == t2c_l_value);
        
        // El bucle continúa si A != T2C-L
        if a != t2c_l_value {
            println!("     → BNE: Continúa bucle (A=0x{:02X} != T2C-L=0x{:02X})", a, t2c_l_value);
        } else {
            println!("     → SALIR: A == T2C-L, el bucle debería terminar");
            break;
        }
        
        // Avanzar unos ciclos para ver si T2C-L cambia
        emulator.step();
        emulator.step();
    }
    
        
        // Verificar si Timer2 ya expiró según IFR
        let ifr = emulator.cpu.bus.via_ifr();
        if (ifr & 0x20) != 0 {
            println!("⚠️  Timer2 YA EXPIRÓ (IFR5=1), pero T2C-L no es 0!");
            println!("    Esto significa que la emulación del contador no está correcta.");
            println!("    El bucle F4EB está atascado porque T2C-L debería ser 0 cuando Timer2 expira.");
        } else {
            println!("ℹ️  Timer2 aún no ha expirado (IFR5=0)");
        }    // Intentar leer múltiples veces para ver si hay volatilidad
    println!("🧪 Lecturas múltiples de T2C-L:");
    for i in 0..5 {
        let val = emulator.cpu.bus.read8(0xD008);
        println!("   Lectura {}: T2C-L = 0x{:02X}", i, val);
    }
}