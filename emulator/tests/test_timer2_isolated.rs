use vectrex_emulator::*;

#[test]
fn test_timer2_isolated_behavior() {
    let bios_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Cannot read BIOS file");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios_data);
    
    println!("=== TESTING TIMER2 ISOLATED BEHAVIOR ===");
    
    // Configurar Timer2 manualmente como lo hace Set_Refresh
    // En la BIOS: LDD $C83D -> STD <VIA_t2_lo 
    // El valor por defecto debería ser $7530 (0x3075)
    let refresh_value = 0x7530u16;
    let refresh_lo = (refresh_value & 0xFF) as u8;
    let refresh_hi = (refresh_value >> 8) as u8;
    
    println!("Configurando Timer2 con valor {:04X} (lo={:02X}, hi={:02X})", refresh_value, refresh_lo, refresh_hi);
    
    // Habilitar Timer2 interrupts en IER primero (bit 5 = Timer2, bit 7 = master enable)
    cpu.bus.write8(0xD00E, 0xA0);  // IER: enable Timer2 interrupts (0x80 | 0x20)
    println!("IER after enable: {:02X}", cpu.bus.via_ier());
    
    // Escribir Timer2 latch
    cpu.bus.write8(0xD008, refresh_lo);  // T2L-L: low byte del latch 
    cpu.bus.write8(0xD009, refresh_hi);  // T2L-H: high byte del latch (esto inicia el timer)
    
    println!("Timer2 configurado. IFR inicial: {:02X}", cpu.bus.via_ifr());
    
    // Simular ciclos para ver si Timer2 funciona
    let mut cycle = 0;
    let max_cycles = refresh_value + 100; // Un poco más del valor esperado
    
    loop {
        cycle += 1;
        cpu.bus.via.tick(1);
        
        let current_ifr = cpu.bus.via_ifr();
        
        // Reportar cada 1000 ciclos o cuando cambie IFR
        if cycle % 1000 == 0 || (current_ifr & 0x20) != 0 {
            println!("Cycle {}: IFR={:02X}", cycle, current_ifr);
        }
        
        // Verificar si Timer2 expiró (IFR bit 5)
        if (current_ifr & 0x20) != 0 {
            println!("🎯 Timer2 EXPIRED at cycle {}! Expected around cycle {}", cycle, refresh_value);
            
            // Verificar si IRQ está activo
            let irq_active = cpu.bus.via.irq_asserted();
            println!("   IRQ line active: {}", irq_active);
            println!("   Final IFR: {:02X}", current_ifr);
            println!("   Final IER: {:02X}", cpu.bus.via_ier());
            
            break;
        }
        
        if cycle >= max_cycles {
            println!("❌ Timer2 did NOT expire after {} cycles (expected ~{})", cycle, refresh_value);
            println!("   Final IFR: {:02X}", cpu.bus.via_ifr());
            println!("   Final IER: {:02X}", cpu.bus.via_ier());
            break;
        }
    }
    
    // Test del comportamiento de lectura IFR (lo que hace Wait_Recal)
    println!("\n--- Testing Wait_Recal behavior ---");
    println!("Wait_Recal does: BITA <VIA_int_flags ; BEQ loop");
    println!("This means: A = 0x20, test A & IFR, branch if zero result");
    
    let test_mask = 0x20u8;  // Timer2 bit
    let ifr = cpu.bus.via_ifr();
    let test_result = test_mask & ifr;
    
    println!("Test: mask={:02X} & IFR={:02X} = {:02X}", test_mask, ifr, test_result);
    println!("BEQ would branch: {}", test_result == 0);
    
    if test_result != 0 {
        println!("✅ Wait_Recal would EXIT the loop (Timer2 ready)");
    } else {
        println!("❌ Wait_Recal would STAY in loop (Timer2 not ready)");
    }
    
    println!("\n=== Timer2 Isolated Test Complete ===");
}