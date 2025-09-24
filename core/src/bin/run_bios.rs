use std::fs;
use std::env;
use vectrex_emulator::CPU; // legacy vector events removed; integrator backend canonical

fn main(){
    // Allow explicit BIOS path via first CLI argument or BIOS_FILE env. Otherwise search known candidates (including vectrexy.bin)
    let args: Vec<String> = env::args().collect();
    let explicit = if args.len()>1 { Some(args[1].clone()) } else { env::var("BIOS_FILE").ok() };
    let bios: Vec<u8> = if let Some(path) = explicit {
        fs::read(&path).expect("Failed to read specified BIOS file")
    } else {
        let candidates = [
            "core/src/bios/vectrexy.bin",
            "core/src/bios/vectrex.bin",
            "core/src/bios/bios.bin",
            "src/bios/vectrexy.bin",
            "src/bios/vectrex.bin",
            "src/bios/bios.bin",
            "vectrexy.bin",
            "vectrex.bin",
            "bios.bin"
        ];
        let mut found: Option<Vec<u8>> = None;
        for c in candidates { if let Ok(d)=fs::read(c) { if d.len()==8192 || d.len()==4096 { found=Some(d); break; } } }
        found.expect("Could not locate a 4K/8K Vectrex BIOS image (try passing a path as first argument)")
    };
    if !(bios.len()==4096 || bios.len()==8192) { panic!("Unsupported BIOS size: {} (expected 4096 or 8192)", bios.len()); }
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    // Enable trace before reset to log vector info
    cpu.trace = true;
    cpu.reset();
    // Configuration knobs (env override optional)
    let max_steps: usize = std::env::var("BIOS_MAX_STEPS").ok().and_then(|v| v.parse().ok()).unwrap_or(200_000);
    let trace_window: usize = std::env::var("BIOS_TRACE_STEPS").ok().and_then(|v| v.parse().ok()).unwrap_or(128);
    let loop_detect_window: usize = 4096; // number of recent PCs to keep for loop detection
    use std::collections::VecDeque;
    let mut recent_pcs: VecDeque<u16> = VecDeque::with_capacity(loop_detect_window);
    let mut loop_break = false;
    let _dump_vectors = false; // vector event dump removed
    for step in 0..max_steps {
        if !cpu.step() { break; }
        if step+1 == trace_window { cpu.trace = false; }
        // Vector event dumping removed.
        // Loop detection: if last 256 PCs pattern repeats (simple heuristic) stop early
        recent_pcs.push_back(cpu.pc);
        if recent_pcs.len() > loop_detect_window { recent_pcs.pop_front(); }
        if recent_pcs.len() == loop_detect_window {
            // Compare first half vs second half as crude steady-state detector
            let half = loop_detect_window/2;
            let stable = recent_pcs.iter().take(half).zip(recent_pcs.iter().skip(half)).all(|(a,b)| a==b);
            if stable { loop_break = true; break; }
        }
    }
    println!("BIOS run complete / halted.");
    if loop_break { println!("Early stop: detected steady-state PC loop pattern (heuristic)"); }
    println!("PC={:04X} A={:02X} B={:02X} X={:04X} Y={:04X} U={:04X} S={:04X}", cpu.pc, cpu.a, cpu.b, cpu.x, cpu.y, cpu.u, cpu.s);
    println!("FrameCount={} Reset0Ref={} PrintStr={}", cpu.frame_count, cpu.reset0ref_count, cpu.print_str_count);
    // (metrics display removed; not exposed in external crate yet)
}
