use std::fs; use vectrex_emulator::CPU;
fn main(){
    let args: Vec<String>=std::env::args().collect();
    if args.len()<2 { eprintln!("uso: emu <rom.bin> [trace]"); return; }
    let data=fs::read(&args[1]).expect("no lee rom");
    let mut cpu = CPU::with_pc(0x0030); // BIOS salta a $0030 tras header
    cpu.load_bin(&data,0x0000);
    cpu.trace = args.iter().any(|s| s=="--trace");
    let mut max_steps: usize = 5000;
    let mut dump_range: Option<(u16,u16)> = None;
    for w in &args[2..] {
        if let Some(rest)=w.strip_prefix("--max=") { if let Ok(v)=rest.parse() { max_steps=v; } }
        else if let Some(path)=w.strip_prefix("--bios=") { if let Ok(b)=fs::read(path) { cpu.load_bios(&b); } else { eprintln!("no se pudo leer BIOS: {}", path); } }
        else if let Some(r)=w.strip_prefix("--dump=") {
            if let Some((a,b))=r.split_once('-') { if let (Ok(sa),Ok(sb))=(u16::from_str_radix(a,16), u16::from_str_radix(b,16)) { dump_range=Some((sa,sb)); } }
        }
    }
    for _ in 0..max_steps { if !cpu.step() { break; } }
    if let Some((s,e))=dump_range { println!("DUMP {:04X}-{:04X}", s,e); let mut addr=s; while addr<=e { print!("{:04X}:", addr); for i in 0..16 { let a=addr.wrapping_add(i); if a>e { break; } print!(" {:02X}", cpu.mem[a as usize]); } println!(); addr = addr.wrapping_add(16);} }
    println!("BIOS calls:");
    for c in cpu.bios_calls { println!("{}", c); }
    println!("Frames:{} Intensity:{:02X} Reset0Ref:{} PrintStr:{} PrintList:{} Cycles:{}", 
        cpu.frame_count, cpu.last_intensity, cpu.reset0ref_count, cpu.print_str_count, cpu.print_list_count, cpu.cycles);
}
