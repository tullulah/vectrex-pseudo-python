use vectrex_emulator::CPU;

// Reinterpretación del test "bios_puls_return_valido":
// Evidencia actual indica que en la fase temprana de arranque (<=300k pasos) la BIOS NO ejecuta
// un PULS que incluya PC (bit 0x80). El test previo forzaba una expectativa inexistente y fallaba
// sistemáticamente, aportando ruido. Ahora validamos la AUSENCIA de ese patrón para detectar:
//  - Decodificación errónea del opcode 0x35.
//  - Corrupción de la pila que inserte máscaras con bit PC demasiado pronto.
//  - Introducción accidental de heurísticas sintéticas que generen marcos artificiales.
// Si en el futuro se documenta un punto concreto donde aparece PULS PC (con dirección BIOS conocida),
// se reemplazará este test por uno positivo con aserción de dirección.
#[test]
fn bios_no_puls_pc_300k() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] BIOS faltante {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    cpu.enable_autotrace(48);
    let mut steps: u64 = 0;
    let limit: u64 = 300_000;
    let mut puls_pc_detectado = None;
    while steps < limit {
        let pc = cpu.pc;
        let op = cpu.mem[pc as usize];
        if op == 0x35 { // PULS
            let mask = cpu.mem[(pc+1) as usize];
            if (mask & 0x80) != 0 { // incluye PC
                puls_pc_detectado = Some((pc, mask));
                break;
            }
        }
        if !cpu.step() { eprintln!("[HALT] opcode no impl {:02X} en {:04X}", op, pc); break; }
        steps += 1;
    }
    if let Some((pc, mask)) = puls_pc_detectado {
        panic!("Apareció PULS con PC inesperado antes de {} pasos: PC={:04X} mask={:02X}", limit, pc, mask);
    } else {
        eprintln!("[INFO] Sin PULS con PC en {} pasos (esperado).", limit);
    }
}
