use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== SIMPLE EMULATOR TEST CON DEBUG ===");
    
    let bios_path = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    // Correr 20 pasos para capturar el CLR indexed bug
    for step in 0..20 {
        let pc_before = cpu.pc;
        let x_before = cpu.x;
        let opcode = cpu.bus.mem[cpu.pc as usize];
        
        cpu.step();
        
        let x_after = cpu.x;
        
        println!("Paso {}: PC={:04X} Op={:02X} X: {:04X}→{:04X}", 
                 step, pc_before, opcode, x_before, x_after);
        
        // Detectar CLR indexed específicamente
        if opcode == 0x6F {
            if x_before != x_after {
                println!("🚨 CLR INDEXED BUG DETECTADO: X cambió de {:04X} a {:04X}", x_before, x_after);
            }
        }
    }
}