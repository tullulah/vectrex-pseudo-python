/// VPy Disassembler - M6809 instruction disassembly
/// Supports arbitrary ROM sizes including multibank

pub fn disassemble_one(mem: &[u8], pc: usize) -> (String, usize) {
    if pc >= mem.len() { 
        return (format!("{:04X}: ?? (out of bounds)", pc), 1); 
    }
    
    let op = mem[pc];
    let b = |o: usize| if pc+o < mem.len() { mem[pc+o] } else { 0 };
    
    match op {
        0x10 => { // prefix group
            let sub = b(1);
            match sub { 
                0x8E => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 8E {:02X} {:02X} LDY #${:02X}{:02X}", pc, hi, lo, hi, lo),4) }
                0xCE => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 CE {:02X} {:02X} LDS #${:02X}{:02X}", pc, hi, lo, hi, lo),4) }
                0x3F => (format!("{:04X}: 10 3F        SWI2" , pc),2),
                0x26 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 26 {:02X} {:02X} LBNE <rel>", pc, hi, lo),4) }
                0x27 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 27 {:02X} {:02X} LBEQ <rel>", pc, hi, lo),4) }
                0x24 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 24 {:02X} {:02X} LBCC <rel>", pc, hi, lo),4) }
                0x25 => { let hi=b(2); let lo=b(3); (format!("{:04X}: 10 25 {:02X} {:02X} LBCS <rel>", pc, hi, lo),4) }
                0x83|0x93|0xB3 => (format!("{:04X}: 10 {:02X}        CMPD <various>", pc, sub),2),
                _ => (format!("{:04X}: 10 {:02X}        (unimpl prefix)" , pc, sub),2)
            }
        }
        0x11 => { let sub=b(1); (format!("{:04X}: 11 {:02X}        (prefix2)" , pc, sub),2) }
        0xBD => { let hi=b(1); let lo=b(2); (format!("{:04X}: BD {:02X} {:02X}    JSR ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x9D => { let off=b(1); (format!("{:04X}: 9D {:02X}       JSR <$DP{:02X}>", pc, off, off),2) }
        0x8D => { let off=b(1); (format!("{:04X}: 8D {:02X}       BSR ${:04X}", pc, off, ((pc+2) as isize + (off as i8 as isize)) as u16),2) }
        0x20|0x22|0x23|0x24|0x25|0x26|0x27|0x28|0x29|0x2A|0x2B|0x2C|0x2D|0x2E|0x2F => {
            let off=b(1) as i8; 
            let target = ((pc+2) as isize + off as isize) as u16; 
            let bname = match op {
                0x20 => "BRA", 0x22 => "BHI", 0x23 => "BLS", 0x24 => "BCC",
                0x25 => "BCS", 0x26 => "BNE", 0x27 => "BEQ", 0x28 => "BVC",
                0x29 => "BVS", 0x2A => "BPL", 0x2B => "BMI", 0x2C => "BGE",
                0x2D => "BLT", 0x2E => "BGT", 0x2F => "BLE", _ => "BR?"
            };
            (format!("{:04X}: {:02X} {:02X}       {} ${:04X}", pc, op, b(1), bname, target),2)
        }
        0xCC|0xCE|0x8E => { 
            let hi=b(1); let lo=b(2); 
            let mnem = match op {
                0xCC => "LDD",
                0xCE => "LDU",
                0x8E => "LDX", 
                _ => "LD?"
            }; 
            (format!("{:04X}: {:02X} {:02X} {:02X}    {} #${:02X}{:02X}", pc, op, hi, lo, mnem, hi, lo),3) 
        }
        0x86|0xC6 => { 
            let imm=b(1); 
            let mnem = if op==0x86 {"LDA"} else {"LDB"}; 
            (format!("{:04X}: {:02X} {:02X}       {} #${:02X}", pc, op, imm, mnem, imm),2) 
        }
        0x81|0xC1 => { 
            let imm=b(1); 
            let mnem = if op==0x81 {"CMPA"} else {"CMPB"}; 
            (format!("{:04X}: {:02X} {:02X}       {} #${:02X}", pc, op, imm, mnem, imm),2) 
        }
        0xB6|0xF6 => { 
            let hi=b(1); let lo=b(2); 
            let mnem = if op==0xB6 {"LDA"} else {"LDB"}; 
            (format!("{:04X}: {:02X} {:02X} {:02X}    {} ${:02X}{:02X}", pc, op, hi, lo, mnem, hi, lo),3) 
        }
        0xFC => { // LDD Extended (3 bytes)
            let hi=b(1); let lo=b(2);
            (format!("{:04X}: FC {:02X} {:02X}    LDD ${:02X}{:02X}", pc, hi, lo, hi, lo),3)
        }
        0xDC => { // LDD Direct Page (2 bytes)
            let off=b(1);
            (format!("{:04X}: DC {:02X}       LDD <$DP{:02X}>", pc, off, off),2)
        }
        0xB7|0xF7 => {
            let hi=b(1); let lo=b(2);
            let mnem = if op==0xB7 {"STA"} else {"STB"};
            (format!("{:04X}: {:02X} {:02X} {:02X}    {} ${:02X}{:02X}", pc, op, hi, lo, mnem, hi, lo),3)
        }
        0xFD => { // STD Extended (3 bytes)
            let hi=b(1); let lo=b(2);
            (format!("{:04X}: FD {:02X} {:02X}    STD ${:02X}{:02X}", pc, hi, lo, hi, lo),3)
        }
        0xBF => { // STX Extended (3 bytes)
            let hi=b(1); let lo=b(2);
            (format!("{:04X}: BF {:02X} {:02X}    STX ${:02X}{:02X}", pc, hi, lo, hi, lo),3)
        }
        0xFE => { // LDU Extended (3 bytes)
            let hi=b(1); let lo=b(2);
            (format!("{:04X}: FE {:02X} {:02X}    LDU ${:02X}{:02X}", pc, hi, lo, hi, lo),3)
        }
        0xDD => { // STD Direct Page (2 bytes)
            let off=b(1); 
            let mnem = "STD";
            (format!("{:04X}: {:02X} {:02X}       {} <$DP{:02X}>", pc, op, off, mnem, off),2) 
        }
        0x39 => (format!("{:04X}: 39           RTS", pc),1),
        0x3B => (format!("{:04X}: 3B           RTI", pc),1),
        0x3E => (format!("{:04X}: 3E           WAI", pc),1),
        0x4F => (format!("{:04X}: 4F           CLRA", pc),1),
        0x5F => (format!("{:04X}: 5F           CLRB", pc),1),
        0x7C => { let hi=b(1); let lo=b(2); (format!("{:04X}: 7C {:02X} {:02X}    INC ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x7A => { let hi=b(1); let lo=b(2); (format!("{:04X}: 7A {:02X} {:02X}    DEC ${:02X}{:02X}", pc, hi, lo, hi, lo),3) }
        0x85 => { let imm=b(1); (format!("{:04X}: 85 {:02X}       BITA #${:02X}", pc, imm, imm),2) }
        0xC4|0xC8|0x8A => { 
            let imm=b(1); 
            let mnem = match op {
                0xC4 => "ANDB",
                0xC8 => "EORB",
                0x8A => "ORA",
                _ => "???"
            };
            (format!("{:04X}: {:02X} {:02X}       {} #${:02X}", pc, op, imm, mnem, imm),2) 
        }
        0x17 => { let hi=b(1); let lo=b(2); (format!("{:04X}: 17 {:02X} {:02X}    LBSR <rel>", pc, hi, lo),3) }
        0x34 => { 
            let pb=b(1); 
            (format!("{:04X}: 34 {:02X}       PSHS (regs)", pc, pb),2) 
        }
        0x35 => { 
            let pb=b(1); 
            (format!("{:04X}: 35 {:02X}       PULS (regs)", pc, pb),2) 
        }
        0x3F => (format!("{:04X}: 3F           SWI", pc),1),
        0x33 => { 
            let pb=b(1); 
            (format!("{:04X}: 33 {:02X}       LEAU (postbyte)", pc, pb),2) 
        }
        0x30 => {
            let pb=b(1);
            (format!("{:04X}: 30 {:02X}       LEAX (indexed)", pc, pb),2)
        }
        0x32 => {
            let pb=b(1);
            (format!("{:04X}: 32 {:02X}       LEAS (indexed)", pc, pb),2)
        }
        0x1F => {
            let pb=b(1);
            (format!("{:04X}: 1F {:02X}       TFR", pc, pb),2)
        }
        0xA7|0xE7 => {
            let pb=b(1);
            let mnem = if op==0xA7 {"STA"} else {"STB"};
            (format!("{:04X}: {:02X} {:02X}       {} (indexed)", pc, op, pb, mnem),2)
        }
        0xED => {
            let pb=b(1);
            (format!("{:04X}: ED {:02X}       STD (indexed)", pc, pb),2)
        }
        0xAE|0xEE => {
            let pb=b(1);
            let mnem = if op==0xAE {"LDX"} else {"LDU"};
            (format!("{:04X}: {:02X} {:02X}       {} (indexed)", pc, op, pb, mnem),2)
        }
        _ => (format!("{:04X}: {:02X}           .db ${:02X}", pc, op, op),1)
    }
}

pub fn disassemble_range(data: &[u8], start: usize, count: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut pc = start;
    let end = (start + count).min(data.len());
    let mut last_was_ff = false;
    let mut ff_start = 0usize;
    let mut ff_count = 0usize;
    
    while pc < end { 
        let op = if pc < data.len() { data[pc] } else { 0 };
        
        // Detectar secuencias de FF (padding/datos no código)
        if op == 0xFF && pc + 1 < end && data.get(pc+1) == Some(&0xFF) {
            if !last_was_ff {
                ff_start = pc;
                ff_count = 0;
                last_was_ff = true;
            }
            ff_count += 1;
            pc += 1;
            continue;
        }
        
        // Si terminó secuencia de FF, mostrar resumen
        if last_was_ff && ff_count > 0 {
            result.push(format!("{:04X}-{:04X}: [FF padding - {} bytes]", ff_start, pc - 1, ff_count));
            last_was_ff = false;
            ff_count = 0;
        }
        
        let (line, adv) = disassemble_one(&data, pc); 
        result.push(line); 
        pc += adv;
    }
    
    // Si terminó con FF padding
    if last_was_ff && ff_count > 0 {
        result.push(format!("{:04X}-{:04X}: [FF padding - {} bytes]", ff_start, end - 1, ff_count));
    }
    
    result
}
