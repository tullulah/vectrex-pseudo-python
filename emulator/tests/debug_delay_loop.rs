//! Test para investigar el comportamiento específico del bucle de delay F4EB-F4EF

use vectrex_emulator::cpu6809::CPU;
use std::fs;

#[test]
fn debug_delay_loop() {
    println!("🔍 Investigando comportamiento del bucle F4EB-F4EF");
    
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    let mut cpu = CPU::default();
    
    // Cargar BIOS y configurar
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    cpu.trace = true;
    
    // Configurar reset vector
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;
    
    println!("Reset vector: 0x{:04X}", reset_vector);
    
    // Ejecutar hasta llegar al bucle de delay
    let mut step_count = 0;
    let max_steps_to_loop = 500000;
    
    while step_count < max_steps_to_loop && !(cpu.pc >= 0xF4EB && cpu.pc <= 0xF4EF) {
        cpu.step();
        step_count += 1;
    }
    
    if cpu.pc >= 0xF4EB && cpu.pc <= 0xF4EF {
        println!("✅ Llegamos al bucle de delay en step {}: PC=0x{:04X}", step_count, cpu.pc);
        
        // Ahora analizar 100 iteraciones del bucle paso a paso
        println!("\n=== ANÁLISIS DETALLADO DEL BUCLE ===");
        let loop_start_pc = cpu.pc;
        let loop_start_b = cpu.b;
        let loop_start_a = cpu.a;
        
        println!("Estado inicial: PC=0x{:04X}, A=0x{:02X}, B=0x{:02X}", loop_start_pc, loop_start_a, loop_start_b);
        
        for iteration in 0..100 {
            let old_pc = cpu.pc;
            let old_a = cpu.a;
            let old_b = cpu.b;
            
            // Leer la instrucción actual
            let opcode = cpu.bus.read8(cpu.pc);
            let operand1 = cpu.bus.read8(cpu.pc.wrapping_add(1));
            let _operand2 = cpu.bus.read8(cpu.pc.wrapping_add(2));
            
            // Ejecutar un paso
            let success = cpu.step();
            if !success {
                println!("❌ CPU se detuvo en iteración {}", iteration);
                break;
            }
            
            // Mostrar qué pasó
            let instruction_desc = match opcode {
                0x86 => format!("LDA #0x{:02X}", operand1),
                0x12 => format!("NOP (0x12 no es opcode válido!)"),
                0x5A => format!("DECB"),
                0x26 => format!("BNE rel8(0x{:02X})", operand1),
                _ => format!("UNKNOWN 0x{:02X} 0x{:02X}", opcode, operand1),
            };
            
            println!("Iter {:2}: 0x{:04X}: {:02X} {:02X} | {} | A: {:02X}→{:02X}, B: {:02X}→{:02X}, PC: {:04X}→{:04X}", 
                     iteration, old_pc, opcode, operand1, instruction_desc, 
                     old_a, cpu.a, old_b, cpu.b, old_pc, cpu.pc);
            
            // Si salimos del rango del bucle, parar
            if !(cpu.pc >= 0xF4EB && cpu.pc <= 0xF4EF) {
                println!("✅ Salimos del bucle en iteración {}: PC=0x{:04X}", iteration, cpu.pc);
                break;
            }
            
            // Si B llega a 0, deberíamos salir
            if cpu.b == 0 {
                println!("🎯 Registro B llegó a 0 en iteración {}", iteration);
            }
        }
        
        println!("\nEstado final del análisis:");
        println!("PC: 0x{:04X}, A: 0x{:02X}, B: 0x{:02X}", cpu.pc, cpu.a, cpu.b);
        println!("Cambio en B: {} -> {} (delta: {})", loop_start_b, cpu.b, (cpu.b as i16) - (loop_start_b as i16));
        
    } else {
        println!("❌ No llegamos al bucle de delay después de {} pasos", max_steps_to_loop);
        println!("PC final: 0x{:04X}", cpu.pc);
    }
}