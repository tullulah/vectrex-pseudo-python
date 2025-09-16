// Moved from core crate: VIA 6522 skeleton
pub struct Via6522 {
    regs: [u8;16],
    t1_counter: u16,
    t1_latch: u16,
    t2_counter: u16,
    t2_latch: u16,
    irq_line: bool,
    pub on_irq_change: Option<Box<dyn Fn(bool) -> ()>>,
    pb7_state: bool,
    sr_bits_remaining: u8,
    shifting: bool,
}

impl Default for Via6522 { fn default() -> Self { Self { regs:[0;16], t1_counter:0, t1_latch:0, t2_counter:0, t2_latch:0, irq_line:false, on_irq_change:None, pb7_state:false, sr_bits_remaining:0, shifting:false } } }

impl Via6522 {
    pub fn new() -> Self { Self::default() }
    pub fn set_irq_callback<F: 'static + Fn(bool)>(&mut self, f:F){ self.on_irq_change = Some(Box::new(f)); }
    fn ifr(&self) -> u8 { self.regs[0x0D] }
    fn ier(&self) -> u8 { self.regs[0x0E] }
    pub fn raw_ifr(&self) -> u8 { self.ifr() }
    pub fn raw_ier(&self) -> u8 { self.ier() }
    fn recompute_irq(&mut self){ let ifr_flags = self.ifr() & 0x7F; let ier_mask = self.ier() & 0x7F; let pending = (ifr_flags & ier_mask) != 0; if pending != self.irq_line { self.irq_line = pending; if let Some(cb) = &self.on_irq_change { cb(pending); } } }
    pub fn read(&mut self, reg: u8) -> u8 {
        let r = (reg & 0x0F) as usize;
        match r {
            0x0D => { // IFR
                let base = self.ifr() & 0x7F;
                let ier = self.ier() & 0x7F;
                let master = if (base & ier) != 0 { 0x80 } else { 0x00 };
                base | master
            }
            0x0E => { // IER
                (self.ier() & 0x7F) | 0x80
            }
            0x04 => { // T1C-L read clears IFR6
                let val = self.t1_counter as u8;
                self.regs[0x0D] &= !0x40;
                self.recompute_irq();
                val
            }
            0x05 => { // T1C-H (also clear IFR6; some code reads high first)
                let val = (self.t1_counter >> 8) as u8;
                if self.ifr() & 0x40 != 0 { self.regs[0x0D] &= !0x40; self.recompute_irq(); }
                val
            }
            0x08 => { // T2C-L (does NOT clear IFR5 per 6522 spec)
                (self.t2_counter & 0xFF) as u8
            }
            0x09 => { // T2C-H read clears IFR5
                let val = (self.t2_counter >> 8) as u8;
                if self.ifr() & 0x20 != 0 { // only touch if set
                    self.regs[0x0D] &= !0x20;
                    self.recompute_irq();
                }
                val
            }
            _ => self.regs[r]
        }
    }
    pub fn write(&mut self, reg: u8, val: u8) { let r = (reg & 0x0F) as usize; match r { 0x0D => { let clear_mask = val & 0x7F; self.regs[0x0D] &= !clear_mask; self.recompute_irq(); }, 0x0E => { let set_mode = (val & 0x80) != 0; let mask = val & 0x7F; let cur = self.ier() & 0x7F; let next = if set_mode { cur | mask } else { cur & !mask }; self.regs[0x0E] = next; self.recompute_irq(); }, 0x04 => { self.regs[0x04] = val; }, 0x05 => { let lo = self.regs[0x04] as u16; let hi = val as u16; let full = (hi << 8) | lo; self.t1_latch = full; self.t1_counter = full; self.regs[0x0D] &= !0x40; self.regs[0x05] = val; self.recompute_irq(); }, 0x08 => { self.regs[0x08] = val; }, 0x09 => { let lo = self.regs[0x08] as u16; let hi = val as u16; let full = (hi << 8) | lo; self.t2_latch = full; self.t2_counter = full; self.regs[0x0D] &= !0x20; self.regs[0x09] = val; self.recompute_irq(); }, 0x0A => { self.regs[0x0A] = val; let acr = self.regs[0x0B]; let mode = (acr >> 2) & 0x07; if mode == 0b100 { self.shifting = true; self.sr_bits_remaining = 8; self.regs[0x0D] &= !0x10; self.recompute_irq(); } }, _ => { self.regs[r] = val; } } }
    pub fn tick(&mut self, cycles: u32){
        if self.t1_counter > 0 {
            let mut remaining_cycles = cycles as u32;
            while remaining_cycles > 0 && self.t1_counter > 0 {
                let step = remaining_cycles.min(self.t1_counter as u32);
                self.t1_counter -= step as u16;
                remaining_cycles -= step;
                if self.t1_counter == 0 {
                    self.regs[0x0D] |= 0x40;
                    let acr = self.regs[0x0B];
                    // ACR bits for Timer1 per 6522 spec:
                    //  bit7 = PB7 output enable (toggle PB7 on each underflow when set)
                    //  bit6 = Timer1 mode (1 = free-run/continuous reload, 0 = one-shot)
                    let pb7_enable = (acr & 0x80) != 0;
                    let continuous = (acr & 0x40) != 0;
                    if pb7_enable { self.pb7_state = !self.pb7_state; }
                    self.recompute_irq();
                    if continuous { self.t1_counter = self.t1_latch; } else { break; }
                }
            }
        }
        if self.t2_counter > 0 {
            if cycles as u16 >= self.t2_counter {
                self.t2_counter = 0;
                self.regs[0x0D] |= 0x20;
                self.recompute_irq();
            } else {
                self.t2_counter -= cycles as u16;
            }
        }
        // Optional legacy hack removed: allow BIOS to explicitly clear IFR5 by reading T2C-H
        if self.shifting {
            let mut bits_advance = (cycles / 8) as u8;
            while bits_advance > 0 && self.sr_bits_remaining > 0 {
                self.regs[0x0A] = (self.regs[0x0A] << 1) | 0x01;
                self.sr_bits_remaining -= 1;
                bits_advance -= 1;
                if self.sr_bits_remaining == 0 {
                    self.regs[0x0D] |= 0x10;
                    self.recompute_irq();
                    let acr = self.regs[0x0B];
                    let mode = (acr >> 2) & 0x07;
                    if mode == 0b100 { self.sr_bits_remaining = 8; break; } else { self.shifting = false; }
                }
            }
        }
    }
    pub fn pb7(&self) -> bool { self.pb7_state } pub fn irq_asserted(&self) -> bool { self.irq_line } pub fn t1_counter(&self) -> u16 { self.t1_counter }
}
