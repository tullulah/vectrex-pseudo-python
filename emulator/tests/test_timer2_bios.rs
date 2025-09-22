use vectrex_emulator::*;
use std::env;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let data = std::fs::read(path).expect("BIOS real requerida para test");
    assert_eq!(data.len(), 8192, "BIOS size inesperado");
    for (i, b) in data.iter().enumerate() { 
        let addr = 0xE000 + i as u16; 
        cpu.mem[addr as usize] = *b; 
        cpu.bus.mem[addr as usize] = *b; 
    }
    cpu.bios_present = true;
}

#[test]
fn test_timer2_bios_progression() {
    // Configurar trazas
    env::set_var("VIA_T2_TRACE", "1");
    env::set_var("IRQ_TRACE", "1");
    env::set_var("DIRECT_TRACE", "1");

    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Configurar interrupción del timer
    cpu.bus.via.write(0x0E, 0x00); // IER = 0 (sin interrupciones habilitadas)
    
    println!("=== Testing Timer2 BIOS progression ===");
    println!("Initial state: PC={:04X}, IFR={:02X}", cpu.pc, cpu.bus.via.read(0x0D));
    
    // Ejecutar hasta llegar al loop F19E (Wait_Recal)
    let mut cycles = 0;
    let max_cycles = 50000;
    
    while cycles < max_cycles {
        let pc = cpu.pc;
        let ifr = cpu.bus.via.read(0x0D);
        
        // Detectar cuando llegamos al bucle Wait_Recal
        if pc == 0xF19E {
            let t2_low = cpu.bus.via.read(0x08);
            let t2_high = cpu.bus.via.read(0x09);
            let timer2 = (t2_high as u16) << 8 | t2_low as u16;
            
            println!("=== REACHED Wait_Recal loop ===");
            println!("PC: {:04X}, Timer2: {:04X}, IFR: {:02X}", pc, timer2, ifr);
            
            // Verificar que el timer tiene un valor razonable
            if timer2 > 0x1000 && timer2 < 0x8000 {
                println!("✅ Timer2 value looks correct: {:04X}", timer2);
            } else {
                println!("❌ Timer2 value suspicious: {:04X}", timer2);
            }
            
            // Ejecutar muchas más instrucciones para dar tiempo a Timer2 de expirar
            for i in 0..10000 {
                let pre_pc = cpu.pc;
                let pre_ifr = cpu.bus.via.read(0x0D);
                let pre_t2 = cpu.bus.via.read(0x08);
                
                cpu.step();
                
                let post_pc = cpu.pc;
                let post_ifr = cpu.bus.via.read(0x0D);
                let post_t2 = cpu.bus.via.read(0x08);
                
                // Imprimir cada 1000 pasos para debug
                if i % 1000 == 0 {
                    println!("Step {}: {:04X}→{:04X}, T2L: {:02X}→{:02X}, IFR: {:02X}→{:02X}", 
                             i, pre_pc, post_pc, pre_t2, post_t2, pre_ifr, post_ifr);
                }
                
                // Si el PC cambia significativamente, hemos salido del loop
                if post_pc != 0xF19E && post_pc != 0xF1A0 {
                    println!("✅ BIOS progressed past Wait_Recal to PC={:04X} after {} steps", post_pc, i);
                    break;
                }
                
                // Si hemos dado demasiadas vueltas, es un loop infinito
                if i > 8000 {
                    println!("❌ Stuck in Wait_Recal loop after {} steps - Timer2 interrupt not working", i);
                    panic!("BIOS stuck in Wait_Recal loop");
                }
            }
            break;
        }
        
        cpu.step();
        cycles += 1;
        
        // Imprimir cada 5000 ciclos para debug
        if cycles % 5000 == 0 {
            println!("Cycles: {}, PC: {:04X}, IFR: {:02X}", cycles, cpu.pc, cpu.bus.via.read(0x0D));
        }
    }
    
    if cycles >= max_cycles {
        panic!("BIOS didn't reach Wait_Recal loop in {} cycles", max_cycles);
    }
    
    println!("=== Test completed successfully ===");
}