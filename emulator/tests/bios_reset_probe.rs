use vectrex_emulator::CPU;

// Simple diagnostic (not a strict assertion test yet): dumps reset vector, entry PC, and first 32 executed PCs/opcodes.
// Helps reconcile expected F000/F002/F004 sequence vs actual BIOS variant present.
#[test]
fn bios_reset_probe() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    // Capture raw reset vector bytes before reset() modifies PC
    let lo = bios[bios.len()-4]; // FFFC relative inside 4K (if 4K) or 8K image; mapping: for 4K base=F000 so offset 0x0FFC
    let hi = bios[bios.len()-3];
    cpu.reset();
    println!("[BIOS] size={} reset_vec_raw={:02X}{:02X} pc_after_reset={:04X}", bios.len(), hi, lo, cpu.pc);
    // Dump first 16 bytes at canonical F000 region (if mapped)
    for addr in 0xF000u16..0xF010 {
        let byte = cpu.mem[addr as usize];
        if addr == 0xF000 { print!("[BIOS][F000..] "); }
        print!("{:02X} ", byte);
    }
    println!();
    // Step first 32 instructions capturing PC/opcode
    let mut pcs = Vec::new();
    for _ in 0..32 {
        let pc = cpu.pc; let op = cpu.mem[pc as usize];
        pcs.push((pc,op));
        if !cpu.step() { println!("[HALT] opcode no implementado en {:04X}", pc); break; }
    }
    println!("[TRACE32] {} entries", pcs.len());
    for (i,(pc,op)) in pcs.iter().enumerate() { println!("  {:02} {:04X}:{:02X}", i, pc, op); }
}
