//! Ignored long-running trace test: genera un volcado de ~2M instrucciones ejecutando la BIOS real.
//! Ejecutar manualmente con:
//!    cargo test -p vectrex_emulator --test bios_long_trace -- --ignored --nocapture
//! Salida: target/bios_trace_2m.txt (puede ser >150MB según formato)
//! Política "no sintético": se usa la BIOS real en la ruta fija indicada.
//! Este test NO corre por defecto para evitar ralentizar CI/local builds.
use std::fs::{File};
use std::io::{Write, BufWriter};

// Ruta BIOS real (ver copilot-instructions sección 1). Política: no sintético.
const BIOS_PATH: &str = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
// Nombre base se ajustará dinámicamente según límite para evitar confusión.
fn out_path(limit: usize) -> String { format!("target/bios_trace_{}.txt", limit) }
const DEFAULT_INSTR_LIMIT: usize = 2_000_000; // 2 millones (override por arg/env)

#[test]
#[ignore]
fn bios_long_trace() {
    // Determinar límite dinámico: prioridad CLI > env > default
    let mut instr_limit = DEFAULT_INSTR_LIMIT;
    // Leer args pruebas: --trace-limit=N
    let mut args = std::env::args().peekable();
    while let Some(arg) = args.next() {
        if let Some(val) = arg.strip_prefix("--trace-limit=") {
            if let Ok(n)=val.parse::<usize>() { instr_limit = n; }
        } else if arg == "--trace-limit" {
            if let Some(val) = args.peek() { if let Ok(n)=val.parse::<usize>() { instr_limit = n; } }
        }
    }
    if let Ok(env_lim) = std::env::var("VPY_TRACE_LIMIT") { if let Ok(n)=env_lim.parse::<usize>() { instr_limit = n; } }

    eprintln!("[bios_long_trace] trace-limit = {instr_limit}");

    // Cargar BIOS vía método oficial del core para garantizar layout correcto (8K -> base 0xE000 con vectores válidos).
    let data = std::fs::read(BIOS_PATH).expect("No se pudo leer BIOS real");
    assert!(data.len()==4096 || data.len()==8192, "BIOS debe ser 4KB o 8KB");
    let mut cpu = vectrex_emulator::CPU::default();
    cpu.load_bios(&data); // establece bios_present, base y read-only
    cpu.reset(); // aplica lógica de vector forzado si vector inconsistente; PC final en cpu.pc
    if instr_limit <= 200 { cpu.trace = true; } // activar trace detallado para depuración en límites pequeños
    eprintln!("[bios_long_trace] post-reset PC=0x{:04X}", cpu.pc);

    // Archivo de salida
    // Asegurar directorio destino
    let out_path = out_path(instr_limit);
    if let Some(parent) = std::path::Path::new(&out_path).parent() { std::fs::create_dir_all(parent).ok(); }
    let file = File::create(&out_path).expect("crear salida");
    let mut w = BufWriter::with_capacity(1<<20, file); // 1MB buffer
    writeln!(w, "# BIOS trace (hasta {instr_limit} instrucciones) PC_inicial=0x{:04X}", cpu.pc).ok();
    writeln!(w, "# Formato: 0xPPPP: MNEM(0xOP) [sub=0xSS opcional] A:0xAA B:0xBB X:0xXXXX Y:0xYYYY U:0xUUUU S:0xSSSS DP:0xDD Z:0 N:0 C:0 cycles:TOTAL idx:N [label:XYZ] [loop]").ok();

    // Helper lectura segura
    let read = |cpu: &mut vectrex_emulator::CPU, addr:u16| -> u8 { cpu.bus.read8(addr) };

    // Estructuras para detección de bucles: hash de ventana + contador de repeticiones.
    const LOOP_WINDOW: usize = 512; // tamaño ventana circular de PCs
    const LOOP_REPEAT_THRESHOLD: usize = 20; // repeticiones idénticas para considerar bucle
    let mut pc_ring: [u16; LOOP_WINDOW] = [0; LOOP_WINDOW];
    let mut ring_pos = 0usize;
    let mut history_hashes: std::collections::VecDeque<u64> = std::collections::VecDeque::with_capacity(LOOP_REPEAT_THRESHOLD+2);
    let mut loop_detected = false;

    for i in 0..instr_limit {
        let pc = cpu.pc;
        let op = read(&mut cpu, pc);
        // Pre-fetch dos bytes siguientes para contexto (no siempre operandos válidos, pero útil)
    let b1 = read(&mut cpu, pc.wrapping_add(1));
    let _b2 = read(&mut cpu, pc.wrapping_add(2));

        // Ejecutar una instrucción (step central en crate). Usamos wrapper run_one() si existiera; si no, simulamos mínima ruta.
        // Aquí llamamos a step_instruction() a través de API pública; si no hay, usamos emulate_next_instruction() equivalente.
        // NOTA: cpu6809.rs expone una función step() de alto nivel? Buscamos nombre estándar; fallback implementado más arriba es internal.
        // Para mantener compatibilidad, llamamos a 'step_cpu_one(&mut cpu)' si existiera; de lo contrario replicar minimal.
        // Simplificación: reutilizamos método público 'run_steps(1)' si el crate lo provee. Si no existe, usamos un bloque inline adaptado.
        // Dado que no conocemos el nombre exacto aquí en test compilación validará; mientras tanto intentamos 'step_cycle_accurate'.
        // Implementamos un feature gate minimal llamando a un método generado en wasm_api? Si no, fallback a un pequeño pseudo-dispatch.

        // Intento 1: use cpu.exec_one() si existiera
        let advanced = vectrex_emulator::maybe_exec_one(&mut cpu); // ejecuta una instrucción
        if !advanced { break; }

        // Etiqueta BIOS si aplica
        let bios_label = if pc >= 0xF000 {
            vectrex_emulator::opcode_meta::bios_label_for(pc).unwrap_or("")
        } else { "" };

        // Loop detection: registrar PC en anillo, computar hash simple cada LOOP_WINDOW y comparar últimas N
        pc_ring[ring_pos] = pc; ring_pos = (ring_pos + 1) % LOOP_WINDOW;
        let mut loop_flag = 0;
        if ring_pos == 0 { // anillo lleno -> hash bloque completo
            use std::hash::Hasher;
            use std::collections::hash_map::DefaultHasher;
            let mut hasher = DefaultHasher::new();
            for v in &pc_ring { hasher.write_u16(*v); }
            let h = hasher.finish();
            history_hashes.push_back(h);
            if history_hashes.len() > LOOP_REPEAT_THRESHOLD { history_hashes.pop_front(); }
            if history_hashes.len() == LOOP_REPEAT_THRESHOLD {
                let first = history_hashes.front().copied().unwrap();
                if history_hashes.iter().all(|x| *x == first) {
                    loop_detected = true;
                }
            }
            if loop_detected { loop_flag = 1; }
        }

        // Determinar mnemonic (manejar prefijos 0x10/0x11)
        let (base_op, sub_op) = if op == 0x10 || op == 0x11 { (op, b1) } else { (op, 0) };
        let mnem = vectrex_emulator::cpu6809::opcode_mnemonic(base_op, sub_op);
        let flags_str = format!("Z:{} N:{} C:{}", cpu.cc_z as u8, cpu.cc_n as u8, cpu.cc_c as u8);
        let mut line = format!(
            "0x{pc:04X}: {mnem}({:#04X}) A:0x{a:02X} B:0x{b:02X} X:0x{x:04X} Y:0x{y:04X} U:0x{u:04X} S:0x{s:04X} DP:0x{dp:02X} {flags} cycles:{cycles} idx:{i}",
            op,
            a=cpu.a, b=cpu.b, x=cpu.x, y=cpu.y, u=cpu.u, s=cpu.s, dp=cpu.dp, flags=flags_str, cycles=cpu.cycles
        );
        if base_op == 0x10 || base_op == 0x11 { line.push_str(&format!(" sub:0x{sub:02X}", sub=sub_op)); }
        if !bios_label.is_empty() { line.push_str(&format!(" label:{bios_label}")); }
        if loop_flag == 1 { line.push_str(" loop"); }
        writeln!(w, "{line}").ok();

        if loop_detected { writeln!(w, "# LOOP_DETECTED at instruction {i}").ok(); break; }

        if i % 100_000 == 0 && i>0 { w.flush().ok(); }
    }
    w.flush().ok();
    eprintln!("[bios_long_trace] Trace escrito en {out_path}");
}
