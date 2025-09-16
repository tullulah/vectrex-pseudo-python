use vectrex_emulator::CPU;
use std::fs;
use std::time::Instant;

// Simple CLI trace runner: loads BIOS (bios.bin path arg or default ./bios.bin), runs instructions
// printing VIA state every N steps.
// Usage: cargo run -p vectrex_emulator --bin via_trace -- <bios_path> [steps] [print_interval]
// Defaults: steps=5_000_000 print_interval=50_000
fn main(){
    let mut args = std::env::args().skip(1);
    let bios_path = args.next().unwrap_or_else(|| "bios.bin".to_string());
    let total_steps: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(5_000_000);
    let print_interval: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(50_000);

    let bios = match fs::read(&bios_path) { Ok(b)=>b, Err(e)=>{ eprintln!("Failed to read BIOS {}: {}", bios_path, e); return; } };
    if !(bios.len()==4096 || bios.len()==8192) { eprintln!("BIOS size {} unsupported", bios.len()); return; }

    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    println!("[via_trace] Running authentic BIOS code");

    // Dump IRQ/FIRQ/NMI vectors for diagnostics
    let irq_lo = cpu.bus.mem[0xFFF6 as usize];
    let irq_hi = cpu.bus.mem[0xFFF7 as usize];
    let firq_lo = cpu.bus.mem[0xFFF4 as usize];
    let firq_hi = cpu.bus.mem[0xFFF5 as usize];
    let nmi_lo = cpu.bus.mem[0xFFFA as usize];
    let nmi_hi = cpu.bus.mem[0xFFFB as usize];
    println!("[vectors] IRQ={:04X} FIRQ={:04X} NMI={:04X}", ((irq_hi as u16)<<8)|irq_lo as u16, ((firq_hi as u16)<<8)|firq_lo as u16, ((nmi_hi as u16)<<8)|nmi_lo as u16);
    // IRQ handler dump (similar to FIRQ) : show 48 bytes starting 0x10 before vector
    let irq_vec = ((irq_hi as u16) << 8) | irq_lo as u16;
    let irq_dump_start = irq_vec.saturating_sub(0x10);
    let irq_dump_end = irq_vec + 0x30;
    print!("[irq_dump  {:04X}-{:04X}] ", irq_dump_start, irq_dump_end - 1);
    for addr in irq_dump_start..irq_dump_end { print!("{:02X}", cpu.bus.mem[addr as usize]); }
    println!("");
    // Dump 32 bytes around presumed FIRQ handler start (FIRQ vector minus ~0x0B) for quick manual disassembly
    let firq_vec = ((firq_hi as u16) << 8) | firq_lo as u16;
    let dump_start = firq_vec.saturating_sub(0x10);
    let dump_end = firq_vec + 0x20; // exclusive upper bound for iteration
    print!("[firq_dump {:04X}-{:04X}] ", dump_start, dump_end - 1);
    for addr in dump_start..dump_end { print!("{:02X}", cpu.bus.mem[addr as usize]); }
    println!("");

    println!("[via_trace] BIOS loaded ({} bytes). Steps={}, interval={}", bios.len(), total_steps, print_interval);
    println!("cols: step pc cycles frame via_t1 via_ifr via_ier irq_line irq_count wai cc_i bytes");

    let start = Instant::now();
    let mut last_report = 0u64;
    let mut last_irq_count = 0u64;
    let mut last_ifr: u8 = cpu.bus.via_ifr();
    let mut last_ier: u8 = cpu.bus.via_ier();
    let mut consecutive_irq_region_reports = 0u32;
    let mut first_irq_region_pc: Option<u16> = None;
    let mut opcode_freq: [u32;256] = [0;256];
    // instrumentation placeholders removed (unused)
    while last_report < total_steps {
        // Run chunk
        for _ in 0..print_interval {
            let pre_pc = cpu.pc;
            // let pre_sp = cpu.s; // not currently used
            // let pre_irq_count = cpu.via_irq_count; // unused
            let pre_in_irq = cpu.in_irq_handler;
            cpu.step();
            // Count opcodes executed in IRQ region (based on prior PC) for frequency insight
            if pre_pc >= 0xF770 && pre_pc <= 0xF7A0 {
                let op = cpu.bus.mem[pre_pc as usize];
                opcode_freq[op as usize] = opcode_freq[op as usize].saturating_add(1);
            }
            // Detect IRQ entry and log stack frame
            if cpu.in_irq_handler && !pre_in_irq {
                println!("[irq-entry] pc={:04X} pushed_frame_sp_final={:04X}", cpu.pc, cpu.s);
                // Dump top 16 bytes of stack (from new SP upward)
                let sp = cpu.s as usize; let end = (sp+16).min(0x10000);
                print!("[stack] ");
                for addr in sp..end { print!("{:02X}", cpu.bus.mem[addr]); }
                println!(" (from {:04X})", sp as u16);
            }
            // Detect RTI (opcode 0x3B executed at pre_pc) and log pop result
            if cpu.bus.mem[pre_pc as usize] == 0x3B {
                println!("[rti] from {:04X} -> pc={:04X} sp={:04X}", pre_pc, cpu.pc, cpu.s);
            }
            // Track last cycle when an IRQ service occurred (irq_count incremented inside CPU logic)
            // if cpu.via_irq_count != pre_irq_count { /* could record timing */ }
        }
        last_report += print_interval;
        let pc = cpu.pc;
        let via_t1 = cpu.bus.via.t1_counter();
        let via_ifr = cpu.bus.via_ifr();
        let via_ier = cpu.bus.via_ier();
        let irq_line = cpu.bus.via.irq_asserted();
        let b0 = cpu.bus.mem[pc as usize];
        let b1 = cpu.bus.mem[pc.wrapping_add(1) as usize];
        let b2 = cpu.bus.mem[pc.wrapping_add(2) as usize];
        println!("{:>9} {:04X} {:>8} {:>5} {:>6} {:02X} {:02X} {:>5} {:>5} {:>3} {:>3} {:02X}{:02X}{:02X}",
            last_report, pc, cpu.cycles, cpu.frame_count, via_t1, via_ifr, via_ier,
            if irq_line {1}else{0}, cpu.via_irq_count, if cpu.wai_halt {1}else{0}, if cpu.cc_i {1}else{0}, b0,b1,b2);
        if cpu.via_irq_count != last_irq_count { println!("[irq] serviced -> count {} (pc now {:04X})", cpu.via_irq_count, pc); last_irq_count = cpu.via_irq_count; }
        if via_ifr != last_ifr || via_ier != last_ier { println!("[via] IFR {:02X}->{:02X} IER {:02X}->{:02X}", last_ifr, via_ifr, last_ier, via_ier); last_ifr = via_ifr; last_ier = via_ier; }
        // IRQ region heuristic: BIOS IRQ handler near F770-F7A0 observed in trace
        if pc >= 0xF770 && pc <= 0xF7A0 {
            consecutive_irq_region_reports += 1;
            if first_irq_region_pc.is_none() { first_irq_region_pc = Some(pc); }
        } else {
            if consecutive_irq_region_reports > 0 { consecutive_irq_region_reports = 0; first_irq_region_pc=None; }
        }
        if consecutive_irq_region_reports == 6 && cpu.via_irq_count <= 1 {
            println!("[warn] Stuck in IRQ region (pc ~{:04X}) without additional IRQ services; possible missing RTI or IFR clearing", first_irq_region_pc.unwrap());
            // Print top 10 most frequent opcodes in region so far
            let mut pairs: Vec<(u8,u32)> = opcode_freq.iter().enumerate().filter(|(_,c)| **c>0).map(|(i,c)|(i as u8,*c)).collect();
            pairs.sort_by_key(|p| std::cmp::Reverse(p.1));
            print!("[irq-opcodes] ");
            for (i,(op,c)) in pairs.iter().take(10).enumerate() { print!("#"); if i>0 { print!(" "); } print!("{:02X}:{}", op, c); }
            println!("");
        }
        if cpu.frame_count >= 3 { break; }
    }
    let dur = start.elapsed();
    println!("[via_trace] Done in {:.3}s ({} steps)", dur.as_secs_f64(), last_report);
}
