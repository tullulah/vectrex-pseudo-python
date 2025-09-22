//! Debug cuando X llega al rango VIA
use vectrex_emulator::CPU;

#[test]
fn debug_bucle_hasta_via() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    // Llegar al bucle F548
    while cpu.pc != 0xF548 { cpu.step(); }
    
    println!("[DEBUG] Iniciando bucle");
    println!("X_inicial={:04X} D_inicial={:04X}", cpu.x, ((cpu.a as u16) << 8) | (cpu.b as u16));
    
    let mut iteraciones = 0;
    let x_inicial = cpu.x;
    let d_inicial = ((cpu.a as u16) << 8) | (cpu.b as u16);
    
    // Ejecutar hasta llegar cerca de VIA (0xD000) o hasta que D llegue a 0
    while cpu.pc == 0xF548 && iteraciones < 1000 {
        let x_antes = cpu.x;
        let d_antes = ((cpu.a as u16) << 8) | (cpu.b as u16);
        
        // Ejecutar una iteración completa (3 instrucciones)
        cpu.step(); // CLR
        cpu.step(); // SUBD
        cpu.step(); // BPL
        
        let x_despues = cpu.x;
        let d_despues = ((cpu.a as u16) << 8) | (cpu.b as u16);
        
        iteraciones += 1;
        
        // Mostrar progreso cada 10 iteraciones o cuando llegue cerca de VIA
        if iteraciones % 10 == 0 || x_despues >= 0xCFF0 {
            println!("Iter {:3}: X={:04X}->{:04X} D={:04X}->{:04X}", 
                     iteraciones, x_antes, x_despues, d_antes, d_despues);
        }
        
        // Alerta especial cuando llegue al rango VIA
        if x_despues >= 0xD000 && x_antes < 0xD000 {
            println!("*** ALERTA: X llegó al rango VIA en iteración {} ***", iteraciones);
            cpu.trace = true; // Habilitar trace detallado
        }
        
        // Terminar si D llega a 0
        if d_despues == 0 {
            println!("*** D llegó a 0 en iteración {} ***", iteraciones);
            break;
        }
        
        // Terminar si sale del bucle
        if cpu.pc != 0xF548 {
            println!("*** Salió del bucle hacia {:04X} en iteración {} ***", cpu.pc, iteraciones);
            break;
        }
    }
    
    let x_final = cpu.x;
    let d_final = ((cpu.a as u16) << 8) | (cpu.b as u16);
    println!("[RESULTADO] {} iteraciones: X={:04X}->{:04X} (+{}) D={:04X}->{:04X} (-{})", 
             iteraciones, x_inicial, x_final, x_final.wrapping_sub(x_inicial), 
             d_inicial, d_final, d_inicial.wrapping_sub(d_final));
}