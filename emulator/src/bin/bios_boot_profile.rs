//! Ejecuta la BIOS real durante un arranque corto y emite:
//!  - Lista de llamadas BIOS únicas (en orden de primera aparición)
//!  - Top N opcodes base ejecutados (frecuencia)
//!  - Número de segmentos generados (si integrator intercept produce algo)
//! Uso:
//!   cargo run -p vectrex_emulator --bin bios_boot_profile --release --  --cycles=200000
//! Flags:
//!   --cycles=N   ciclos máximos a simular (por defecto ~500_000)
//!   --trace      activa cpu.trace para ver PCs iniciales
use vectrex_emulator::CPU;
use std::collections::HashSet; // HashMap eliminado (no usado)

const BIOS_PATH: &str = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";

fn main(){
    let mut max_cycles: u64 = 500_000; // ~ un par de frames iniciales
    let mut want_trace = false;
    for arg in std::env::args().skip(1){
        if let Some(v)=arg.strip_prefix("--cycles=") { if let Ok(n)=v.parse(){ max_cycles=n; } }
        else if arg=="--trace" { want_trace=true; }
    }
    let data = std::fs::read(BIOS_PATH).expect("No se pudo leer BIOS real");
    assert!(data.len()==4096 || data.len()==8192);
    let mut cpu = CPU::default();
    cpu.load_bios(&data);
    cpu.reset();
    cpu.trace = want_trace;

    let mut bios_seen: HashSet<u16> = HashSet::new();
    let mut bios_order: Vec<u16> = Vec::new();
    let mut opcode_counts: [u64;256] = [0;256];

    while cpu.cycles < max_cycles {
        let pc=cpu.pc; let op=cpu.bus.read8(pc);
        opcode_counts[op as usize]+=1;
        if pc>=0xF000 && !bios_seen.contains(&pc){ bios_seen.insert(pc); bios_order.push(pc); }
        if !cpu.step(){ break; }
    }

    println!("== BIOS Boot Profile ==");
    println!("Cycles executed: {}", cpu.cycles);
    println!("Unique BIOS call PCs encountered: {}", bios_order.len());
    for pc in &bios_order { if let Some(label)=vectrex_emulator::opcode_meta::bios_label_for(*pc){ println!("  {:04X} {}", pc, label); } else { println!("  {:04X}", pc); } }

    // Top opcodes
    let mut pairs: Vec<(u8,u64)> = opcode_counts.iter().enumerate().map(|(i,&c)|(i as u8,c)).filter(|(_,c)| *c>0).collect();
    pairs.sort_by_key(|&(_,c)| std::cmp::Reverse(c));
    println!("\nTop 24 opcodes:");
    for (i,(op,c)) in pairs.iter().take(24).enumerate(){
        let m = vectrex_emulator::cpu6809::opcode_mnemonic(*op,0);
        println!(" {:2}. {:02X} {:>8} {}", i+1, op, c, m);
    }

    // Segment stats
    let segs = cpu.integrator.segments_slice();
    println!("\nSegments emitted: {}", segs.len());
    if !segs.is_empty(){
        let mut minx= f32::MAX; let mut maxx= f32::MIN; let mut miny=f32::MAX; let mut maxy=f32::MIN;
        for s in segs { minx=minx.min(s.x0.min(s.x1)); maxx=maxx.max(s.x0.max(s.x1)); miny=miny.min(s.y0.min(s.y1)); maxy=maxy.max(s.y0.max(s.y1)); }
        println!("Bounding box: x[{:.1},{:.1}] y[{:.1},{:.1}]", minx,maxx,miny,maxy);
    }
}
