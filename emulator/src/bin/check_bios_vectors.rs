use vectrex_emulator::CPU;
use std::env; use std::fs;
fn main(){
    let path = env::args().nth(1).expect("uso: check_bios_vectors <ruta/bios.bin>");
    let data = match fs::read(&path) {
        Ok(d)=>d,
        Err(e)=>{
            eprintln!("Error leyendo BIOS en '{}': {}", path, e);
            eprintln!("Sugerencias: pruebe rutas relativas como core\\src\\bios\\bios.bin o especifique ruta absoluta.");
            std::process::exit(1);
        }
    };
    if !(data.len()==4096 || data.len()==8192){ eprintln!("Tamaño BIOS inesperado: {}", data.len()); return; }
    let mut cpu = CPU::default();
    cpu.load_bios(&data);
    // Leer vector reset (FFFE/FFFF) desde bus.mem
    let hi = cpu.bus.mem[0xFFFE]; let lo = cpu.bus.mem[0xFFFF];
    let reset_vec = ((hi as u16)<<8)|lo as u16;
    println!("BIOS size={} base=${:04X} reset_vector=${:04X}", data.len(), cpu.bus.test_bios_base(), reset_vec);
    // Dump primeros bytes en reset_vec
    for i in 0..8 { print!("{:02X} ", cpu.bus.mem[reset_vec.wrapping_add(i) as usize]); } println!();
}
