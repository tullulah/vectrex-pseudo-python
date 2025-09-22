#[cfg(feature="wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature="wasm")]
use crate::Emulator;
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
    draw_vl: u64,
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
    hot00: Vec<(u16,u64)>,
    hotff: Vec<(u16,u64)>,
    // Input (host -> emu) snapshot
    input_x: i16,
    input_y: i16,
    input_buttons: u8,
    // --- Audio PSG (fase inicial) ---
    psg_samples: u64,
    psg_tone_toggles: u64,
    psg_noise_shifts: u64,
    psg_env_steps: u64,
}

#[cfg(feature="wasm")]
#[wasm_bindgen]
pub struct WasmEmu { 
    emulator: Emulator 
}

#[cfg(feature="wasm")]
#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)] 
    pub fn new() -> WasmEmu { 
        WasmEmu { emulator: Emulator::new() } 
    }
    
    #[wasm_bindgen] 
    pub fn load_bios(&mut self, data:&[u8])->bool { 
        let len=data.len(); 
        if !(len==4096 || len==8192){return false;} 
        self.emulator.load_bios(data); 
        true 
    }
    
    #[wasm_bindgen] 
    pub fn load_bin(&mut self, base:u16, data:&[u8]){ 
        self.emulator.load_cartridge_at(base, data); 
    }
    
    #[wasm_bindgen] 
    pub fn reset(&mut self){ 
        self.emulator.reset(); 
    }
    
    #[wasm_bindgen] 
    pub fn reset_stats(&mut self){ 
        self.emulator.reset_stats(); 
    }
    
    #[wasm_bindgen] 
    pub fn step(&mut self, count:u32)->u32 { 
        self.emulator.step_multiple(count)
    }
    /// Ejecuta instrucciones hasta que el frame_count cambie (heurística WAIT_RECAL) o se alcance el límite.
    /// Devuelve el número de instrucciones ejecutadas. Reintroducido tras refactor.
    #[wasm_bindgen] 
    pub fn run_until_wait_recal(&mut self, max_instructions: u32) -> u32 {
        let start = self.emulator.cpu.frame_count;
        let mut executed = 0u32;
        while executed < max_instructions {
            if !self.emulator.step() { break; }
            executed += 1;
            if self.emulator.cpu.frame_count != start { break; }
        }
        // Fallback visual: si tras algún frame no hay ningún segmento acumulado, generar un triángulo demo.
        // No altera estado BIOS / memoria; sólo usa integrator (visual). Se puede desactivar vía flag wasm.
        if self.emulator.cpu.frame_count > 0 && self.emulator.cpu.integrator_total_segments == 0 {
            if self.auto_demo_enabled() { self.demo_triangle(); }
        }
        executed
    }
    #[wasm_bindgen] 
    pub fn registers_json(&self)->String { 
        format!("{{\"a\":{},\"b\":{},\"dp\":{},\"x\":{},\"y\":{},\"u\":{},\"s\":{},\"pc\":{},\"cycles\":{},\"frame_count\":{},\"cycle_frame\":{},\"last_intensity\":{} }}", 
            self.emulator.cpu.a,
            self.emulator.cpu.b,
            self.emulator.cpu.dp,
            self.emulator.cpu.x,
            self.emulator.cpu.y,
            self.emulator.cpu.u,
            self.emulator.cpu.s,
            self.emulator.cpu.pc,
            self.emulator.cpu.cycles,
            self.emulator.cpu.frame_count,
            self.emulator.cpu.cycle_frame,
            self.emulator.cpu.last_intensity) 
    }
    // Return pointer to unified bus memory so BIOS region (written via Bus) is visible to JS.
    #[wasm_bindgen] 
    pub fn memory_ptr(&self)->*const u8 { 
        self.emulator.cpu.bus.mem.as_ptr() 
    }
    
    /// Read a single byte from unified bus memory (debug helper for JS console).
    #[wasm_bindgen] 
    pub fn read_mem8(&self, addr: u16) -> u8 { 
        self.emulator.cpu.bus.mem[addr as usize] 
    }
    
    /// Return the base address where BIOS was loaded (F000 for 4K, E000 for 8K) or default if not present yet.
    #[wasm_bindgen] 
    pub fn bios_base(&self) -> u16 { 
        self.emulator.cpu.bus.test_bios_base() 
    }
    // ---- Trace API ----
    #[wasm_bindgen] 
    pub fn enable_trace(&mut self, en: bool, limit: u32) { 
        self.emulator.cpu.trace_enabled = en; 
        if en { 
            self.emulator.cpu.trace_limit = limit.min(200_000) as usize; 
        } 
    }
    
    #[wasm_bindgen] 
    pub fn trace_clear(&mut self) { 
        self.emulator.cpu.trace_buf.clear(); 
    }
    #[wasm_bindgen] 
    pub fn trace_len(&self) -> u32 { 
        self.emulator.cpu.trace_buf.len() as u32 
    }
    
    #[wasm_bindgen] 
    pub fn trace_log_json(&self) -> String {
        use serde::Serialize; 
        #[derive(Serialize)] 
        struct Row { 
            pc:u16, op:u8, sub:u8, hex:String, m:&'static str, a:u8,b:u8,x:u16,y:u16,u:u16,s:u16,dp:u8, 
            operand: Option<String>, repeat:u32, flags:u8, cycles:u32, illegal:bool, depth:u16 
        }
        let mut out = Vec::with_capacity(self.emulator.cpu.trace_buf.len());
        for e in &self.emulator.cpu.trace_buf {
            let hex = if e.sub!=0 && (e.opcode==0x10 || e.opcode==0x11) { 
                format!("{:02X} {:02X}", e.opcode, e.sub) 
            } else { 
                format!("{:02X}", e.opcode) 
            };
            out.push(Row{ 
                pc:e.pc, op:e.opcode, sub:e.sub, hex, m: crate::cpu6809::opcode_mnemonic(e.opcode, e.sub), 
                a:e.a,b:e.b,x:e.x,y:e.y,u:e.u,s:e.s,dp:e.dp, operand: e.op_str.clone(), repeat: e.loop_count, 
                flags:e.flags, cycles:e.cycles, illegal:e.illegal, depth:e.call_depth 
            });
        }
        serde_json::to_string(&out).unwrap_or_else(|_|"[]".into())
    }
    #[wasm_bindgen] 
    pub fn metrics_json(&self)->String {
        // Usar las nuevas métricas del emulador, complementadas con datos del CPU para compatibilidad
        let debug_state = self.emulator.debug_state();
        let m = self.emulator.cpu.opcode_metrics();
        
        // Compute average cycles per frame if we have at least 1 frame
        let avg_cpf = if self.emulator.cpu.cycle_frame > 0 { 
            Some(self.emulator.cpu.cycles as f64 / self.emulator.cpu.cycle_frame as f64) 
        } else { 
            None 
        };
        
        // Collect top 8 opcodes by count (excluding zero)
        let mut pairs: Vec<(u8,u64)> = m.counts.iter().enumerate()
            .filter_map(|(op,&c)| if c>0 { Some((op as u8, c)) } else { None })
            .collect();
        pairs.sort_by(|a,b| b.1.cmp(&a.1));
        pairs.truncate(8);
        let first_unimpl = m.unique_unimplemented.first().copied();
        
        let js = JsMetrics {
            total: m.total,
            unimplemented: m.unimplemented,
            frames: self.emulator.cpu.frame_count,
            cycle_frame: self.emulator.cpu.cycle_frame,
            bios_frame: debug_state.bios_frame, // Usar el nuevo campo
            
            last_intensity: self.emulator.cpu.last_intensity,
            unique_unimplemented: m.unique_unimplemented,
            cycles: self.emulator.cpu.cycles,
            avg_cycles_per_frame: avg_cpf,
            top_opcodes: pairs,
            first_unimpl,
            via_t1: self.emulator.cpu.bus.via.t1_counter(),
            via_irq_count: self.emulator.cpu.via_irq_count,
            via_irq_line: self.emulator.cpu.bus.via.irq_asserted(),
            via_ifr: self.emulator.cpu.bus.via_ifr(),
            via_ier: self.emulator.cpu.bus.via_ier(),
            cart_loaded: self.emulator.cpu.cart_loaded,
            irq_frames_generated: self.emulator.cpu.irq_frames_generated,
            jsr_sample: self.emulator.cpu.jsr_log[..self.emulator.cpu.jsr_log_len.min(16)].to_vec(),
            vector_backend: "integrator",
            integrator_segments: self.emulator.cpu.integrator.segments.len(),
            integrator_last_frame_segments: self.emulator.cpu.integrator_last_frame_segments,
            integrator_max_frame_segments: self.emulator.cpu.integrator_max_frame_segments,
            integrator_total_segments: self.emulator.cpu.integrator_total_segments,
            integrator_auto_drain: self.emulator.cpu.integrator_auto_drain,
            // Real Draw_VL invocation counter
            draw_vl: self.emulator.cpu.draw_vl_count,
            reads_unmapped: self.emulator.cpu.bus.stats.reads_unmapped,
            writes_unmapped: self.emulator.cpu.bus.stats.writes_unmapped,
            writes_bios_ignored: self.emulator.cpu.bus.stats.writes_bios_ignored,
            cart_oob_reads: self.emulator.cpu.bus.stats.cart_oob_reads,
            cart_valid: self.emulator.cpu.cart_valid,
            cart_title: {
                let raw=&self.emulator.cpu.cart_title; 
                let end=raw.iter().position(|&c| c==0).unwrap_or(raw.len());
                String::from_utf8(raw[..end].to_vec()).unwrap_or_default()
            },
            irq_count: self.emulator.cpu.irq_count,
            firq_count: self.emulator.cpu.firq_count,
            t1_expiries: self.emulator.cpu.t1_expiries,
            t2_expiries: self.emulator.cpu.t2_expiries,
            avg_lines_per_frame: if self.emulator.cpu.lines_per_frame_samples>0 { 
                Some(self.emulator.cpu.lines_per_frame_accum as f64 / self.emulator.cpu.lines_per_frame_samples as f64) 
            } else { 
                None 
            },
            hot00: self.emulator.cpu.hot00.iter().copied().filter(|(_,c)| *c>0).collect(),
            hotff: self.emulator.cpu.hotff.iter().copied().filter(|(_,c)| *c>0).collect(),
            input_x: self.emulator.cpu.input_state.x,
            input_y: self.emulator.cpu.input_state.y,
            input_buttons: self.emulator.cpu.input_state.buttons,
            psg_samples: self.emulator.cpu.bus.psg.metric_samples,
            psg_tone_toggles: self.emulator.cpu.bus.psg.metric_tone_toggles,
            psg_noise_shifts: self.emulator.cpu.bus.psg.metric_noise_shifts,
            psg_env_steps: self.emulator.cpu.bus.psg.metric_env_steps,
        };
        serde_json::to_string(&js).unwrap_or_else(|_|"{}".into())
    }
    #[wasm_bindgen] 
    pub fn integrator_segments_json(&mut self)->String {
        let segs = self.emulator.cpu.integrator.take_segments();
        // Simple JSON array of [x0,y0,x1,y1,intensity,frame]
        let mut out = String::from("[");
        for (i,s) in segs.iter().enumerate() {
            if i>0 { out.push(','); }
            out.push_str(&format!("[{:.2},{:.2},{:.2},{:.2},{},{}]", s.x0,s.y0,s.x1,s.y1,s.intensity,s.frame));
        }
        out.push(']');
        out
    }
    // --- Audio PSG PCM Export (fase inicial) ---
    /// Prepara snapshot lineal del ring de audio actual. Devuelve número de muestras copiadas.
    #[wasm_bindgen] 
    pub fn psg_prepare_pcm(&mut self) -> u32 { 
        self.emulator.cpu.bus.psg.prepare_export() as u32 
    }
    
    /// Retorna puntero base del buffer lineal preparado (i16 little-endian). Llamar tras psg_prepare_pcm.
    #[wasm_bindgen] 
    pub fn psg_pcm_ptr(&self) -> *const i16 { 
        self.emulator.cpu.bus.psg.export_ptr() 
    }
    
    /// Número de muestras (i16) en el buffer preparado.
    #[wasm_bindgen] 
    pub fn psg_pcm_len(&self) -> u32 { 
        self.emulator.cpu.bus.psg.export_len() as u32 
    }
    
    /// Serial incremental para detectar si cambió el contenido desde la última preparación.
    #[wasm_bindgen] 
    pub fn psg_pcm_serial(&self) -> u64 { 
        self.emulator.cpu.bus.psg.export_serial() 
    }
    
    /// Stride (en bytes) por muestra (siempre 2 actualmente, separado por robustez futura).
    #[wasm_bindgen] 
    pub fn psg_pcm_stride(&self) -> u32 { 
        std::mem::size_of::<i16>() as u32 
    }
    
    /// Sample rate nominal del generador de audio (Hz)
    #[wasm_bindgen] 
    pub fn psg_sample_rate(&self) -> u32 { 
        self.emulator.cpu.bus.psg.sample_rate() 
    }
    
    // --- Audio PSG Delta PCM Export ---
    /// Prepara delta de muestras nuevas desde la última export (full o delta). Devuelve número de muestras nuevas.
    #[wasm_bindgen] 
    pub fn psg_prepare_delta_pcm(&mut self) -> u32 { 
        self.emulator.cpu.bus.psg.prepare_delta_export() as u32 
    }
    
    /// Puntero al buffer delta (i16). Vacío si no hay muestras nuevas.
    #[wasm_bindgen] 
    pub fn psg_delta_pcm_ptr(&self) -> *const i16 { 
        self.emulator.cpu.bus.psg.delta_ptr() 
    }
    /// Longitud en muestras del delta actual.
    #[wasm_bindgen] 
    pub fn psg_delta_pcm_len(&self) -> u32 { 
        self.emulator.cpu.bus.psg.delta_len() as u32 
    }
    
    /// Indica si se produjo overflow (se perdió parte del delta y se devolvió snapshot completo en vez de delta parc.)
    #[wasm_bindgen] 
    pub fn psg_delta_overflow(&self) -> bool { 
        self.emulator.cpu.bus.psg.delta_overflow() 
    }
    
    // --- BIOS call stack export (TODO 13) ---
    /// Devuelve las últimas llamadas BIOS registradas (máx 256) en formato JSON array de strings "FFFF:LABEL".
    #[wasm_bindgen] 
    pub fn bios_calls_json(&self) -> String {
        if self.emulator.cpu.bios_calls.is_empty() { return "[]".into(); }
        // Limitar a las últimas 256 para no crecer sin límite en sesiones largas.
        let slice = if self.emulator.cpu.bios_calls.len() > 256 { 
            &self.emulator.cpu.bios_calls[self.emulator.cpu.bios_calls.len()-256..] 
        } else { 
            &self.emulator.cpu.bios_calls[..] 
        };
        // Exportar simple array de strings (sin envolver en objetos) para consumo directo.
        serde_json::to_string(slice).unwrap_or_else(|_|"[]".into())
    }
    
    /// Limpia el buffer de llamadas BIOS (útil en depuración / reinicios parciales en la UI).
    #[wasm_bindgen] 
    pub fn clear_bios_calls(&mut self){ 
        self.emulator.cpu.bios_calls.clear(); 
    }
    
    // Non-draining JSON view (does not clear internal buffer)
    #[wasm_bindgen] 
    pub fn integrator_segments_peek_json(&self)->String {
        let segs = self.emulator.cpu.integrator.segments_slice();
        let mut out = String::from("[");
        for (i,s) in segs.iter().enumerate() {
            if i>0 { out.push(','); }
            out.push_str(&format!("[{:.2},{:.2},{:.2},{:.2},{},{}]", s.x0,s.y0,s.x1,s.y1,s.intensity,s.frame));
        }
        out.push(']'); out
    }
    
    // Shared memory export helpers: we allocate a temporary copy buffer each call; UI can read via pointer.
    // For persistent zero-copy, future work could maintain a ring buffer.
    #[wasm_bindgen] 
    pub fn integrator_segments_ptr(&mut self) -> *const u8 {
        // Store copy in CPU reusable staging vec (added field) OR allocate ephemeral (here ephemeral for simplicity)
        self.emulator.cpu.temp_segments_c = self.emulator.cpu.integrator.segments_c_copy();
        self.emulator.cpu.temp_segments_c.as_ptr() as *const u8
    }
    
    #[wasm_bindgen] 
    pub fn integrator_segments_len(&self) -> u32 { 
        self.emulator.cpu.integrator.segments.len() as u32 
    }
    
    #[wasm_bindgen] 
    pub fn integrator_segment_stride(&self) -> u32 { 
        std::mem::size_of::<crate::integrator::BeamSegmentC>() as u32 
    }
    
    /// Devuelve el número de segmentos actualmente acumulados SIN copiar ni drenar.
    /// Útil para saber si hay algo antes de decidir usar JSON o acceso compartido.
    #[wasm_bindgen] 
    pub fn integrator_segments_count(&self) -> u32 { 
        self.emulator.cpu.integrator.segments.len() as u32 
    }
    
    #[wasm_bindgen] 
    pub fn integrator_drain_segments(&mut self){ 
        self.emulator.cpu.integrator.take_segments(); 
    }
    #[wasm_bindgen] 
    pub fn demo_triangle(&mut self) {
        // Genera un triángulo equilátero aproximado de tamaño moderado centrado cerca del origen.
        // Evitamos usar velocidades enormes * ciclos que antes saturaban (clamp) y producían distorsión.
        // Estrategia: líneas definidas por puntos absolutos, velocidad = delta / cycles.
        let inten = if self.emulator.cpu.last_intensity==0 { 0x5F } else { self.emulator.cpu.last_intensity };
        self.emulator.cpu.last_intensity = inten;
        self.emulator.cpu.integrator.set_intensity(inten);
        // Puntos del triángulo (equilátero aproximado). Dibujamos en un solo frame con continuidad.
        let frame = self.emulator.cpu.cycle_frame;
        let p0 = (0.0_f32, 180.0_f32);
        let p1 = (-155.9_f32, -90.0_f32); // 180*sin(60)=155.88
        let p2 = (155.9_f32, -90.0_f32);
        // Colocar en p0 sin trazar segmento (teleport), luego encender haz y recorrer p1, p2, p0.
        self.emulator.cpu.integrator.instant_move(p0.0, p0.1);
        self.emulator.cpu.integrator.beam_on();
        let draw_cont = |emu: &mut WasmEmu, from:(f32,f32), to:(f32,f32)| {
            let cycles: u32 = 100; // más ciclos = línea más larga; mantiene uniformidad
            let dx = to.0 - from.0; let dy = to.1 - from.1;
            let vx = dx / (cycles as f32);
            let vy = dy / (cycles as f32);
            emu.emulator.cpu.integrator.set_velocity(vx, vy);
            emu.emulator.cpu.integrator.tick(cycles, frame);
            emu.emulator.cpu.integrator.set_velocity(0.0, 0.0);
        };
        draw_cont(self, p0, p1);
        draw_cont(self, p1, p2);
        draw_cont(self, p2, p0); // cierre
        self.emulator.cpu.integrator.beam_off();
        let added = self.emulator.cpu.integrator.segments.len() as u32;
        if added > 0 && self.emulator.cpu.integrator_total_segments == 0 {
            self.emulator.cpu.integrator_last_frame_segments = added;
            if added > self.emulator.cpu.integrator_max_frame_segments { 
                self.emulator.cpu.integrator_max_frame_segments = added; 
            }
            self.emulator.cpu.integrator_total_segments = added as u64;
        }
    }
    // --- Auto demo toggle ---
    #[wasm_bindgen] 
    pub fn set_auto_demo(&mut self, en: bool) { 
        self.emulator.cpu.auto_demo = en; 
    }
    
    #[wasm_bindgen] 
    pub fn auto_demo_enabled(&self) -> bool { 
        self.emulator.cpu.auto_demo 
    }
    
    #[wasm_bindgen] 
    pub fn loop_watch_json(&self)->String {
        let mut out: Vec<JsLoopSample> = Vec::new();
        for s in &self.emulator.cpu.loop_watch_slots {
            if s.pc != 0 { 
                out.push(JsLoopSample{ 
                    pc:s.pc, a:s.a, b:s.b, x:s.x, y:s.y, u:s.u, s:s.s, dp:s.dp,
                    via_ifr:s.via_ifr, via_ier:s.via_ier, via_acr:s.via_acr, 
                    via_pcr:s.via_pcr, cycles:s.cycles 
                }); 
            }
        }
        serde_json::to_string(&out).unwrap_or_else(|_|"[]".into())
    }
    
    #[wasm_bindgen] 
    pub fn set_irq_frame_fallback(&mut self, en: bool) { 
        self.emulator.cpu.enable_irq_frame_fallback = en; 
    }
    
    #[wasm_bindgen] 
    pub fn irq_frame_fallback(&self) -> bool { 
        self.emulator.cpu.enable_irq_frame_fallback 
    }
    
    // --- New controls for vector backend & integrator line merging ---
    // Backend now fixed to integrator; setter/getter removed.
    #[wasm_bindgen] 
    pub fn set_integrator_merge_lines(&mut self, merge: bool) { 
        self.emulator.cpu.integrator.set_merge(merge); 
    }
    
    #[wasm_bindgen] 
    pub fn integrator_merge_lines(&self) -> bool { 
        self.emulator.cpu.integrator.merge_lines 
    }
    
    #[wasm_bindgen] 
    pub fn reset_integrator_segments(&mut self) { 
        self.emulator.cpu.integrator.segments.clear(); 
    }
    
    #[wasm_bindgen] 
    pub fn set_integrator_auto_drain(&mut self, en: bool) { 
        self.emulator.cpu.integrator_auto_drain = en; 
    }
    
    #[wasm_bindgen] 
    pub fn integrator_auto_drain(&self) -> bool { 
        self.emulator.cpu.integrator_auto_drain 
    }
    // --- Input API ---
    /// Actualiza estado de entrada (joystick analógico -128..127, botones bits 0..3)
    #[wasm_bindgen] 
    pub fn set_input_state(&mut self, x: i16, y: i16, buttons: u8) {
        let clamped_x = x.clamp(-128,127);
        let clamped_y = y.clamp(-128,127);
        self.emulator.cpu.input_state.x = clamped_x;
        self.emulator.cpu.input_state.y = clamped_y;
        self.emulator.cpu.input_state.buttons = buttons & 0x0F; // solo 4 botones
        // Map simple: escribir valores en RAM fija si BIOS los sondea (provisional 0x00F0..0x00F2)
        // 0x00F0: X (unsigned bias 128)
        // 0x00F1: Y (unsigned bias 128)
        // 0x00F2: botones (bit0..bit3)
        let base = 0x00F0u16;
        let bx = (clamped_x as i32 + 128) as u8;
        let by = (clamped_y as i32 + 128) as u8;
        self.emulator.cpu.bus.mem[base as usize] = bx;
        self.emulator.cpu.bus.mem[(base+1) as usize] = by;
        self.emulator.cpu.bus.mem[(base+2) as usize] = self.emulator.cpu.input_state.buttons;
    }

    /// Debug function to get detailed emulator state including vector info
    #[wasm_bindgen] 
    pub fn debug_state_json(&self) -> String {
        use serde_json::json;
        let segs = self.emulator.cpu.integrator.segments_slice();
        json!({
            "pc": format!("0x{:04X}", self.emulator.cpu.pc),
            "cycles": self.emulator.cpu.cycles,
            "bios_frame": self.emulator.cpu.bios_frame,
            "cycle_frame": self.emulator.cpu.cycle_frame,
            "segments_count": segs.len(),
            "last_intensity": self.emulator.cpu.last_intensity,
            "bios_loaded": self.emulator.cpu.bus.test_bios_base() != 0,
            "bios_base": format!("0x{:04X}", self.emulator.cpu.bus.test_bios_base()),
            "via_writes": self.emulator.cpu.via_write_count,
            "recent_segments": segs.iter().rev().take(5).map(|s| 
                format!("({:.1},{:.1})->({:.1},{:.1}) i={} f={}", s.x0, s.y0, s.x1, s.y1, s.intensity, s.frame)
            ).collect::<Vec<_>>()
        }).to_string()
    }
}
