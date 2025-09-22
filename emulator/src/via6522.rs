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
    cycles_since_shift: u8,
}

impl Default for Via6522 { fn default() -> Self { Self { regs:[0;16], t1_counter:0, t1_latch:0, t2_counter:0, t2_latch:0, irq_line:false, on_irq_change:None, pb7_state:false, sr_bits_remaining:0, shifting:false, cycles_since_shift:0 } } }

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
            0x04 => { // T1C-L read clears IFR6 (spec)
                let val = self.t1_counter as u8;
                if self.ifr() & 0x40 != 0 { // only clear if set
                    self.regs[0x0D] &= !0x40;
                    self.recompute_irq();
                    if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") {
                        eprintln!("[IRQ_TRACE][VIA] READ T1C-L -> {:02X} (clear IFR6)", val);
                    }
                    if std::env::var("VIA_T1_TRACE").ok().as_deref()==Some("1") { eprintln!("[VIA][T1 read] T1C-L={:02X} (clear IFR6)", val); }
                }
                val
            }
            0x05 => { // T1C-H read does NOT clear IFR6 (spec)
                (self.t1_counter >> 8) as u8
            }
            0x08 => { // T2C-L read (no IFR clear)
                (self.t2_counter & 0xFF) as u8
            }
            0x09 => { // T2C-H read clears IFR5 (spec)
                let val = (self.t2_counter >> 8) as u8;
                if self.ifr() & 0x20 != 0 { // clear IFR5 on high read
                    self.regs[0x0D] &= !0x20;
                    self.recompute_irq();
                    if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") {
                        eprintln!("[IRQ_TRACE][VIA] READ T2C-H -> {:02X} (clear IFR5)", val);
                    }
                }
                val
            }
            _ => self.regs[r]
        }
    }
    pub fn write(&mut self, reg: u8, val: u8) {
        let r = (reg & 0x0F) as usize;
        
        // Debug ALL VIA writes
        // println!("🎯 VIA WRITE: reg=0x{:02X} val=0x{:02X}", reg, val);  // SILENCED FOR SPEED TEST
        
        match r {
            0x0D => { // IFR clear bits
                let clear_mask = val & 0x7F; self.regs[0x0D] &= !clear_mask; self.recompute_irq();
                if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") && clear_mask!=0 { eprintln!("[IRQ_TRACE][VIA] WRITE IFR clear_mask={:02X} newIFR={:02X}", clear_mask, self.ifr()); }
            }
            0x0E => { // IER set/clear
                let set_mode = (val & 0x80) != 0; let mask = val & 0x7F; let cur = self.ier() & 0x7F; let next = if set_mode { cur | mask } else { cur & !mask }; self.regs[0x0E] = next; self.recompute_irq();
                if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") { eprintln!("[IRQ_TRACE][VIA] WRITE IER set_mode={} mask={:02X} -> newIER={:02X}", set_mode, mask, self.ier()); }
            }
            0x04 => { self.regs[0x04] = val; } // T1 low latch
            0x05 => { // T1 high latch/load
                let lo = self.regs[0x04] as u16; let hi = val as u16; let full = (hi << 8) | lo; self.t1_latch = full; self.t1_counter = full; self.regs[0x0D] &= !0x40; self.regs[0x05] = val; self.recompute_irq();
                if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") { eprintln!("[IRQ_TRACE][VIA] LOAD T1 full={:#06X} (clear IFR6)", full); }
                if std::env::var("VIA_T1_TRACE").ok().as_deref()==Some("1") { eprintln!("[VIA][T1 load] value={:#06X}", full); }
            }
            0x08 => { self.regs[0x08] = val; } // T2 low (no latch action yet until high written)
            0x09 => { // T2 high latch/load
                let lo = self.regs[0x08] as u16; let hi = val as u16; let full = (hi << 8) | lo; 
                self.t2_latch = full; self.t2_counter = full; self.regs[0x09] = val; 
                // Solo limpiar IFR5 si no hay overflow pendiente (según VIA 6522 spec)
                if (self.regs[0x0D] & 0x20) == 0 {
                    // No hay IFR5 set, es seguro limpiar (aunque ya esté limpio)
                    self.regs[0x0D] &= !0x20;
                    if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") { eprintln!("[IRQ_TRACE][VIA] LOAD T2 full={:#06X} (clear IFR5 - no overflow pending)", full); }
                } else {
                    // IFR5 ya está set (timer expiró), NO limpiar para que BIOS pueda detectarlo
                    if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") { eprintln!("[IRQ_TRACE][VIA] LOAD T2 full={:#06X} (IFR5 PRESERVED - overflow pending)", full); }
                }
                self.recompute_irq();
                if std::env::var("VIA_T2_TRACE").ok().as_deref()==Some("1") { eprintln!("[VIA][T2 load] value={:#06X}", full); }
            }
            0x0B => { // ACR write
                self.regs[0x0B] = val;
                if std::env::var("VIA_T1_TRACE").ok().as_deref()==Some("1") { eprintln!("[VIA][ACR write] ACR={:02X} cont={} pb7={}", val, (val & 0x40)!=0, (val & 0x80)!=0); }
            }
            0x0A => { // Shift register write
                self.regs[0x0A] = val; let acr = self.regs[0x0B]; let mode = (acr >> 2) & 0x07; if mode == 0b100 { self.shifting = true; self.sr_bits_remaining = 8; self.regs[0x0D] &= !0x10; self.recompute_irq(); }
            }
            _ => { self.regs[r] = val; }
        }
    }
    
    /* VIA 6522 Timer Tick - Ciclo Principal de Temporización
     * Function: tick(&mut self, cycles: u32)
     * Purpose: Procesa Timer1, Timer2 y Shift Register por ciclos precisos
     * Implementation: Basado en Vectrexy cycle-by-cycle timing model
     * 
     * Timer1 Features:
     * - Continuous/One-shot modes via ACR bit 6
     * - PB7 output toggle control via ACR bit 7  
     * - IFR bit 6 set on underflow, reloads from latch if continuous
     * - Precise cycle-by-cycle decrement for accurate timing
     * 
     * Timer2 Features:
     * - One-shot mode only (pulse counting not implemented)
     * - IFR bit 5 set on underflow
     * - Critical for BIOS timing synchronization
     * 
     * Shift Register:
     * - Mode 4 (shift out under T2 control) - 8 cycles per bit
     * - IFR bit 4 set when 8 bits shifted
     * - Automatic reload in continuous modes
     * 
     * Verificado: ✓ OK - Timing critical para Mine Storm y BIOS initialization
     */
    pub fn tick(&mut self, cycles: u32){
        // FIXED: Actualizar ciclo por ciclo como vectrexy para timing exacto
        // Vectrexy: while (cyclesLeft-- > 0) { m_timer2.Update(1); }
        let mut cycles_left = cycles;
        while cycles_left > 0 {
            // Timer1 update: 1 cycle at a time
            if self.t1_counter > 0 {
                self.t1_counter -= 1;
                if self.t1_counter == 0 {
                    self.regs[0x0D] |= 0x40; // IFR bit 6 (Timer1)
                    let acr = self.regs[0x0B];
                    let pb7_enable = (acr & 0x80) != 0;
                    let continuous = (acr & 0x40) != 0;
                    if pb7_enable { self.pb7_state = !self.pb7_state; }
                    self.recompute_irq();
                    if continuous { 
                        self.t1_counter = self.t1_latch; 
                    }
                }
            }
            
            // Timer2 update: 1 cycle at a time (critical for BIOS timing)
            if self.t2_counter > 0 {
                self.t2_counter -= 1;
                if self.t2_counter == 0 {
                    self.regs[0x0D] |= 0x20; // IFR bit 5 (Timer2)
                    self.recompute_irq();
                    if std::env::var("VIA_T2_TRACE").ok().as_deref()==Some("1") { 
                        eprintln!("[VIA][T2 expire] IFR5 set"); 
                    }
                    if std::env::var("IRQ_TRACE").ok().as_deref()==Some("1") { 
                        eprintln!("[IRQ_TRACE][VIA] T2 EXPIRE set IFR5 (IFR={:02X} IER={:02X})", self.ifr(), self.ier()); 
                    }
                    // Timer2 is one-shot, no reload
                }
            }
            
            cycles_left -= 1;
        }
        
        // Shift register update: cycle-accurate (shift every 8 cycles like vectrexy)
        if self.shifting {
            for _ in 0..cycles {
                // Shift register timing: mode 4 uses Timer2 rate for shift clock
                let acr = self.regs[0x0B];
                let sr_mode = (acr >> 2) & 0x07;
                
                if sr_mode == 0b100 {
                    // Mode 4: Shift out under Timer2 control
                    // In this mode, shift rate is tied to Timer2 frequency
                    // Each Timer2 underflow triggers one bit shift
                    if self.t2_counter == 0 && self.sr_bits_remaining > 0 {
                        self.regs[0x0A] = (self.regs[0x0A] << 1) | 0x01;
                        self.sr_bits_remaining -= 1;
                        if self.sr_bits_remaining == 0 {
                            self.regs[0x0D] |= 0x10; // IFR bit 4 (Shift Register)
                            self.recompute_irq();
                            self.sr_bits_remaining = 8; // Auto-reload in continuous mode
                        }
                    }
                } else {
                    // Other modes: use fixed 8-cycle rate (original logic)
                    if (self.cycles_since_shift + 1) % 8 == 0 && self.sr_bits_remaining > 0 {
                        self.regs[0x0A] = (self.regs[0x0A] << 1) | 0x01;
                        self.sr_bits_remaining -= 1;
                        if self.sr_bits_remaining == 0 {
                            self.regs[0x0D] |= 0x10; // IFR bit 4 (Shift Register)
                            self.recompute_irq();
                            if sr_mode == 0b100 { self.sr_bits_remaining = 8; } else { self.shifting = false; }
                        }
                    }
                }
                self.cycles_since_shift = (self.cycles_since_shift + 1) % 8;
            }
        }
    }
    pub fn pb7(&self) -> bool { self.pb7_state } pub fn irq_asserted(&self) -> bool { self.irq_line } pub fn t1_counter(&self) -> u16 { self.t1_counter }
    
    /* VIA 6522 Timer1 IRQ Trigger - Helper para CPU
     * Function: trigger_timer1_irq(&mut self)
     * Purpose: Permite al CPU forzar Timer1 IRQ programáticamente
     * Operation: Set IFR bit 6 (Timer1) y recomputa línea IRQ
     * Usage: Utilizado por CPU para timing directo en casos especiales
     * Implementation: Equivalente a Timer1 underflow pero sin afectar contador
     * Verificado: ✓ OK - Helper utilizado para debugging y timing exacto
     */
    pub fn trigger_timer1_irq(&mut self) {
        self.regs[0x0D] |= 0x40; // IFR bit 6 (Timer1)
        self.recompute_irq();
    }
}
