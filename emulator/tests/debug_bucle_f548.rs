//! Debug detallado del bucle en F548
use vectrex_emulator::CPU;

#[test]
fn debug_bucle_f548() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    // Ejecutar hasta llegar al bucle F548
    let mut steps = 0;
    while cpu.pc != 0xF548 && steps < 1000 {
        cpu.step();
        steps += 1;
    }
    
    if cpu.pc != 0xF548 {
        println!("[ERROR] No llegó al bucle F548 en {} pasos, PC={:04X}", steps, cpu.pc);
        return;
    }
    
    println!("[BUCLE] Llegó a F548 en {} pasos", steps);
    
    // Habilitar trace y ejecutar 10 iteraciones del bucle
    cpu.trace = true;
    for i in 0..10 {
        println!("\n=== ITERACION {} ===", i);
        let d_reg = ((cpu.a as u16) << 8) | (cpu.b as u16);
        println!("Estado antes: PC={:04X} A={:02X} B={:02X} D={:04X} Z={} N={}", 
                 cpu.pc, cpu.a, cpu.b, d_reg, cpu.cc_z, cpu.cc_n);
        
        // Ejecutar las 3 instrucciones del bucle
        for j in 0..3 {
            let pc = cpu.pc;
            let op = cpu.mem[pc as usize];
            println!("  Step {}: PC={:04X} OP={:02X}", j, pc, op);
            cpu.step();
            let d_reg = ((cpu.a as u16) << 8) | (cpu.b as u16);
            println!("    -> PC={:04X} A={:02X} B={:02X} D={:04X} Z={} N={}", 
                     cpu.pc, cpu.a, cpu.b, d_reg, cpu.cc_z, cpu.cc_n);
        }
        
        // Si no volvió a F548, salir
        if cpu.pc != 0xF548 {
            println!("[EXIT] Salió del bucle hacia PC={:04X}", cpu.pc);
            break;
        }
    }
}