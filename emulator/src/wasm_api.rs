#[cfg(feature="wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature="wasm")]
use crate::CPU;
#[cfg(feature="wasm")]
use serde::Serialize;

#[cfg(feature="wasm")]
#[derive(Serialize)]
struct JsLoopSample { pc:u16,a:u8,b:u8,x:u16,y:u16,u:u16,s:u16,dp:u8,via_ifr:u8,via_ier:u8,via_acr:u8,via_pcr:u8,cycles:u64 }
#[cfg(feature="wasm")]
#[derive(Serialize)]
struct JsMetrics {
    total: u64,
    unimplemented: u64,
    frames: u64, // legacy (mirrors cycle_frame; retained for compatibility)
    cycle_frame: u64,
    bios_frame: u64,
    last_intensity: u8,
    unique_unimplemented: Vec<u8>,
    cycles: u64,
    avg_cycles_per_frame: Option<f64>,
    // Top N opcode counts (opcode, count)
    top_opcodes: Vec<(u8,u64)>,
    // Optional first unimplemented opcode encountered (for quick hint)
    first_unimpl: Option<u8>,
    via_t1: u16,
    via_irq_count: u64,
    via_irq_line: bool,
    via_ifr: u8,
    via_ier: u8,
    cart_loaded: bool,
    irq_frames_generated: u64,
    jsr_sample: Vec<u16>,
    vector_backend: &'static str,
    integrator_segments: usize,
    integrator_last_frame_segments: u32,
    integrator_max_frame_segments: u32,
    integrator_total_segments: u64,
    integrator_auto_drain: bool,
    reads_unmapped: u64,
    writes_unmapped: u64,
    writes_bios_ignored: u64,
    cart_oob_reads: u64,
    cart_valid: bool,
    cart_title: String,
    irq_count: u64,
    firq_count: u64,
    t1_expiries: u64,
    t2_expiries: u64,
    avg_lines_per_frame: Option<f64>,
}

#[cfg(feature="wasm")]
#[wasm_bindgen]
pub struct WasmEmu { cpu: CPU }

#[cfg(feature="wasm")]
#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)] pub fn new() -> WasmEmu { WasmEmu { cpu: CPU::default() } }
    #[wasm_bindgen] pub fn load_bios(&mut self, data:&[u8])->bool { let len=data.len(); if !(len==4096 || len==8192){return false;} self.cpu.load_bios(data); true }
    #[wasm_bindgen] pub fn load_bin(&mut self, base:u16, data:&[u8]){ for (i,b) in data.iter().enumerate(){ let addr=base as usize + i; if addr<65536 { self.cpu.mem[addr]=*b; self.cpu.bus.mem[addr]=*b; } } }
    #[wasm_bindgen] pub fn reset(&mut self){ self.cpu.reset(); }
    #[wasm_bindgen] pub fn step(&mut self, count:u32)->u32 { let mut ex=0; for _ in 0..count { if !self.cpu.step(){ break; } ex +=1; } ex }
    #[wasm_bindgen] pub fn run_until_wait_recal(&mut self, max_instr:u32)->u32 { let start=self.cpu.bios_frame; let mut ex=0; while ex<max_instr { if !self.cpu.step(){ break; } ex+=1; if self.cpu.bios_frame != start { break; } } ex }
    #[wasm_bindgen] pub fn registers_json(&self)->String { format!("{{\"a\":{},\"b\":{},\"dp\":{},\"x\":{},\"y\":{},\"u\":{},\"s\":{},\"pc\":{},\"cycles\":{},\"frame_count\":{},\"cycle_frame\":{},\"bios_frame\":{},\"last_intensity\":{} }}", self.cpu.a,self.cpu.b,self.cpu.dp,self.cpu.x,self.cpu.y,self.cpu.u,self.cpu.s,self.cpu.pc,self.cpu.cycles,self.cpu.frame_count,self.cpu.cycle_frame,self.cpu.bios_frame,self.cpu.last_intensity) }
    #[wasm_bindgen] pub fn memory_ptr(&self)->*const u8 { self.cpu.mem.as_ptr() }
    #[wasm_bindgen] pub fn metrics_json(&self)->String {
        let m = self.cpu.opcode_metrics();
        // Compute average cycles per frame if we have at least 1 frame
    let avg_cpf = if self.cpu.cycle_frame > 0 { Some(self.cpu.cycles as f64 / self.cpu.cycle_frame as f64) } else { None };
        // Collect top 8 opcodes by count (excluding zero)
    let mut pairs: Vec<(u8,u64)> = m.counts.iter().enumerate().filter_map(|(op,&c)| if c>0 { Some((op as u8, c)) } else { None }).collect();
        pairs.sort_by(|a,b| b.1.cmp(&a.1));
        pairs.truncate(8);
        let first_unimpl = m.unique_unimplemented.first().copied();
        let js = JsMetrics {
            total: m.total,
            unimplemented: m.unimplemented,
            frames: self.cpu.frame_count,
            cycle_frame: self.cpu.cycle_frame,
            bios_frame: self.cpu.bios_frame,
            last_intensity: self.cpu.last_intensity,
            unique_unimplemented: m.unique_unimplemented,
            cycles: self.cpu.cycles,
            avg_cycles_per_frame: avg_cpf,
            top_opcodes: pairs,
            first_unimpl,
            via_t1: self.cpu.bus.via.t1_counter(),
            via_irq_count: self.cpu.via_irq_count,
            via_irq_line: self.cpu.bus.via.irq_asserted(),
            via_ifr: self.cpu.bus.via_ifr(),
            via_ier: self.cpu.bus.via_ier(),
            cart_loaded: self.cpu.cart_loaded,
            irq_frames_generated: self.cpu.irq_frames_generated,
            jsr_sample: self.cpu.jsr_log[..self.cpu.jsr_log_len.min(16)].to_vec(),
            vector_backend: "integrator",
            integrator_segments: self.cpu.integrator.segments.len(),
            integrator_last_frame_segments: self.cpu.integrator_last_frame_segments,
            integrator_max_frame_segments: self.cpu.integrator_max_frame_segments,
            integrator_total_segments: self.cpu.integrator_total_segments,
            integrator_auto_drain: self.cpu.integrator_auto_drain,
            reads_unmapped: self.cpu.bus.stats.reads_unmapped,
            writes_unmapped: self.cpu.bus.stats.writes_unmapped,
            writes_bios_ignored: self.cpu.bus.stats.writes_bios_ignored,
            cart_oob_reads: self.cpu.bus.stats.cart_oob_reads,
            cart_valid: self.cpu.cart_valid,
            cart_title: {
                let raw=&self.cpu.cart_title; let end=raw.iter().position(|&c| c==0).unwrap_or(raw.len());
                String::from_utf8(raw[..end].to_vec()).unwrap_or_default()
            },
            irq_count: self.cpu.irq_count,
            firq_count: self.cpu.firq_count,
            t1_expiries: self.cpu.t1_expiries,
            t2_expiries: self.cpu.t2_expiries,
            avg_lines_per_frame: if self.cpu.lines_per_frame_samples>0 { Some(self.cpu.lines_per_frame_accum as f64 / self.cpu.lines_per_frame_samples as f64) } else { None },
        };
        serde_json::to_string(&js).unwrap_or_else(|_|"{}".into())
    }
    #[wasm_bindgen] pub fn integrator_segments_json(&mut self)->String {
        let segs = self.cpu.integrator.take_segments();
        // Simple JSON array of [x0,y0,x1,y1,intensity,frame]
        let mut out = String::from("[");
        for (i,s) in segs.iter().enumerate() {
            if i>0 { out.push(','); }
            out.push_str(&format!("[{:.2},{:.2},{:.2},{:.2},{},{}]", s.x0,s.y0,s.x1,s.y1,s.intensity,s.frame));
        }
        out.push(']');
        out
    }
        // Non-draining JSON view (does not clear internal buffer)
        #[wasm_bindgen] pub fn integrator_segments_peek_json(&self)->String {
            let segs = self.cpu.integrator.segments_slice();
            let mut out = String::from("[");
            for (i,s) in segs.iter().enumerate() {
                if i>0 { out.push(','); }
                out.push_str(&format!("[{:.2},{:.2},{:.2},{:.2},{},{}]", s.x0,s.y0,s.x1,s.y1,s.intensity,s.frame));
            }
            out.push(']'); out
        }
        // Shared memory export helpers: we allocate a temporary copy buffer each call; UI can read via pointer.
        // For persistent zero-copy, future work could maintain a ring buffer.
        #[wasm_bindgen] pub fn integrator_segments_ptr(&mut self) -> *const u8 {
            // Store copy in CPU reusable staging vec (added field) OR allocate ephemeral (here ephemeral for simplicity)
            self.cpu.temp_segments_c = self.cpu.integrator.segments_c_copy();
            self.cpu.temp_segments_c.as_ptr() as *const u8
        }
        #[wasm_bindgen] pub fn integrator_segments_len(&self) -> u32 { self.cpu.temp_segments_c.len() as u32 }
        #[wasm_bindgen] pub fn integrator_segment_stride(&self) -> u32 { std::mem::size_of::<crate::integrator::BeamSegmentC>() as u32 }
        #[wasm_bindgen] pub fn integrator_drain_segments(&mut self){ self.cpu.integrator.take_segments(); }
    #[wasm_bindgen] pub fn loop_watch_json(&self)->String {
        let mut out: Vec<JsLoopSample> = Vec::new();
        for s in &self.cpu.loop_watch_slots {
            if s.pc != 0 { out.push(JsLoopSample{ pc:s.pc,a:s.a,b:s.b,x:s.x,y:s.y,u:s.u,s:s.s,dp:s.dp,via_ifr:s.via_ifr,via_ier:s.via_ier,via_acr:s.via_acr,via_pcr:s.via_pcr,cycles:s.cycles }); }
        }
        serde_json::to_string(&out).unwrap_or_else(|_|"[]".into())
    }
    #[wasm_bindgen] pub fn set_irq_frame_fallback(&mut self, en: bool) { self.cpu.enable_irq_frame_fallback = en; }
    #[wasm_bindgen] pub fn irq_frame_fallback(&self) -> bool { self.cpu.enable_irq_frame_fallback }
    // --- New controls for vector backend & integrator line merging ---
    // Backend now fixed to integrator; setter/getter removed.
    #[wasm_bindgen] pub fn set_integrator_merge_lines(&mut self, merge: bool) { self.cpu.integrator.set_merge(merge); }
    #[wasm_bindgen] pub fn integrator_merge_lines(&self) -> bool { self.cpu.integrator.merge_lines }
    #[wasm_bindgen] pub fn reset_integrator_segments(&mut self) { self.cpu.integrator.segments.clear(); }
    #[wasm_bindgen] pub fn set_integrator_auto_drain(&mut self, en: bool) { self.cpu.integrator_auto_drain = en; }
    #[wasm_bindgen] pub fn integrator_auto_drain(&self) -> bool { self.cpu.integrator_auto_drain }
}
