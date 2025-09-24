use std::fs;
use std::env;

// Very small subset disassembler matching implemented opcodes; expands as emulator grows.
fn disasm_one(mem: &[u8], pc: usize) -> (String, usize) {
    if pc >= 0x10000 { return ("".into(), 1); }
    let op = mem[pc];
    let b = |o: usize| if pc+o < 0x10000 { mem[pc+o] } else { 0 };
    match op {
        0x10 => { // prefix group (handle a few)
            let sub = b(1);
            match sub { 0x8E => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 8E     LDY #${:02X}{:02X}", pc, hi, lo),4) }
                        ,0xCE => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 CE     LDS #${:02X}{:02X}", pc, hi, lo),4) }
                        ,0x3F => (format!("{:04X}: 10 3F     SWI2" , pc),2)
                        ,0x26 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 26 {:02X}{:02X} LBNE <rel>", pc, hi, lo),4) }
                        ,0x27 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 27 {:02X}{:02X} LBEQ <rel>", pc, hi, lo),4) }
                        ,0x83|0x93|0xB3 => (format!("{:04X}: 10 {:02X}     CMPD <various>", pc, sub),2)
                        ,_ => (format!("{:04X}: 10 {:02X}     (unimpl prefix)" , pc, sub),2)}
        }
        0x11 => { let sub=b(1); (format!("{:04X}: 11 {:02X}     (prefix2)" , pc, sub),2) }
        0xBD => { let hi=b(1); let lo=b(2); (format!("{:04X}: BD {:02X} {:02X} JSR ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x9D => { let off=b(1); (format!("{:04X}: 9D {:02X}    JSR <$DP{:02X}>", pc, off, off),2) }
        0x8D => { let off=b(1); (format!("{:04X}: 8D {:02X}    BSR ${:04X}", pc, off, ((pc+2) as isize + (off as i8 as isize)) as u16),2) }
        0x20|0x26|0x27|0x22|0x23|0x24|0x25|0x28|0x29|0x2A|0x2B|0x2C|0x2F => {
            let off=b(1) as i8; let target = ((pc+2) as isize + off as isize) as u16; (format!("{:04X}: {:02X} {:02X}    BR? ${:04X}", pc, op, b(1), target),2)
        }
        0xCC|0xCE|0x8E => { let hi=b(1); let lo=b(2); let mnem = match op {0xCC=>"LDD",0xCE=>"LDU",0x8E=>"LDX", _=>"LD?"}; (format!("{:04X}: {:02X} {:02X} {:02X} {} #${:02X}{:02X}", pc, op, hi, lo, mnem, hi, lo),3) }
        0x86|0xC6 => { let imm=b(1); let mnem = if op==0x86 {"LDA"} else {"LDB"}; (format!("{:04X}: {:02X} {:02X}    {} #${:02X}", pc, op, imm, mnem, imm),2) }
        0x81|0xC1 => { let imm=b(1); let mnem = if op==0x81 {"CMPA"} else {"CMPB"}; (format!("{:04X}: {:02X} {:02X}    {} #${:02X}", pc, op, imm, mnem, imm),2) }
        0xB6|0xF6 => { let hi=b(1); let lo=b(2); let mnem = if op==0xB6 {"LDA"} else {"LDB"}; (format!("{:04X}: {:02X} {:02X} {:02X} {} ${:02X}{:02X}", pc, op, hi, lo, mnem, hi, lo),3) }
        0xFD|0xDD => { let off=b(1); let mnem= if op==0xFD {"STD"} else {"STD"}; (format!("{:04X}: {:02X} {:02X}    {} <$DP{:02X}>", pc, op, off, mnem, off),2) }
        0x39 => (format!("{:04X}: 39        RTS", pc),1),
        0x3B => (format!("{:04X}: 3B        RTI", pc),1),
        0x3E => (format!("{:04X}: 3E        WAI", pc),1),
        0x4F => (format!("{:04X}: 4F        CLRA", pc),1),
        0x5F => (format!("{:04X}: 5F        CLRB", pc),1),
        0x7C => { let hi=b(1); let lo=b(2); (format!("{:04X}: 7C {:02X} {:02X} INC ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x7A => { let hi=b(1); let lo=b(2); (format!("{:04X}: 7A {:02X} {:02X} DEC ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x85 => { let imm=b(1); (format!("{:04X}: 85 {:02X}    BITA #${:02X}", pc, imm, imm),2) }
        0xC4|0xC8|0x8A => { let imm=b(1); (format!("{:04X}: {:02X} {:02X}    IMM8", pc, op, imm),2) }
        _ => (format!("{:04X}: {:02X}        .db ${:02X}", pc, op, op),1)
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: disasm <bios.bin> [start_hex] [count]" );
        return;
    }
    let data = fs::read(&args[1]).expect("read bios");
    let start = if args.len()>2 { u16::from_str_radix(&args[2],16).unwrap_or(0xF000) } else { 0xF000 } as usize;
    let count = if args.len()>3 { args[3].parse::<usize>().unwrap_or(256) } else { 256 };
    // Map using same rule as emulator
    let mut mem = [0u8;65536];
    match data.len() { 4096 => { mem[0xF000..0xF000+data.len()].copy_from_slice(&data); }, 8192 => { mem[0xE000..0xE000+data.len()].copy_from_slice(&data); }, _ => { eprintln!("Unsupported BIOS size"); return; } }
    let mut pc = start;
    let end = start + count.min(0x10000-start);
    while pc < end { let (line, adv) = disasm_one(&mem, pc); println!("{}", line); pc += adv; }
}
