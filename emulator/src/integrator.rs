//! Minimal analog beam integrator model.
//! This first pass translates requests to move/draw (from CPU heuristics or future VIA DAC writes)
//! into straight line segments with a simple velocity * cycles integration.
//! Later versions will incorporate decay, ramp delays, and brightness curves.

#[derive(Clone, Copy, Debug, Default)]
pub struct BeamState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub intensity: u8,
    pub beam_on: bool,
}

#[derive(Clone, Debug)]
pub struct BeamSegment {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub intensity: u8,
    pub frame: u64,
}

// FFI-friendly packed representation (fixed size / alignment) for WASM shared memory reads.
// We keep f32 coordinates and u8 intensity; pad to 32 bytes for alignment simplicity.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BeamSegmentC {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub intensity: u8,
    pub _pad0: u8,
    pub _pad1: u8,
    pub _pad2: u8,
    pub frame: u64,
    pub _reserved: u64,
}

#[derive(Default)]
pub struct Integrator {
    state: BeamState,
    pub segments: Vec<BeamSegment>,
    last_frame: u64,
    // If enabled, attempt to merge consecutive collinear segments with same intensity.
    pub merge_lines: bool,
    // Optional: clamp coordinates to a virtual DAC range (Vectrex ~ 9-bit signed). Use symmetric range for simplicity.
    coord_min: f32,
    coord_max: f32,
    // Maximum length of a single emitted segment before splitting (helps downstream rasterizers / overlap artifacts).
    max_seg_len: f32,
    // Accumulate fractional cycle time for smoother velocity integration (sub-cycle remainder).
    frac_cycles: f32,
    // When beam is blanked but moving, optionally record "slew" motion as zero-intensity diagnostics if enabled.
    record_blank_slews: bool,
    // Intensity decay factor per tick when beam on (placeholder for future phosphor simulation). 1.0 = none.
    intensity_decay: f32,
    // Origin reset coordinates (e.g., after RESET0REF BIOS call) used by instant_move_or_soft.
    origin_x: f32,
    origin_y: f32,
}

impl Integrator {
    pub fn new() -> Self { Self { merge_lines: true, coord_min:-512.0, coord_max:512.0, max_seg_len:400.0, record_blank_slews:false, intensity_decay:1.0, ..Self::default() } }

    pub fn set_velocity(&mut self, vx: f32, vy: f32) { self.state.vx = vx; self.state.vy = vy; }
    pub fn set_intensity(&mut self, inten: u8) { self.state.intensity = inten; }
    pub fn beam_on(&mut self) { self.state.beam_on = true; }
    pub fn beam_off(&mut self) { self.state.beam_on = false; }
    pub fn instant_move(&mut self, x: f32, y: f32) { self.state.x = x; self.state.y = y; }
    pub fn reset_origin(&mut self){ self.origin_x=self.state.x; self.origin_y=self.state.y; }
    pub fn instant_move_or_soft(&mut self, x: f32, y: f32){
        // If distance large, just teleport (simulating DAC recentre), else integrate as a low-intensity slew.
        let dx=x-self.state.x; let dy=y-self.state.y; let dist=(dx*dx+dy*dy).sqrt();
        if dist>200.0 { self.state.x=x; self.state.y=y; self.reset_origin(); } else {
            // Mark a blanked slew segment if enabled.
            if self.record_blank_slews { self.segments.push(BeamSegment{ x0:self.state.x,y0:self.state.y,x1:x,y1:y,intensity:0,frame:self.last_frame }); }
            self.state.x=x; self.state.y=y;
        }
    }
    pub fn set_coord_range(&mut self, min:f32, max:f32){ self.coord_min=min; self.coord_max=max; }
    pub fn set_max_segment_length(&mut self, len:f32){ self.max_seg_len=len.max(1.0); }
    pub fn set_record_blank_slews(&mut self, en:bool){ self.record_blank_slews=en; }
    pub fn set_intensity_decay(&mut self, decay: f32){ self.intensity_decay=decay.max(0.0); }

    // Advance by 'cycles' CPU cycles; integrate position and emit segment if drawing.
    pub fn tick(&mut self, cycles: u32, frame: u64) {
        if cycles == 0 { return; }
        self.last_frame = frame;
        // Integrate with fractional remainder (future-proof if we later scale velocities differently).
        let total = cycles as f32 + self.frac_cycles;
        let whole = total.floor();
        self.frac_cycles = total - whole; // keep remainder
        let advance = whole.max(0.0);
        if advance <= 0.0 { return; }
        let mut target_x = self.state.x + self.state.vx * advance;
        let mut target_y = self.state.y + self.state.vy * advance;
        // Clamp coordinates
        target_x = target_x.clamp(self.coord_min, self.coord_max);
        target_y = target_y.clamp(self.coord_min, self.coord_max);
        if self.state.beam_on && self.state.intensity > 0 {
            // Apply optional intensity decay (simple linear placeholder)
            if self.intensity_decay != 1.0 {
                let decayed = (self.state.intensity as f32 * self.intensity_decay).round().clamp(0.0,255.0) as u8;
                self.state.intensity = decayed;
            }
            let seg_intensity = self.state.intensity;
            let x0=self.state.x; let y0=self.state.y;
            // Possibly split long segment
            let dx = target_x - x0; let dy = target_y - y0; let dist = (dx*dx+dy*dy).sqrt();
            if dist > self.max_seg_len {
                let parts = (dist / self.max_seg_len).ceil() as i32;
                if parts > 1 {
                    for i in 1..=parts {
                        let t = (i as f32)/(parts as f32);
                        let px = x0 + dx * t; let py = y0 + dy * t;
                        self.push_segment(x0 + dx * ((i-1) as f32)/(parts as f32), y0 + dy * ((i-1) as f32)/(parts as f32), px, py, seg_intensity, frame);
                    }
                } else {
                    self.push_segment(x0,y0,target_x,target_y,seg_intensity,frame);
                }
            } else {
                self.push_segment(x0,y0,target_x,target_y,seg_intensity,frame);
            }
        } else {
            // Record blank slews optionally
            if self.record_blank_slews && (self.state.x != target_x || self.state.y != target_y) {
                self.push_segment(self.state.x,self.state.y,target_x,target_y,0,frame);
            }
        }
        self.state.x = target_x; self.state.y = target_y;
    }

    fn push_segment(&mut self,x0:f32,y0:f32,x1:f32,y1:f32,intensity:u8,frame:u64){
        if x0==x1 && y0==y1 { return; }
        let new_seg = BeamSegment { x0,y0,x1,y1,intensity,frame };
        if self.merge_lines && intensity>0 {
            if let Some(last)=self.segments.last_mut() {
                if last.frame==new_seg.frame && last.intensity==intensity {
                    let vx_prev = last.x1 - last.x0; let vy_prev = last.y1 - last.y0;
                    let vx_new = new_seg.x1 - new_seg.x0; let vy_new = new_seg.y1 - new_seg.y0;
                    let cross = vx_prev * vy_new - vy_prev * vx_new; let dot = vx_prev * vx_new + vy_prev * vy_new;
                    if cross.abs() < 0.0001 && dot >= 0.0 {
                        last.x1=new_seg.x1; last.y1=new_seg.y1; return;
                    }
                }
            }
        }
        self.segments.push(new_seg);
    }

    pub fn take_segments(&mut self) -> Vec<BeamSegment> { let mut v=Vec::new(); std::mem::swap(&mut v,&mut self.segments); v }
    pub fn velocity(&self) -> (f32,f32) { (self.state.vx, self.state.vy) }
    pub fn set_merge(&mut self, en: bool){ self.merge_lines = en; }

    // Non-draining immutable view
    pub fn segments_slice(&self) -> &[BeamSegment] { &self.segments }
    pub fn origin(&self) -> (f32,f32) { (self.origin_x, self.origin_y) }

    // Produce a Vec<BeamSegmentC> copy (caller can cache / reuse)
    pub fn segments_c_copy(&self) -> Vec<BeamSegmentC> {
        self.segments.iter().map(|s| BeamSegmentC {
            x0:s.x0, y0:s.y0, x1:s.x1, y1:s.y1, intensity:s.intensity,
            _pad0:0,_pad1:0,_pad2:0, frame:s.frame, _reserved:0
        }).collect()
    }

    // Direct relative line emission (used by early BIOS Draw_VL decode shortcut until full VIA/DAC execution path is implemented).
    // This purposefully bypasses velocity integration and emits a single segment per list entry.
    pub fn line_to_rel(&mut self, dx:f32, dy:f32, intensity:u8, frame:u64){
        let x0 = self.state.x; let y0 = self.state.y;
        let mut x1 = x0 + dx; let mut y1 = y0 + dy;
        x1 = x1.clamp(self.coord_min, self.coord_max);
        y1 = y1.clamp(self.coord_min, self.coord_max);
        // Create segment for ALL movements - intensity=0 represents positioning moves that are still part of text formation
        self.push_segment(x0,y0,x1,y1,intensity,frame);
        self.state.x = x1; self.state.y = y1;
    }
    // Pure movement relative without emitting a segment (used for first vector in Draw_VL family = reposition).
    pub fn move_rel(&mut self, dx:f32, dy:f32){
        let mut x1 = self.state.x + dx; let mut y1 = self.state.y + dy;
        x1 = x1.clamp(self.coord_min, self.coord_max);
        y1 = y1.clamp(self.coord_min, self.coord_max);
        self.state.x = x1; self.state.y = y1;
    }
}
