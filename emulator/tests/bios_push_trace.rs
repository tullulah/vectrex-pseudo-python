use vectrex_emulator::CPU;

// Rastrea todos los PSHS (0x34) y PSHU (0x36) ejecutados antes del primer PULS (0x35)
// y muestra los bytes escritos en la pila para localizar dónde se originan 0x80 0x73
// que luego forman el PC de retorno (7380) observado en bios_puls_probe.
#[test]
fn bios_push_trace() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] falta BIOS real {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    let mut steps: u32 = 0;
    let limit: u32 = 120_000; // margen amplio
    let mut found_puls=false;
    while steps < limit {
        let pc = cpu.pc; let op = cpu.mem[pc as usize];
        if op == 0x35 { // PULS - detener
            println!("[STOP] PULS encontrado en {:04X}", pc);
            found_puls=true; break;
        }
        if op == 0x34 || op == 0x36 { // PSHS / PSHU
            // Capturar estado antes
            let s_before = cpu.s;
            let u_before = cpu.u;
            // Leer máscara (siguiente byte)
            let mask = cpu.mem[pc.wrapping_add(1) as usize];
            // Ejecutar instrucción
            let _ok = cpu.step();
            // Determinar pila usada: 0x34 usa S, 0x36 usa U como SP temporal
            let (label, sp_after, base_used) = if op==0x34 { ("PSHS", cpu.s, s_before) } else { ("PSHU", cpu.u, u_before) };
            // Calcular bytes empujados según máscara: orden 6809 (cond cc, A, B, DP, X, Y, U, PC) bits 0..7
            let mut pushed: Vec<(u16,u8)> = Vec::new();
            // Reconstituir rango: el stack decrece; sp_after es SP final; base_used era SP inicial
            // Bytes escritos = base_used - sp_after
            let count = base_used.wrapping_sub(sp_after) as usize; // safe
            for i in 0..count { let addr = sp_after.wrapping_add(i as u16); let b = cpu.mem[addr as usize]; pushed.push((addr,b)); }
            print!("[PUSH] {:4} pc={:04X} mask={:02X} bytes={} ", label, pc, mask, pushed.len());
            for (_a,b) in &pushed { print!("{:02X}", b); }
            println!();
        } else {
            if !cpu.step() { eprintln!("[HALT] opcode no implementado {:02X} en {:04X}", op, pc); break; }
        }
        steps += 1;
    }
    if !found_puls { eprintln!("[INFO] No se alcanzó PULS en {} pasos", steps); }
}
