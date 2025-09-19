use std::fs;
use std::time::Instant;
use vectrex_emulator::cpu6809::CPU;

// Herramienta de diagnóstico: corre sólo la BIOS y observa la intro.
// Loggea cada retorno de Wait_Recal: ciclo, PC, Vec_Loop_Count (16-bit) y Vec_Music_Flag.
// Detecta transición: Vec_Music_Flag pasa a 0 y loop_count > umbral => se espera salto a Mine Storm o cart.
// No fuerza estados; lectura directa de RAM emulada (no sintético).
fn main() {
    let bios_path = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    // Usar reset() para ejecutar secuencia completa Init_OS -> Init_OS_RAM -> Init_VIA -> Set_Refresh.
    // Evita falso negativo donde T2 nunca se carga si sólo forzamos PC y estado intermedio.
    cpu.reset();
    // Flag opcional de tracing CPU (--trace) via variable de entorno simple para no ampliar parser.
    if std::env::var("BIOS_INTRO_TRACE").ok().as_deref()==Some("1") { cpu.trace = true; }

    let start = Instant::now();
    let mut last_report_cycles = 0u64;
    let mut last_loop = 0u16;
    let mut last_music = 0u8;
    let mut wait_recal_count = 0u64;

    // Direcciones en página C8 (direct page cuando DP=$C8):
    // Vec_Loop_Count = $C80C/$C80D (según disasm estándar) -- Confirmar: usamos búsqueda heurística si cambia.
    // Aquí asumimos layout BIOS original: (tomado de vectrex disasm común) pero por robustez detectaremos la palabra que incrementa exactamente 1 cada Wait_Recal.
    // Para simplicidad inicial usaremos direcciones conocidas: Loop_Count en $C80C (lo) $C80D (hi). Music_Flag en $C80F? (en disasm dado es <Vec_Music_Flag). Usamos etiquetas directas si exportadas.
    // NOTA: Como aún no exponemos símbolos desde el core, accedemos memoria directa.

    // Hardcode offsets por ahora (ajustar si difiere): se pueden refinar capturando un delta table.
    let loop_lo = 0xC800 + 0x0C; // Vec_Loop_Count
    let loop_hi = 0xC800 + 0x0D;
    let music_flag = 0xC800 + 0x0E; // <Vec_Music_Flag en disasm: se escribe early (#$81) y se limpia tras música.

    while cpu.cycles < 30_000_000 { // límite generoso
        let pc_before = cpu.pc;
        cpu.step();
        // Cuando se registra llamada a Wait_Recal (record_bios_call incrementa wait_recal_calls) podemos samplear tras su retorno.
        // Simplificación: cuando PC vuelve a la región F0A4..F110 después de haber pasado por F192.
        if pc_before == 0xF192 { // entrada Wait_Recal
            wait_recal_count += 1;
        }
        if cpu.wait_recal_returns > 0 && cpu.wait_recal_returns != (wait_recal_count as u64) {
            // returns se incrementa en RTS/RTI correspondiente; sincronizamos con contador local si difiere.
            wait_recal_count = cpu.wait_recal_returns as u64;
        }
        if cpu.cycles - last_report_cycles >= 150_000 { // ~100 Hz logs a ~15M cps
            let lo = cpu.test_read8(loop_lo as u16) as u16;
            let hi = cpu.test_read8(loop_hi as u16) as u16;
            let loop_val = (hi << 8) | lo;
            let music = cpu.test_read8(music_flag as u16);
            if loop_val != last_loop || music != last_music {
                println!("[intro] cyc={} frame_like={} loop={} music_flag={:02X} wait_calls={} pc={:04X}",
                    cpu.cycles, cpu.bios_frame, loop_val, music, cpu.wait_recal_calls, cpu.pc);
                last_loop = loop_val; last_music = music;
            }
            last_report_cycles = cpu.cycles;
            // Condición de salida: música terminada (flag=0) y loop avanzado suficiente (>= 0x0080) -> debería saltar a juego.
            if music == 0 && loop_val >= 0x0080 { println!("[intro] música terminada y loop>=128; esperar salto a juego pronto."); }
        }
        // Si detectamos PC fuera de BIOS alta (>=E000 y <F000) lo notificamos como potencial entrada a Mine Storm.
        if (0xE000..0xF000).contains(&cpu.pc) {
            println!("[intro] salto a código Mine Storm/cart detectado pc={:04X} cyc={}", cpu.pc, cpu.cycles);
            break;
        }
    }
    let elapsed = start.elapsed();
    println!("[done] cycles={} elapsed={:.3}s cps={:.0} wait_calls={} wait_returns={}", cpu.cycles, elapsed.as_secs_f64(), (cpu.cycles as f64)/elapsed.as_secs_f64(), cpu.wait_recal_calls, cpu.wait_recal_returns);
}
