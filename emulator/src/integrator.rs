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
}

impl Integrator {
    pub fn new() -> Self { Self { merge_lines: true, ..Self::default() } }

    pub fn set_velocity(&mut self, vx: f32, vy: f32) { self.state.vx = vx; self.state.vy = vy; }
    pub fn set_intensity(&mut self, inten: u8) { self.state.intensity = inten; }
    pub fn beam_on(&mut self) { self.state.beam_on = true; }
    pub fn beam_off(&mut self) { self.state.beam_on = false; }
    pub fn instant_move(&mut self, x: f32, y: f32) { self.state.x = x; self.state.y = y; }

    // Advance by 'cycles' CPU cycles; integrate position and emit segment if drawing.
    pub fn tick(&mut self, cycles: u32, frame: u64) {
        if cycles == 0 { return; }
        // 1 cycle == arbitrary time unit; velocity is in units per cycle.
        if self.state.beam_on && self.state.intensity > 0 {
            let x0 = self.state.x; let y0 = self.state.y;
            self.state.x += self.state.vx * cycles as f32;
            self.state.y += self.state.vy * cycles as f32;
            let new_seg = BeamSegment { x0, y0, x1: self.state.x, y1: self.state.y, intensity: self.state.intensity, frame };
            if self.merge_lines {
                if let Some(last) = self.segments.last_mut() {
                    if last.frame == new_seg.frame && last.intensity == new_seg.intensity {
                        let vx_prev = last.x1 - last.x0; let vy_prev = last.y1 - last.y0;
                        let vx_new = new_seg.x1 - new_seg.x0; let vy_new = new_seg.y1 - new_seg.y0;
                        // Collinear if cross product ~ 0 (allow tiny epsilon) and same direction sign on dot.
                        let cross = vx_prev * vy_new - vy_prev * vx_new;
                        let dot = vx_prev * vx_new + vy_prev * vy_new;
                        if cross.abs() < 0.0001 && dot >= 0.0 {
                            // Extend last segment
                            last.x1 = new_seg.x1; last.y1 = new_seg.y1;
                        } else {
                            self.segments.push(new_seg);
                        }
                    } else {
                        self.segments.push(new_seg);
                    }
                } else {
                    self.segments.push(new_seg);
                }
            } else {
                self.segments.push(new_seg);
            }
        } else {
            // Even when blanked, still integrate (beam slews while off)
            self.state.x += self.state.vx * cycles as f32;
            self.state.y += self.state.vy * cycles as f32;
        }
        self.last_frame = frame;
    }

    pub fn take_segments(&mut self) -> Vec<BeamSegment> { let mut v=Vec::new(); std::mem::swap(&mut v,&mut self.segments); v }
    pub fn velocity(&self) -> (f32,f32) { (self.state.vx, self.state.vy) }
    pub fn set_merge(&mut self, en: bool){ self.merge_lines = en; }

    // Non-draining immutable view
    pub fn segments_slice(&self) -> &[BeamSegment] { &self.segments }

    // Produce a Vec<BeamSegmentC> copy (caller can cache / reuse)
    pub fn segments_c_copy(&self) -> Vec<BeamSegmentC> {
        self.segments.iter().map(|s| BeamSegmentC {
            x0:s.x0, y0:s.y0, x1:s.x1, y1:s.y1, intensity:s.intensity,
            _pad0:0,_pad1:0,_pad2:0, frame:s.frame, _reserved:0
        }).collect()
    }
}
