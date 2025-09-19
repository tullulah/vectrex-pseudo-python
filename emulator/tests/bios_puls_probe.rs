use vectrex_emulator::CPU;

// Test diagnóstico: localiza la primera instrucción PULS (0x35) ejecutada por la BIOS
// y captura:
//  - PC del PULS y byte máscara
//  - SP (S) antes y después
//  - 32 bytes de pila alrededor de S previo
//  - Nuevo PC tras ejecutar el PULS
// No afirma todavía valores concretos (la máscara puede variar según variante de BIOS),
// pero sirve para detectar si estamos saltando a RAM vacía inmediatamente después.
#[test]
fn bios_puls_probe() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] falta BIOS real {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    let mut steps: u32 = 0;
    let limit: u32 = 80_000; // margen amplio
    let mut found = false;
    let mut _before_s = 0u16; // underscore to silence unused warnings (solo logging)
    let mut _puls_pc = 0u16;
    let mut _mask = 0u8;
    while steps < limit {
        let pc = cpu.pc; let op = cpu.mem[pc as usize];
        if op == 0x35 { // PULS
            // Leer máscara sin ejecutar todavía
            _mask = cpu.mem[pc.wrapping_add(1) as usize];
            _puls_pc = pc;
            _before_s = cpu.s;
            // Dump de pila (S apunta al siguiente byte a extraer)
            print!("[PULS] pc={:04X} op=35 mask={:02X} S_before={:04X}\n", pc, _mask, _before_s);
            for line in 0..2 { // 32 bytes
                let base = _before_s.wrapping_add((line*16) as u16);
                print!(" {:04X}: ", base);
                for o in 0..16 { let a = base.wrapping_add(o); let b = cpu.mem[a as usize]; print!("{:02X} ", b); }
                println!();
            }
            // Ejecutar la instrucción
            let ok = cpu.step();
            print!("[PULS] ok={} S_after={:04X} new_pc={:04X}\n", ok, cpu.s, cpu.pc);
            found = true;
            break;
        }
        if !cpu.step() { eprintln!("[HALT] opcode no implementado {:02X} en {:04X}", op, pc); break; }
        steps += 1;
    }
    if !found { eprintln!("[INFO] No se encontró PULS en {} pasos", limit); }
}
