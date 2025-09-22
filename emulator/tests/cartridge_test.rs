use vectrex_emulator::emulator::Emulator;
use vectrex_emulator::bus::Bus;

#[test]
fn test_with_minimal_cartridge() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
    
    // Crear un cartridge mínimo que contenga código en 0x00F0
    // Esto simula tener un programa real cargado
    let mut cartridge = vec![0xFF; 32768]; // 32KB cartridge
    
    // Código mínimo en 0x00F0: JMP 0xF000 para ir a la BIOS
    // 0x00F0: JMP extended (0x7E)
    // 0x00F1: High byte (0xF0)  
    // 0x00F2: Low byte (0x00)
    cartridge[0x00F0] = 0x7E; // JMP extended
    cartridge[0x00F1] = 0xF0; // High byte
    cartridge[0x00F2] = 0x00; // Low byte
    
    println!("=== RUST EMULATOR WITH MINIMAL CARTRIDGE ===");
    println!("Cartridge creado: {} bytes", cartridge.len());
    println!("Código en 0x00F0: JMP 0xF000");
    
    // Crear emulador con bus personalizado
    let mut emulator = Emulator::new();
    
    // Configurar bus con BIOS y cartridge
    let mut bus = Bus::new();
    bus.load_bios(&bios_data);
    bus.load_cartridge(&cartridge);
    *emulator.bus_mut() = bus;
    
    // Reset del emulador
    emulator.reset();
    
    // Verificar estado inicial
    let initial_pc = emulator.cpu().pc();
    println!("PC inicial: 0x{:04X}", initial_pc);
    
    // Verificar reset vector en BIOS
    let reset_vector_lo = emulator.bus().read(0xFFFE);
    let reset_vector_hi = emulator.bus().read(0xFFFF);
    let reset_vector = ((reset_vector_hi as u16) << 8) | (reset_vector_lo as u16);
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Verificar que el PC apunta al reset vector
    assert_eq!(initial_pc, reset_vector, "PC should match reset vector");
    
    // Verificar contenido en 0x00F0 (nuestro cartridge)
    let byte_f0 = emulator.bus().read(0x00F0);
    let byte_f1 = emulator.bus().read(0x00F1);
    let byte_f2 = emulator.bus().read(0x00F2);
    println!("Contenido en 0x00F0: 0x{:02X} 0x{:02X} 0x{:02X}", byte_f0, byte_f1, byte_f2);
    
    // Verificar contenido en 0xF000 (BIOS)
    let bios_f000 = emulator.bus().read(0xF000);
    println!("Contenido en 0xF000 (BIOS): 0x{:02X}", bios_f000);
    
    // Capturar secuencia de opcodes como antes
    println!("\n=== RUST OPCODE CAPTURE WITH CARTRIDGE ===");
    println!("┌──────┬──────┬────────┬────┬────┬──────┬──────┬──────┬──────┬────┬────┐");
    println!("│ Step │  PC  │ Opcode │ A  │ B  │  X   │  Y   │  S   │  U   │ DP │ CC │");
    println!("├──────┼──────┼────────┼────┼────┼──────┼──────┼──────┼──────┼────┼────┤");
    
    let mut successful_steps = 0;
    for step in 0..20 {
        let pc = emulator.cpu().pc();
        let opcode = emulator.bus().read(pc);
        
        let reg_a = emulator.cpu().a();
        let reg_b = emulator.cpu().b();
        let reg_x = emulator.cpu().x();
        let reg_y = emulator.cpu().y();
        let reg_s = emulator.cpu().s();
        let reg_u = emulator.cpu().u();
        let reg_dp = emulator.cpu().dp();
        let reg_cc = emulator.cpu().cc();
        
        println!(
            "│ {:4} │ {:04X} │   0x{:02X}   │ {:02X} │ {:02X} │ {:04X} │ {:04X} │ {:04X} │ {:04X} │ {:02X} │ {:02X} │",
            step, pc, opcode, reg_a, reg_b, reg_x, reg_y, reg_s, reg_u, reg_dp, reg_cc
        );
        
        // Ejecutar un paso
        emulator.step();
        successful_steps += 1;
        
        // Detectar bucles simples
        if step > 2 && emulator.cpu().pc() == pc {
            println!("│      │      │ ⚠ PC unchanged, possible infinite loop │");
            break;
        }
    }
    
    println!("└──────┴──────┴────────┴────┴────┴──────┴──────┴──────┴──────┴────┴────┘");
    println!("✓ Capturados {} pasos exitosos con cartridge", successful_steps);
}