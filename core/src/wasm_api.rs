#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Removed: now provided by vectrex_emulator crate

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmEmu {
    cpu: CPU,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmu {
        let cpu = CPU::default();
        WasmEmu { cpu }
    }

    /// Load a BIOS image (4K -> F000, 8K -> E000). Returns true if accepted.
    #[wasm_bindgen]
    pub fn load_bios(&mut self, data: &[u8]) -> bool {
        let len = data.len();
        if !(len == 4096 || len == 8192) { return false; }
        self.cpu.load_bios(data);
        true
    }

    /// Load a binary blob (cartridge/user program) at supplied base address.
    #[wasm_bindgen]
    pub fn load_bin(&mut self, base: u16, data: &[u8]) {
        for (i,b) in data.iter().enumerate() { let addr = base as usize + i; if addr < 65536 { self.cpu.bus.mem[addr] = *b; } }
    }

    /// Perform a soft reset (re-evaluates vectors, clears dynamic flags).
    #[wasm_bindgen]
    pub fn reset(&mut self) { self.cpu.reset(); }

    /// Step up to `count` instructions (or until an unimplemented opcode halts). Returns executed count.
    #[wasm_bindgen]
    pub fn step(&mut self, count: u32) -> u32 {
        let mut executed = 0; for _ in 0..count { if !self.cpu.step() { break; } executed += 1; } executed
    }

    /// Run until a WAIT_RECAL BIOS call is observed or max instructions hit. Returns executed instructions.
    /// (Simplistic heuristic: track frame_count before & after; stops when it increments.)
    #[wasm_bindgen]
    pub fn run_until_wait_recal(&mut self, max_instructions: u32) -> u32 {
        let start_frames = self.cpu.frame_count; let mut executed = 0;
        while executed < max_instructions { if !self.cpu.step() { break; } executed += 1; if self.cpu.frame_count != start_frames { break; } }
        executed
    }

    /// Get registers as a JSON string (simple, avoids struct mapping for now).
    #[wasm_bindgen]
    pub fn registers_json(&self) -> String {
        format!("{{\"a\":{},\"b\":{},\"dp\":{},\"x\":{},\"y\":{},\"u\":{},\"s\":{},\"pc\":{},\"cycles\":{},\"frame_count\":{},\"last_intensity\":{},\"draw_vl_count\":{} }}",
            self.cpu.a, self.cpu.b, self.cpu.dp, self.cpu.x, self.cpu.y, self.cpu.u, self.cpu.s, self.cpu.pc, self.cpu.cycles, self.cpu.frame_count, self.cpu.last_intensity, self.cpu.draw_vl_count)
    }

    /// Direct pointer to full 64K memory (read-only view in JS via Uint8Array::new(memory_buffer)).
    #[wasm_bindgen]
    pub fn memory_ptr(&self) -> *const u8 { self.cpu.bus.mem.as_ptr() }

    /// Snapshot opcode metrics (human readable multiline string).
    #[wasm_bindgen]
    pub fn metrics(&self) -> String { self.cpu.metrics_pretty() }
// End of wasm_api.rs
}
