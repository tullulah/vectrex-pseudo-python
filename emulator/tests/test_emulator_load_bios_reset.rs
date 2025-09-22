use std::fs;
use vectrex_emulator::Emulator;

#[test]
fn test_emulator_load_bios_with_reset() {
    println!("🔍 TEST EMULATOR LOAD_BIOS CON RESET AUTOMÁTICO");
    
    // Leer BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to read BIOS file");
    println!("📁 BIOS leída: {} bytes", bios_data.len());
    
    // Crear emulador nuevo
    let mut emulator = Emulator::new();
    
    // 1. PC inicial sin BIOS (debe ser 0x0000)
    println!("🔧 PC inicial sin BIOS: 0x{:04X}", emulator.cpu.pc);
    assert_eq!(emulator.cpu.pc, 0x0000, "PC inicial debe ser 0x0000 sin BIOS");
    assert!(!emulator.cpu.bios_present, "BIOS no debe estar presente inicialmente");
    
    // 2. Cargar BIOS (esto debería hacer reset automáticamente con nuestro fix)
    let result = emulator.load_bios(&bios_data);
    assert!(result, "load_bios debe retornar true");
    
    // 3. PC después de load_bios debe estar en rango BIOS debido al reset automático
    let pc_after_bios = emulator.cpu.pc;
    println!("✅ PC después de load_bios: 0x{:04X}", pc_after_bios);
    
    // Verificar que PC está en rango BIOS (0xE000-0xFFFF para BIOS 8K)
    assert!(pc_after_bios >= 0xE000, "PC debe estar en rango BIOS después de load_bios");
    
    // 4. Verificar que BIOS está cargada
    assert!(emulator.cpu.bios_present, "BIOS debe estar presente");
    
    // 5. Verificar que el vector de reset es válido
    let vector_high = emulator.cpu.bus.read8(0xFFFC);
    let vector_low = emulator.cpu.bus.read8(0xFFFD);
    let reset_vector = ((vector_high as u16) << 8) | (vector_low as u16);
    println!("📋 Vector de reset: 0x{:04X} (bytes: 0x{:02X} 0x{:02X})", 
             reset_vector, vector_high, vector_low);
    
    // En BIOS real, el vector debería apuntar a código válido
    if reset_vector >= 0xE000 {
        println!("✅ Vector de reset apunta a BIOS: 0x{:04X}", reset_vector);
        assert_eq!(pc_after_bios, reset_vector, "PC debe igual al vector de reset");
    } else {
        println!("⚠️  Vector de reset usa fallback: PC=0x{:04X} (vector=0x{:04X})", 
                 pc_after_bios, reset_vector);
        assert_eq!(pc_after_bios, 0xF000, "PC debe usar fallback 0xF000");
    }
    
    println!("🎯 CONCLUSIÓN: Emulator::load_bios() hace reset automático y configura PC correctamente");
    
    // 6. Test adicional: verificar que ahora cargar cartucho NO afecta el PC
    println!("\n🧪 CARGANDO CARTUCHO PARA VERIFICAR PC NO CAMBIA...");
    let old_pc = emulator.cpu.pc;
    
    // Crear datos dummy de cartucho
    let cart_data = vec![0x12, 0x34, 0x56, 0x78]; // Datos dummy
    emulator.load_cartridge(&cart_data);
    
    let pc_after_cart = emulator.cpu.pc;
    println!("📦 PC después de cargar cartucho: 0x{:04X}", pc_after_cart);
    assert_eq!(pc_after_cart, old_pc, "Cargar cartucho NO debe cambiar PC");
    
    println!("✅ CORRECTO: PC permanece en BIOS después de cargar cartucho");
}