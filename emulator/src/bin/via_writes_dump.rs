use vectrex_emulator::cpu6809::CPU;
use std::env;
use std::fs;
use std::time::Instant;

// Pequeña utilidad: ejecuta BIOS hasta N ciclos y para cada llamada a Draw_VL/Draw_VLc imprime
// las últimas escrituras VIA cercanas (ventana configurable) para guiar migración del intercept.
fn main(){
    // Args: cycles window [--cart=path] [--stop=N] [--seconds=S]
    let args: Vec<String> = env::args().skip(1).collect();
    let cycles: u64 = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(1_500_000);
    let win: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(48);
    let mut cart_path: Option<String> = None;
    let mut stop_after: usize = usize::MAX; // infinito por defecto
    let mut seconds_limit: Option<f64> = None;
    for a in &args {
        if let Some(rest)=a.strip_prefix("--cart=") { cart_path=Some(rest.to_string()); }
    if let Some(rest)=a.strip_prefix("--stop=") { if let Ok(n)=rest.parse() { stop_after=n; } }
    if let Some(rest)=a.strip_prefix("--seconds=") { if let Ok(s)=rest.parse::<f64>() { seconds_limit=Some(s); } }
    }
    // Rutas fijas (no sintético)
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    if let Some(p) = cart_path.clone() {
        let cart = fs::read(&p).expect("no se pudo leer cartucho");
        cpu.load_bin(&cart,0x0000);
    }
    cpu.pc = 0xF000; // cold start
    let mut captures = 0usize;
    let start_wall = Instant::now();
    let mut next_progress = 200_000u64;
    while cpu.cycles < cycles && captures < stop_after {
        if let Some(sl) = seconds_limit { if start_wall.elapsed().as_secs_f64() >= sl { break; } }
        let pc_before = cpu.pc;
        cpu.step();
        if cpu.cycles >= next_progress {
            let elapsed = start_wall.elapsed();
            let cps = (cpu.cycles as f64) / elapsed.as_secs_f64().max(1e-6);
            println!("[progress] cyc={} cps={:.0} via_writes={} bios_calls={} pc={:04X} captures={} wall={:.2}s", cpu.cycles, cps, cpu.via_writes.len(), cpu.bios_calls.len(), cpu.pc, captures, elapsed.as_secs_f64());
            next_progress += 200_000;
        }
        if let Some(last) = cpu.bios_calls.last() {
            if last.starts_with("F3DD:") || last.starts_with("F3CE:") {
                captures +=1;
                let via_log = cpu.via_writes.as_slice();
                println!("=== Capture {} Draw_VL* {} pc={:04X} cycle={} total_via_writes={} ===", captures, last, pc_before, cpu.cycles, via_log.len());
                let start = via_log.len().saturating_sub(win);
                let slice = &via_log[start..];
                for (i,w) in slice.iter().enumerate() {
                    println!("  [{:02}] cyc={} pc={:04X} addr={:04X} reg={} val={:02X}", i, w.cycle, w.pc, w.addr, w.reg, w.val);
                }
                let mut counts = [0u32;16];
                let mut last_val = [None;16];
                for w in slice { counts[w.reg as usize]+=1; last_val[w.reg as usize]=Some(w.val); }
                println!("  --- resumen ventana ---");
                for r in 0..16 { if counts[r]>0 { if let Some(v)=last_val[r] { println!("    reg {:X} count={} last={:02X}", r, counts[r], v); } } }
            }
        }
    }
    let elapsed = start_wall.elapsed();
    let cps = (cpu.cycles as f64)/elapsed.as_secs_f64().max(1e-6);
    println!("[done] cycles={} captures={} cart_loaded={} wall={:.3}s cps={:.0}", cpu.cycles, captures, cart_path.is_some(), elapsed.as_secs_f64(), cps);
}
