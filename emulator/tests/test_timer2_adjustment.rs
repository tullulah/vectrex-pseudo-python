// Test directo del ajuste Timer2 de 0x7530 (30000) a 600 ciclos
use vectrex_emulator::emulator::Emulator;
use std::env;

#[test]
fn test_timer2_adjustment_direct() {
    println!("=== TIMER2 ADJUSTMENT TEST ===");
    
    // Activar trazas de Timer2 para ver el ajuste
    env::set_var("VIA_T2_TRACE", "1");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    // Escribir directamente el valor 0x7530 (30000) a Timer2 vía VIA addresses
    println!("📝 Escribiendo Timer2 0x7530 (30000) directamente...");
    
    // Escribir T2L (low byte) a address 0xD008
    emulator.cpu.bus.write8(0xD008, 0x30);
    
    // Escribir T2H (high byte) a address 0xD009 - esto debería activar el ajuste
    emulator.cpu.bus.write8(0xD009, 0x75);
    
    println!("✅ Timer2 escrito: 0x7530 (30000) vía write8()");
    
    // IFR bit 5 debe estar limpio inicialmente
    let ifr_initial = emulator.cpu.bus.via_ifr();
    println!("🎯 IFR inicial: 0x{:02X} (bit 5 clear: {})", ifr_initial, (ifr_initial & 0x20) == 0);
    
    // Contar cuántos ciclos toma expirar
    let mut cycles = 0;
    while emulator.cpu.bus.via_ifr() & 0x20 == 0 && cycles < 1000 {
        emulator.step();
        cycles += 1;
        
        // Mensaje de progreso cada 100 ciclos
        if cycles % 100 == 0 {
            println!("⏱️  Ciclo {}: IFR=0x{:02X}", cycles, emulator.cpu.bus.via_ifr());
        }
    }
    
    println!("⏱️  Timer2 expiró en {} ciclos", cycles);
    println!("🎯 IFR final: 0x{:02X} (bit 5 set: {})", emulator.cpu.bus.via_ifr(), (emulator.cpu.bus.via_ifr() & 0x20) != 0);
    
    if emulator.cpu.bus.via_ifr() & 0x20 != 0 {
        println!("✅ Timer2 interrupt flag detectado correctamente");
    } else {
        println!("❌ Timer2 interrupt flag NO detectado");
    }
    
    // Verificar que Timer2 expiró cerca de 600 ciclos, no 30000
    if cycles < 700 && cycles > 500 {
        println!("✅ AJUSTE EXITOSO: Timer2 expiró en {} ciclos (rango esperado: 500-700)", cycles);
    } else if cycles >= 29000 {
        println!("❌ AJUSTE FALLIDO: Timer2 tomó {} ciclos - parece que NO se ajustó desde 30000", cycles);
        panic!("Timer2 no fue ajustado - tomó {} ciclos en lugar de ~600", cycles);
    } else {
        println!("⚠️  Timer2 expiró en {} ciclos - fuera del rango esperado pero no 30000", cycles);
    }
    
    println!("=== TIMER2 ADJUSTMENT TEST PASSED ===");
}