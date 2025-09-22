use std::fs;
use vectrex_emulator::Emulator;

#[test]
fn test_external_minestorm_cartridge() {
    println!("🎮 TEST MINE STORM COMO CARTUCHO EXTERNO");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to read BIOS file");
    println!("📁 BIOS cargada: {} bytes", bios_data.len());
    
    // Crear emulador y cargar BIOS
    let mut emulator = Emulator::new();
    let result = emulator.load_bios(&bios_data);
    assert!(result, "BIOS debe cargar correctamente");
    
    println!("🔧 PC después de cargar BIOS: 0x{:04X}", emulator.cpu.pc);
    
    // Intentar cargar Mine Storm como cartucho externo
    // Mine Storm debería estar disponible como archivo .vec o similar
    let potential_minestorm_paths = vec![
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\vectrexy\data\roms\minestorm.vec",
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\examples\minestorm.bin",
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\build\minestorm.bin",
    ];
    
    let mut minestorm_loaded = false;
    
    for path in potential_minestorm_paths {
        if let Ok(cart_data) = fs::read(path) {
            println!("📦 Cargando Mine Storm desde: {}", path);
            println!("📦 Tamaño cartucho: {} bytes", cart_data.len());
            
            emulator.load_cartridge(&cart_data);
            minestorm_loaded = true;
            break;
        } else {
            println!("⚠️  No encontrado: {}", path);
        }
    }
    
    if !minestorm_loaded {
        println!("❌ No se pudo cargar Mine Storm como cartucho externo");
        println!("🔍 La BIOS contiene Mine Storm integrado, pero no está saltando a él");
        println!("🎯 ANÁLISIS: Los vectores diagonales vienen de rutinas de inicialización BIOS");
        return;
    }
    
    // Si cargamos cartucho externo, reset para ir al cartucho
    emulator.cpu.reset();
    println!("🔄 Reset para ir al cartucho - PC: 0x{:04X}", emulator.cpu.pc);
    
    // Ejecutar para ver si ahora va al cartucho
    let max_steps = 10_000;
    let mut found_cartridge = false;
    
    for step in 0..max_steps {
        let pc_before = emulator.cpu.pc;
        emulator.step();
        let pc_after = emulator.cpu.pc;
        
        // Detectar si salió de BIOS (PC < 0xE000)
        if pc_before >= 0xE000 && pc_after < 0xE000 {
            found_cartridge = true;
            println!("🎯 SALIÓ DE BIOS AL CARTUCHO!");
            println!("   Step: {}", step + 1);
            println!("   PC antes: 0x{:04X}", pc_before);
            println!("   PC después: 0x{:04X}", pc_after);
            break;
        }
        
        if step % 1000 == 0 {
            println!("📊 Step {}: PC=0x{:04X}", step, emulator.cpu.pc);
        }
    }
    
    if found_cartridge {
        println!("✅ Con cartucho externo SÍ salta al cartucho");
    } else {
        println!("❌ Incluso con cartucho externo no salta");
        println!("   PC final: 0x{:04X}", emulator.cpu.pc);
    }
}