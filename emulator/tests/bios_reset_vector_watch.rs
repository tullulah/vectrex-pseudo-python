use vectrex_emulator::CPU;

// Diagnóstico: ejecutar la BIOS desde reset y cuando el PC alcance el área de vectores (FF80..FFFF)
// volcar un snapshot de memoria final (FF80..FFFF) y mostrar el contenido exacto del vector RESET (FFFC/FFFD)
// en ese momento, además de la secuencia previa de PCs para correlacionar cómo llegó allí.
// No usa BIOS sintética: se carga la BIOS real obligatoria.
#[test]
fn bios_reset_vector_watch() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin"; // ruta canónica
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS real en {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    // Registrar primeros PCs hasta que lleguemos a la página FFxx (o límite de seguridad)
    let mut pcs: Vec<(u16,u8)> = Vec::new();
    let mut steps: u32 = 0;
    let limit: u32 = 50_000; // amplio pero defensivo; BIOS debería alcanzar vectores por interrupciones o lectura tardía
    let mut reached_vector=false;
    while steps < limit {
        let pc = cpu.pc; let op = cpu.bus.mem[pc as usize];
        if pcs.len() < 256 { pcs.push((pc,op)); }
        if pc >= 0xFF80 { reached_vector=true; break; }
        if !cpu.step() { eprintln!("[HALT] opcode no implementado en {:04X}", pc); break; }
        steps += 1;
    }
    println!("[WATCH] steps={} reached_vector={} last_pc={:04X}", steps, reached_vector, cpu.pc);
    // Dump vectores si estamos ya en zona
    for addr in (0xFF80u16..=0xFFFF).step_by(16) { 
        print!("{:04X}: ", addr);
        for o in 0..16 { let a=addr+o; let b=cpu.bus.mem[a as usize]; print!("{:02X} ", b); }
        println!();
    }
    let vec_reset_lo = cpu.bus.mem[0xFFFC];
    let vec_reset_hi = cpu.bus.mem[0xFFFD];
    println!("[VECTORS] RESET raw={:02X}{:02X} -> {:04X}", vec_reset_hi, vec_reset_lo, ((vec_reset_hi as u16)<<8)|vec_reset_lo as u16);
    println!("[PC TRACE <=256]");
    for (i,(pc,op)) in pcs.iter().enumerate() { println!("{:03} {:04X}:{:02X}", i, pc, op); }
    // No assert todavía: es puramente informativo; se podrían añadir checks mínimos futuros.
}
