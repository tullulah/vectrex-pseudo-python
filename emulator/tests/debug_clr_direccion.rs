//! Debug de la instrucción CLR 6F en el bucle
use vectrex_emulator::CPU;

#[test]
fn debug_clr_direccion() {
    const BIOS_PATH: &str = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin";
    let bios = match std::fs::read(BIOS_PATH) { Ok(b)=>b, Err(e)=> { eprintln!("[SKIP] No BIOS at {} ({})", BIOS_PATH, e); return; } };
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    // Llegar al bucle F548
    while cpu.pc != 0xF548 { cpu.step(); }
    
    println!("[DEBUG] En bucle F548");
    println!("PC={:04X} D={:04X} X={:04X} Y={:04X} U={:04X} S={:04X}", 
             cpu.pc, ((cpu.a as u16) << 8) | (cpu.b as u16), cpu.x, cpu.y, cpu.u, cpu.s);
    
    // Examinar la instrucción CLR
    let pc = cpu.pc;
    println!("Instrucción en {:04X}: {:02X} {:02X}", pc, cpu.mem[pc as usize], cpu.mem[(pc+1) as usize]);
    
    // Habilitar trace para ver la dirección exacta
    cpu.trace = true;
    
    // Ejecutar solo el CLR
    cpu.step();
    
    println!("Después del CLR: PC={:04X} D={:04X} X={:04X} Y={:04X} U={:04X} S={:04X}", 
             cpu.pc, ((cpu.a as u16) << 8) | (cpu.b as u16), cpu.x, cpu.y, cpu.u, cpu.s);
}