use vectrex_emulator::Emulator;

fn main() {
    println!("🔍 TEST DEBUG: Investigando last_via_write");
    
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    
    match std::fs::read(bios_path) {
        Ok(bios_data) => {
            emulator.cpu.bus.load_block(0xE000, &bios_data, false); // BIOS en ROM
            println!("✅ BIOS cargada desde: {}", bios_path);
        },
        Err(e) => {
            println!("❌ Error cargando BIOS: {}", e);
            return;
        }
    }
    
    println!("\n=== TEST DEBUG: Trazando last_via_write ===");
    
    // Crear una instancia sin BIOS para comparar
    let mut emulator_no_bios = Emulator::new();
    
    println!("\n--- Test con BIOS ---");
    println!("Paso 1: Escribir Port B (0xD000)");
    emulator.cpu.test_write8(0xD000, 0x00);
    println!("Estado last_via_write después de escribir Port B: {:?}", emulator.cpu.bus.last_via_write);
    
    println!("Paso 2: Ejecutar step()");
    emulator.step();
    println!("Estado last_via_write después de step(): {:?}", emulator.cpu.bus.last_via_write);
    
    println!("\nPaso 3: Escribir Port A (0xD001)");
    emulator.cpu.test_write8(0xD001, 100);
    println!("Estado last_via_write después de escribir Port A: {:?}", emulator.cpu.bus.last_via_write);
    
    println!("Paso 4: Ejecutar step()");
    emulator.step();
    println!("Estado last_via_write después de step(): {:?}", emulator.cpu.bus.last_via_write);
    
    println!("\n--- Test SIN BIOS ---");
    println!("Paso 1: Escribir Port B (0xD000)");
    emulator_no_bios.cpu.test_write8(0xD000, 0x00);
    println!("Estado last_via_write después de escribir Port B: {:?}", emulator_no_bios.cpu.bus.last_via_write);
    
    println!("Paso 2: Ejecutar step()");
    emulator_no_bios.step();
    println!("Estado last_via_write después de step(): {:?}", emulator_no_bios.cpu.bus.last_via_write);
    
    println!("\nPaso 3: Escribir Port A (0xD001)");
    emulator_no_bios.cpu.test_write8(0xD001, 100);
    println!("Estado last_via_write después de escribir Port A: {:?}", emulator_no_bios.cpu.bus.last_via_write);
    
    println!("Paso 4: Ejecutar step()");
    emulator_no_bios.step();
    println!("Estado last_via_write después de step(): {:?}", emulator_no_bios.cpu.bus.last_via_write);
    
    println!("\n=== RESUMEN ===");
    println!("Este test nos dirá si:");
    println!("1. last_via_write se establece correctamente");
    println!("2. step() consume last_via_write como esperado");
    println!("3. Hay diferencia entre tener BIOS cargada vs no tenerla");
}