use vectrex_emulator::Emulator;
use std::path::Path;

#[test]
fn test_bios_cartridge_detection_logic() {
    let mut emulator = Emulator::new();
    
    // Cargar BIOS real
    let bios_path = Path::new(r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin");
    if !bios_path.exists() {
        panic!("BIOS no encontrada en: {:?}", bios_path);
    }
    
    let bios_data = std::fs::read(bios_path).unwrap();
    let success = emulator.load_bios(&bios_data);
    if !success {
        panic!("No se pudo cargar la BIOS");
    }
    
    let debug_state = emulator.debug_state();
    println!("BIOS cargada, PC inicial: 0x{:04X}", debug_state.cpu_pc);
    
    // Ejecutar menos pasos pero con más detalle de las áreas críticas
    for step in 0..20000 {
        let debug_before = emulator.debug_state();
        let pc_before = debug_before.cpu_pc;
        
        // Detalle especial para el rango donde se queda atascado
        if pc_before >= 0xF4E0 && pc_before <= 0xF500 {
            // Leer contenido de memoria en PC para ver qué instrucciones son
            let opcode = emulator.cpu.bus.read8(pc_before);
            let operand1 = emulator.cpu.bus.read8(pc_before.wrapping_add(1));
            let operand2 = emulator.cpu.bus.read8(pc_before.wrapping_add(2));
            
            println!("Step {}: PC=0x{:04X} → Opcode=0x{:02X} 0x{:02X} 0x{:02X}", 
                    step, pc_before, opcode, operand1, operand2);
        }
        
        emulator.step();
        let debug_after = emulator.debug_state();
        let pc_after = debug_after.cpu_pc;
        
        // Si el PC hace un salto significativo, logearlo
        let pc_diff = if pc_after > pc_before { 
            pc_after - pc_before 
        } else { 
            pc_before - pc_after 
        };
        
        if pc_diff > 100 {
            println!("Step {}: JUMP PC 0x{:04X} → 0x{:04X} (diff: {})", 
                    step, pc_before, pc_after, pc_diff);
        }
        
        // Revisar direcciones de memoria críticas para detección de cartuchos
        // En Vectrex, típicamente se revisa 0x0000 para ver si hay cartucho
        if step % 5000 == 0 {
            let cart_check = emulator.cpu.bus.read8(0x0000);
            println!("Step {}: Cartucho en 0x0000 = 0x{:02X}, PC = 0x{:04X}", 
                    step, cart_check, pc_after);
        }
        
        // Si llegamos a una dirección fuera de BIOS, ¡éxito!
        if pc_after < 0xE000 {
            println!("¡ÉXITO! PC salió de BIOS en step {}: 0x{:04X}", step, pc_after);
            break;
        }
        
        // Si nos quedamos en un bucle muy pequeño, analizar
        if step > 1000 && pc_after == pc_before {
            println!("Posible bucle infinito en step {}, PC: 0x{:04X}", step, pc_after);
            break;
        }
    }
    
    let final_debug = emulator.debug_state();
    let final_pc = final_debug.cpu_pc;
    println!("PC final: 0x{:04X}", final_pc);
    
    // Verificar el estado de memoria donde debería estar Mine Storm
    println!("=== ANÁLISIS DE MEMORIA ===");
    println!("Contenido en 0x0000: 0x{:02X}", emulator.cpu.bus.read8(0x0000));
    println!("Contenido en 0x8000: 0x{:02X}", emulator.cpu.bus.read8(0x8000));
    
    // Buscar patrones de instrucciones que podrían indicar detección de cartucho
    for addr in [0x0000, 0x8000, 0xC000] {
        let val = emulator.cpu.bus.read8(addr);
        println!("Dirección cartucho 0x{:04X}: 0x{:02X}", addr, val);
    }
}