use vectrex_emulator::*;

#[test]
fn test_via_timer1_countdown() {
    let bios_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Cannot read BIOS file");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios_data);
    
    // Configurar Timer1 directamente (simulando lo que hace la BIOS)
    // Escribir latch Timer1 con valor pequeño para que expire rápido
    cpu.bus.write8(0xD006, 0x10);  // T1L-L: low byte del latch
    cpu.bus.write8(0xD007, 0x00);  // T1L-H: high byte del latch
    
    // Habilitar Timer1 interrupts en IER (bit 6 = Timer1, bit 7 = master enable)
    cpu.bus.write8(0xD00E, 0xC0);  // IER: enable Timer1 interrupts (0x80 | 0x40)
    
    // Iniciar Timer1 escribiendo en T1C-H
    cpu.bus.write8(0xD005, 0x00);  // T1C-H: esto inicia el timer
    
    println!("=== TIMER1 COUNTDOWN TEST ===");
    println!("Antes de configurar latch:");
    println!("  T1L-L: 0x{:02X}, T1L-H: 0x{:02X}", cpu.bus.read8(0xD006), cpu.bus.read8(0xD007));
    
    // Configurar Timer1 directamente (simulando lo que hace la BIOS)
    // Escribir latch Timer1 con valor pequeño para que expire rápido
    cpu.bus.write8(0xD004, 0x10);  // T1C-L/T1L-L: low byte del latch 
    
    println!("Después de escribir T1 low:");
    println!("  Register 0xD004: 0x{:02X}", cpu.bus.read8(0xD004));
    
    // Habilitar Timer1 interrupts en IER (bit 6 = Timer1, bit 7 = master enable)
    cpu.bus.write8(0xD00E, 0xC0);  // IER: enable Timer1 interrupts (0x80 | 0x40)
    
    // Iniciar Timer1 escribiendo en T1C-H
    cpu.bus.write8(0xD005, 0x00);  // T1C-H: esto inicia el timer y carga el counter desde latch
    
    println!("Después de escribir T1C-H (0xD005):");
    println!("  T1 latch final: 0x{:04X}", cpu.bus.via.t1_latch());
    println!("  IER: 0x{:02X}", cpu.bus.via_ier());
    println!("  IFR: 0x{:02X}", cpu.bus.via_ifr());
    println!("  Timer1 counter: 0x{:04X}", cpu.bus.via.t1_counter());
    println!("  T1 enabled: {}", cpu.bus.via.t1_enabled());
    println!("  T1 int enabled: {}", cpu.bus.via.t1_int_enabled());
    println!("  CPU IRQ pending: {}", cpu.irq_pending);
    println!("  CPU cc_i: {}", cpu.cc_i);
    
    // Hacer tick por muchos ciclos para ver si el timer expira
    for step in 1..=50 {
        cpu.bus.via.tick(1);
        
        let counter = cpu.bus.via.t1_counter();
        let ifr = cpu.bus.via_ifr();
        let irq_line = cpu.bus.via.irq_asserted();
        
        if step <= 20 || counter <= 5 || ifr != 0 || irq_line {
            println!("Step {}: T1_counter=0x{:04X}, IFR=0x{:02X}, IRQ_line={}, CPU_irq_pending={}", 
                step, counter, ifr, irq_line, cpu.irq_pending);
        }
        
        if irq_line {
            println!("🎯 TIMER1 INTERRUPT GENERADO en step {}", step);
            break;
        }
        
        if counter == 0xFFFF {
            println!("🔄 TIMER1 EXPIRÓ (underflow) en step {}", step);
        }
    }
    
    println!("=== RESULTADO FINAL ===");
    println!("Timer1 counter final: 0x{:04X}", cpu.bus.via.t1_counter());
    println!("IFR final: 0x{:02X}", cpu.bus.via_ifr());
    println!("IRQ line final: {}", cpu.bus.via.irq_asserted());
    println!("CPU IRQ pending final: {}", cpu.irq_pending);
    println!("VIA IRQ count: {}", cpu.via_irq_count);
}