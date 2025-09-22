//! Test diagnóstico para entender por qué no se habilitan las interrupciones de Timer2
//! en el flujo normal de la BIOS.

use vectrex_emulator::CPU;
use std::fs;

fn load_bios() -> Option<Vec<u8>> {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
}

#[test]
fn debug_timer_irq_initialization() {
    let bios = match load_bios() { 
        Some(b) => b, 
        None => { 
            eprintln!("[SKIP] BIOS real no encontrada"); 
            return; 
        } 
    };
    
    let mut cpu = CPU::with_pc(0xF000);
    cpu.load_bios(&bios);
    
    // Leer vector RESET
    let reset_vec = {
        let hi = cpu.bus.mem[0xFFFE];
        let lo = cpu.bus.mem[0xFFFF];
        ((hi as u16) << 8) | lo as u16
    };
    if reset_vec != 0 { cpu.pc = reset_vec; }
    
    println!("Vector de reset: {:04X}", cpu.pc);
    
    // Ejecutar y registrar todos los cambios en IER durante los primeros pasos
    let mut last_ier = cpu.bus.via_ier();
    let mut ier_changes = Vec::new();
    
    for step in 0..50000 {
        let old_pc = cpu.pc;
        let old_ier = cpu.bus.via_ier();
        
        cpu.step();
        
        let new_ier = cpu.bus.via_ier();
        if new_ier != old_ier {
            ier_changes.push((step, old_pc, old_ier, new_ier));
            println!("Step {}: PC {:04X} -> IER cambió de {:02X} a {:02X}", step, old_pc, old_ier, new_ier);
        }
        
        // Si vemos que se habilitó Timer2 (bit 5), parar
        if (new_ier & 0x20) != 0 {
            println!("¡Timer2 interrupt habilitado en step {}!", step);
            break;
        }
        
        // Detectar el loop problemático y mostrar estado del registro B
        if cpu.pc == 0xF4EB || cpu.pc == 0xF4EF {
            if step % 1000 == 0 {  // Solo cada 1000 iteraciones para no spam
                println!("DELAY LOOP: Step {}: PC {:04X}, B = {:02X}", step, cpu.pc, cpu.b);
            }
        }
        
        // Mostrar primeros pasos y cuando hay saltos significativos
        if step < 20 || (cpu.pc >= 0xF000 && cpu.pc != old_pc + 1 && cpu.pc != old_pc + 2) {
            println!("Step {}: PC {:04X} -> {:04X}", step, old_pc, cpu.pc);
        }
    }
    
    println!("\nEstado final:");
    println!("IER: {:02X}", cpu.bus.via_ier());
    println!("IFR: {:02X}", cpu.bus.via_ifr());
    let t2_low = cpu.bus.via.read(0x08);
    let t2_high = cpu.bus.via.read(0x09);
    let t2_value = (t2_high as u16) << 8 | t2_low as u16;
    println!("Timer2 countdown: {:04X}", t2_value);
    println!("Cambios en IER: {}", ier_changes.len());
    
    if ier_changes.is_empty() {
        println!("PROBLEMA: ¡Nunca se habilitaron interrupciones!");
    }
}