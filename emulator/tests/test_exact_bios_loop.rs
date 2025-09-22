// Test para verificar el bucle exacto F4EB-F4EF con valor inicial real de BIOS
use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_exact_bios_f4eb_loop() {
    println!("🧪 Testing EXACT BIOS F4EB-F4EF loop");
    
    let mut cpu = CPU::default();
    
    // Setup exacto como en la BIOS
    cpu.pc = 0xF4EB;  // Empezar en el inicio del bucle
    
    // Escribir el bucle exacto que encontramos en la BIOS
    cpu.bus.mem[0xF4EB] = 0x86;  // LDA #$81
    cpu.bus.mem[0xF4EC] = 0x81;  // operand #$81
    cpu.bus.mem[0xF4ED] = 0x12;  // NOP
    cpu.bus.mem[0xF4EE] = 0x5A;  // DECB
    cpu.bus.mem[0xF4EF] = 0x26;  // BNE
    cpu.bus.mem[0xF4F0] = 0xFA;  // offset -6 (back to F4EB)
    cpu.bus.mem[0xF4F1] = 0x97;  // siguiente instrucción después del bucle
    
    // Valor inicial de B: probar con valor alto como en el trace real
    cpu.b = 0x39;  // 57 decimal - valor observado en trace
    println!("Estado inicial: B={:02X} ({}), Z={}", cpu.b, cpu.b, cpu.cc_z);
    
    let mut iterations = 0;
    let mut stuck_count = 0;
    
    while iterations < 1000 {  // Límite de seguridad
        let last_pc = cpu.pc;
        let last_b = cpu.b;
        let _last_z = cpu.cc_z;
        
        if !cpu.step() {
            println!("❌ CPU step failed at iteration {}", iterations);
            break;
        }
        
        iterations += 1;
        
        if iterations <= 5 || (iterations % 10 == 0) {
            println!("Iteración {}: PC {:04X} -> {:04X}, B {:02X} -> {:02X}, Z={}", 
                iterations, last_pc, cpu.pc, last_b, cpu.b, cpu.cc_z);
        }
        
        // Detectar si salimos del bucle
        if cpu.pc == 0xF4F0 { // PC después del BNE cuando no salta
            println!("✅ Bucle terminado correctamente después de {} iteraciones", iterations);
            println!("   Estado final: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
            return;
        }
        
        // Detectar atasco
        if cpu.pc == last_pc {
            stuck_count += 1;
            if stuck_count > 5 {
                println!("❌ CPU atascada en PC {:04X} después de {} iteraciones", cpu.pc, iterations);
                break;
            }
        } else {
            stuck_count = 0;
        }
        
        // Si B llega a 0 pero no sale del bucle, hay un problema
        if cpu.b == 0 && cpu.pc != 0xF4F0 {
            println!("❌ B llegó a 0 pero PC no salió del bucle (PC={:04X})", cpu.pc);
            println!("   Estado: B={:02X}, Z={}, esperábamos PC=F4F0", cpu.b, cpu.cc_z);
            break;
        }
    }
    
    if iterations >= 1000 {
        println!("❌ Bucle NO terminó después de 1000 iteraciones");
        println!("   Estado final: B={:02X}, Z={}, PC={:04X}", cpu.b, cpu.cc_z, cpu.pc);
    }
    
    // Este test DEBE fallar si el bug existe
    assert!(cpu.pc == 0xF4F0, "El bucle debe terminar en F4F0, pero terminó en {:04X}", cpu.pc);
}

#[test]  
fn test_decb_flag_behavior() {
    println!("🧪 Testing DECB flag behavior specifically");
    
    let mut cpu = CPU::default();
    
    // Test edge cases around 0
    let test_values = vec![0x03, 0x02, 0x01, 0x00];
    
    for initial_b in test_values {
        cpu.pc = 0x1000;
        cpu.b = initial_b;
        cpu.cc_z = false;  // Reset Z flag
        
        // Simple DECB instruction
        cpu.bus.mem[0x1000] = 0x5A;  // DECB
        cpu.bus.mem[0x1001] = 0x12;  // NOP (to stop)
        
        println!("\n🔹 Test con B inicial={:02X}", initial_b);
        println!("  Antes: B={:02X}, Z={}", cpu.b, cpu.cc_z);
        
        cpu.step();  // Execute DECB
        
        println!("  Después: B={:02X}, Z={}", cpu.b, cpu.cc_z);
        
        // Verificaciones importantes
        if initial_b == 0x01 {
            assert_eq!(cpu.b, 0x00, "B debe ser 0 después de DECB con B=01");
            assert!(cpu.cc_z, "Z flag debe estar SET cuando B se vuelve 0");
        } else if initial_b == 0x00 {
            assert_eq!(cpu.b, 0xFF, "B debe wraparound a FF cuando DECB con B=00");
            assert!(!cpu.cc_z, "Z flag debe estar CLEAR cuando B=FF");
        }
    }
}