#[cfg(test)]
mod tests {
    use std::path::Path;
    use vectrex_emulator::emulator::Emulator;

    #[test]
    fn test_via_register_0a_behavior() {
        println!("=== TEST VIA REGISTRO 0x0A (SR) ===");
        
        let bios_path = Path::new(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin");
        let mut emulator = Emulator::new();
        emulator.load_bios(&std::fs::read(bios_path).expect("Error leyendo BIOS"));
        emulator.reset();

        println!("🔧 Test directo del registro VIA 0x0A:");
        
        // Test 1: Escritura y lectura básica
        let test_value = 0xE6; // Byte bajo de X (0xCBE6)
        emulator.cpu.bus.write8(0xD00A, test_value);
        let read_back = emulator.cpu.bus.read8(0xD00A);
        println!("   Escribir 0x{:02X} → Leer 0x{:02X} {}", test_value, read_back, 
                if read_back == test_value { "✅" } else { "❌" });
        
        // Test 2: Escritura en 0xD05A (dirección del problema)
        let x_low = 0xE6; // Byte bajo de 0xCBE6
        emulator.cpu.bus.write8(0xD05A, x_low);
        let read_d05a = emulator.cpu.bus.read8(0xD05A);
        println!("   Escribir 0x{:02X} en 0xD05A → Leer 0x{:02X} {}", x_low, read_d05a,
                if read_d05a == x_low { "✅" } else { "❌" });
        
        // Test 3: Verificar que ambas direcciones mapean al mismo registro
        emulator.cpu.bus.write8(0xD00A, 0x12);
        let read_from_mirror = emulator.cpu.bus.read8(0xD05A);
        println!("   Escribir 0x12 en 0xD00A → Leer desde 0xD05A: 0x{:02X} {}", read_from_mirror,
                if read_from_mirror == 0x12 { "✅ (mirror correcto)" } else { "❌ (mirror falla)" });
        
        // Test 4: Estado inicial del registro
        emulator.reset();
        let initial_value = emulator.cpu.bus.read8(0xD05A);
        println!("   Valor inicial en 0xD05A después de reset: 0x{:02X}", initial_value);
        
        // Test 5: Simular la secuencia exacta del bucle F4EB
        println!("\n🎯 Simulación secuencia F4EB:");
        println!("   DP=0xD0, entonces <$5A = DP<<8 + 0x5A = 0xD05A");
        
        // STX <$5A con X=0xCBE6
        let x_value = 0xCBE6u16;
        let x_low_byte = (x_value & 0xFF) as u8;
        let x_high_byte = (x_value >> 8) as u8;
        
        // STX escribe primero el byte alto, luego el bajo (orden 6809)
        emulator.cpu.bus.write8(0xD05A, x_low_byte);  // En realidad STX debería escribir ambos bytes
        emulator.cpu.bus.write8(0xD05B, x_high_byte); // pero el bucle solo lee desde 0xD05A
        
        let stored_low = emulator.cpu.bus.read8(0xD05A);
        println!("   STX 0xCBE6 → byte bajo en 0xD05A: 0x{:02X}", stored_low);
        println!("   Esperado para salir del bucle: 0x81");
        println!("   ¿Bucle termina? {}", if stored_low == 0x81 { "SÍ ✅" } else { "NO ❌" });
        
        // Test 6: ¿Qué pasa si escribimos directamente 0x81?
        emulator.cpu.bus.write8(0xD05A, 0x81);
        let verification = emulator.cpu.bus.read8(0xD05A);
        println!("   Escribir directamente 0x81 → Leer: 0x{:02X} {}", verification,
                if verification == 0x81 { "✅" } else { "❌" });
        
        println!("\n📊 RESULTADO:");
        if read_d05a != 0xFF {
            println!("   ✅ El registro 0x0A funciona correctamente");
            println!("   🔍 El problema F4EB NO es mapeo de memoria");
            println!("   💡 Investigar: ¿Por qué BIOS espera 0x81 pero obtiene 0x{:02X}?", read_d05a);
        } else {
            println!("   ❌ El registro 0x0A devuelve 0xFF (no mapeado)");
            println!("   🔧 Problema confirmado en mapeo de memoria VIA");
        }
    }
}