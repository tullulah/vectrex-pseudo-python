//! Debug de qué pasa DESPUÉS del bucle de limpieza
use vectrex_emulator::CPU;

#[test]
fn debug_despues_bucle() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    // Llegar al bucle F548 y ejecutarlo completamente
    while cpu.pc != 0xF548 { cpu.step(); }
    
    // Ejecutar el bucle completo
    while cpu.pc == 0xF548 { 
        cpu.step(); cpu.step(); cpu.step(); 
    }
    
    println!("[DESPUES_BUCLE] PC={:04X}", cpu.pc);
    
    // Habilitar trace y ejecutar las siguientes 20 instrucciones
    cpu.trace = true;
    for i in 0..20 {
        let pc = cpu.pc;
        let op = cpu.mem[pc as usize];
        println!("Step {:2}: PC={:04X} OP={:02X}", i, pc, op);
        
        if !cpu.step() {
            println!("[HALT] Opcode no implementado en {:04X}", pc);
            break;
        }
        
        // Si llega a hacer un bucle infinito, terminar
        if i > 5 && cpu.pc == pc {
            println!("[LOOP] Bucle detectado en {:04X}", pc);
            break;
        }
    }
}