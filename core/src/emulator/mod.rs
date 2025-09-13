pub struct CPU {
    pub a: u8, pub b: u8, pub dp: u8, pub x: u16, pub u: u16, pub pc: u16,
    pub call_stack: Vec<u16>, pub cc_z: bool, pub cc_n: bool, pub cc_c: bool,
    pub mem: [u8;65536], pub trace: bool, pub bios_calls: Vec<String>,
    pub frame_count: u64, pub last_intensity: u8, pub draw_vl_count: u64,
    pub reset0ref_count: u64, pub print_str_count: u64, pub print_list_count: u64,
    pub moveto_count: u64, pub bios_present: bool, pub cycles: u64,
}

impl Default for CPU { fn default()->Self { CPU { a:0,b:0,dp:0xD0,x:0,u:0,pc:0,call_stack:Vec::new(),cc_z:false,cc_n:false,cc_c:false,mem:[0;65536],trace:false,bios_calls:Vec::new(),frame_count:0,last_intensity:0,draw_vl_count:0,reset0ref_count:0,print_str_count:0,print_list_count:0,moveto_count:0,bios_present:false,cycles:0 } } }

impl CPU {
    pub fn load_bin(&mut self, data:&[u8], base:u16) {
        for (i, b) in data.iter().enumerate() {
            let addr = base as usize + i;
            if addr < 65536 { self.mem[addr] = *b; }
        }
    }
    pub fn load_bios(&mut self,data:&[u8]){ if data.len()==8192 { self.load_bin(data,0xF000); self.bios_present=true; } }
    fn d(&self)->u16 { ((self.a as u16)<<8)|self.b as u16 }
    fn set_d(&mut self,v:u16){ self.a=(v>>8) as u8; self.b=v as u8; }
    fn update_nz16(&mut self,v:u16){ self.cc_z=v==0; self.cc_n=(v & 0x8000)!=0; }
    fn update_nz8(&mut self,v:u8){ self.cc_z=v==0; self.cc_n=(v & 0x80)!=0; }
    fn record_bios_call(&mut self, addr:u16) {
        let name = match addr {
            0xF192 => { // WAIT_RECAL
                self.dp = 0xD0; // BIOS leaves DP=$D0
                self.frame_count += 1;
                "WAIT_RECAL"
            },
            0xF2A5 => { // INTENSITY_5F
                self.last_intensity = 0x5F;
                "INTENSITY_5F"
            },
            0xF2AB => { // INTENSITY_A
                self.last_intensity = self.a;
                "INTENSITY_A"
            },
            0xF37A => { self.print_str_count += 1; "PRINT_STR_D" },
            0xF38A => { self.print_list_count += 1; "PRINT_LIST" },
            0xF38C => "PRINT_LIST_CHK",
            0xF312 => { self.moveto_count += 1; "MOVETO_D" },
            0xF354 => { self.reset0ref_count += 1; "RESET0REF" },
            0xF1AF => { self.dp = 0xC8; "DP_TO_C8" },
            0xF3DD => { self.draw_vl_count += 1; "DRAW_VL" },
            0xFD0D => "MUSIC1",
            _ => "BIOS_UNKNOWN",
        };
        self.bios_calls.push(format!("{:04X}:{}", addr, name));
        if self.trace { println!("[BIOS CALL] {}", name); }
    }
    pub fn step(&mut self)->bool{
        let pc0=self.pc; let op=self.mem[self.pc as usize];
        if self.trace { print!("{:04X}: {:02X} ", self.pc, op); }
        self.pc=self.pc.wrapping_add(1); let mut cyc=1u32;
        match op {
            // LDD immediate
            0xCC => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; self.set_d(((hi as u16)<<8)|lo as u16); self.update_nz16(self.d()); if self.trace { println!("LDD #${:04X}", self.d()); } cyc=3; }
            // LDD extended
            0xFC => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; let ah=self.mem[addr as usize]; let al=self.mem[addr.wrapping_add(1) as usize]; self.set_d(((ah as u16)<<8)|al as u16); self.update_nz16(self.d()); if self.trace { println!("LDD ${:04X} -> ${:04X}", addr, self.d()); } cyc=5; }
            // SUBD immediate / direct / extended
            0x83|0x93|0xB3 => {
                let val = match op { 0x83 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; ((hi as u16)<<8)|lo as u16 }, 0x93 => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; ((self.mem[addr as usize] as u16)<<8)|self.mem[addr.wrapping_add(1) as usize] as u16 }, _ => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; ((self.mem[addr as usize] as u16)<<8)|self.mem[addr.wrapping_add(1) as usize] as u16 } };
                let d=self.d(); let res=d.wrapping_sub(val); self.set_d(res); self.update_nz16(res); if self.trace { println!("SUBD {:04X} -> {:04X}", val, res); }
            }
            // ANDA direct / extended
            0x94|0xB4 => { let v = if op==0x94 { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize] } else { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize] }; self.a &= v; self.update_nz8(self.a); if self.trace { println!("ANDA -> {:02X}", self.a); } }
            // ANDB direct / extended
            0xD4|0xF4 => { let v = if op==0xD4 { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize] } else { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize] }; self.b &= v; self.update_nz8(self.b); if self.trace { println!("ANDB -> {:02X}", self.b); } }
            // ORA immediate
            0x8A => { let v=self.mem[self.pc as usize]; self.pc+=1; self.a |= v; self.update_nz8(self.a); if self.trace { println!("ORA #${:02X} -> {:02X}", v, self.a);} }
            // LDA immediate
            0x86 => { let v=self.mem[self.pc as usize]; self.pc+=1; self.a=v; self.update_nz8(self.a); if self.trace { println!("LDA #${:02X}", self.a);} }
            // LDB immediate
            0xC6 => { let v=self.mem[self.pc as usize]; self.pc+=1; self.b=v; self.update_nz8(self.b); if self.trace { println!("LDB #${:02X}", self.b);} }
            // LDX extended (0xBE)
            0xBE => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; let vh=self.mem[addr as usize]; let vl=self.mem[addr.wrapping_add(1) as usize]; self.x = ((vh as u16)<<8)|vl as u16; if self.trace { println!("LDX ${:04X} -> {:04X}", addr, self.x);} }
            // LDU immediate (0xCE)
            0xCE => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; self.u = ((hi as u16)<<8)|lo as u16; if self.trace { println!("LDU #${:04X}", self.u);} }
            // LDU extended (0xFE)
            0xFE => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; let uh=self.mem[addr as usize]; let ul=self.mem[addr.wrapping_add(1) as usize]; self.u=((uh as u16)<<8)|ul as u16; if self.trace { println!("LDU ${:04X} -> {:04X}", addr, self.u);} }
        // JSR extended (BIOS intercept if addr>=F000)
        0xBD => {
            let hi = self.mem[self.pc as usize];
            let lo = self.mem[self.pc as usize + 1];
            self.pc += 2;
            let addr = ((hi as u16) << 8) | lo as u16;
            if self.trace { println!("JSR ${:04X}", addr); }
            if addr >= 0xF000 {
                if !self.bios_present {
                    if self.trace { println!("Missing BIOS for call ${:04X}", addr); }
                    return false;
                }
                // intercept: simulate call (no push)
                self.record_bios_call(addr);
            } else {
                let ret = self.pc;
                self.call_stack.push(ret);
                self.pc = addr;
            }
            cyc = 7;
        }
        // JSR direct (BIOS intercept)
        0x9D => {
            let off = self.mem[self.pc as usize];
            self.pc += 1;
            let addr = ((self.dp as u16) << 8) | off as u16;
            if self.trace { println!("JSR ${:04X} (direct)", addr); }
            if addr >= 0xF000 {
                if !self.bios_present {
                    if self.trace { println!("Missing BIOS for call ${:04X}", addr); }
                    return false;
                }
                self.record_bios_call(addr);
            } else {
                let ret = self.pc;
                self.call_stack.push(ret);
                self.pc = addr;
            }
            cyc = 7;
        }
            // RTS
            0x39 => { if let Some(r)=self.call_stack.pop(){ if self.trace { println!("RTS -> {:04X}", r);} self.pc=r; } else if self.trace { println!("RTS (empty stack)"); } cyc=5; }
            0x4F => { self.a=0; self.update_nz8(self.a); if self.trace { println!("CLRA"); } }
            0x5F => { self.b=0; self.update_nz8(self.b); if self.trace { println!("CLRB"); } }
            // LDA direct
            0x96 => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.a=self.mem[addr as usize]; self.update_nz8(self.a); if self.trace { println!("LDA ${:04X}", addr);} }
            // STA direct
            0x97 => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize]=self.a; self.update_nz8(self.a); if self.trace { println!("STA ${:04X}", addr);} }
            // STX direct
            0x9F => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize]=(self.x>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=self.x as u8; if self.trace { println!("STX ${:04X}", addr);} }
            // SUBA immediate / direct
            0x80|0x90 => { let val = if op==0x80 { let v=self.mem[self.pc as usize]; self.pc+=1; v } else { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize] }; let res = self.a.wrapping_sub(val); self.a=res; self.update_nz8(self.a); if self.trace { println!("SUBA {:02X} -> {:02X}", val, self.a);} }
            // SUBA extended
            0xB0 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; let val=self.mem[addr as usize]; let res=self.a.wrapping_sub(val); self.a=res; self.update_nz8(self.a); if self.trace { println!("SUBA ${:04X} ({:02X}) -> {:02X}", addr, val, self.a);} }
            // CMPA immediate
            0x81 => { let val=self.mem[self.pc as usize]; self.pc+=1; let res=self.a.wrapping_sub(val); self.cc_z=res==0; self.cc_n=(res & 0x80)!=0; if self.trace { println!("CMPA #${:02X} (res {:02X})", val, res);} }
            // LDB direct
            0xD6 => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.b=self.mem[addr as usize]; self.update_nz8(self.b); if self.trace { println!("LDB ${:04X}", addr);} }
            // STB direct
            0xD7 => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize]=self.b; self.update_nz8(self.b); if self.trace { println!("STB ${:04X}", addr);} }
            // STU direct (0xDF)
            0xDF => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; self.mem[addr as usize]=(self.u>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=self.u as u8; if self.trace { println!("STU ${:04X}", addr);} }
            // LDA extended
            0xB6 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.a=self.mem[addr as usize]; self.update_nz8(self.a); if self.trace { println!("LDA ${:04X}", addr);} }
            // STA extended
            0xB7 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize]=self.a; self.update_nz8(self.a); if self.trace { println!("STA ${:04X}", addr);} }
            // LDB extended
            0xF6 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.b=self.mem[addr as usize]; self.update_nz8(self.b); if self.trace { println!("LDB ${:04X}", addr);} }
            // STB extended
            0xF7 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize]=self.b; self.update_nz8(self.b); if self.trace { println!("STB ${:04X}", addr);} }
            // STD direct
            0xDD => { let off=self.mem[self.pc as usize]; self.pc+=1; let addr=((self.dp as u16)<<8)|off as u16; let d=self.d(); self.mem[addr as usize]=(d>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=d as u8; self.update_nz16(d); if self.trace { println!("STD ${:04X}", addr);} }
            // LDX immediate
            0x8E => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; self.x=((hi as u16)<<8)|lo as u16; if self.trace { println!("LDX #${:04X}", self.x);} }
            // STD extended
            0xFD => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; let d=self.d(); self.mem[addr as usize]=(d>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=d as u8; self.update_nz16(d); if self.trace { println!("STD ${:04X}", addr);} }
            // STU extended (0xFF)
            0xFF => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize]=(self.u>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=self.u as u8; if self.trace { println!("STU ${:04X}", addr);} }
            // STX extended (0xBF)
            0xBF => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let addr=((hi as u16)<<8)|lo as u16; self.mem[addr as usize]=(self.x>>8) as u8; self.mem[addr.wrapping_add(1) as usize]=self.x as u8; if self.trace { println!("STX ${:04X}", addr);} }
            // STX indexed placeholder (0xAF) consume postbyte only
            0xAF => { let post=self.mem[self.pc as usize]; self.pc+=1; if self.trace { println!("STX (idx post={:02X}) [ignored]", post);} }
            // BRA short
            0x20 => { let off=self.mem[self.pc as usize] as i8; self.pc+=1; let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BRA {:04X}", new);} self.pc=new; }
            // BEQ short
            0x27 => { let off=self.mem[self.pc as usize] as i8; self.pc+=1; if self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BEQ taken {:04X}", new);} self.pc=new; } else if self.trace { println!("BEQ not"); } }
            // BNE short
            0x26 => { let off=self.mem[self.pc as usize] as i8; self.pc+=1; if !self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BNE taken {:04X}", new);} self.pc=new; } else if self.trace { println!("BNE not"); } }
            // BLE short (simplified using N or Z)
            0x2F => { let off=self.mem[self.pc as usize] as i8; self.pc+=1; if self.cc_n || self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BLE taken {:04X}", new);} self.pc=new; } else if self.trace { println!("BLE not"); } }
            // BGE short (simplified !N)
            0x2C => { let off=self.mem[self.pc as usize] as i8; self.pc+=1; if !self.cc_n { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BGE taken {:04X}", new);} self.pc=new; } else if self.trace { println!("BGE not"); } }
            // LBRA
            0x16 => { let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let off = ((hi as u16)<<8)|lo as u16; let new = self.pc.wrapping_add(off as i16 as u16); if self.trace { println!("LBRA {:04X}", new);} self.pc=new; }
            // Long branch prefix 0x10
            0x10 => { let bop=self.mem[self.pc as usize]; self.pc+=1; let hi=self.mem[self.pc as usize]; let lo=self.mem[self.pc as usize+1]; self.pc+=2; let off=((hi as u16)<<8)|lo as u16; let target = self.pc.wrapping_add(off as i16 as u16); match bop { 0x26 => { if !self.cc_z { if self.trace { println!("LBNE {:04X}", target);} self.pc=target; } else if self.trace { println!("LBNE not"); } }, 0x27 => { if self.cc_z { if self.trace { println!("LBEQ {:04X}", target);} self.pc=target; } else if self.trace { println!("LBEQ not"); } }, _ => { if self.trace { println!("UNIMPL LONG BR {:02X}", bop);} return false; } } }
            // ORCC (ignored)
            0x1A => { self.pc+=1; if self.trace { println!("ORCC (ignored)"); } }
            op => { if self.trace { println!("UNIMPL OP {:02X} at {:04X}", op, pc0);} return false; }
        }
        self.cycles += cyc as u64; true
    }
    pub fn run(&mut self,max_steps:usize){ for _ in 0..max_steps { if !self.step() { if self.trace { println!("Stopped at {:04X}", self.pc);} break; } } }
}
