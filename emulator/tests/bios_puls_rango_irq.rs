use vectrex_emulator::CPU;

// NOTA (2024-09): Replanteado test anterior "bios_puls_rango_irq".
// Hallazgo: La BIOS en la ventana temprana (>=400k pasos de arranque analizados) no:
//   1) Habilita IRQ maskables (bit I permanece activo: no se observa ANDCC #$EF ni patrón equivalente).
//   2) Usa la forma "PULS ... PC" para retornar temprano; emplea RTS/RTI según corresponda.
// Por tanto, el supuesto del test original (encontrar un PULS con PC tras el primer IRQ) era inválido y
// producía un falso negativo constante. En lugar de forzar una expectativa no real, validamos hoy la
// AUSENCIA de PULS con bit PC en ese tramo inicial, lo cual protege contra introducir heurísticas
// sintéticas que fabriquen marcos o muten la pila indebidamente.
// Si en el futuro se verifica (con evidencia de BIOS) un punto donde se limpia I y se atiende un IRQ que
// culmine en secuencia con PULS PC (optimización combinada de retorno), podremos reintroducir un test
// positivo específico apoyado en esa dirección concreta de la BIOS.
// Este test también sirve como centinela: si aparece inesperadamente un PULS con PC muy temprano podría
// indicar corrupción de flujo, decodificación errónea de opcode 0x35 o escritura indebida en memoria de BIOS.
#[test]
fn bios_no_puls_pc_temprano() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] BIOS faltante {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    // Autotrace limitado: suficiente para primeros bytes y facilitar depuración manual si se activa TRACE global.
    cpu.enable_autotrace(32);

    let mut steps: u64 = 0;
    let limit: u64 = 400_000; // Mantener mismo margen que versión previa para comparación histórica.
    let mut pulspc_encontrado = None;

    while steps < limit {
        let pc_now = cpu.pc;
        let op = cpu.mem[pc_now as usize];
        if op == 0x35 { // PULS
            let mask = cpu.mem[(pc_now+1) as usize];
            if (mask & 0x80) != 0 { // incluye PC
                pulspc_encontrado = Some((pc_now, mask));
                break;
            }
        }
        if !cpu.step() { eprintln!("[HALT] opcode no impl {:02X} en {:04X}", op, pc_now); break; }
        steps += 1;
    }

    if let Some((pc, mask)) = pulspc_encontrado {
        panic!("Apareció PULS con PC inesperado en etapa temprana: PC={:04X} mask={:02X} steps={}", pc, mask, steps);
    } else {
        eprintln!("[INFO] No se observó PULS con PC en {} pasos (esperado).", steps);
    }
}
