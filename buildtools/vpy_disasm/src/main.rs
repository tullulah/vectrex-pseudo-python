use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: vpy_disasm <rom.bin> [start_hex] [count]");
        eprintln!("  Supports arbitrary ROM sizes (including multibank)");
        std::process::exit(1);
    }
    
    let data = match fs::read(&args[1]) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading ROM: {}", e);
            std::process::exit(1);
        }
    };
    
    let start = if args.len() > 2 {
        u32::from_str_radix(&args[2], 16).unwrap_or(0) as usize
    } else {
        0
    };
    
    let count = if args.len() > 3 {
        args[3].parse::<usize>().unwrap_or(256)
    } else {
        256
    };
    
    eprintln!("📖 Disassembling {}: {} bytes total", &args[1], data.len());
    eprintln!("   Starting at offset 0x{:04X}, range {} bytes", start, count);
    eprintln!();
    
    let lines = vpy_disasm::disassemble_range(&data, start, count);
    
    for line in lines {
        println!("{}", line);
    }
    
    eprintln!();
    eprintln!("✓ Disassembly complete");
}
