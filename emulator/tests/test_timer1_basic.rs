use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_timer1_basic_countdown() {
    let mut cpu = CPU::default(); // Con BIOS cargada
    
    println!("🔧 INITIAL STATE: timer1_low={}, timer1_high={}, timer1_counter={}, timer1_enabled={}", 
             cpu.timer1_low, cpu.timer1_high, cpu.timer1_counter, cpu.timer1_enabled);
    
    // Configurar Timer1: Low=0x7F, High=0x00 como en Mine Storm
    cpu.test_write8(0xD004, 0x7F); // Timer1 Low = 127
    println!("🔧 AFTER LOW WRITE: timer1_low={}, timer1_counter={}", 
             cpu.timer1_low, cpu.timer1_counter);
             
    cpu.test_write8(0xD005, 0x00); // Timer1 High = 0 -> counter = 0x007F = 127
    println!("🔧 AFTER HIGH WRITE: timer1_high={}, timer1_counter={}, timer1_enabled={}", 
             cpu.timer1_high, cpu.timer1_counter, cpu.timer1_enabled);
    
    // Verificar que timer se configuró
    assert_eq!(cpu.timer1_low, 0x7F);
    assert_eq!(cpu.timer1_high, 0x00);
    assert_eq!(cpu.timer1_counter, 127);
    assert_eq!(cpu.timer1_enabled, true);
    
    println!("⏰ Timer1 inicial: counter={}, enabled={}", cpu.timer1_counter, cpu.timer1_enabled);
    
    // Ejecutar suficientes steps para que expire el timer
    let mut steps = 0;
    let initial_expiries = cpu.t1_expiries;
    
    while cpu.timer1_enabled && steps < 200 { // Máximo 200 steps para seguridad
        cpu.step();
        steps += 1;
        
        if steps % 50 == 0 {
            println!("⏰ Step {}: counter={}, enabled={}, expiries={}", 
                    steps, cpu.timer1_counter, cpu.timer1_enabled, cpu.t1_expiries);
        }
    }
    
    // Verificar que el timer expiró y generó IRQ
    assert_eq!(cpu.timer1_enabled, false, "Timer debería estar deshabilitado tras expirar");
    assert_eq!(cpu.timer1_counter, 0, "Counter debería ser 0 tras expirar");
    assert!(cpu.t1_expiries > initial_expiries, "Debería haber al menos una expiración");
    
    println!("✅ Timer1 expiró correctamente en {} steps, expiries={}", steps, cpu.t1_expiries);
    
    // Verificar que IRQ está pendiente (depende de IER)
    let ifr = cpu.bus.via_ifr();
    println!("📋 IFR final: 0x{:02X} (bit 6={}, master bit={})", 
             ifr, (ifr & 0x40) != 0, (ifr & 0x80) != 0);
    
    // El bit 6 (Timer1) debería estar set en IFR
    assert_ne!(ifr & 0x40, 0, "Timer1 IRQ bit (6) debería estar activo en IFR");
}