use crate::bus::Bus;
use crate::integrator::Integrator;

// Canonical 6809 vector addresses (high, low)
const VEC_SWI3: u16 = 0xFFF0; // SWI3
const VEC_SWI2: u16 = 0xFFF2; // SWI2
const VEC_FIRQ: u16 = 0xFFF4; // FIRQ
const VEC_IRQ:  u16 = 0xFFF6; // IRQ
const VEC_SWI:  u16 = 0xFFF8; // SWI
const VEC_NMI:  u16 = 0xFFFA; // NMI
const VEC_RESET:u16 = 0xFFFC; // RESET

// Extracted CPU implementation from previous mod.rs
// (functionality unchanged; to be refactored to use Bus & VIA later)
// TODO Vector map alignment:
//  6809 standard vectors:
//   FFF0/FFF1: SWI3, FFF2/FFF3: SWI2, FFF4/FFF5: FIRQ, FFF6/FFF7: IRQ, FFF8/FFF9: SWI, FFFA/FFFB: NMI, FFFC/FFFD: RESET
//  Current implementation temporarily uses FFF8/FFF9 for IRQ and SWI. Adjust once BIOS expectations confirmed.
// TODO Add distinct service_firq() with partial stacking (no E, only CC+PC) and vector FFF4/FFF5.
// TODO Implement SWI2/SWI3 if needed by higher-level tooling.

pub struct CPU {
    pub a: u8, pub b: u8, pub dp: u8, pub x: u16, pub y: u16, pub u: u16, pub pc: u16,
    pub call_stack: Vec<u16>,
    // Core CC bits
    pub cc_z: bool, pub cc_n: bool, pub cc_c: bool, pub cc_v: bool, pub cc_h: bool, pub cc_f: bool, pub cc_e: bool,
    pub mem: [u8;65536],
    pub bus: Bus,
    pub trace: bool, pub bios_calls: Vec<String>,
    // Legacy frame_count kept for compatibility (mirrors cycle_frame). New cycle_frame is authoritative.
    pub frame_count: u64,
    pub cycle_frame: u64,
    pub bios_frame: u64,
    pub cycles_per_frame: u64,
    pub cycle_accumulator: u64,
    pub last_intensity: u8,
    pub reset0ref_count: u64, pub print_str_count: u64, pub print_list_count: u64,
    pub bios_present: bool, pub cycles: u64,
    pub irq_pending: bool,
    pub firq_pending: bool,
    pub nmi_pending: bool,
    pub wai_halt: bool,
    pub cc_i: bool, // Interrupt mask (I flag)
    pub s: u16,     // Hardware stack pointer (simplified)
    pub in_irq_handler: bool, // tracking if currently executing IRQ handler
    // Metrics
    pub opcode_total: u64,
    pub opcode_unimplemented: u64,
    pub opcode_counts: [u64;256],
    pub opcode_unimpl_bitmap: [bool;256],
    pub via_irq_count: u64,
    // Debug helper: ensure we only bootstrap VIA once
    pub debug_bootstrap_via_done: bool,
    // Track if a WAI instruction already pushed the full frame so IRQ shouldn't push again
    pub wai_pushed_frame: bool,
    pub forced_irq_vector: bool,
    // BIOS code always executed for accuracy (single canonical path; legacy alternate removed).
    // Loop diagnostics
    pub loop_watch_slots: [LoopSample; 16],
    pub loop_watch_idx: usize,
    pub loop_watch_count: u64,
    // Track depth of call stack when entering WAIT_RECAL to increment frame on its real return
    pub wait_recal_depth: Option<usize>,
    pub current_x: i16, pub current_y: i16, pub beam_on: bool,
    // Frame instrumentation
    pub wait_recal_calls: u64,
    pub wait_recal_returns: u64,
    pub force_frame_heuristic: bool,
    pub last_forced_frame_cycle: u64,
    pub cart_loaded: bool,
    pub jsr_log: [u16;128],
    pub jsr_log_len: usize,
    /// Deprecated: previously enabled heuristic generation of BIOS frames from IRQ cadence
    /// when no WAIT_RECAL had been observed. Cycle-based frame timing has replaced this and
    /// the fallback now has no effect (retained only to avoid breaking existing UI bindings).
    pub enable_irq_frame_fallback: bool,
    pub irq_frames_generated: u64,
    pub last_irq_frame_cycles: u64,
    // Experimental beam integrator (feature-in-progress)
    pub integrator: Integrator,
    // Integrator segment stats / controls
    pub integrator_auto_drain: bool,
    pub integrator_last_frame_segments: u32,
    pub integrator_max_frame_segments: u32,
    pub integrator_total_segments: u64,
    // Cartridge validation & diagnostics
    pub cart_valid: bool,
    pub cart_title: [u8;16],
    pub cart_validation_done: bool,
    // Extended diagnostics counters
    pub firq_count: u64,
    pub irq_count: u64,
    pub t1_expiries: u64,
    pub t2_expiries: u64,
    pub lines_per_frame_accum: u64,
    pub lines_per_frame_samples: u64,
    // Temporary staging buffer for WASM shared memory segment export
    pub temp_segments_c: Vec<crate::integrator::BeamSegmentC>,
    // Last synthetic extended (prefix) coverage gaps captured by recompute_opcode_coverage
    pub last_extended_unimplemented: Vec<u16>,
    // Hot opcode sampling (dev diagnostic): up to 4 distinct PCs for 0x00 & 0xFF
    pub hot00: [(u16,u64);4],
    pub hotff: [(u16,u64);4],
}

pub struct CPUOpcodeMetrics {
    pub total: u64,
    pub unimplemented: u64,
    pub counts: [u64;256],
    pub unique_unimplemented: Vec<u8>,
    // Extended (prefix 0x10/0x11) coverage details: store 16-bit (prefix<<8 | sub) for missing sub-opcodes
    pub extended_unimplemented: Vec<u16>,
}

// Valid 6809 extended (prefixed) opcode sub-values for prefix 0x10 and 0x11.
// Anything outside these lists is officially unassigned/invalid and should not
// be counted against synthetic coverage metrics.
pub const VALID_PREFIX10: &[u8] = &[
    // Long branches (all conditional forms) 0x21-0x2F
    0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2A,0x2B,0x2C,0x2D,0x2E,0x2F,
    // SWI2
    0x3F,
    // CMPD & CMPY families
    0x83,0x93,0xA3,0xB3, // CMPD imm/dir/idx/ext
    0x8C,0x9C,0xAC,0xBC, // CMPY imm/dir/idx/ext
    // LDY/STY
    0x8E, // LDY immediate
    0x9E,0xAE,0xBE, // LDY direct/indexed/extended
    0x9F,0xAF,0xBF, // STY direct/indexed/extended
    // LDS/STS
    0xCE, // LDS immediate
    0xDE,0xEE,0xFE, // LDS direct/indexed/extended
    0xDF,0xEF,0xFF, // STS direct/indexed/extended
];

pub const VALID_PREFIX11: &[u8] = &[
    // SWI3
    0x3F,
    // CMPU & CMPS families
    0x83,0x93,0xA3,0xB3, // CMPU imm/dir/idx/ext
    0x8C,0x9C,0xAC,0xBC, // CMPS imm/dir/idx/ext
];

// Legacy VectorEvent system removed; integrator backend is canonical.

#[derive(Clone,Copy,Debug,Default)]
pub struct LoopSample {
    pub pc: u16,
    pub a: u8, pub b: u8, pub x: u16, pub y: u16, pub u: u16, pub s: u16, pub dp: u8,
    pub via_ifr: u8, pub via_ier: u8, pub via_acr: u8, pub via_pcr: u8,
    pub cycles: u64,
}

impl Default for CPU { fn default()->Self {
    #[cfg(not(target_arch="wasm32"))]
    let freq = std::env::var("VPY_CPU_FREQ").ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(1_500_000);
    #[cfg(target_arch="wasm32")]
    let freq = 1_500_000u64;
    let cpf = freq / 50; // integer division; leftover cycles accumulate in cycle_accumulator
    // Backend selection environment variable ignored; integrator is always enabled.
    CPU { a:0,b:0,dp:0xD0,x:0,y:0,u:0,pc:0,call_stack:Vec::new(),cc_z:false,cc_n:false,cc_c:false,cc_v:false,cc_h:false,cc_f:false,cc_e:false,
        mem:[0;65536],bus:Bus::default(),trace:false,bios_calls:Vec::new(),
        frame_count:0, cycle_frame:0, bios_frame:0, cycles_per_frame:cpf, cycle_accumulator:0,
    last_intensity:0,reset0ref_count:0,print_str_count:0,print_list_count:0,bios_present:false,cycles:0,
        irq_pending:false, firq_pending:false, nmi_pending:false, wai_halt:false, cc_i:false, s:0xD000, in_irq_handler:false,
    opcode_total:0, opcode_unimplemented:0, opcode_counts:[0;256], opcode_unimpl_bitmap:[false;256], via_irq_count:0,
    debug_bootstrap_via_done:false, wai_pushed_frame:false, forced_irq_vector:false,
        loop_watch_slots:[LoopSample::default();16], loop_watch_idx:0, loop_watch_count:0, wait_recal_depth:None, current_x:0, current_y:0, beam_on:false,
        wait_recal_calls:0, wait_recal_returns:0, force_frame_heuristic:false, last_forced_frame_cycle:0, cart_loaded:false,
    jsr_log:[0;128], jsr_log_len:0, enable_irq_frame_fallback:false, irq_frames_generated:0, last_irq_frame_cycles:0,
    integrator: { let mut i=Integrator::new(); #[cfg(not(target_arch="wasm32"))] { if std::env::var("VPY_NO_MERGE").ok().as_deref()==Some("1"){ i.set_merge(false); } } i },
        integrator_auto_drain: false,
        integrator_last_frame_segments: 0,
        integrator_max_frame_segments: 0,
        integrator_total_segments: 0,
        cart_valid:false, cart_title:[0;16], cart_validation_done:false,
        firq_count:0, irq_count:0, t1_expiries:0, t2_expiries:0, lines_per_frame_accum:0, lines_per_frame_samples:0,
        temp_segments_c: Vec::new(),
        last_extended_unimplemented: Vec::new(),
        hot00: [(0,0);4], hotff: [(0,0);4],
    }
} }

impl CPU {
    /// Reset dynamic execution statistics without altering loaded memory or BIOS/cart state.
    pub fn reset_stats(&mut self) {
        self.cycles = 0;
        self.frame_count = 0;
        self.cycle_frame = 0;
        self.bios_frame = 0;
        self.cycle_accumulator = 0;
        self.last_intensity = 0;
        self.opcode_total = 0;
        self.opcode_unimplemented = 0;
        self.opcode_counts = [0;256];
        self.opcode_unimpl_bitmap = [false;256];
        self.last_extended_unimplemented.clear();
        self.via_irq_count = 0;
        self.wait_recal_calls = 0;
        self.wait_recal_returns = 0;
        self.jsr_log_len = 0;
        self.loop_watch_slots = [LoopSample::default();16];
        self.loop_watch_idx = 0; self.loop_watch_count = 0;
        self.integrator_last_frame_segments = 0;
        self.integrator_max_frame_segments = 0;
        self.integrator_total_segments = 0;
        self.irq_count = 0; self.firq_count = 0;
        self.t1_expiries = 0; self.t2_expiries = 0;
        self.lines_per_frame_accum = 0; self.lines_per_frame_samples = 0;
        self.irq_frames_generated = 0; self.last_irq_frame_cycles = 0;
        self.bus.stats.reads_unmapped = 0;
        self.bus.stats.writes_unmapped = 0;
        self.bus.stats.writes_bios_ignored = 0;
        self.bus.stats.cart_oob_reads = 0;
        // Clear hotspot sampling buffers
        self.hot00 = [(0,0);4];
        self.hotff = [(0,0);4];
    self.integrator.segments.clear();
    }
    // Lightweight clone used only by recompute_opcode_coverage; does not duplicate integrator segments or bus side effects precisely.
    fn coverage_clone(&self) -> CPU {
        CPU {
            a:self.a,b:self.b,dp:self.dp,x:self.x,y:self.y,u:self.u,pc:self.pc,call_stack:Vec::new(),
            cc_z:self.cc_z,cc_n:self.cc_n,cc_c:self.cc_c,cc_v:self.cc_v,cc_h:self.cc_h,cc_f:self.cc_f,cc_e:self.cc_e,
            mem:self.mem, // copy array
            bus: Bus::default(), // fresh bus (safe for isolated opcode exec)
            trace:false,bios_calls:Vec::new(),
            frame_count:0, cycle_frame:0, bios_frame:0, cycles_per_frame:self.cycles_per_frame, cycle_accumulator:0,
            last_intensity:0,reset0ref_count:0,print_str_count:0,print_list_count:0,bios_present:false,cycles:0,
            irq_pending:false,firq_pending:false,nmi_pending:false,wai_halt:false,cc_i:false,s:self.s,in_irq_handler:false,
            opcode_total:0, opcode_unimplemented:0, opcode_counts:[0;256], opcode_unimpl_bitmap:[false;256], via_irq_count:0,
            debug_bootstrap_via_done:false, wai_pushed_frame:false, forced_irq_vector:false,
            loop_watch_slots:[LoopSample::default();16], loop_watch_idx:0, loop_watch_count:0, wait_recal_depth:None, current_x:0, current_y:0, beam_on:false,
            wait_recal_calls:0, wait_recal_returns:0, force_frame_heuristic:false, last_forced_frame_cycle:0, cart_loaded:false,
            jsr_log:[0;128], jsr_log_len:0, enable_irq_frame_fallback:false, irq_frames_generated:0, last_irq_frame_cycles:0,
            integrator: Integrator::new(), integrator_auto_drain:false, integrator_last_frame_segments:0, integrator_max_frame_segments:0, integrator_total_segments:0,
            cart_valid:false, cart_title:[0;16], cart_validation_done:false,
            firq_count:0, irq_count:0, t1_expiries:0, t2_expiries:0, lines_per_frame_accum:0, lines_per_frame_samples:0,
            temp_segments_c: Vec::new(),
            last_extended_unimplemented: Vec::new(),
            hot00: [(0,0);4], hotff: [(0,0);4],
        }
    }
    #[cfg(not(target_arch="wasm32"))]
    fn env_trace_frame(&self) -> bool { std::env::var("TRACE_FRAME").ok().as_deref()==Some("1") }
    #[cfg(target_arch="wasm32")]
    fn env_trace_frame(&self) -> bool { false }
    fn handle_intensity_change(&mut self){
        let new_on = self.last_intensity > 0;
        if new_on != self.beam_on {
            self.beam_on = new_on;
        }
        // Propagate to integrator experimental model
        self.integrator.set_intensity(self.last_intensity);
        if self.last_intensity>0 { self.integrator.beam_on(); } else { self.integrator.beam_off(); }
    }

    fn validate_cartridge_if_needed(&mut self){
        if self.cart_validation_done || !self.cart_loaded { return; }
        // Heuristic: look for an ASCII run near 0x0040..0x00A0 that contains uppercase letters/spaces and length >= 6.
        let mut best_start=None; let mut best_len=0;
        for start in 0x0040..0x00A0 { if start+6 <= 0x00A0 { let mut len=0; for off in 0..32 { let a=start+off; if a>=0x00A0 { break; } let c=self.mem[a]; let ok = (c>=0x20 && c<=0x5A) || c==0x00; if !ok { break; } if c==0 { break; } len+=1; }
            if len>=6 && len>best_len { best_start=Some(start); best_len=len; } } }
        if let Some(s)=best_start { let copy_len=best_len.min(16); for i in 0..copy_len { self.cart_title[i]=self.mem[s+i]; } self.cart_valid=true; }
        self.cart_validation_done=true;
    }
    pub fn opcode_metrics(&self) -> CPUOpcodeMetrics {
        let uniques: Vec<u8> = self.opcode_unimpl_bitmap.iter().enumerate()
            .filter_map(|(i, b)| if *b { Some(i as u8) } else { None })
            .collect();
        // Placeholder: recompute_opcode_coverage() now populates an internal scratch we will soon emit; for now keep empty.
        CPUOpcodeMetrics { total: self.opcode_total, unimplemented: self.opcode_unimplemented, counts: self.opcode_counts, unique_unimplemented: uniques, extended_unimplemented: self.last_extended_unimplemented.clone() }
    }
    // Backwards-compatible alias used by some tests naming metrics_snapshot()
    pub fn metrics_snapshot(&self) -> CPUOpcodeMetrics { self.opcode_metrics() }

    /// Recompute a synthetic coverage view: iterate all 256 opcode slots and classify as Implemented/Unimplemented
    /// based on whether executing them results in an unimplemented trap. We do this non-destructively by cloning
    /// CPU state and executing a single step per opcode with a safe program counter region.
    pub fn recompute_opcode_coverage(&mut self) -> (usize, usize, Vec<u8>) {
        // Clear existing bitmap & counters
        self.opcode_unimpl_bitmap = [false;256];
        self.opcode_unimplemented = 0; self.opcode_total = 0; self.opcode_counts = [0;256];
        let mut extended_unimpl: Vec<u16> = Vec::new();
        // We will place each opcode at 0x0100 with a harmless operand byte (0) following when needed.
        for op in 0u16..=255u16 {
            // Clone minimal register state to keep side effects isolated
            let mut clone = CPU { ..Default::default() };
            clone.pc = 0x0100;
            clone.mem[0x0100] = op as u8;
            // Some instructions that read an operand byte must not run off end; ensure 0x0101 exists.
            clone.mem[0x0101] = 0x00; clone.mem[0x0102] = 0x00; clone.mem[0x0103] = 0x00;
            // Provide a reset vector so any unexpected reset fetch doesn't crash.
            clone.mem[0xFFFC] = 0x00; clone.mem[0xFFFD] = 0x02; // -> 0x0200
            if op as u8 == 0x10 || op as u8 == 0x11 {
                // Extended prefix: iterate only valid sub-opcodes (exclude invalid/unassigned)
                let prefix = op as u8;
                let valid_list: &[u8] = if prefix == 0x10 { VALID_PREFIX10 } else { VALID_PREFIX11 };
                let mut any_impl = false;
                for &sub in valid_list {
                    let mut ec = clone.coverage_clone();
                    ec.mem[0x0101] = sub; // sub-opcode byte
                    let ok = ec.step();
                    if ok { any_impl = true; } else { extended_unimpl.push(((prefix as u16)<<8)|sub as u16); }
                }
                if !any_impl && !valid_list.is_empty() { self.opcode_unimpl_bitmap[op as usize] = true; }
            } else {
                let ok = clone.step();
                if !ok { self.opcode_unimpl_bitmap[op as usize] = true; }
            }
        }
        let unimpl: Vec<u8> = self.opcode_unimpl_bitmap.iter().enumerate().filter_map(|(i,b)| if *b {Some(i as u8)} else {None}).collect();
        let implemented = 256 - unimpl.len();
        // Store extended results somewhere observable: currently we just print later; could retain in a field if desired.
        // For now we drop duplicates (an extended prefix may triage many).
        extended_unimpl.sort_unstable();
        extended_unimpl.dedup();
        self.last_extended_unimplemented = extended_unimpl.clone();
        (implemented, unimpl.len(), unimpl)
    }
    // take_vector_events removed.
    pub fn sync_mem_to_bus(&mut self){
        // One-time sync (idempotent) to keep bus memory identical to legacy mem array
        for i in 0..65536 { self.bus.mem[i] = self.mem[i]; }
    }

    pub fn reset(&mut self){
        if !self.cart_loaded {
            for addr in 0x0000usize..0xC000usize { self.mem[addr]=0xFF; self.bus.mem[addr]=0xFF; }
        }
        // Ensure all execution/statistical counters are cleared as part of a reset so UI does not
        // need to issue a separate stats reset (still exposed separately for a "soft" stats clear).
        self.reset_stats();
        // Gather vector bytes for diagnostics
        let sw3_lo=self.read8(VEC_SWI3); let sw3_hi=self.read8(VEC_SWI3+1);
        let sw2_lo=self.read8(VEC_SWI2); let sw2_hi=self.read8(VEC_SWI2+1);
        let firq_lo=self.read8(VEC_FIRQ); let firq_hi=self.read8(VEC_FIRQ+1);
        let irq_lo=self.read8(VEC_IRQ); let irq_hi=self.read8(VEC_IRQ+1);
        let swi_lo=self.read8(VEC_SWI); let swi_hi=self.read8(VEC_SWI+1);
        let nmi_lo=self.read8(VEC_NMI); let nmi_hi=self.read8(VEC_NMI+1);
        let rst_lo=self.read8(VEC_RESET); let rst_hi=self.read8(VEC_RESET+1);
        let vec = ((rst_hi as u16)<<8)|rst_lo as u16;
        let mut pc_set = vec;
        if self.bios_present {
            // Validate RESET vector: must point inside BIOS window (>=E000) and start with a plausible first opcode.
            let first = self.read8(vec);
            let plausible_opcode = matches!(first, 0x8E|0xCE|0xCC|0xBD|0x86|0x1F|0x1A|0x34|0x10|0x11|0x8D|0x16);
            let in_bios_window = vec >= 0xE000;
            if !in_bios_window || !plausible_opcode {
                pc_set = 0xF000; // canonical Vectrex entry point
            }
        }
        self.pc = pc_set;
        // Clear dynamic flags / pending states
        self.cc_e=false; self.cc_f=false; self.cc_h=false; self.cc_i=false; self.cc_n=false; self.cc_z=false; self.cc_v=false; self.cc_c=false;
        self.irq_pending=false; self.firq_pending=false; self.nmi_pending=false; self.wai_halt=false; self.in_irq_handler=false;
        if self.trace {
            println!("[VECTORS] SWI3={:02X}{:02X} SWI2={:02X}{:02X} FIRQ={:02X}{:02X} IRQ={:02X}{:02X} SWI={:02X}{:02X} NMI={:02X}{:02X} RESET={:02X}{:02X} (raw={:04X})",
                sw3_hi,sw3_lo, sw2_hi,sw2_lo, firq_hi,firq_lo, irq_hi,irq_lo, swi_hi,swi_lo, nmi_hi,nmi_lo, rst_hi,rst_lo, vec);
            println!("[RESET] PC set to {:04X}{}", self.pc, if pc_set!=vec {" (forced canonical BIOS entry)"} else {""});
        }
        // Debug bootstrap of VIA to force early Timer1 IRQs / frame progress if BIOS hasn't yet configured it.
        // Guard: only once, only if BIOS loaded, and IER still zero (meaning no BIOS init done yet).
        if self.bios_present && !self.debug_bootstrap_via_done && self.bus.via_ier()==0 {
            // Program Timer2 (refresh) with a small value so BIOS Wait_Recal style loop can proceed.
            // VIA registers: T2 low = D008, T2 high = D009 (writing high loads counter). Enable T2 interrupt (bit 0x20).
            self.bus.write8(0xD008, 0x30); // T2 low
            self.bus.write8(0xD009, 0x00); // T2 high -> counter = 0x0030
            // Minimal ACR: leave PB7 toggle off for clarity, ensure T1 not free-run interfering -> use 0x00
            self.bus.write8(0xD00B, 0x00); // ACR
            // Enable T2 interrupt: write 0xA0 (bit7=1 to set, bit5 = 1 for T2)
            self.bus.write8(0xD00E, 0xA0);
            self.debug_bootstrap_via_done = true;
            if self.trace { println!("[BOOTSTRAP VIA] T2=0x0030 ACR=0x00 IER=0x20"); }
        }

        // Validate / force IRQ vector if BIOS provides an unexpected address (outside BIOS window)
        if self.bios_present {
            let irq_lo = self.read8(VEC_IRQ); let irq_hi = self.read8(VEC_IRQ+1); let irq_vec = ((irq_hi as u16)<<8)|irq_lo as u16;
            if !(0xE000..=0xFFFF).contains(&irq_vec) || irq_vec == 0x0000 {
                // Force to WAIT_RECAL (F192) which is canonical Vectrex IRQ handler
                self.bus.mem[VEC_IRQ as usize] = 0x92; // low byte
                self.bus.mem[(VEC_IRQ+1) as usize] = 0xF1; // high byte
                self.forced_irq_vector = true;
                if self.trace { println!("[FORCE IRQ VECTOR] original={:04X} -> F192", irq_vec); }
            }
        }

        // Debug: optional internal vector list smoke test (both formats) when env flag set at process start.
        // We can only read env in std builds; for wasm this will be ignored (no std::env).
        #[cfg(not(target_arch="wasm32"))]
        {
            if std::env::var("TEST_VL").ok().as_deref()==Some("1") {
                self.install_internal_vector_tests();
            }
        }
    }

    #[allow(dead_code)]
    fn install_internal_vector_tests(&mut self){
        // Layout two small lists into high RAM (choose an address unlikely to collide with BIOS): 0xC000 region.
        // 1) Runtime command list: count=4 -> START, LINE, INT (change intensity), LINE, then END (implicit by count) producing 2 lines.
        // Format: count, (y,x,cmd)... extra intensity byte after CMD_INT.
        let runtime_addr: u16 = 0xC100;
        let mut p = runtime_addr;
        let runtime_bytes: [u8; 1+3*4+1] = [
            4,         // count
            0,0,0,     // START at (0,0)
            5,10,1,    // LINE dy=5 dx=10
            0,0,3, 0x40, // INT set intensity 0x40
            250,250,1,  // LINE dy=-6 dx=-6 (250 as signed -6)
        ];
        for b in runtime_bytes { self.write8(p,b); p=p.wrapping_add(1); }

        // 2) Legacy list: (dy,dx) pairs, end bit on final dy. First move then one line.
        let legacy_addr: u16 = 0xC140;
        let legacy_bytes: [u8;4] = [ 10, 5, 0x86, 0xFA ]; // (10,5) then (-122? end) but using small example: second dy has end bit set (0x80|6)
        for (i,b) in legacy_bytes.iter().enumerate(){ self.write8(legacy_addr + i as u16, *b); }

        // Simulate two calls to BIOS Draw_VL by directly invoking parser path: push fake call stack and call record_bios_call
        // We'll set U to each list and call record_bios_call with 0xF3DD (Draw_VL address) to reuse parsing.
        // This avoids needing actual BIOS code execution for the test.
        let saved_u = self.u; let saved_pc = self.pc; let saved_dp = self.dp;
        self.u = runtime_addr; self.pc = 0xF3DD; self.record_bios_call(0xF3DD);
        self.u = legacy_addr;  self.pc = 0xF3DD; self.record_bios_call(0xF3DD);
        self.u = saved_u; self.pc = saved_pc; self.dp = saved_dp;
    }

    fn read8(&mut self, addr:u16)->u8 {
        // For <0xD000 we treat internal self.mem as authoritative so that tests which poke cpu.mem
        // directly remain valid. We still emulate cartridge OOB semantics (return 0x01) and update
        // stats manually. All >=0xD000 (including VIA window and BIOS/unmapped) are delegated to Bus.
        if (addr & 0xFFF0)==0xD000 { return self.bus.read8(addr); }
        if addr < 0xD000 {
            let a = addr as usize;
            if a < 0x8000 {
                let cart_len = self.bus.test_cart_len();
                if cart_len>0 && a >= cart_len {
                    // Cartridge out-of-bounds read -> 0x01
                    self.bus.stats.cart_oob_reads = self.bus.stats.cart_oob_reads.wrapping_add(1);
                    return 0x01;
                }
            }
            return self.mem[a];
        }
        self.bus.read8(addr)
    }
    fn write8(&mut self, addr:u16, val:u8){
        if (addr & 0xFFF0)==0xD000 { self.bus.write8(addr,val); return; }
        if addr < 0xD000 {
            // Write through to both mirrors so opcode fetch (self.mem) and bus semantics stay coherent.
            self.mem[addr as usize]=val;
            self.bus.mem[addr as usize]=val;
            return;
        }
        self.bus.write8(addr,val);
    }

    // Public test helper APIs (intentionally always compiled so integration tests can use them)
    pub fn test_read8(&mut self, addr:u16)->u8 { self.read8(addr) }
    pub fn test_write8(&mut self, addr:u16, val:u8){ self.write8(addr,val) }

    pub fn load_bin(&mut self, data:&[u8], base:u16) {
        for (i, b) in data.iter().enumerate() {
            let addr = base as usize + i;
            if addr < 65536 { self.mem[addr] = *b; self.bus.mem[addr] = *b; }
        }
        // If this looks like a cartridge load (base 0) track length for OOB read semantics
        if base == 0 { self.bus.set_cart_len(data.len()); }
        if base == 0 { self.cart_loaded = true; self.validate_cartridge_if_needed(); }
    }
    pub fn load_bios(&mut self,data:&[u8]){
        // Map BIOS: 4K -> 0xF000, 8K -> 0xE000 to align vectors at FFF0-FFFD.
        match data.len() {
            4096 => { self.load_bin(data,0xF000); self.bus.set_bios_base(0xF000); },
            8192 => { self.load_bin(data,0xE000); self.bus.set_bios_base(0xE000); },
            _ => return,
        }
        self.bios_present=true; self.bus.set_bios_read_only(true);
    }
    fn d(&self)->u16 { ((self.a as u16)<<8)|self.b as u16 }
    fn set_d(&mut self,v:u16){ self.a=(v>>8) as u8; self.b=v as u8; }
    fn update_nz16(&mut self,v:u16){ self.cc_z=v==0; self.cc_n=(v & 0x8000)!=0; }
    fn update_nz8(&mut self,v:u8){ self.cc_z=v==0; self.cc_n=(v & 0x80)!=0; }
    fn record_bios_call(&mut self, addr:u16) {
        let name = match addr {
            // Timing / frame
            0xF192 => { // WAIT_RECAL (instrumentation only)
                // NOTE: The deterministic frame boundary is now governed solely by cycle_frame.
                // bios_frame is an observational counter incremented only when we detect a *real* return
                // from an actual BIOS WAIT_RECAL call (RTS/RTI depth match) OR via explicit legacy
                // heuristics that we plan to deprecate. We no longer synthesize a bios_frame here.
                self.dp = 0xD0;
                self.wait_recal_calls = self.wait_recal_calls.wrapping_add(1);
                // Track initial call stack depth to attribute a subsequent RTS/RTI as a WAIT_RECAL return.
                if self.wait_recal_depth.is_none() { self.wait_recal_depth = Some(self.call_stack.len()); }
                if self.trace || self.env_trace_frame() { println!("[FRAME][BIOS] WAIT_RECAL call depth={:?} calls={}", self.wait_recal_depth, self.wait_recal_calls); }
                "WAIT_RECAL" },
            0xF1AF => { self.dp = 0xC8; "DP_TO_C8" },
            0xF2A5 => { self.last_intensity = 0x5F; self.handle_intensity_change(); "INTENSITY_5F" },
            0xF2AB => { self.last_intensity = self.a; self.handle_intensity_change(); "INTENSITY_A" },
            0xF289 => "RESET_VIA", // guess label for call seen in trace
            0xF2A9 => { // Assume A holds base, Y low 8 bits maybe variant; just reuse A for now
                self.last_intensity = self.a; self.handle_intensity_change(); "INTENSITY_AY" },
            // Movement / drawing
            0xF312 => { // MOVETO_D: minimal prototype line drawing
                let dx = self.a as i8 as i16; let dy = self.b as i8 as i16;
                let start_x = self.current_x as f32; let start_y = self.current_y as f32;
                self.current_x = self.current_x.wrapping_add(dx);
                self.current_y = self.current_y.wrapping_add(dy);
                let end_x = self.current_x as f32; let end_y = self.current_y as f32;
                let ddx = end_x - start_x; let ddy = end_y - start_y;
                // Heuristic duration: 4 cycles per max axis magnitude (min 1)
                let span = ddx.abs().max(ddy.abs()).max(1.0);
                let draw_cycles = (span * 4.0).ceil() as u32; // crude pacing
                if self.last_intensity > 0 {
                    // Set integrator to starting point (only if large jump) and draw via velocity
                    self.integrator.beam_on();
                    self.integrator.set_intensity(self.last_intensity);
                    // position in integrator follows CPU logical point; we don't teleport for small displacements
                    self.integrator.set_velocity(ddx / draw_cycles as f32, ddy / draw_cycles as f32);
                    // Manually advance cycles for the drawing portion so segment appears immediately.
                    // We reuse advance_cycles so timing stays consistent (but avoid double counting: subtract first, then re-add)
                    self.advance_cycles(draw_cycles);
                    // Stop beam motion (velocity zero) after segment
                    self.integrator.set_velocity(0.0, 0.0);
                } else {
                    // If beam off, just update integrator position silently
                    self.integrator.instant_move(end_x, end_y);
                }
                "MOVETO_D" },
            0xF354 => { self.reset0ref_count += 1; self.integrator.reset_origin(); "RESET0REF" },
            0xF37A => { self.print_str_count += 1; "PRINT_STR_D" },
            0xF385 => "PRINT_CHR", // routine just before PRINT_LIST area
            0xF38A => { self.print_list_count += 1; "PRINT_LIST" },
            0xF38C => "PRINT_LIST_CHK",
            0xF3DD => { // DRAW_VL legacy heuristic removed
                "DRAW_VL" },
            0xF434 => "SOUND_INIT?", // observed
            0xF687 => "MATH_ROUT", // placeholder for mid BIOS math/vector routine
            0xF84F => "INIT_MUSIC?", // placeholder based on early call
            0xFD0D => "MUSIC1",
            _ => "BIOS_UNKNOWN",
        };
        self.bios_calls.push(format!("{:04X}:{}", addr, name));
        if self.trace { println!("[BIOS CALL] {}", name); }
    }
    fn push8(&mut self, v:u8){ self.s = self.s.wrapping_sub(1); self.write8(self.s, v); }
    fn push16(&mut self, v:u16){
        // Push high first then low so that low resides at top of descending stack for easy pop (low then high)
        self.push8((v>>8) as u8); self.push8((v & 0xFF) as u8);
    }
    fn pop8(&mut self)->u8 { let v = self.read8(self.s); self.s = self.s.wrapping_add(1); v }
    fn pop16(&mut self)->u16 { let lo = self.pop8(); let hi = self.pop8(); ((hi as u16)<<8)|lo as u16 }
    // Register code mapping for TFR/EXG (subset):
    // 0: D, 1: X, 2: Y, 3: U, 4: S, 5: PC, 8: A, 9: B, A: CC, B: DP
    fn reg_width(&self, code:u8)->u8 {
        match code { 0|1|2|3|4|5 => 2, 8|9|0xA|0xB => 1, _ => 0 }
    }
    fn read_reg(&self, code:u8)->u16 {
        match code {
            0 => self.d(), 1 => self.x, 2 => self.y, 3 => self.u, 4 => self.s, 5 => self.pc,
            8 => self.a as u16, 9 => self.b as u16, 0xA => self.pack_cc() as u16, 0xB => self.dp as u16,
            _ => 0
        }
    }
    fn write_reg(&mut self, code:u8, val:u16){
        match code {
            0 => self.set_d(val), 1 => self.x = val, 2 => self.y = val, 3 => self.u = val, 4 => self.s = val, 5 => self.pc = val,
            8 => self.a = (val & 0xFF) as u8, 9 => self.b = (val & 0xFF) as u8, 0xA => self.unpack_cc((val & 0xFF) as u8), 0xB => self.dp = (val & 0xFF) as u8,
            _ => {}
        }
    }
    fn pack_cc(&self) -> u8 {
        // 6809 CC bits: EFHINZVC (bit7=E ... bit0=C)
        (if self.cc_e {0x80} else {0}) |
        (if self.cc_f {0x40} else {0}) |
        (if self.cc_h {0x20} else {0}) |
        (if self.cc_i {0x10} else {0}) |
        (if self.cc_n {0x08} else {0}) |
        (if self.cc_z {0x04} else {0}) |
        (if self.cc_v {0x02} else {0}) |
        (if self.cc_c {0x01} else {0})
    }

    // Flag helpers for subtraction / compare (A - B) without modifying original if compare.
    fn flags_sub8(&mut self, a: u8, b: u8, result: u8) {
        // N: bit7 of result
        self.cc_n = (result & 0x80) != 0;
        // Z: result == 0
        self.cc_z = result == 0;
        // V: overflow if sign of (a^b) & (a^result) bit7 set
        self.cc_v = (((a ^ b) & (a ^ result)) & 0x80) != 0;
        // C: borrow: if a < b (unsigned)
        self.cc_c = (a as u16) < (b as u16);
    }
    fn flags_sub16(&mut self, a: u16, b: u16, result: u16) {
        self.cc_n = (result & 0x8000) != 0;
        self.cc_z = result == 0; // For 6809, Z set if result zero (not accumulation across CMPD sequences here)
        // Overflow: (a^b)&(a^result) bit15
        self.cc_v = (((a ^ b) & (a ^ result)) & 0x8000) != 0;
        self.cc_c = (a as u32) < (b as u32);
    }
    fn unpack_cc(&mut self, v:u8){
        self.cc_e = (v & 0x80)!=0;
        self.cc_f = (v & 0x40)!=0;
        self.cc_h = (v & 0x20)!=0;
        self.cc_i = (v & 0x10)!=0;
        self.cc_n = (v & 0x08)!=0;
        self.cc_z = (v & 0x04)!=0;
        self.cc_v = (v & 0x02)!=0;
        self.cc_c = (v & 0x01)!=0;
    }
    // ---------------------------------------------------------------------
    // RMW (Read-Modify-Write) helpers centralizing flag semantics for 8-bit ops
    // These return the modified 8-bit value; caller is responsible for writing
    // it back to the appropriate destination (memory or accumulator) except for
    // TST/CLR where result may be ignored or constant.
    // Semantics follow existing inline implementations exactly.
    // ---------------------------------------------------------------------
    fn rmw_neg(&mut self, m:u8)->u8 {
        let res = (0u16).wrapping_sub(m as u16) as u8;
        self.cc_n = (res & 0x80)!=0; self.cc_z = res==0; self.cc_v = res==0x80; self.cc_c = m!=0; res
    }
    fn rmw_com(&mut self, m:u8)->u8 {
        let res = !m; self.cc_n = (res & 0x80)!=0; self.cc_z = res==0; self.cc_v=false; self.cc_c=true; res
    }
    fn rmw_lsr(&mut self, m:u8)->u8 {
        self.cc_c = (m & 0x01)!=0; let res = m>>1; self.cc_n=false; self.cc_z=res==0; self.cc_v=false; res
    }
    fn rmw_ror(&mut self, m:u8)->u8 {
        let cin = if self.cc_c {0x80} else {0}; self.cc_c = (m & 0x01)!=0; let res = (m>>1)|cin; self.cc_n = (res & 0x80)!=0; self.cc_z = res==0; self.cc_v=false; res
    }
    fn rmw_asr(&mut self, m:u8)->u8 {
        self.cc_c = (m & 0x01)!=0; let msb = m & 0x80; let res = (m>>1)|msb; self.cc_n = (res & 0x80)!=0; self.cc_z = res==0; self.cc_v=false; res
    }
    fn rmw_asl(&mut self, m:u8)->u8 {
        self.cc_c = (m & 0x80)!=0; let res = m.wrapping_shl(1); self.cc_n = (res & 0x80)!=0; self.cc_z = res==0; self.cc_v = ((m ^ res) & 0x80)!=0; res
    }
    fn rmw_rol(&mut self, m:u8)->u8 {
        let cin = if self.cc_c {1} else {0}; self.cc_c = (m & 0x80)!=0; let res = ((m as u16)<<1 | cin as u16) & 0xFF; let r = res as u8; self.cc_n = (r & 0x80)!=0; self.cc_z = r==0; self.cc_v = ((m ^ r) & 0x80)!=0; r
    }
    fn rmw_dec(&mut self, m:u8)->u8 {
        let res = m.wrapping_sub(1); self.update_nz8(res); self.cc_v = res==0x7F; // C unaffected
        res
    }
    fn rmw_inc(&mut self, m:u8)->u8 {
        let res = m.wrapping_add(1); self.update_nz8(res); self.cc_v = res==0x80; // C unaffected
        res
    }
    fn rmw_tst(&mut self, m:u8)->u8 {
        self.cc_n = (m & 0x80)!=0; self.cc_z = m==0; self.cc_v=false; self.cc_c=false; m
    }
    fn rmw_clr(&mut self)->u8 {
        self.cc_n=false; self.cc_z=true; self.cc_v=false; self.cc_c=false; 0
    }
    fn service_irq(&mut self){
        if self.trace { println!("[IRQ SERVICE]"); }
        // If WAI already pushed the full frame, do NOT push it again.
        if !self.wai_pushed_frame {
            // Push order (first to last): PC, U, Y, X, DP, B, A, CC so that CC is on top for RTI pop.
            self.cc_e = true; // full frame indicator
            let saved_pc = self.pc;
            self.push16(saved_pc);
            self.push16(self.u); self.push16(self.y); self.push16(self.x);
            self.push8(self.dp); self.push8(self.b); self.push8(self.a);
            let cc = self.pack_cc(); self.push8(cc);
        } else {
            // Frame already on stack from WAI; clear marker so a nested IRQ (shouldn't happen) would push normally.
            self.wai_pushed_frame = false;
        }
        self.cc_i = true; // mask further IRQs until RTI restores CC
        let lo = self.read8(VEC_IRQ); let hi = self.read8(VEC_IRQ+1); self.pc = ((hi as u16)<<8)|lo as u16;
        self.irq_pending = false; self.wai_halt = false; self.in_irq_handler = true;
        self.via_irq_count += 1; // legacy field
        self.irq_count = self.irq_count.wrapping_add(1);
    }
    fn service_firq(&mut self){
        if self.trace { println!("[FIRQ SERVICE]"); }
        // Keep simple partial frame behavior (not currently used after remap)
        self.cc_e = false;
        let saved_pc = self.pc; self.push16(saved_pc);
        let cc = self.pack_cc(); self.push8(cc);
        self.cc_i = true; self.cc_f = true;
        let lo = self.read8(VEC_FIRQ); let hi = self.read8(VEC_FIRQ+1); self.pc = ((hi as u16)<<8)|lo as u16;
        self.firq_pending = false; self.wai_halt = false; self.in_irq_handler = true;
        self.firq_count = self.firq_count.wrapping_add(1);
    }
    fn service_nmi(&mut self){
        if self.trace { println!("[NMI SERVICE]"); }
        self.cc_e = true; // full frame
        let saved_pc = self.pc;
        self.push16(saved_pc);
        self.push16(self.u); self.push16(self.y); self.push16(self.x);
        self.push8(self.dp); self.push8(self.b); self.push8(self.a);
        let cc = self.pack_cc(); self.push8(cc);
        self.cc_i = true; // mask normal IRQs
        let lo = self.read8(VEC_NMI); let hi = self.read8(VEC_NMI+1); self.pc = ((hi as u16)<<8)|lo as u16;
        self.nmi_pending = false; self.wai_halt = false; self.in_irq_handler = true;
    }
    fn service_swi_generic(&mut self, vec:u16, label:&str){
        if self.trace { println!("[{} SERVICE]", label); }
        // SWI / SWI2 / SWI3 all push full frame just like IRQ, but set F flag per spec (F=1) and E=1.
        self.cc_e = true; self.cc_f = true; self.cc_i = true; // mask IRQ, and set F flag
        let saved_pc = self.pc;
        self.push16(saved_pc);
        self.push16(self.u); self.push16(self.y); self.push16(self.x);
        self.push8(self.dp); self.push8(self.b); self.push8(self.a);
        let cc = self.pack_cc(); self.push8(cc);
        let lo = self.read8(vec); let hi = self.read8(vec+1); self.pc = ((hi as u16)<<8)|lo as u16;
        self.wai_halt = false; self.in_irq_handler = true; // treat similarly for clearing WAI
    }
    pub fn step(&mut self) -> bool {
        // Poll VIA IRQ state each instruction boundary
    // Map VIA interrupt line to normal IRQ so BIOS polling / handler expectations match simplified model.
    let via_irq = self.bus.via.irq_asserted();
    self.irq_pending = via_irq;
        // Optional forced frame heuristic: if enabled via env TRACE_FRAME_FORCE=1 and we have at least one WAIT_RECAL call
        // but no frame yet after a large cycle budget, synthesize a frame to unblock higher layers (debug only).
        #[cfg(not(target_arch="wasm32"))]
        {
            if self.bios_frame==0 && self.wait_recal_calls>0 && !self.force_frame_heuristic {
                if std::env::var("TRACE_FRAME_FORCE").ok().as_deref()==Some("1") {
                    // If cycles exceed threshold (e.g., 3 million) since start, force a frame.
                    if self.cycles > 3_000_000 { 
                        self.bios_frame = 1; self.force_frame_heuristic=true; self.last_forced_frame_cycle=self.cycles;
                        if self.trace || self.env_trace_frame() { println!("[FRAME][BIOS] forced heuristic after {} cycles (calls={})", self.cycles, self.wait_recal_calls); }
                    }
                }
            }
        }
        if let Some((addr,val)) = self.bus.last_via_write.take() {
            let reg = (addr & 0x000F) as u8;
            // Record raw event (debug)
            // Experimental mapping of VIA registers to integrator controls:
            //  - 0x00 (ORB): horizontal velocity (signed)
            //  - 0x01 (ORA): vertical velocity (signed)
            //  - 0x0A (SR): treat as direct intensity (placeholder)
            const VEL_SCALE: f32 = 0.5; // arbitrary scaling from raw byte to coordinate units per cycle
            match reg {
                0x00 => { let vx = (val as i8 as f32) * VEL_SCALE; self.integrator.set_velocity(vx, self.integrator_state_vy()); },
                0x01 => { let vy = (val as i8 as f32) * VEL_SCALE; self.integrator.set_velocity(self.integrator_state_vx(), vy); },
                0x0A => { // Shift register write used here as a placeholder intensity channel
                    self.last_intensity = val; self.handle_intensity_change();
                }
                _ => {}
            }
        }
        // IRQ frame fallback deprecated: cycle_frame is authoritative and bios_frame is purely observational now.
        // Leaving previous code path removed intentionally; toggle kept for compatibility only.
        // Loop watch capture before executing opcode (PC points to current)
        match self.pc {
            0xF6CA|0xF6CC|0xF6CE|0xF6D5|0xF6DB => {
                // Sample every 64 iterations to limit overhead
                if (self.loop_watch_count & 0x3F)==0 {
                    let idx=self.loop_watch_idx & 0x0F;
                    self.loop_watch_slots[idx] = LoopSample{ pc:self.pc,a:self.a,b:self.b,x:self.x,y:self.y,u:self.u,s:self.s,dp:self.dp, via_ifr:self.bus.via_ifr(), via_ier:self.bus.via_ier(), via_acr:self.bus.read8(0xD00B), via_pcr:self.bus.read8(0xD00C), cycles:self.cycles };
                    self.loop_watch_idx = (self.loop_watch_idx+1)&0x0F;
                }
                self.loop_watch_count += 1;
            }
            _=>{}
        }
        // Interrupt priority
        if self.nmi_pending { self.service_nmi(); return true; }
    if self.firq_pending && !self.cc_f { self.service_firq(); return true; }
    if self.irq_pending && !self.cc_i { self.service_irq(); return true; }
        if self.wai_halt { // remain halted until an unmasked interrupt serviced; still tick VIA one cycle per step
            self.advance_cycles(1); return true;
        }
    // Fetch opcode directly from internal mem array to honor tests that manipulate cpu.mem
    let pc0 = self.pc; let op = self.mem[self.pc as usize]; self.pc = self.pc.wrapping_add(1);
        if self.jsr_log_len < self.jsr_log.len() {
            match op {
                0xBD => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); let tgt=((hi as u16)<<8)|lo as u16; self.jsr_log[self.jsr_log_len]=tgt; self.jsr_log_len+=1; }
                0x8D => { let off=self.read8(self.pc) as i8 as i16; let tgt=(self.pc as i16 + 1 + off) as u16; self.jsr_log[self.jsr_log_len]=tgt; self.jsr_log_len+=1; }
                _=>{}
            }
        }
        self.opcode_total += 1; self.opcode_counts[op as usize] += 1;
        // Lightweight hotspot sampling for suspicious opcodes (NEG direct 0x00 and extended RMW group tail 0xFF)
        // We track up to 4 most frequent PCs executing each of these opcodes using a small LFU replacement.
        if op==0x00 || op==0xFF {
            let slots = if op==0x00 { &mut self.hot00 } else { &mut self.hotff };
            let mut found=false; let mut empty_idx=None; let mut min_idx=0; let mut min_count= u64::MAX;
            for (i,(pc,count)) in slots.iter_mut().enumerate() {
                if *count==0 && empty_idx.is_none() { empty_idx=Some(i); }
                if *pc==pc0 { *count+=1; found=true; break; }
                if *count < min_count { min_count=*count; min_idx=i; }
            }
            if !found { if let Some(ei)=empty_idx { slots[ei]=(pc0,1); } else { slots[min_idx]=(pc0,1); } }
        }
        if self.trace { print!("{:04X}: {:02X} ", pc0, op); }
        // Base cycle seed (approximate) to refine in opcode handlers
        let mut cyc: u32 = match op {
            // Immediate 2-cycle loads / ops (added ABA 0x1B, ADCB imm 0xC9)
            0x86|0xC6|0x8E|0xCE|0xCC|0x1B|0xC9 => 2,
            // Direct addressing (approx 4 cycles) (added 0xC3 ADDD immediate classification here for timing approx)
            0x91|0x94|0x96|0x97|0x98|0x9A|0x9E|0x9C|0x99|0x9B|0xC3|0xD6|0xD7|0xDA|0xDB|0xDC|0xDD|0xDE|0xDF => 4,
            // Extended addressing group (5 cycles)
            0xB6|0xB7|0xB4|0xB9|0xBB|0xF0|0xF4|0xF6|0xF7|0xF8|0xF9|0xFA|0xFC|0xFD|0xFE|0xFF => 5,
            // Indexed group (5 cycles baseline) (added 0xAE LDX indexed)
            0xA6|0xA7|0xA0|0xA2|0xA4|0xA5|0xA8|0xA9|0xAA|0xAB|0xAE|0xE0|0xE1|0xE3|0xE4|0xE6|0xE7|0xEA|0xEB|0xEC|0xED|0xEE => 5,
            0xBD => 7, 0x9D => 6, 0x39 => 5,
            // Short branches (2 cycles base +1 if taken handled inline)
            0x20|0x22|0x23|0x24|0x25|0x26|0x27|0x28|0x29|0x2A|0x2B|0x2C|0x2D|0x2E|0x2F => 2,
            0x8D => 7, 0x34|0x35|0x36|0x37 => 5, 0x3B => 6, 0x3E => 4,
            // Direct RMW/control cluster
            0x00|0x03|0x04|0x06|0x07|0x08|0x09|0x0A|0x0C|0x0D|0x0E|0x0F|0x16|0x1D => 6,
            // Indexed RMW cluster (baseline)
            0x60|0x63|0x64|0x66|0x67|0x68|0x69|0x6A|0x6C|0x6D|0x6E|0x6F => 6,
            // Extended RMW cluster
            0x70|0x73|0x74|0x76|0x77|0x78|0x79|0x7A|0x7C|0x7D|0x7E|0x7F => 7,
            // Accumulator RMW / tests
            0x40|0x43|0x44|0x46|0x47|0x48|0x49|0x4D|0x50|0x53|0x54|0x56|0x57|0x58|0x59|0x5D|0x4F|0x5F => 2,
            0x5A|0x5C => 2,
            // Immediate compare family (approx)
            0x81|0xA1|0xB1|0xC1|0xD1|0xF1 => 4, // (E1 now classified in indexed group) (0x91 handled in direct group)
            // Misc immediate logical/arith (2 cycles)
            0xC4|0xD4|0x84|0x85|0x89|0x8A|0xC5|0xC8|0xCA|0xCB|0xD8|0xE8 => 2, // (F8 in extended group)
            0x30|0x31|0x32|0x33 => 5,
            0x1A|0x1C|0x12|0x19 => 2,
            0x1E|0x1F => 6,
            _ => 1,
        };
        match op {
            0x4C => { // INCA (ensure early dispatch)
                let old = self.a; let res = old.wrapping_add(1); self.a = res; self.update_nz8(res); self.cc_v = res==0x80; if self.trace { println!("INCA -> {:02X}", res);} }
            // -------------------------------------------------------------------------
            // Extended memory RMW/JMP cluster 0x70..0x7F subset
            // -------------------------------------------------------------------------
            0x70|0x73|0x74|0x76|0x77|0x78|0x79|0x7A|0x7C|0x7D|0x7E|0x7F => {
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                match op {
                    0x70 => { let m=self.read8(addr); let r=self.rmw_neg(m); self.write8(addr,r); if self.trace { println!("NEG ${:04X} -> {:02X}", addr,r);} }
                    0x73 => { let m=self.read8(addr); let r=self.rmw_com(m); self.write8(addr,r); if self.trace { println!("COM ${:04X} -> {:02X}", addr,r);} }
                    0x74 => { let m=self.read8(addr); let r=self.rmw_lsr(m); self.write8(addr,r); if self.trace { println!("LSR ${:04X} -> {:02X}", addr,r);} }
                    0x76 => { let m=self.read8(addr); let r=self.rmw_ror(m); self.write8(addr,r); if self.trace { println!("ROR ${:04X} -> {:02X}", addr,r);} }
                    0x77 => { let m=self.read8(addr); let r=self.rmw_asr(m); self.write8(addr,r); if self.trace { println!("ASR ${:04X} -> {:02X}", addr,r);} }
                    0x78 => { let m=self.read8(addr); let r=self.rmw_asl(m); self.write8(addr,r); if self.trace { println!("ASL ${:04X} -> {:02X}", addr,r);} }
                    0x79 => { let m=self.read8(addr); let r=self.rmw_rol(m); self.write8(addr,r); if self.trace { println!("ROL ${:04X} -> {:02X}", addr,r);} }
                    0x7A => { let m=self.read8(addr); let r=self.rmw_dec(m); self.write8(addr,r); if self.trace { println!("DEC ${:04X} -> {:02X}", addr,r);} }
                    0x7C => { let m=self.read8(addr); let r=self.rmw_inc(m); self.write8(addr,r); if self.trace { println!("INC ${:04X} -> {:02X}", addr,r);} }
                    0x7D => { let m=self.read8(addr); let _=self.rmw_tst(m); if self.trace { println!("TST ${:04X}", addr);} }
                    0x7E => { self.pc = addr; if self.trace { println!("JMP ${:04X}", addr);} }
                    0x7F => { let _=self.rmw_clr(); self.write8(addr,0); if self.trace { println!("CLR ${:04X}", addr);} }
                    _ => {}
                }
            }
            0x12 => { if self.trace { println!("NOP"); } }
            0x3E => { // WAI: Push full CPU state then halt until interrupt; do not push again inside IRQ service.
                if self.trace { println!("WAI (enter)"); }
                if !self.wai_halt { // Only push once if repeated WAI while already halted (unlikely)
                    self.cc_e = true; // full frame indicator per 6809 conventions for RTI
                    // Push in same order service_irq expects so RTI works uniformly
                    let saved_pc = self.pc; // points to next instruction after WAI
                    self.push16(saved_pc);
                    self.push16(self.u); self.push16(self.y); self.push16(self.x);
                    self.push8(self.dp); self.push8(self.b); self.push8(self.a);
                    let cc = self.pack_cc(); self.push8(cc);
                    self.wai_pushed_frame = true;
                }
                self.wai_halt = true; return true;
            }
            // --- Begin large opcode set from legacy implementation (partial) ---
            // -------------------------------------------------------------------------
            // Accumulator RMW A
            // -------------------------------------------------------------------------
            0x40 => { let r=self.rmw_neg(self.a); self.a=r; if self.trace { println!("NEGA -> {:02X}", r);} }
            0x43 => { let r=self.rmw_com(self.a); self.a=r; if self.trace { println!("COMA -> {:02X}", r);} }
            0x44 => { let r=self.rmw_lsr(self.a); self.a=r; if self.trace { println!("LSRA -> {:02X}", r);} }
            0x46 => { let r=self.rmw_ror(self.a); self.a=r; if self.trace { println!("RORA -> {:02X}", r);} }
            0x47 => { let r=self.rmw_asr(self.a); self.a=r; if self.trace { println!("ASRA -> {:02X}", r);} }
            0x48 => { let r=self.rmw_asl(self.a); self.a=r; if self.trace { println!("ASLA -> {:02X}", r);} }
            0x49 => { let r=self.rmw_rol(self.a); self.a=r; if self.trace { println!("ROLA -> {:02X}", r);} }
            0x4D => { let v=self.a; self.cc_n=(v&0x80)!=0; self.cc_z=v==0; self.cc_v=false; self.cc_c=false; if self.trace { println!("TSTA"); } }
            // Accumulator RMW B
            0x50 => { let r=self.rmw_neg(self.b); self.b=r; if self.trace { println!("NEGB -> {:02X}", r);} }
            0x53 => { let r=self.rmw_com(self.b); self.b=r; if self.trace { println!("COMB -> {:02X}", r);} }
            0x54 => { let r=self.rmw_lsr(self.b); self.b=r; if self.trace { println!("LSRB -> {:02X}", r);} }
            0x56 => { let r=self.rmw_ror(self.b); self.b=r; if self.trace { println!("RORB -> {:02X}", r);} }
            0x57 => { let r=self.rmw_asr(self.b); self.b=r; if self.trace { println!("ASRB -> {:02X}", r);} }
            0x58 => { let r=self.rmw_asl(self.b); self.b=r; if self.trace { println!("ASLB -> {:02X}", r);} }
            0x59 => { let r=self.rmw_rol(self.b); self.b=r; if self.trace { println!("ROLB -> {:02X}", r);} }
            0x5D => { let v=self.b; self.cc_n=(v&0x80)!=0; self.cc_z=v==0; self.cc_v=false; self.cc_c=false; if self.trace { println!("TSTB"); } }
            0x5A => { // DECB
                let old = self.b; let res = old.wrapping_sub(1); self.b = res; self.update_nz8(res); self.cc_v = res==0x7F; if self.trace { println!("DECB -> {:02X}", res);} }
            0x5C => { // INCB
                let old = self.b; let res = old.wrapping_add(1); self.b = res; self.update_nz8(res); self.cc_v = res==0x80; if self.trace { println!("INCB -> {:02X}", res);} }
            0x4F => { // CLRA
                self.a = 0; self.cc_n=false; self.cc_z=true; self.cc_v=false; self.cc_c=false; if self.trace { println!("CLRA"); }
            }
            0x5F => { // CLRB
                self.b = 0; self.cc_n=false; self.cc_z=true; self.cc_v=false; self.cc_c=false; if self.trace { println!("CLRB"); }
            }
            0x6F => { // CLR indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); self.write8(ea,0); self.cc_n=false; self.cc_z=true; self.cc_v=false; self.cc_c=false; if self.trace { println!("CLR [{}]", ea); } }
            // (Removed duplicate indexed RMW cluster; implemented explicitly below)
            // Load/store & arithmetic subset (partial — extend as needed)
            0x86 => { let v=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.a=v; self.update_nz8(self.a); if self.trace { println!("LDA #${:02X}", self.a);} }
            0xC6 => { let v=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.b=v; self.update_nz8(self.b); if self.trace { println!("LDB #${:02X}", self.b);} }
            0x8B => { // ADDA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let a=self.a; let res=(a as u16)+(imm as u16); let r=(res & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(res & 0x100)!=0; self.cc_v=(!((a^imm) as u16) & ((a^r) as u16) & 0x80)!=0; if self.trace { println!("ADDA #${:02X}", imm);} }
            0xC0 => { // SUBB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let b0=self.b; let res=b0.wrapping_sub(imm); self.b=res; self.flags_sub8(b0,imm,res); if self.trace { println!("SUBB #${:02X} -> {:02X}", imm,res);} }
            0xC1 => { // CMPB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let b0=self.b; let res=b0.wrapping_sub(imm); self.flags_sub8(b0,imm,res); if self.trace { println!("CMPB #${:02X}", imm);} }
            0x81 => { // CMPA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let a=self.a; let res=a.wrapping_sub(imm); self.flags_sub8(a,imm,res); if self.trace { println!("CMPA #${:02X}", imm);} }
            0x8D => { // BSR
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); let ret=self.pc; self.push16(ret); let target=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BSR {:04X}", target);} self.pc=target; }
            0x39 => { // RTS
                let depth_before = self.call_stack.len();
                if let Some(r)=self.call_stack.pop(){ if self.trace { println!("RTS -> {:04X}", r);} self.pc=r; } else if self.trace { println!("RTS (empty)"); }
                if self.in_irq_handler { self.wai_halt=false; self.in_irq_handler=false; }
                if let Some(d)=self.wait_recal_depth { if depth_before == d && self.call_stack.len() == d { self.bios_frame = self.bios_frame.wrapping_add(1); self.wait_recal_depth=None; self.wait_recal_returns=self.wait_recal_returns.wrapping_add(1); if self.trace || self.env_trace_frame() { println!("[FRAME][BIOS] increment (RTS) bios_frame={} returns={}", self.bios_frame, self.wait_recal_returns); } } }
            }
            0x3B => { // RTI
                let pull8 = |cpu: &mut CPU| { let v = cpu.read8(cpu.s); cpu.s = cpu.s.wrapping_add(1); v };
                let pull16 = |cpu: &mut CPU| { let lo = pull8(cpu); let hi = pull8(cpu); ((hi as u16)<<8)|lo as u16 };
                let cc = pull8(self); self.unpack_cc(cc);
                if self.cc_e { self.a=pull8(self); self.b=pull8(self); self.dp=pull8(self); self.x=pull16(self); self.y=pull16(self); self.u=pull16(self); self.pc=pull16(self); } else { self.pc=pull16(self); }
                if self.trace { println!("RTI -> {:04X}", self.pc); }
                self.in_irq_handler=false; self.wai_halt=false;
                if let Some(d)=self.wait_recal_depth { if self.call_stack.len()==d { self.bios_frame=self.bios_frame.wrapping_add(1); self.wait_recal_depth=None; self.wait_recal_returns=self.wait_recal_returns.wrapping_add(1); if self.trace || self.env_trace_frame() { println!("[FRAME][BIOS] increment (RTI) bios_frame={} returns={}", self.bios_frame, self.wait_recal_returns); } } }
            }
            0x3F => { self.service_swi_generic(VEC_SWI, "SWI"); }
            0x34 => { // PSHS
                let mask=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                if (mask & 0x01)!=0 { self.push8(self.pack_cc()); }
                if (mask & 0x02)!=0 { self.push8(self.a); }
                if (mask & 0x04)!=0 { self.push8(self.b); }
                if (mask & 0x08)!=0 { self.push8(self.dp); }
                if (mask & 0x10)!=0 { self.push16(self.x); }
                if (mask & 0x20)!=0 { self.push16(self.y); }
                if (mask & 0x40)!=0 { self.push16(self.u); }
                if (mask & 0x80)!=0 { self.push16(self.pc); }
                if self.trace { println!("PSHS ${:02X}", mask); }
            }
            0x35 => { // PULS
                let mask=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                if (mask & 0x80)!=0 { self.pc=self.pop16(); }
                if (mask & 0x40)!=0 { self.u=self.pop16(); }
                if (mask & 0x20)!=0 { self.y=self.pop16(); }
                if (mask & 0x10)!=0 { self.x=self.pop16(); }
                if (mask & 0x08)!=0 { self.dp=self.pop8(); }
                if (mask & 0x04)!=0 { self.b=self.pop8(); }
                if (mask & 0x02)!=0 { self.a=self.pop8(); }
                if (mask & 0x01)!=0 { let cc=self.pop8(); self.unpack_cc(cc); }
                if self.trace { println!("PULS ${:02X}", mask); }
            }
            0x36 => { // PSHU (reuse helpers with temp swap)
                let mask=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let orig_s=self.s; let sp=self.u; self.s=sp;
                if (mask & 0x01)!=0 { self.push8(self.pack_cc()); }
                if (mask & 0x02)!=0 { self.push8(self.a); }
                if (mask & 0x04)!=0 { self.push8(self.b); }
                if (mask & 0x08)!=0 { self.push8(self.dp); }
                if (mask & 0x10)!=0 { self.push16(self.x); }
                if (mask & 0x20)!=0 { self.push16(self.y); }
                if (mask & 0x40)!=0 { self.push16(orig_s); }
                if (mask & 0x80)!=0 { self.push16(self.pc); }
                self.u=self.s; self.s=orig_s; if self.trace { println!("PSHU ${:02X}", mask); }
            }
            0x37 => { // PULU
                let mask=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let orig_s=self.s; let sp=self.u; self.s=sp;
                if (mask & 0x80)!=0 { self.pc=self.pop16(); }
                if (mask & 0x40)!=0 { self.s=self.pop16(); }
                if (mask & 0x20)!=0 { self.y=self.pop16(); }
                if (mask & 0x10)!=0 { self.x=self.pop16(); }
                if (mask & 0x08)!=0 { self.dp=self.pop8(); }
                if (mask & 0x04)!=0 { self.b=self.pop8(); }
                if (mask & 0x02)!=0 { self.a=self.pop8(); }
                if (mask & 0x01)!=0 { let cc=self.pop8(); self.unpack_cc(cc); }
                let new_sp=self.s; self.s=orig_s; self.u=new_sp; if self.trace { println!("PULU ${:02X}", mask); }
            }
            0x1A => { let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let mut cc=self.pack_cc(); cc|=imm; self.unpack_cc(cc); if self.trace { println!("ORCC #${:02X}", imm);} }
            0x1C => { let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let mut cc=self.pack_cc(); cc&=imm; self.unpack_cc(cc); if self.trace { println!("ANDCC #${:02X}", imm);} }
            0x1B => { // ABA (A = A + B)
                let a0=self.a; let b0=self.b; let sum=(a0 as u16)+(b0 as u16); let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a0^b0) as u16) & ((a0^r) as u16) & 0x80)!=0; if self.trace { println!("ABA -> {:02X}", r);} }
            0x1F => { // TFR src,dst
                let reg = self.read8(self.pc); self.pc = self.pc.wrapping_add(1);
                let src = (reg >> 4) & 0x0F; let dst = reg & 0x0F;
                let w_src = self.reg_width(src); let w_dst = self.reg_width(dst);
                if w_src != 0 && w_src == w_dst {
                    let val = self.read_reg(src);
                    self.write_reg(dst, val);
                    if self.trace { println!("TFR {}->{}", src, dst); }
                } else if self.trace { println!("TFR (ignored) src={} dst={} w{} w{}", src,dst,w_src,w_dst); }
            }
            0x1E => { // EXG src,dst
                let reg = self.read8(self.pc); self.pc = self.pc.wrapping_add(1);
                let r1 = (reg >> 4) & 0x0F; let r2 = reg & 0x0F;
                let w1 = self.reg_width(r1); let w2 = self.reg_width(r2);
                if w1 != 0 && w1 == w2 {
                    let v1 = self.read_reg(r1); let v2 = self.read_reg(r2);
                    self.write_reg(r1, v2); self.write_reg(r2, v1);
                    if self.trace { println!("EXG {}<->{}", r1, r2); }
                } else if self.trace { println!("EXG (ignored) {} {}", r1, r2); }
            }
            0x20 => { let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BRA {:04X}", new);} self.pc=new; cyc=3; }
            0x16 => { // LBRA
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let off=((hi as u16)<<8)|lo as u16; let target=self.pc.wrapping_add(off as i16 as u16); if self.trace { println!("LBRA {:04X}", target);} self.pc=target; cyc=5; }
            0x23 => { // BLS (C or Z set)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if self.cc_c || self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BLS {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BLS not"); }
            }
            0x22 => { // BHI (C=0 and Z=0)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if !self.cc_c && !self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BHI {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BHI not"); }
            }
            0x24 => { // BCC (Carry clear)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if !self.cc_c { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BCC {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BCC not"); }
            }
            0x26 => { let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if !self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BNE {:04X}", new);} self.pc=new; cyc=3;} else if self.trace { println!("BNE not"); } }
            0x27 => { let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if self.cc_z { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BEQ {:04X}", new);} self.pc=new; cyc=3;} else if self.trace { println!("BEQ not"); } }
            0x29 => { // BVS (V set)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if self.cc_v { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BVS {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BVS not"); } }
            0x1D => { // SEX
                self.a = if (self.b & 0x80)!=0 {0xFF} else {0x00}; let d=self.d(); self.update_nz16(d); self.cc_v=false; if self.trace { println!("SEX -> D={:04X}", d);} }
            0x30|0x31|0x32|0x33 => { let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_)=self.decode_indexed(post,self.x,self.y,self.u,self.s); match op { 0x30=>{ self.x=ea; self.update_nz16(self.x); if self.trace { println!("LEAX {:04X}", ea);} } 0x31=>{ self.y=ea; self.update_nz16(self.y); if self.trace { println!("LEAY {:04X}", ea);} } 0x32=>{ self.s=ea; if self.trace { println!("LEAS {:04X}", ea);} } _=>{ self.u=ea; self.update_nz16(self.u); if self.trace { println!("LEAU {:04X}", ea);} } } }
            0x8E => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); self.x=((hi as u16)<<8)|lo as u16; if self.trace { println!("LDX #${:04X}", self.x);} }
            0xCE => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); self.u=((hi as u16)<<8)|lo as u16; if self.trace { println!("LDU #${:04X}", self.u);} }
            0xCC => { // LDD immediate (A=high, B=low)
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2);
                self.a=hi; self.b=lo; self.update_nz16(self.d()); if self.trace { println!("LDD #${:02X}{:02X}", hi, lo);} }
            0xDC => { // LDD direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16;
                let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1)); self.a=hi; self.b=lo; self.update_nz16(self.d()); if self.trace { println!("LDD ${:04X}", addr);} }
            0xDE => { // LDU direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16;
                let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1)); self.u=((hi as u16)<<8)|lo as u16; self.update_nz16(self.u); if self.trace { println!("LDU ${:04X}", addr);} }
            0xDD => { // STD direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16;
                self.write8(addr,self.a); self.write8(addr.wrapping_add(1), self.b); self.update_nz16(self.d()); if self.trace { println!("STD ${:04X}", addr);} }
            0xDF => { // STU direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let hi=(self.u>>8) as u8; let lo=self.u as u8;
                self.write8(addr,hi); self.write8(addr.wrapping_add(1),lo); self.update_nz16(self.u); if self.trace { println!("STU ${:04X}", addr);} }
            0xFD => { // STD extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                self.write8(addr, self.a); self.write8(addr.wrapping_add(1), self.b); self.update_nz16(self.d()); if self.trace { println!("STD ${:04X}", addr);} }
            0xFE => { // LDU extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1)); self.u=((hi2 as u16)<<8)|lo2 as u16; self.update_nz16(self.u); if self.trace { println!("LDU ${:04X}", addr);} }
            0xB6 => { // LDA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                let v=self.read8(addr); self.a=v; self.update_nz8(v); if self.trace { println!("LDA ${:04X}", addr);} }
            0xB7 => { // STA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                let v=self.a; self.write8(addr,v); self.update_nz8(v); if self.trace { println!("STA ${:04X} -> {:02X}", addr,v);} }
            0xB1 => { // CMPA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let a0=self.a; let res=a0.wrapping_sub(m); self.flags_sub8(a0,m,res); if self.trace { println!("CMPA ${:04X}", addr);} }
            0xBE => { // LDX extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2);
                let addr=((hi as u16)<<8)|lo as u16; let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1));
                let val=((hi2 as u16)<<8)|lo2 as u16; self.x=val; self.update_nz16(val);
                if self.trace { println!("LDX ${:04X} -> {:04X}", addr,val); }
            }
            0xBF => { // STX extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16;
                self.write8(addr, (self.x>>8) as u8); self.write8(addr.wrapping_add(1), self.x as u8); self.update_nz16(self.x); if self.trace { println!("STX ${:04X}", addr);} }
            0x80 => { // SUBA immediate
                let imm = self.read8(self.pc); self.pc = self.pc.wrapping_add(1);
                let a0 = self.a; let res = a0.wrapping_sub(imm);
                self.a = res; self.flags_sub8(a0, imm, res);
                if self.trace { println!("SUBA #${:02X} -> {:02X}", imm, res); }
            }
            0xC4 => { // ANDB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.b &= imm; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ANDB #${:02X}", imm);} }
            0x85 => { // BITA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let r=self.a & imm; self.cc_n=(r & 0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITA #${:02X}", imm);} }
            0x89 => { // ADCA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let a=self.a; let c= if self.cc_c {1}else{0}; let sum=(a as u16)+(imm as u16)+c as u16; let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^imm) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADCA #${:02X}", imm);} }
            0x90 => { // SUBA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let a0=self.a; let res=a0.wrapping_sub(m); self.a=res; self.flags_sub8(a0,m,res); if self.trace { println!("SUBA ${:04X} -> {:02X}", addr,res);} }
            0x99 => { // ADCA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let a=self.a; let c= if self.cc_c {1}else{0}; let sum=(a as u16)+(m as u16)+c as u16; let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADCA ${:04X}", addr);} }
            0x9B => { // ADDA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let a=self.a; let sum=(a as u16)+(m as u16); let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADDA ${:04X}", addr);} }
            0x9C => { // CMPX direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1)); let val=((hi as u16)<<8)|lo as u16; let x0=self.x; let res=x0.wrapping_sub(val); self.flags_sub16(x0,val,res); if self.trace { println!("CMPX ${:04X}", addr);} }
            0x9A => { // ORA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.a |= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ORA ${:04X}", addr);} }
            0x06 => { // ROR direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let r=self.rmw_ror(m); self.write8(addr,r); if self.trace { println!("ROR ${:04X} -> {:02X}", addr,r);} }
            0xE1 => { // CMPB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let b0=self.b; let res=b0.wrapping_sub(m); self.flags_sub8(b0,m,res); if self.trace { println!("CMPB [{}]", ea);} }
            0x09 => { // ROL direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let r=self.rmw_rol(m); self.write8(addr,r); if self.trace { println!("ROL ${:04X} -> {:02X}", addr,r);} }
            0x0C => { // INC direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let r=self.rmw_inc(m); self.write8(addr,r); if self.trace { println!("INC ${:04X} -> {:02X}", addr,r);} }
            0x0D => { // TST direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.rmw_tst(m); if self.trace { println!("TST ${:04X}", addr);} }
            0x0E => { // JMP direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; self.pc=addr; if self.trace { println!("JMP ${:04X}", addr);} }
            0x0F => { // CLR direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; self.rmw_clr(); self.write8(addr,0); if self.trace { println!("CLR ${:04X}", addr);} }
            0xA0 => { // SUBA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let a0=self.a; let res=a0.wrapping_sub(m); self.a=res; self.flags_sub8(a0,m,res); if self.trace { println!("SUBA [{}] -> {:02X}", ea,res);} }
            0xA4 => { // ANDA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.a &= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ANDA [{}]", ea);} }
            0xA9 => { // ADCA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let a=self.a; let c= if self.cc_c {1}else{0}; let sum=(a as u16)+(m as u16)+c as u16; let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADCA [{}]", ea);} }
            0xAA => { // ORA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.a |= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ORA [{}]", ea);} }
            0xAB => { // ADDA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let a=self.a; let sum=(a as u16)+(m as u16); let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADDA [{}]", ea);} }
            0xE0 => { // SUBB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let b0=self.b; let res=b0.wrapping_sub(m); self.b=res; self.flags_sub8(b0,m,res); if self.trace { println!("SUBB [{}] -> {:02X}", ea,res);} }
            0xB4 => { // ANDA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); self.a &= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ANDA ${:04X}", addr);} }
            0xB9 => { // ADCA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let a=self.a; let c= if self.cc_c {1}else{0}; let sum=(a as u16)+(m as u16)+c as u16; let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADCA ${:04X}", addr);} }
            0xBB => { // ADDA extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let a=self.a; let sum=(a as u16)+(m as u16); let r=(sum & 0xFF) as u8; self.a=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((a^m) as u16)&((a^r) as u16)&0x80)!=0; if self.trace { println!("ADDA ${:04X}", addr);} }
            0xE3 => { // ADDD indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); let val=((hi as u16)<<8)|lo as u16; let d0=self.d(); let sum=(d0 as u32)+(val as u32); let res=(sum & 0xFFFF) as u16; self.set_d(res); self.update_nz16(res); self.cc_c=(sum & 0x10000)!=0; self.cc_v=(!((d0^val) as u32) & ((d0^res) as u32) & 0x8000)!=0; if self.trace { println!("ADDD [{}] -> {:04X}", ea,res);} }
            0xE4 => { // ANDB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.b &= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ANDB [{}]", ea);} }
            0xEA => { // ORB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.b |= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ORB [{}]", ea);} }
            0xF0 => { // SUBB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let b0=self.b; let res=b0.wrapping_sub(m); self.b=res; self.flags_sub8(b0,m,res); if self.trace { println!("SUBB ${:04X} -> {:02X}", addr,res);} }
            0xF4 => { // ANDB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); self.b &= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ANDB ${:04X}", addr);} }
            0xF3 => { // ADDD extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1)); let val=((hi2 as u16)<<8)|lo2 as u16; let d0=self.d(); let sum=(d0 as u32)+(val as u32); let res=(sum & 0xFFFF) as u16; self.set_d(res); self.update_nz16(res); self.cc_c=(sum & 0x10000)!=0; self.cc_v=(!((d0^val) as u32)&((d0^res) as u32)&0x8000)!=0; if self.trace { println!("ADDD ${:04X} -> {:04X}", addr,res);} }
            0xCA => { // ORB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.b |= imm; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ORB #${:02X}", imm);} }
            0xC5 => { // BITB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let r=self.b & imm; self.cc_n=(r & 0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITB #${:02X}", imm);} }
            0xDA => { // ORB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.b |= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ORB ${:04X}", addr);} }
            0xFA => { // ORB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); self.b |= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ORB ${:04X}", addr);} }
            0xF8 => { // EORB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); self.b ^= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("EORB ${:04X}", addr);} }
            0x04 => { // LSR direct (moved from decode_indexed_basic)
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.cc_c=(m & 0x01)!=0; let res=m>>1; self.write8(addr,res); self.cc_n=false; self.cc_z=res==0; self.cc_v=false; if self.trace { println!("LSR ${:04X} -> {:02X}", addr,res);} }
            0x3D => { if self.trace { println!("SYNC (treated as NOP)"); } }
            0x9D => { // JSR direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr=((self.dp as u16)<<8)|off as u16;
                if self.trace { println!("JSR ${:04X} (direct)", addr);} 
                if addr>=0xF000 { 
                    if !self.bios_present { if self.trace { println!("Missing BIOS ${:04X}", addr);} return false; }
                    self.record_bios_call(addr);
                    let ret=self.pc; self.call_stack.push(ret); self.pc=addr;
                } else { let ret=self.pc; self.call_stack.push(ret); self.pc=addr; }
                cyc=7; }
            0xBD => { // JSR absolute
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2);
                let addr=((hi as u16)<<8)|lo as u16; if self.trace { println!("JSR ${:04X}", addr);} 
                if addr>=0xF000 { 
                    if !self.bios_present { if self.trace { println!("Missing BIOS ${:04X}", addr);} return false; }
                    self.record_bios_call(addr);
                    let ret=self.pc; self.call_stack.push(ret); self.pc=addr;
                } else { let ret=self.pc; self.call_stack.push(ret); self.pc=addr; }
                cyc=7; }
            0x97 => { // STA direct
                let off = self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr = ((self.dp as u16) << 8) | off as u16;
                let val = self.a; self.write8(addr, val);
                self.update_nz8(val);
                if self.trace { println!("STA ${:04X} -> {:02X}", addr, val); }
            }
            0x94 => { // ANDA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr);
                self.a &= m; self.update_nz8(self.a); self.cc_v=false;
                if self.trace { println!("ANDA ${:04X}", addr); }
            }
            0x9E => { // LDX direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr=((self.dp as u16)<<8)|off as u16;
                let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1));
                self.x=((hi as u16)<<8)|lo as u16; self.update_nz16(self.x);
                if self.trace { println!("LDX ${:04X} -> {:04X}", addr, self.x); }
            }
            0x9F => { // STX direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr=((self.dp as u16)<<8)|off as u16; let x=self.x;
                self.write8(addr,(x>>8) as u8); self.write8(addr.wrapping_add(1), x as u8);
                self.update_nz16(x); if self.trace { println!("STX ${:04X}", addr); }
            }
            0x96 => { // LDA direct (needed for VIA register reads in BIOS interrupt handlers)
                let off = self.read8(self.pc); self.pc = self.pc.wrapping_add(1);
                let addr = ((self.dp as u16) << 8) | off as u16;
                let v = self.read8(addr); self.a = v; self.update_nz8(v);
                if self.trace { println!("LDA ${:04X} -> {:02X}", addr, v); }
            }
            0xD7 => { // STB direct
                let off = self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr = ((self.dp as u16) << 8) | off as u16;
                let val = self.b; self.write8(addr, val);
                self.update_nz8(val);
                if self.trace { println!("STB ${:04X} -> {:02X}", addr, val); }
            }
            0xD4 => { // ANDB direct
                let off = self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr = ((self.dp as u16) << 8) | off as u16; let m = self.read8(addr);
                self.b &= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("ANDB ${:04X}", addr);} }
            0xD6 => { // LDB direct
                let off = self.read8(self.pc); self.pc = self.pc.wrapping_add(1);
                let addr = ((self.dp as u16) << 8) | off as u16;
                let v = self.read8(addr); self.b = v; self.update_nz8(v);
                if self.trace { println!("LDB ${:04X} -> {:02X}", addr, v); }
            }
            0xE6 => { // LDB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let v=self.read8(ea); self.b=v; self.update_nz8(v); if self.trace { println!("LDB [{}] -> {:02X}", ea,v);} }
            0xA6 => { // LDA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let v=self.read8(ea); self.a=v; self.update_nz8(v); if self.trace { println!("LDA [{}] -> {:02X}", ea,v);} }
            0xA7 => { // STA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let v=self.a; self.write8(ea,v); self.update_nz8(v); if self.trace { println!("STA [{}] -> {:02X}", ea,v);} }
            0xAE => { // LDX indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); let val=((hi as u16)<<8)|lo as u16; self.x=val; self.update_nz16(val); if self.trace { println!("LDX [{}] -> {:04X}", ea,val);} }
            0xA1 => { // CMPA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let a=self.a; let res=a.wrapping_sub(m); self.flags_sub8(a,m,res); if self.trace { println!("CMPA [{}]", ea);} }
            0xAF => { // STX indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); self.write8(ea,(self.x>>8) as u8); self.write8(ea.wrapping_add(1), self.x as u8); self.update_nz16(self.x); if self.trace { println!("STX [{}]", ea);} }
            0xED => { // STD indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); self.write8(ea,self.a); self.write8(ea.wrapping_add(1), self.b); self.update_nz16(self.d()); if self.trace { println!("STD [{}]", ea);} }
            0xF1 => { // CMPB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let b0=self.b; let res=b0.wrapping_sub(m); self.flags_sub8(b0,m,res); if self.trace { println!("CMPB ${:04X}", addr);} }
            0x10 => { // prefix group 1
                let bop=self.read8(self.pc);
                // Snapshot flags for branch condition evaluation
                let f_c = self.cc_c; let f_z = self.cc_z; let f_v = self.cc_v; let f_n = self.cc_n;
                match bop { 0x8E => { self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); self.y=((hi as u16)<<8)|lo as u16; self.update_nz16(self.y); if self.trace { println!("LDY #${:04X}", self.y);} }
                    0x9E => { // LDY direct
                        self.pc=self.pc.wrapping_add(1); let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                        let addr=((self.dp as u16)<<8)|off as u16; let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1)); self.y=((hi as u16)<<8)|lo as u16; self.update_nz16(self.y); if self.trace { println!("LDY ${:04X} -> {:04X}", addr, self.y);} }
                    0xAE => { // LDY indexed
                        self.pc=self.pc.wrapping_add(1); let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); self.y=((hi as u16)<<8)|lo as u16; self.update_nz16(self.y); if self.trace { println!("LDY [{}] -> {:04X}", ea, self.y);} }
                    0xBE => { // LDY extended
                        self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1)); self.y=((hi2 as u16)<<8)|lo2 as u16; self.update_nz16(self.y); if self.trace { println!("LDY ${:04X} -> {:04X}", addr, self.y);} }
                    0x9F => { // STY direct
                        self.pc=self.pc.wrapping_add(1); let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let y=self.y; self.write8(addr,(y>>8) as u8); self.write8(addr.wrapping_add(1), y as u8); self.update_nz16(y); if self.trace { println!("STY ${:04X}", addr);} }
                    0xAF => { // STY indexed
                        self.pc=self.pc.wrapping_add(1); let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let y=self.y; self.write8(ea,(y>>8) as u8); self.write8(ea.wrapping_add(1), y as u8); self.update_nz16(y); if self.trace { println!("STY [{}]", ea);} }
                    0xBF => { // STY extended
                        self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let y=self.y; self.write8(addr,(y>>8) as u8); self.write8(addr.wrapping_add(1), y as u8); self.update_nz16(y); if self.trace { println!("STY ${:04X}", addr);} }
                    0xCE => { self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); self.s=((hi as u16)<<8)|lo as u16; self.update_nz16(self.s); if self.trace { println!("LDS #${:04X}", self.s);} }
                    // CMPD family: immediate (0x83), direct (0x93), indexed (0xA3) NEW, extended (0xB3)
                    0x83|0x93|0xA3|0xB3 => { self.pc=self.pc.wrapping_add(1); let val = match bop {
                        0x83 => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); ((hi as u16)<<8)|lo as u16 }
                        0x93 => { let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                        0xA3 => { let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); ((self.read8(ea) as u16)<<8)|self.read8(ea.wrapping_add(1)) as u16 }
                        _ => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                    }; let d=self.d(); let res=d.wrapping_sub(val); self.flags_sub16(d,val,res); if self.trace { println!("CMPD ${:04X} -> {:04X}", val,res);} }
                    // CMPY immediate/direct/indexed/extended: 0x8C,0x9C,0xAC,0xBC
                    0x8C|0x9C|0xAC|0xBC => { self.pc=self.pc.wrapping_add(1); let val = match bop {
                        0x8C => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); ((hi as u16)<<8)|lo as u16 }
                        0x9C => { let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                        0xAC => { let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); ((self.read8(ea) as u16)<<8)|self.read8(ea.wrapping_add(1)) as u16 }
                        _ => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                    }; let y0=self.y; let res=y0.wrapping_sub(val); self.flags_sub16(y0,val,res); if self.trace { println!("CMPY ${:04X} -> {:04X}", val,res);} }
                    // LDS direct/indexed/extended: 0xDE,0xEE,0xFE; STS 0xDF,0xEF,0xFF
                    0xDE => { self.pc=self.pc.wrapping_add(1); let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let hi=self.read8(addr); let lo=self.read8(addr.wrapping_add(1)); self.s=((hi as u16)<<8)|lo as u16; self.update_nz16(self.s); if self.trace { println!("LDS ${:04X} -> {:04X}", addr,self.s);} }
                    0xEE => { self.pc=self.pc.wrapping_add(1); let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); self.s=((hi as u16)<<8)|lo as u16; self.update_nz16(self.s); if self.trace { println!("LDS [{}] -> {:04X}", ea,self.s);} }
                    0xFE => { self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1)); self.s=((hi2 as u16)<<8)|lo2 as u16; self.update_nz16(self.s); if self.trace { println!("LDS ${:04X} -> {:04X}", addr,self.s);} }
                    0xDF => { self.pc=self.pc.wrapping_add(1); let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let s=self.s; self.write8(addr,(s>>8) as u8); self.write8(addr.wrapping_add(1), s as u8); self.update_nz16(s); if self.trace { println!("STS ${:04X}", addr);} }
                    0xEF => { self.pc=self.pc.wrapping_add(1); let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let s=self.s; self.write8(ea,(s>>8) as u8); self.write8(ea.wrapping_add(1), s as u8); self.update_nz16(s); if self.trace { println!("STS [{}]", ea);} }
                    0xFF => { self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let s=self.s; self.write8(addr,(s>>8) as u8); self.write8(addr.wrapping_add(1), s as u8); self.update_nz16(s); if self.trace { println!("STS ${:04X}", addr);} }
                    0x3F => { self.pc=self.pc.wrapping_add(1); self.service_swi_generic(VEC_SWI2, "SWI2"); }
                    0x26|0x27 => { self.pc=self.pc.wrapping_add(1); let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let off=((hi as u16)<<8)|lo as u16; let target=self.pc.wrapping_add(off as i16 as u16); match bop { 0x26 => { if !self.cc_z { if self.trace { println!("LBNE {:04X}", target);} self.pc=target; } else if self.trace { println!("LBNE not"); } } 0x27 => { if self.cc_z { if self.trace { println!("LBEQ {:04X}", target);} self.pc=target; } else if self.trace { println!("LBEQ not"); } } _=>{} } }
                    // Long branch set (0x21-0x2F). 0x26/0x27 already handled; 0x16 LBRA lives in page0 per spec.
                    0x21..=0x2F => { // All long branch conditions except 0x26/0x27 (handled earlier) and excluding 0x16 LBRA (page0)
                        let cond = match bop {
                            0x21 => false, // LBRN
                            0x22 => (f_c || f_z)==false, // LBHI
                            0x23 => (f_c || f_z)!=false, // LBLS
                            0x24 => f_c==false, // LBHS/LBCC
                            0x25 => f_c!=false, // LBLO/LBCS
                            0x28 => f_v==false, // LBVC
                            0x29 => f_v!=false, // LBVS
                            0x2A => f_n==false, // LBPL
                            0x2B => f_n!=false, // LBMI
                            0x2C => (f_n ^ f_v)==false, // LBGE
                            0x2D => (f_n ^ f_v)!=false, // LBLT
                            0x2E => (f_z || (f_n ^ f_v))==false, // LBGT
                            0x2F => (f_z || (f_n ^ f_v))!=false, // LBLE
                            _ => { if self.trace { println!("UNIMPL 0x10 {:02X}", bop);} return false; }
                        };
                        // Consume sub-op byte
                        self.pc = self.pc.wrapping_add(1);
                        let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2);
                        let off=((hi as u16)<<8)|lo as u16; let target=self.pc.wrapping_add(off as i16 as u16);
                        let name = match bop {
                            0x21=>"LBRN",0x22=>"LBHI",0x23=>"LBLS",0x24=>"LBHS/LBCC",0x25=>"LBLO/LBCS",0x28=>"LBVC",0x29=>"LBVS",0x2A=>"LBPL",0x2B=>"LBMI",0x2C=>"LBGE",0x2D=>"LBLT",0x2E=>"LBGT",0x2F=>"LBLE", _=>"?" };
                        if cond { if self.trace { println!("{} {:04X}", name, target);} self.pc=target; cyc = cyc.saturating_add(6); } else { if self.trace { println!("{} not", name);} cyc = cyc.saturating_add(5); }
                    }
                    _ => { if self.trace { println!("UNIMPL 0x10 {:02X}", bop);} return false; }
                }
            }
            0x11 => { // prefix group 2
                let bop=self.read8(self.pc);
                match bop {
                    // CMPU immediate/direct/indexed/extended: 0x83,0x93,0xA3,0xB3
                    0x83|0x93|0xA3|0xB3 => { self.pc=self.pc.wrapping_add(1); let val = match bop {
                        0x83 => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); ((hi as u16)<<8)|lo as u16 }
                        0x93 => { let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                        0xA3 => { let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); ((self.read8(ea) as u16)<<8)|self.read8(ea.wrapping_add(1)) as u16 }
                        _ => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                    }; let u0=self.u; let res=u0.wrapping_sub(val); self.flags_sub16(u0,val,res); if self.trace { println!("CMPU ${:04X} -> {:04X}", val,res);} }
                    // CMPS immediate/direct/indexed/extended: 0x8C,0x9C,0xAC,0xBC
                    0x8C|0x9C|0xAC|0xBC => { self.pc=self.pc.wrapping_add(1); let val = match bop {
                        0x8C => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); ((hi as u16)<<8)|lo as u16 }
                        0x9C => { let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                        0xAC => { let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); ((self.read8(ea) as u16)<<8)|self.read8(ea.wrapping_add(1)) as u16 }
                        _ => { let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; ((self.read8(addr) as u16)<<8)|self.read8(addr.wrapping_add(1)) as u16 }
                    }; let s0=self.s; let res=s0.wrapping_sub(val); self.flags_sub16(s0,val,res); if self.trace { println!("CMPS ${:04X} -> {:04X}", val,res);} }
                    0x3F => { self.pc=self.pc.wrapping_add(1); self.service_swi_generic(VEC_SWI3, "SWI3"); }
                    _ => { if self.trace { println!("UNIMPL 0x11 {:02X}", bop);} return false; }
                }
            }
            0x00 => { // NEG direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let addr=((self.dp as u16)<<8)|off as u16;
                let m=self.read8(addr); let res=(0u16).wrapping_sub(m as u16) as u8;
                self.write8(addr,res); self.cc_n=(res&0x80)!=0; self.cc_z=res==0; self.cc_v=res==0x80; self.cc_c=m!=0;
                if self.trace { println!("NEG ${:04X} -> {:02X}", addr,res);} 
            }
            0x03 => { // COM direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let res=!m; self.write8(addr,res); self.cc_n=(res & 0x80)!=0; self.cc_z=res==0; self.cc_v=false; self.cc_c=true; if self.trace { println!("COM ${:04X} -> {:02X}", addr,res);} }
            0x0A => { // CLV (Clear V flag)
                self.cc_v = false;
                if self.trace { println!("CLV"); }
            }
            0x2A => { // BPL (Branch if N=0)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if !self.cc_n { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BPL {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BPL not"); }
            }
            0x2B => { // BMI (Branch if N=1)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if self.cc_n { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BMI {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BMI not"); }
            }
            0x2D => { // BLT (N^V==1)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if self.cc_n ^ self.cc_v { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BLT {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BLT not"); }
            }
            0x2E => { // BGT (Z==0 and N^V==0)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if !self.cc_z && !(self.cc_n ^ self.cc_v) { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BGT {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BGT not"); }
            }
            0x2F => { // BLE (Z==1 or N^V==1)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if self.cc_z || (self.cc_n ^ self.cc_v) { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BLE {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BLE not"); }
            }
            0x2C => { // BGE (Branch if >= : N^V == 0)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1);
                if (self.cc_n ^ self.cc_v)==false { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BGE {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BGE not"); }
            }
            // -------------------------------------------------------------------------
            // Indexed RMW operations
            // -------------------------------------------------------------------------
            0x60 => { // NEG indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_neg(m); self.write8(ea,r); if self.trace { println!("NEG [{}] -> {:02X}", ea,r);} }
            0x63 => { // COM indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_com(m); self.write8(ea,r); if self.trace { println!("COM [{}] -> {:02X}", ea,r);} }
            0x64 => { // LSR indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_lsr(m); self.write8(ea,r); if self.trace { println!("LSR [{}] -> {:02X}", ea,r);} }
            0x66 => { // ROR indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_ror(m); self.write8(ea,r); if self.trace { println!("ROR [{}] -> {:02X}", ea,r);} }
            0x67 => { // ASR indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_asr(m); self.write8(ea,r); if self.trace { println!("ASR [{}] -> {:02X}", ea,r);} }
            0x68 => { // ASL indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_asl(m); self.write8(ea,r); if self.trace { println!("ASL [{}] -> {:02X}", ea,r);} }
            0x69 => { // ROL indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_rol(m); self.write8(ea,r); if self.trace { println!("ROL [{}] -> {:02X}", ea,r);} }
            0x6A => { // DEC indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_dec(m); self.write8(ea,r); if self.trace { println!("DEC [{}] -> {:02X}", ea,r);} }
            0x6E => { // JMP indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); self.pc=ea; if self.trace { println!("JMP [{}]", ea);} }
            0x6C => { // INC indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.rmw_inc(m); self.write8(ea,r); if self.trace { println!("INC [{}] -> {:02X}", ea,r);} }
            0x6D => { // TST indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.rmw_tst(m); if self.trace { println!("TST [{}]", ea); }
            }
            0x82 => { // SBCA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                let a=self.a; let c=if self.cc_c {1} else {0};
                let res=a.wrapping_sub(imm).wrapping_sub(c);
                self.a=res; self.flags_sub8(a,imm.wrapping_add(c),res);
                if self.trace { println!("SBCA #${:02X} -> {:02X}", imm, res); }
            }
            0x83 => { // SUBD immediate
                let hi = self.read8(self.pc); let lo = self.read8(self.pc+1); self.pc = self.pc.wrapping_add(2);
                let val = ((hi as u16) << 8) | lo as u16; let d0 = self.d(); let res = d0.wrapping_sub(val);
                self.set_d(res); self.flags_sub16(d0,val,res);
                if self.trace { println!("SUBD #${:04X} -> {:04X}", val,res); }
            }
            0xC3 => { // ADDD immediate
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let val=((hi as u16)<<8)|lo as u16; let d0=self.d(); let sum=(d0 as u32)+(val as u32); let res=(sum & 0xFFFF) as u16; self.set_d(res); self.update_nz16(res); self.cc_c=(sum & 0x10000)!=0; self.cc_v=(!((d0^val) as u32) & ((d0^res) as u32) & 0x8000)!=0; if self.trace { println!("ADDD #${:04X} -> {:04X}", val,res);} }
            0x84 => { // ANDA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1);
                self.a &= imm; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ANDA #${:02X}", imm);} }
            0x88 => { // EORA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.a ^= imm; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("EORA #${:02X}", imm);} }
            0x8A => { // ORA immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.a |= imm; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("ORA #${:02X}", imm);} }
            0xC9 => { // ADCB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let b0=self.b; let c= if self.cc_c {1}else{0}; let sum=(b0 as u16)+(imm as u16)+c as u16; let r=(sum & 0xFF) as u8; self.b=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((b0^imm) as u16)&((b0^r) as u16)&0x80)!=0; if self.trace { println!("ADCB #${:02X}", imm);} }
            0x0B => { // SEV (Set V flag)
                self.cc_v = true; if self.trace { println!("SEV"); } }
            0x19 => { // DAA (Decimal Adjust Accumulator) after addition on A
                // Reference 6809 rules (derived from 6800/6802 family but with C interaction):
                // If (lower nibble > 9) or H set -> add 0x06 to A.
                // If (upper nibble > 9) or C set or (upper nibble >9 after first adjust) -> add 0x60.
                // C is set if a carry out of the high nibble occurs due to adding 0x60.
                // H is undefined after instruction (we leave unchanged to minimize side effects; could clear).
                // Z,N updated from result; V cleared per spec; C updated as above.
                let mut adjust = 0u8; let a0 = self.a; let low = a0 & 0x0F; let high = (a0 >> 4) & 0x0F;
                let mut carry = self.cc_c; // prior carry may influence high adjust
                let half = self.cc_h; // existing half-carry state
                if low > 9 || half { adjust = adjust.wrapping_add(0x06); }
                let mut high_after = high;
                if adjust != 0 { // simulate low adjust effect to evaluate high nibble overflow
                    let tmp = a0.wrapping_add(0x06);
                    high_after = (tmp >> 4) & 0x0F;
                }
                if high > 9 || high_after > 9 || carry { adjust = adjust.wrapping_add(0x60); carry = true; }
                let res = a0.wrapping_add(adjust);
                self.a = res;
                self.update_nz8(res);
                self.cc_v = false; // DAA clears V
                self.cc_c = carry; // Updated carry (set if high adjust applied)
                if self.trace { println!("DAA -> {:02X} (adj={:02X})", res, adjust); }
            }
            0x21 => { // BRN (Branch never) - consume offset only
                let _off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if self.trace { println!("BRN (no branch)"); } }
            0x28 => { // BVC (Branch if V=0)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if !self.cc_v { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BVC {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BVC not"); } }
            0x4A => { // DECA
                let a0=self.a; let res=a0.wrapping_sub(1); self.a=res; self.update_nz8(res); self.cc_v = res==0x7F; if self.trace { println!("DECA -> {:02X}", res);} }
            0x07 => { // ASR direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.cc_c=(m & 0x01)!=0; let msb=m & 0x80; let res=(m>>1)|msb; self.write8(addr,res); self.cc_n=(res&0x80)!=0; self.cc_z=res==0; self.cc_v=false; if self.trace { println!("ASR ${:04X} -> {:02X}", addr,res);} }
            0x08 => { // ASL/LSL direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let res=m<<1; self.cc_c=(m & 0x80)!=0; let res8=(res & 0xFF) as u8; self.write8(addr,res8); self.cc_n=(res8&0x80)!=0; self.cc_z=res8==0; self.cc_v=((m^res8)&0x80)!=0; if self.trace { println!("ASL ${:04X} -> {:02X}", addr,res8);} }
            0x25 => { // BCS (branch if Carry set)
                let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); if self.cc_c { let new=(self.pc as i32 + off as i32) as u16; if self.trace { println!("BCS {:04X}", new);} self.pc=new; cyc=3; } else if self.trace { println!("BCS not"); } }
            0x18 => { // Treat undefined 6809 opcode as NOP (clears nothing)
                if self.trace { println!("(undefined 0x18 treated as NOP)"); } }
            0x61 => { // Undefined / unimplemented in this subset -> NOP
                if self.trace { println!("(undefined 0x61 treated as NOP)"); } }
            0x91 => { // CMPA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let a0=self.a; let res=a0.wrapping_sub(m); self.flags_sub8(a0,m,res); if self.trace { println!("CMPA ${:04X}", addr);} }
            0x93 => { // SUBD direct
                let off = self.read8(self.pc); self.pc = self.pc.wrapping_add(1); let addr = ((self.dp as u16)<<8)|off as u16;
                let hi = self.read8(addr); let lo = self.read8(addr.wrapping_add(1)); let val = ((hi as u16)<<8)|lo as u16; let d0 = self.d(); let res = d0.wrapping_sub(val);
                self.set_d(res); self.flags_sub16(d0,val,res); if self.trace { println!("SUBD ${:04X} -> {:04X}", addr,res);} }
            0x98 => { // EORA direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.a ^= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("EORA ${:04X}", addr);} }
            0xA2 => { // SBCA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let a0=self.a; let c=if self.cc_c {1} else {0}; let res=a0.wrapping_sub(m).wrapping_sub(c); self.a=res; self.flags_sub8(a0,m.wrapping_add(c),res); if self.trace { println!("SBCA [{}] -> {:02X}", ea,res);} }
            0xA3 => { // SUBD indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s);
                let hi = self.read8(ea); let lo = self.read8(ea.wrapping_add(1)); let val = ((hi as u16)<<8)|lo as u16; let d0 = self.d(); let res = d0.wrapping_sub(val);
                self.set_d(res); self.flags_sub16(d0,val,res); if self.trace { println!("SUBD [{}] -> {:04X}", ea,res);} }
            0xA5 => { // BITA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.a & m; self.cc_n=(r&0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITA [{}]", ea);} }
            0xA8 => { // EORA indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.a ^= m; self.update_nz8(self.a); self.cc_v=false; if self.trace { println!("EORA [{}]", ea);} }
            0xC8 => { // EORB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); self.b ^= imm; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("EORB #${:02X}", imm);} }
            0xCB => { // ADDB immediate
                let imm=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let b0=self.b; let sum=(b0 as u16)+(imm as u16); let r=(sum & 0xFF) as u8; self.b=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((b0^imm) as u16) & ((b0^r) as u16) & 0x80)!=0; if self.trace { println!("ADDB #${:02X}", imm);} }
            0xDB => { // ADDB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let b0=self.b; let sum=(b0 as u16)+(m as u16); let r=(sum & 0xFF) as u8; self.b=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((b0^m) as u16) & ((b0^r) as u16) & 0x80)!=0; if self.trace { println!("ADDB ${:04X}", addr);} }
            0xE5 => { // BITB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let r=self.b & m; self.cc_n=(r&0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITB [{}]", ea);} }
            0xEB => { // ADDB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); let b0=self.b; let sum=(b0 as u16)+(m as u16); let r=(sum & 0xFF) as u8; self.b=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((b0^m) as u16) & ((b0^r) as u16) & 0x80)!=0; if self.trace { println!("ADDB [{}]", ea);} }
            0xEE => { // LDU indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); let val=((hi as u16)<<8)|lo as u16; self.u=val; self.update_nz16(val); if self.trace { println!("LDU [{}] -> {:04X}", ea,val);} }
            0xEF => { // STU indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let u=self.u; self.write8(ea,(u>>8) as u8); self.write8(ea.wrapping_add(1), u as u8); self.update_nz16(u); if self.trace { println!("STU [{}] -> {:04X}", ea,u);} }
            0xF7 => { // STB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let v=self.b; self.write8(addr,v); self.update_nz8(v); if self.trace { println!("STB ${:04X} -> {:02X}", addr,v);} }
            0xF9 => { // ADCB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let b0=self.b; let c= if self.cc_c {1}else{0}; let sum=(b0 as u16)+(m as u16)+c as u16; let r=(sum & 0xFF) as u8; self.b=r; self.update_nz8(r); self.cc_c=(sum & 0x100)!=0; self.cc_v=(!((b0^m) as u16)&((b0^r) as u16)&0x80)!=0; if self.trace { println!("ADCB ${:04X}", addr);} }
            0xB3 => { // SUBD extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let mhi=self.read8(addr); let mlo=self.read8(addr.wrapping_add(1)); let val=((mhi as u16)<<8)|mlo as u16; let d0=self.d(); let res=d0.wrapping_sub(val);
                self.set_d(res); self.flags_sub16(d0,val,res);
                if self.trace { println!("SUBD #${:04X} -> {:04X}", val,res); }
            }
            0xD0 => { // SUBB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let b0=self.b; let res=b0.wrapping_sub(m); self.b=res; self.flags_sub8(b0,m,res); if self.trace { println!("SUBB ${:04X}", addr);} }
            0xD1 => { // CMPB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let b0=self.b; let res=b0.wrapping_sub(m); self.flags_sub8(b0,m,res); if self.trace { println!("CMPB ${:04X}", addr);} }
            0xD5 => { // BITB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); let r=self.b & m; self.cc_n=(r&0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITB ${:04X}", addr);} }
            // (Removed stray brace that incorrectly closed the match here)
            0xE7 => { // STB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let v=self.b; self.write8(ea,v); self.update_nz8(v); if self.trace { println!("STB [{}] -> {:02X}", ea,v);} }
            0xEC => { // LDD indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let hi=self.read8(ea); let lo=self.read8(ea.wrapping_add(1)); let val=((hi as u16)<<8)|lo as u16; self.set_d(val); self.update_nz16(val); if self.trace { println!("LDD [{}] -> {:04X}", ea,val);} }
            0xF2 => { // SBCB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let b0=self.b; let c=if self.cc_c {1} else {0}; let res=b0.wrapping_sub(m).wrapping_sub(c); self.b=res; self.flags_sub8(b0,m.wrapping_add(c),res); if self.trace { println!("SBCB ${:04X} -> {:02X}", addr,res);} }
            0xF5 => { // BITB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let m=self.read8(addr); let r=self.b & m; self.cc_n=(r&0x80)!=0; self.cc_z=r==0; self.cc_v=false; if self.trace { println!("BITB ${:04X}", addr);} }
            0xF6 => { // LDB extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let v=self.read8(addr); self.b=v; self.update_nz8(v); if self.trace { println!("LDB ${:04X} -> {:02X}", addr,v);} }
            0xD8 => { // EORB direct
                let off=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let addr=((self.dp as u16)<<8)|off as u16; let m=self.read8(addr); self.b ^= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("EORB ${:04X}", addr);} }
            0xE8 => { // EORB indexed
                let post=self.read8(self.pc); self.pc=self.pc.wrapping_add(1); let (ea,_) = self.decode_indexed(post,self.x,self.y,self.u,self.s); let m=self.read8(ea); self.b ^= m; self.update_nz8(self.b); self.cc_v=false; if self.trace { println!("EORB [{}]", ea);} }
            0xFC => { // LDD extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let hi2=self.read8(addr); let lo2=self.read8(addr.wrapping_add(1)); let val=((hi2 as u16)<<8)|lo2 as u16; self.set_d(val); self.update_nz16(val); if self.trace { println!("LDD ${:04X} -> {:04X}", addr,val);} }
            0xFF => { // STU extended
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let addr=((hi as u16)<<8)|lo as u16; let u=self.u; self.write8(addr,(u>>8) as u8); self.write8(addr.wrapping_add(1), u as u8); self.update_nz16(u); if self.trace { println!("STU ${:04X} -> {:04X}", addr,u);} }
            0x8C => { // CMPX immediate
                let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2);
                let val=((hi as u16)<<8)|lo as u16;
                let x=self.x; let res=x.wrapping_sub(val);
                self.flags_sub16(x,val,res);
                if self.trace { println!("CMPX #${:04X}", val); }
            }
            0x7B => { // Placeholder (undefined/unused in subset) - treat as NOP
                if self.trace { println!("(placeholder 0x7B NOP)"); }
            }
            0x38 => { if self.trace { println!("NOP (0x38)"); } }
            0xCF => { if self.trace { println!("NOP (0xCF)"); } }
            0x8F => { // Placeholder (undocumented in current subset) - treat as NOP
                if self.trace { println!("(placeholder 0x8F NOP)"); }
            }
            0x01|0x02|0x05|0x45|0x4E|0x52 => { if self.trace { println!("(illegal/unused treated as NOP)"); } }
            op_unhandled => { if self.trace { println!("UNIMPL OP {:02X} at {:04X}", op_unhandled, pc0);} if !self.opcode_unimpl_bitmap[op_unhandled as usize] { self.opcode_unimpl_bitmap[op_unhandled as usize]=true; } self.opcode_unimplemented += 1; }
        }
        self.advance_cycles(cyc);
        true
    }

    // Centralized cycle advancement so VIA timers, frame timing, and future integrator stay in lockstep per instruction.
    fn advance_cycles(&mut self, cyc: u32) {
        if cyc == 0 { return; }
        self.bus.tick(cyc);
        self.cycles += cyc as u64;
        self.cycle_accumulator += cyc as u64;
        // Update experimental integrator with current frame (cycle-based authoritative frame number)
        self.integrator.tick(cyc, self.cycle_frame);
        // Detect timer expiries (IFR bits 6: T1, 5: T2) and count rising events.
        let ifr = self.bus.via_ifr();
        if (ifr & 0x40)!=0 { self.t1_expiries = self.t1_expiries.wrapping_add(1); }
        if (ifr & 0x20)!=0 { self.t2_expiries = self.t2_expiries.wrapping_add(1); }
        while self.cycle_accumulator >= self.cycles_per_frame {
            self.cycle_accumulator -= self.cycles_per_frame;
            self.cycle_frame = self.cycle_frame.wrapping_add(1);
            // Mirror legacy field for existing frontend until it migrates to cycle_frame
            self.frame_count = self.cycle_frame;
            // Integrator auto-drain & stats collection
            let seg_count = self.integrator.segments.len() as u32;
            self.integrator_last_frame_segments = seg_count;
            if seg_count > self.integrator_max_frame_segments { self.integrator_max_frame_segments = seg_count; }
            self.integrator_total_segments = self.integrator_total_segments.wrapping_add(seg_count as u64);
            // Collect lines_per_frame sampling (same as segment count for now)
            self.lines_per_frame_accum = self.lines_per_frame_accum.wrapping_add(seg_count as u64);
            self.lines_per_frame_samples = self.lines_per_frame_samples.wrapping_add(1);
            if self.integrator_auto_drain { self.integrator.segments.clear(); }
            // Attempt cartridge validation after first frame if not already done
            if !self.cart_validation_done { self.validate_cartridge_if_needed(); }
        }
    }

    // Helper accessors to avoid adding public getters to integrator for velocity split update
    fn integrator_state_vx(&self) -> f32 { self.integrator.velocity().0 }
    fn integrator_state_vy(&self) -> f32 { self.integrator.velocity().1 }

    // Basic subset of 6809 indexed addressing decoder (from legacy implementation)
    fn decode_indexed_basic(&mut self, post: u8, x: u16, y: u16, u: u16, s: u16) -> (u16, u8) {
        let group = post & 0xE0;
        let base = match group { 0x80=>x,0xA0=>y,0xC0=>u,0xE0=>s,_=>x };
        let mut effective=base;
        match post & 0x1F {
            // Removed incorrect placement of direct LSR (opcode 0x04) from here.
            0x00 => { effective=base; match group {0x80=>{ self.x=self.x.wrapping_add(1); },0xA0=>{ self.y=self.y.wrapping_add(1); },0xC0=>{ self.u=self.u.wrapping_add(1); },0xE0=>{ self.s=self.s.wrapping_add(1); }, _=>{} } }
            0x01 => { effective=base; match group {0x80=>{ self.x=self.x.wrapping_add(2); },0xA0=>{ self.y=self.y.wrapping_add(2); },0xC0=>{ self.u=self.u.wrapping_add(2); },0xE0=>{ self.s=self.s.wrapping_add(2); }, _=>{} } }
            0x02 => { match group {0x80=>{ self.x=self.x.wrapping_sub(1); effective=self.x; },0xA0=>{ self.y=self.y.wrapping_sub(1); effective=self.y; },0xC0=>{ self.u=self.u.wrapping_sub(1); effective=self.u; },0xE0=>{ self.s=self.s.wrapping_sub(1); effective=self.s; }, _=>{} } }
            0x03 => { match group {0x80=>{ self.x=self.x.wrapping_sub(2); effective=self.x; },0xA0=>{ self.y=self.y.wrapping_sub(2); effective=self.y; },0xC0=>{ self.u=self.u.wrapping_sub(2); effective=self.u; },0xE0=>{ self.s=self.s.wrapping_sub(2); effective=self.s; }, _=>{} } }
            0x08 => { let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); effective=base.wrapping_add(off as i16 as u16); }
            _ => {}
        }
        (effective,0)
    }
    fn decode_indexed(&mut self, post:u8, x:u16, y:u16, u:u16, s:u16)->(u16,u8){
        if (post & 0x80)!=0 { match post & 0x1F {0x00|0x01|0x02|0x03|0x04|0x08 => return self.decode_indexed_basic(post,x,y,u,s), _=>{} } }
        let group_masked=post & 0xE7;
        if matches!(group_masked & 0x07,0x04|0x05|0x06|0x07) && (group_masked & 0x07)!=0x07 {
            let reg_code=(post>>5)&0x03; let base=match reg_code {0=>x,1=>y,2=>u,_=>s};
            let eff = match group_masked & 0x07 {
                0x04 => base.wrapping_add(self.a as u16),
                0x05 => base.wrapping_add(self.b as u16),
                0x06 => base.wrapping_add(self.d()),
                _ => base,
            };
            return (eff,0);
        } else if (group_masked & 0x07)==0x07 {
            let reg_code=(post>>5)&0x03; let base=match reg_code {0=>x,1=>y,2=>u,_=>s};
            let ptr=base.wrapping_add(self.d()); let hi=self.read8(ptr); let lo=self.read8(ptr.wrapping_add(1)); return ((((hi as u16)<<8)|lo as u16),2);
        }
        let reg_code=(post>>5)&0x03; let mut base=match reg_code {0=>x,1=>y,2=>u,_=>s};
        let mode=(post>>3)&0x03; let low3=post & 0x07; let mut extra=0u8;
        match (mode,low3) {
            (0,0)=>{ let eff=base; match reg_code {0=>{self.x=self.x.wrapping_add(1);},1=>{self.y=self.y.wrapping_add(1);},2=>{self.u=self.u.wrapping_add(1);},_=>{self.s=self.s.wrapping_add(1);} }; return (eff,0); }
            (0,1)=>{ let eff=base; match reg_code {0=>{self.x=self.x.wrapping_add(2);},1=>{self.y=self.y.wrapping_add(2);},2=>{self.u=self.u.wrapping_add(2);},_=>{self.s=self.s.wrapping_add(2);} }; return (eff,0); }
            (0,2)=>{ match reg_code {0=>{self.x=self.x.wrapping_sub(1); base=self.x;},1=>{self.y=self.y.wrapping_sub(1); base=self.y;},2=>{self.u=self.u.wrapping_sub(1); base=self.u;},_=>{self.s=self.s.wrapping_sub(1); base=self.s;}}; return (base,0); }
            (0,3)=>{ match reg_code {0=>{self.x=self.x.wrapping_sub(2); base=self.x;},1=>{self.y=self.y.wrapping_sub(2); base=self.y;},2=>{self.u=self.u.wrapping_sub(2); base=self.u;},_=>{self.s=self.s.wrapping_sub(2); base=self.s;}}; return (base,0); }
            (0,4)=>{ return (base,0); }
            (0,5)=>{ let off=self.read8(self.pc) as i8; self.pc=self.pc.wrapping_add(1); return (base.wrapping_add(off as i16 as u16),0); }
            (0,6)=>{ let hi=self.read8(self.pc); let lo=self.read8(self.pc+1); self.pc=self.pc.wrapping_add(2); let off=((hi as u16)<<8)|lo as u16; return (base.wrapping_add(off as i16 as u16),1); }
            (0,7)=>{ let sub=post & 0x1F; let acc_sel=sub & 0x07; let mut eff=base; match acc_sel {0x04=>{ eff=base.wrapping_add(self.a as u16);},0x05=>{ eff=base.wrapping_add(self.b as u16);},0x06=>{ eff=base.wrapping_add(self.d()); extra+=1;},0x00..=0x03=>{ let five=(sub & 0x1F) as i8; let signed= if five & 0x10 !=0 { (five as i8) | !0x1F } else { five }; eff=base.wrapping_add(signed as i16 as u16);} _=>{} }; return (eff,extra); }
            _=>{}
        }
        (base,extra)
    }
}
