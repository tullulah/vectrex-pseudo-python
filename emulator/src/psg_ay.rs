//! AY-3-8912 Programmable Sound Generator (Scaffold Fase 1)
//!
//! Política: no introducir heurísticas sintéticas de música ni shortcuts no basados en registros reales.
//! Esta primera fase implementa solamente:
//! - Registro de 16 bytes
//! - Divisores de tono para 3 canales (A,B,C) y toggling de onda cuadrada
//! - LFSR de ruido con periodo configurable
//! - Mezcla lineal simple amplitud 0..15 (sin curva log todavía, se documentará cuando se añada)
//! - Acumulación de ciclos y producción de muestras PCM enteras (i16) a sample_rate fijo
//!
//! Pendiente fases siguientes (ver SUPER_SUMMARY sección 33.1 / 33.2):
//! - Envolvente (regs 11-13)
//! - Curva log de amplitud + tabla
//! - Integración con bus / API WASM (write_reg indirecto)
//! - Ring buffer compartido WASM y export incremental
//! - Métricas expuestas (toggles, shifts, samples)
//! - Validación de clock real AY en Vectrex (const provisional AY_CLOCK_HZ = CPU_CLOCK)
//!
//! Notas:
//! - El periodo efectivo del AY clásico: tone_period = (coarse<<8 | fine); si 0 => se trata como 1.
//!   La salida alterna estado cuando un contador decrementa desde period a 0.
//! - Ruido: LFSR tap típica bits 0 y 3 (polinomio 1 + x^4 + x^5 + x^17 variantes). Usamos 17-bit estándar.
//! - Esta implementación busca ser determinista: no usa RNG externo.

#[derive(Default)]
pub struct AyPsg {
    pub regs: [u8; 16],
    // Tone channel divisors & counters
    tone_period: [u16;3],
    tone_counter: [u16;3],
    tone_out: [bool;3],
    // Noise
    noise_period: u8,
    noise_counter: u8,
    lfsr: u32, // 17-bit LFSR (lower 17 bits used)
    noise_out: bool,
    // Envelope (fase 2)
    env_period: u16,      // periodo base (1..65535)
    env_counter: u16,     // cuenta decreciente hasta 0 -> siguiente paso
    env_step: u8,         // 0..15 índice de nivel actual
    env_active: bool,     // se solicitó (flag canales con bit 4)
    env_dir_up: bool,     // dirección actual (true ascendente)
    env_hold: bool,       // congelado (no más pasos)
    env_shape: u8,        // copia reg 13 (bits shape)
    // Mixing / sample gen
    sample_rate: u32,
    cycle_accum: u64,
    cycles_per_sample: u64,
    // PCM ring buffer
    pcm: Vec<i16>,
    pcm_write: usize,
    pcm_mask: usize,
    // Metrics (export futura)
    pub metric_tone_toggles: u64,
    pub metric_noise_shifts: u64,
    pub metric_samples: u64,
    pub envelope_requested_count: u64,
    pub metric_env_steps: u64,
    // Staging lineal para export (se rellena bajo demanda; no se actualiza cada tick para evitar coste)
    export_staging: Vec<i16>,
    export_serial: u64,
    // Delta export state
    last_export_write: usize, // posición de escritura en el ring en el último prepare (full o delta)
    delta_staging: Vec<i16>,  // buffer staging para delta actual
    delta_overflow: bool,     // true si el delta excede la capacidad (o envolvió más veces que el ring)
    // PSG Control Interface (BC1/BDIR from VIA Port B)
    bc1: bool,               // Bus Control 1 (Port B bit 3)
    bdir: bool,              // Bus Direction (Port B bit 4) 
    address_reg: u8,         // Current address register (0-15)
    data_bus: u8,            // Current data bus value (from Port A)
}

impl AyPsg {
    pub fn new(clock_hz: u32, sample_rate: u32, ring_pow2: u32) -> Self {
        let size = 1usize << ring_pow2.min(18); // máx 256K samples ring
        let cycles_per_sample = (clock_hz as u64).max(1) / (sample_rate as u64).max(1);
        Self {
            regs: [0;16],
            tone_period: [1;3],
            tone_counter: [1;3],
            tone_out: [false;3],
            noise_period: 1,
            noise_counter: 1,
            lfsr: 0x1FFFF, // estado inicial no cero
            noise_out: false,
            env_period: 1,
            env_counter: 1,
            env_step: 0,
            env_active: false,
            env_dir_up: true,
            env_hold: false,
            env_shape: 0,
            sample_rate,
            cycle_accum: 0,
            cycles_per_sample: cycles_per_sample.max(1),
            pcm: vec![0; size],
            pcm_write: 0,
            pcm_mask: size - 1,
            metric_tone_toggles: 0,
            metric_noise_shifts: 0,
            metric_samples: 0,
            envelope_requested_count: 0,
            metric_env_steps: 0,
            export_staging: Vec::new(),
            export_serial: 0,
            last_export_write: 0,
            delta_staging: Vec::new(),
            delta_overflow: false,
            bc1: false,
            bdir: false,
            address_reg: 0,
            data_bus: 0,
        }
    }

    /* PSG Control Interface - BC1/BDIR State Control
     * Function: set_bc1_bdir(bc1: bool, bdir: bool, data: u8)
     * Purpose: Control PSG via VIA Port B bits 3-4 y Port A data bus
     * States: 00=INACTIVE, 01=LATCH ADDRESS, 11=LATCH DATA, 10=READ DATA
     * Implementation: Basado en Vectrexy PSG control logic
     * Verificado: ✓ Pending - Implementación inicial
     */
    pub fn set_bc1_bdir(&mut self, bc1: bool, bdir: bool, data: u8) {
        let prev_bc1 = self.bc1;
        let prev_bdir = self.bdir;
        self.bc1 = bc1;
        self.bdir = bdir;
        self.data_bus = data;
        
        // State machine: only act on transitions to specific states
        match (bdir, bc1) {
            (false, false) => {
                // INACTIVE state - no action
            }
            (false, true) => {
                // LATCH ADDRESS - store data as address register
                if !prev_bc1 || prev_bdir {  // transition into this state
                    self.address_reg = data & 0x0F;  // only 16 registers
                }
            }
            (true, true) => {
                // LATCH DATA - write data to current address register  
                if prev_bc1 != bc1 || prev_bdir != bdir {  // transition into this state
                    self.write_reg(self.address_reg, data);
                }
            }
            (true, false) => {
                // READ DATA - read from current address register (not implemented for now)
                // En el Vectrex, esto se usa para leer joystick data, pero no es crítico para audio
            }
        }
    }

    pub fn bc1(&self) -> bool { self.bc1 }
    pub fn bdir(&self) -> bool { self.bdir }

    pub fn write_reg(&mut self, idx: u8, val: u8) {
        let i = (idx & 0x0F) as usize;
        self.regs[i] = val;
        match i {
            0|1|2|3|4|5 => self.recompute_tone_periods(),
            6 => { let p = val & 0x1F; self.noise_period = if p==0 {1} else {p}; },
            7 => {/* mixer mask handled in mix */},
            8|9|10 => {/* amplitude handled on mix */},
            11|12 => { self.env_period = (((self.regs[12] as u16) <<8) | self.regs[11] as u16).max(1); self.restart_envelope_if_used(); },
            13 => { // shape write reinicia ciclo según spec
                self.env_shape = val & 0x0F; // bits C A Alt Hold (3..0)
                self.restart_envelope_if_used();
            },
            _ => {}
        }
    }

    fn restart_envelope_if_used(&mut self){
        // Solo reiniciar si algún canal usa envolvente (bit4 en regs 8-10)
        if (self.regs[8] & 0x10)!=0 || (self.regs[9] & 0x10)!=0 || (self.regs[10] & 0x10)!=0 {
            self.env_active = true;
            self.envelope_requested_count +=1;
            self.env_counter = self.env_period;
            let attack = (self.env_shape & 0b0100) !=0; // bit2 Attack (A)
            self.env_dir_up = attack;
            self.env_step = if attack { 0 } else { 15 };
            self.env_hold = false;
        }
    }

    fn recompute_tone_periods(&mut self) {
        // Channels: A: 0(fine),1(coarse[3:0]); B:2,3; C:4,5
        for ch in 0..3 {
            let fine = self.regs[ch*2] as u16;
            let coarse = (self.regs[ch*2+1] & 0x0F) as u16;
            let p = ((coarse << 8) | fine).max(1);
            self.tone_period[ch] = p;
            if self.tone_counter[ch] > p { self.tone_counter[ch] = p; }
        }
    }

    pub fn tick(&mut self, cycles: u32) {
        self.cycle_accum += cycles as u64;
        while self.cycle_accum >= self.cycles_per_sample {
            self.cycle_accum -= self.cycles_per_sample;
            self.step_internal();
            let s = self.mix_sample();
            self.pcm[self.pcm_write & self.pcm_mask] = s;
            self.pcm_write = self.pcm_write.wrapping_add(1);
            self.metric_samples +=1;
        }
    }

    fn step_internal(&mut self) {
        // Tone channels
        for ch in 0..3 { if self.tone_counter[ch] > 0 { self.tone_counter[ch]-=1; } if self.tone_counter[ch]==0 { self.tone_counter[ch]= self.tone_period[ch]; self.tone_out[ch] = !self.tone_out[ch]; self.metric_tone_toggles +=1; }}
        // Noise
        if self.noise_counter>0 { self.noise_counter -=1; }
        if self.noise_counter==0 { self.noise_counter = self.noise_period; // LFSR tap bits 0,3 (x^0 ^ x^3)
            let bit = ((self.lfsr ^ (self.lfsr >>3)) & 1) !=0; // XOR taps
            self.lfsr = (self.lfsr >>1) | ((bit as u32) <<16);
            self.noise_out = (self.lfsr & 1)!=0; self.metric_noise_shifts +=1; }
        // Envelope avance
        if self.env_active && !self.env_hold {
            if self.env_counter>0 { self.env_counter -=1; }
            if self.env_counter==0 {
                self.env_counter = self.env_period;
                // Paso
                let prev = self.env_step;
                if self.env_dir_up { if self.env_step < 15 { self.env_step +=1; } } else { if self.env_step>0 { self.env_step -=1; } }
                if prev == self.env_step { // reached boundary
                    // Fin de ciclo
                    let cont = (self.env_shape & 0b1000)!=0;     // bit3 Continue (C)
                    let alt  = (self.env_shape & 0b0010)!=0;     // bit1 Alternate
                    let hold = (self.env_shape & 0b0001)!=0;     // bit0 Hold
                    if !cont { // one-shot: mantener extremo (0 o 15)
                        self.env_hold = true;
                    } else {
                        if hold { self.env_hold = true; }
                        if alt && !self.env_hold { self.env_dir_up = !self.env_dir_up; }
                        if !hold && !alt { // repetir mismo patrón (reiniciar dirección basada en attack)
                            let attack = (self.env_shape & 0b0100)!=0;
                            self.env_dir_up = attack;
                            self.env_step = if attack { 0 } else { 15 }; // reinicio abrupto
                        }
                    }
                }
                self.metric_env_steps +=1;
            }
        }
    }

    fn mix_sample(&self) -> i16 {
        // Mixer register 7: bit0 disable tone A, bit1 disable tone B, bit2 disable tone C, bit3 disable noise A, bit4 disable noise B, bit5 disable noise C
        let mix = self.regs[7];
        let mut acc = 0.0f32;
        // Tabla log (aprox) normalizada (16 niveles) - valores estándar AY escalados 0..1
        const VOL: [f32;16] = [0.0,0.004,0.008,0.011,0.016,0.023,0.033,0.047,0.067,0.094,0.132,0.187,0.263,0.371,0.525,0.749];
        let env_level = self.env_step as usize;
        let env_amp = VOL[env_level];
        for ch in 0..3 {
            let tone_enabled = (mix & (1<<ch)) == 0;
            let noise_enabled = (mix & (1<<(ch+3))) == 0;
            let raw = self.regs[8+ch];
            let use_env = (raw & 0x10)!=0 && self.env_active; // bit4 activa envolvente
            let amp_reg = raw & 0x0F;
            let amp = if use_env { env_amp } else { VOL[amp_reg as usize] };
            if amp <= 0.0 { continue; }
            let mut v = 0.0f32;
            if tone_enabled { v += if self.tone_out[ch] { 1.0 } else { -1.0 }; }
            if noise_enabled { v += if self.noise_out { 1.0 } else { -1.0 }; }
            const HEADROOM: f32 = 0.33; // reduce clipping multi-canal
            acc += v * amp * HEADROOM;
        }
        // Clamp y convertir a i16
        let s = acc.max(-1.0).min(1.0);
        (s * 32767.0) as i16
    }

    /// Drena las muestras acumuladas (retorna copia). Futuro: devolver puntero y longitud para zero-copy WASM.
    pub fn drain_pcm(&mut self) -> Vec<i16> {
        // Estrategia simple: copiar todo el ring en orden lógico (puede optimizarse más adelante)
        let mut out = Vec::with_capacity(self.pcm.len());
        let base = self.pcm_write & self.pcm_mask;
        out.extend_from_slice(&self.pcm[base..]);
        if base>0 { out.extend_from_slice(&self.pcm[..base]); }
        out
    }

    /// Prepara un snapshot lineal del ring buffer actual en `export_staging` y devuelve número de muestras.
    /// No drena el ring; la UI puede llamar periódicamente y consumir delta mediante serial si se desea.
    pub fn prepare_export(&mut self) -> usize {
        let len = self.pcm.len();
        if self.export_staging.len() != len { self.export_staging.resize(len, 0); }
        let base = self.pcm_write & self.pcm_mask;
        let (first, second) = self.pcm.split_at(base);
        // Orden lógico: desde base hasta final y luego inicio..base
        self.export_staging[..(len-base)].copy_from_slice(&second);
        self.export_staging[(len-base)..].copy_from_slice(&first[..base]);
        self.export_serial = self.export_serial.wrapping_add(1);
        // Actualizamos last_export_write para que delta parta de aquí la próxima vez
        self.last_export_write = self.pcm_write;
        len
    }

    pub fn export_ptr(&self) -> *const i16 { self.export_staging.as_ptr() }
    pub fn export_len(&self) -> usize { self.export_staging.len() }
    pub fn export_serial(&self) -> u64 { self.export_serial }

    // ---- Delta PCM export ----
    // Devuelve número de muestras nuevas desde el último export (full o delta). Si overflow (se han generado
    // más muestras que el tamaño del ring) se marca delta_overflow=true y se devuelve el ring completo (snapshot lógico).
    pub fn prepare_delta_export(&mut self) -> usize {
        let ring_len = self.pcm.len();
        let last = self.last_export_write; // posición absoluta anterior
        let now = self.pcm_write;          // posición absoluta actual
        let produced = now.wrapping_sub(last); // puede wrappear pero en usize se asume no overflow práctico dado ritmo
        if produced == 0 {
            self.delta_staging.clear();
            self.delta_overflow = false;
            return 0;
        }
        // Overflow si produjo >= ring_len (perdimos muestras intermedias)
        if produced >= ring_len {
            // fallback: export lógico completo
            if self.export_staging.len() != ring_len { self.export_staging.resize(ring_len,0);}            
            let base = self.pcm_write & self.pcm_mask;
            let (first, second) = self.pcm.split_at(base);
            self.export_staging[..(ring_len-base)].copy_from_slice(&second);
            self.export_staging[(ring_len-base)..].copy_from_slice(&first[..base]);
            self.delta_staging.clear();
            self.delta_overflow = true;
            self.export_serial = self.export_serial.wrapping_add(1);
            self.last_export_write = self.pcm_write;
            return ring_len;
        }
        // Caso normal: produced < ring_len
        // Copiamos segmento lineal desde last..now en orden cronológico usando el ring.
        self.delta_staging.clear();
        self.delta_staging.reserve(produced);
        for idx in 0..produced {
            let pos = last.wrapping_add(idx) & self.pcm_mask;
            self.delta_staging.push(self.pcm[pos]);
        }
        self.delta_overflow = false;
        self.export_serial = self.export_serial.wrapping_add(1);
        self.last_export_write = self.pcm_write;
        produced
    }
    pub fn delta_ptr(&self) -> *const i16 { self.delta_staging.as_ptr() }
    pub fn delta_len(&self) -> usize { self.delta_staging.len() }
    pub fn delta_overflow(&self) -> bool { self.delta_overflow }
    pub fn sample_rate(&self) -> u32 { self.sample_rate }
}
