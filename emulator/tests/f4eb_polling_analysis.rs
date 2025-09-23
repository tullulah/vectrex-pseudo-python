// Análisis específico del polling en F4EB
use vectrex_emulator::emulator::Emulator;

#[test]
fn test_f4eb_polling_analysis() {
    println!("=== ANÁLISIS DE POLLING F4EB ===");
    
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to read BIOS file");
    let mut emulator = Emulator::new();
    emulator.load_bios(&bios_data);
    
    // Ejecutar hasta llegar a F4EB  
    let mut step_count = 0;
    while emulator.cpu.pc != 0xF4EB && step_count < 10000 {
        emulator.step();
        step_count += 1;
    }
    
    println!("🎯 Estado al llegar a F4EB:");
    println!("   PC: 0x{:04X}", emulator.cpu.pc);
    println!("   DP: 0x{:02X} (apunta a 0x{:04X}00)", emulator.cpu.dp, emulator.cpu.dp);
    println!("   A: 0x{:02X}, B: 0x{:02X}", emulator.cpu.a, emulator.cpu.b);
    println!("   IFR: 0x{:02X}, IER: 0x{:02X}", emulator.cpu.bus.via_ifr(), emulator.cpu.bus.via_ier());
    
    // Analizar la instrucción en F4ED
    let opcode_f4ed = emulator.cpu.bus.read8(0xF4ED);
    let operand_f4ee = emulator.cpu.bus.read8(0xF4EE);
    println!("🔍 Instrucción en F4ED: opcode=0x{:02X} operand=0x{:02X}", opcode_f4ed, operand_f4ee);
    
    // Si es direct page (0x12 = STX direct), calculamos la dirección real
    if opcode_f4ed == 0x12 {
        let dp_address = (emulator.cpu.dp as u16) << 8 | (operand_f4ee as u16);
        println!("   → STX <$5A significa dirección real: 0x{:04X}", dp_address);
    }
    
    // Examinar qué hay en la dirección 0x5A con DP actual
    let dp_base = (emulator.cpu.dp as u16) << 8;
    let test_address = dp_base | 0x5A;
    let value_at_5a = emulator.cpu.bus.read8(test_address);
    println!("   Valor en dirección 0x{:04X}: 0x{:02X}", test_address, value_at_5a);
    
    // Si está apuntando al VIA, identifiquemos qué registro
    if dp_base == 0xD000 {
        let via_reg = 0x5A & 0x0F;  // Los registros VIA están en D000-D00F
        match via_reg {
            0x0D => println!("   → Registro VIA 0x0D: IFR (Interrupt Flag Register)"),
            0x0E => println!("   → Registro VIA 0x0E: IER (Interrupt Enable Register)"),
            0x0A => println!("   → Registro VIA 0x0A: T2C-L (Timer 2 Counter Low)"),
            _ => println!("   → Registro VIA 0x{:02X}", via_reg),
        }
    }
    
    // Ejecutar el bucle y monitorear lecturas
    println!("🔄 Monitoreando las primeras 10 iteraciones del bucle:");
    
    for i in 0..10 {
        // Guardar estado antes
        let pc_before = emulator.cpu.pc;
        let a_before = emulator.cpu.a;
        
        // Ejecutar instrucción
        emulator.step();
        
        let pc_after = emulator.cpu.pc;
        let a_after = emulator.cpu.a;
        
        // Si fue una lectura (cambió de F4ED a F4EF), mostrar detalles
        if pc_before == 0xF4ED && pc_after == 0xF4EF {
            let current_value = emulator.cpu.bus.read8(test_address);
            println!("   Iteración {}: Leyó 0x{:02X} de dirección 0x{:04X}", i, current_value, test_address);
            
            // Verificar si IFR cambió después de la lectura
            let ifr_after_read = emulator.cpu.bus.via_ifr();
            println!("     IFR después de lectura: 0x{:02X}", ifr_after_read);
        } else if pc_before == 0xF4EB {
            println!("   Iteración {}: LDA #$81 → A=0x{:02X}", i, a_after);
        } else if pc_before == 0xF4EF {
            println!("   Iteración {}: BNE ejecutado", i);
        }
        
        if pc_after == 0xF4EB && i > 3 {
            println!("   ⚠️  Bucle confirmado - continúa en F4EB");
            break;
        }
    }
    
    // Intentar romper el bucle simulando una lectura de IFR
    println!("🧪 EXPERIMENTO: Simular lectura de IFR para limpiar flags...");
    
    if test_address == 0xD00D {  // Si está leyendo IFR
        println!("   El bucle está leyendo IFR - esto debería limpiar los flags");
        let ifr_before = emulator.cpu.bus.via_ifr();
        let _dummy_read = emulator.cpu.bus.read8(0xD00D);  // Leer IFR para limpiarlo
        let ifr_after = emulator.cpu.bus.via_ifr();
        println!("   IFR antes: 0x{:02X}, después: 0x{:02X}", ifr_before, ifr_after);
        
        // Probar el bucle después de limpiar IFR
        println!("   Probando bucle después de limpiar IFR...");
        for i in 0..5 {
            let pc_before = emulator.cpu.pc;
            emulator.step();
            let pc_after = emulator.cpu.pc;
            println!("     {}: PC 0x{:04X}→0x{:04X}", i, pc_before, pc_after);
            
            if pc_after != 0xF4EB && pc_after != 0xF4ED && pc_after != 0xF4EF {
                println!("   ✅ ¡Bucle roto! Continúa en 0x{:04X}", pc_after);
                break;
            }
        }
    }
}