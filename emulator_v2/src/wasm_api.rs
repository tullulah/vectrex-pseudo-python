// WASM API for Vectrex Emulator V2
// Replicates JSVecx API surface for drop-in replacement in the IDE
//
// JSVecx Reference: ide/frontend/public/jsvecx_deploy/vecx.js
// API Pattern Match: 1:1 with JSVecx methods and structures

#![cfg(feature = "wasm")]

use crate::core::{
    engine_types::{AudioContext, Input, RenderContext},
    Emulator,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

// Set up panic hook to log detailed panic info to console
#[wasm_bindgen(start)]
pub fn wasm_init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// Embedded BIOS ROM data (8192 bytes: 4096 real + 4096 padding)
mod bios_rom_data {
    include!("bios_rom.rs");
}

/// Vector structure matching JSVecx vector_t
/// JSVecx Original: function vector_t() { this.x0 = 0; this.y0 = 0; this.x1 = 0; this.y1 = 0; this.color = 0; }
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vector {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
    pub color: u8,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
            color: 0,
        }
    }
}

/// Main WASM Emulator class matching JSVecx VecX API
/// JSVecx Original: function VecX()
#[wasm_bindgen]
pub struct VectrexEmulator {
    emulator: Emulator,
    render_context: RenderContext,
    audio_context: AudioContext,
    input: Input,

    // Vector output matching JSVecx
    vectors_draw: Vec<Vector>,
    vector_draw_cnt: usize,

    // Metrics matching JSVecx getMetrics()
    total_cycles: u64,
    instruction_count: u64,
    frame_count: u64,
    running: bool,

    // Debug info
    last_pc: u16,
    last_opcode: u8,
    last_error: String,
    pc_history: Vec<(u16, u8)>, // (PC, opcode) history buffer

    // Input state matching JSVecx
    left_held: bool,
    right_held: bool,
    up_held: bool,
    down_held: bool,
    button1_held: bool,
    button2_held: bool,
    button3_held: bool,
    button4_held: bool,
}

#[wasm_bindgen]
impl VectrexEmulator {
    /// Constructor matching JSVecx: new VecX()
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize with reasonable buffer sizes
        const VECTOR_CNT: usize = 50000; // JSVecx: VECTREX_MHZ / VECTREX_PDECAY

        Self {
            emulator: Emulator::new(),
            render_context: RenderContext::new(),
            audio_context: AudioContext::new(1500000.0 / 44100.0), // 1.5MHz / 44.1kHz
            input: Input::new(),
            vectors_draw: Vec::with_capacity(VECTOR_CNT),
            vector_draw_cnt: 0,
            total_cycles: 0,
            instruction_count: 0,
            frame_count: 0,
            running: false,
            last_pc: 0,
            last_opcode: 0,
            last_error: String::new(),
            pc_history: Vec::with_capacity(100), // Last 100 instructions
            left_held: false,
            right_held: false,
            up_held: false,
            down_held: false,
            button1_held: false,
            button2_held: false,
            button3_held: false,
            button4_held: false,
        }
    }

    /// Initialize emulator with BIOS
    /// JSVecx Pattern: init() + loadBios()
    /// Auto-loads embedded BIOS ROM (8192 bytes: 4KB real + 4KB padding)
    #[wasm_bindgen]
    pub fn init(&mut self) -> bool {
        // Initialize emulator structure
        self.emulator.init("");

        // Load embedded BIOS ROM
        let success = self.load_embedded_bios();

        // CRITICAL: Reset CPU to read reset vector and jump to BIOS entry point (0xF000)
        // Without this, PC stays at 0x0000 instead of reading vector at 0xFFFE
        if success {
            self.emulator.reset();
        }

        success
    }

    /// Load embedded BIOS ROM (auto-called by init)
    /// Uses 8192-byte BIOS embedded in WASM binary (4KB real + 4KB padding)
    fn load_embedded_bios(&mut self) -> bool {
        // C++ Original pattern: LoadBiosRom(const uint8_t* data, size_t size)
        // Load from embedded constant BIOS_ROM
        self.emulator.load_bios_from_bytes(bios_rom_data::BIOS_ROM)
    }

    /// Load BIOS from bytes (for custom BIOS)
    /// JSVecx Pattern: loadBiosFromBytes() (custom extension for WASM)
    #[wasm_bindgen(js_name = loadBiosBytes)]
    pub fn load_bios_bytes(&mut self, bios_data: &[u8]) -> bool {
        self.emulator.load_bios_from_bytes(bios_data)
    }

    /// Load ROM/cartridge
    /// JSVecx Pattern: loadRom(file)
    #[wasm_bindgen(js_name = loadRom)]
    pub fn load_rom(&mut self, rom_path: &str) -> bool {
        self.emulator.load_rom(rom_path)
    }

    /// Reset emulator
    /// JSVecx Pattern: reset()
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.stop();
        self.emulator.reset();
        self.vectors_draw.clear();
        self.vector_draw_cnt = 0;
        self.total_cycles = 0;
        self.instruction_count = 0;
        self.frame_count = 0;
    }

    /// Start emulation loop
    /// JSVecx Pattern: start()
    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop emulation loop
    /// JSVecx Pattern: stop()
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running
    /// JSVecx Pattern: isRunning()
    #[wasm_bindgen(js_name = isRunning)]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Execute one frame (called by JS animation loop)
    /// JSVecx Pattern: vecx_emu(cycles, 0) called in loop
    #[wasm_bindgen(js_name = runFrame)]
    pub fn run_frame(&mut self, cycles: u64) {
        if !self.running {
            return;
        }

        // Update input state from held buttons
        self.update_input();

        // Clear vectors from previous frame (CRITICAL: must clear BEFORE executing)
        self.vectors_draw.clear();
        self.vector_draw_cnt = 0;
        self.render_context.lines.clear();

        // Execute instructions for the frame
        let mut cycles_remaining = cycles;
        while cycles_remaining > 0 && self.running {
            let cpu_cycles = match self.emulator.execute_instruction(
                &self.input,
                &mut self.render_context,
                &mut self.audio_context,
            ) {
                Ok(cycles) => cycles,
                Err(e) => {
                    console::error_1(&format!("CPU Error: {}", e).into());
                    self.last_error = format!("CPU Error: {}", e);
                    self.running = false;
                    break;
                }
            };

            self.total_cycles += cpu_cycles;
            self.instruction_count += 1;

            if cpu_cycles > cycles_remaining {
                break;
            }
            cycles_remaining -= cpu_cycles;
        }

        // Convert RenderContext lines to JSVecx-compatible vectors
        for line in &self.render_context.lines {
            if self.vector_draw_cnt < self.vectors_draw.capacity() {
                self.vectors_draw.push(Vector {
                    x0: line.p0.x as i32,
                    y0: line.p0.y as i32,
                    x1: line.p1.x as i32,
                    y1: line.p1.y as i32,
                    color: (line.brightness * 128.0) as u8, // Convert 0.0-1.0 to 0-128
                });
                self.vector_draw_cnt += 1;
            }
        }

        // CRITICAL FIX: Clear render_context AFTER copying to vectors
        // This prevents accumulation between frames (matching step() behavior)
        self.render_context.lines.clear();

        self.frame_count += 1;
    }

    /// Update input state based on held keys
    fn update_input(&mut self) {
        // JSVecx Pattern: Updates alg_jch0/alg_jch1 from leftHeld/rightHeld/upHeld/downHeld
        // Maps to -127 to 127 range (0x00 = left/down, 0x80 = center, 0xFF = right/up)

        let x_axis = if self.left_held {
            -127
        } else if self.right_held {
            127
        } else {
            0
        };

        let y_axis = if self.down_held {
            -127
        } else if self.up_held {
            127
        } else {
            0
        };

        // Update Input structure - use actual Input API
        self.input.set_analog_axis_x(0, x_axis);
        self.input.set_analog_axis_y(0, y_axis);

        // Update button state - joystick 0, buttons 0-3
        self.input.set_button(0, 0, self.button1_held);
        self.input.set_button(0, 1, self.button2_held);
        self.input.set_button(0, 2, self.button3_held);
        self.input.set_button(0, 3, self.button4_held);
    }

    /// Get vector count
    /// JSVecx Pattern: this.vector_draw_cnt
    #[wasm_bindgen(js_name = getVectorCount)]
    pub fn get_vector_count(&self) -> usize {
        self.vector_draw_cnt
    }

    /// Get vector at index (returns raw values for JS to construct object)
    /// JSVecx Pattern: accessing this.vectors_draw[i]
    #[wasm_bindgen(js_name = getVector)]
    pub fn get_vector(&self, index: usize) -> Option<Vector> {
        if index < self.vector_draw_cnt {
            Some(self.vectors_draw[index])
        } else {
            None
        }
    }

    /// Get all vectors as JSON
    /// JSVecx Extension: For easier consumption from JS
    #[wasm_bindgen(js_name = getVectorsJson)]
    pub fn get_vectors_json(&self) -> String {
        let json = serde_json::to_string(&self.vectors_draw[..self.vector_draw_cnt])
            .unwrap_or_else(|_| "[]".to_string());

        json
    }

    /// Clear accumulated vectors (call after drawing each frame)
    #[wasm_bindgen(js_name = clearVectors)]
    pub fn clear_vectors(&mut self) {
        self.vectors_draw.clear();
        self.vector_draw_cnt = 0;
    }

    /// Get metrics as JSON
    /// JSVecx Pattern: getMetrics() returns { totalCycles, instructionCount, frameCount, running }
    #[wasm_bindgen(js_name = getMetrics)]
    pub fn get_metrics(&self) -> String {
        format!(
            r#"{{"totalCycles":{},"instructionCount":{},"frameCount":{},"running":{}}}"#,
            self.total_cycles, self.instruction_count, self.frame_count, self.running
        )
    }

    /// Get CPU registers as JSON
    /// JSVecx Pattern: getRegisters() returns { PC, A, B, X, Y, U, S, DP, CC }
    #[wasm_bindgen(js_name = getRegisters)]
    pub fn get_registers(&mut self) -> String {
        let cpu = self.emulator.get_cpu();
        let regs = cpu.registers();
        format!(
            r#"{{"PC":{},"A":{},"B":{},"X":{},"Y":{},"U":{},"S":{},"DP":{},"CC":{}}}"#,
            regs.pc,
            regs.a,
            regs.b,
            regs.x,
            regs.y,
            regs.u,
            regs.s,
            regs.dp,
            regs.cc.to_u8()
        )
    }

    /// Read memory byte
    /// JSVecx Pattern: read8(address)
    #[wasm_bindgen(js_name = read8)]
    pub fn read8(&mut self, address: u16) -> u8 {
        self.emulator.get_memory_bus().read(address)
    }

    /// Write memory byte
    /// JSVecx Pattern: write8(address, value)
    #[wasm_bindgen(js_name = write8)]
    pub fn write8(&mut self, address: u16, value: u8) {
        self.emulator.get_memory_bus().write(address, value);
    }

    // === Input Handlers (matching JSVecx key handling) ===

    /// Handle key down
    /// JSVecx Pattern: onkeydown(event)
    #[wasm_bindgen(js_name = onKeyDown)]
    pub fn on_key_down(&mut self, key_code: u32) {
        match key_code {
            37 | 76 => self.left_held = true,       // Left arrow or L
            38 | 80 => self.up_held = true,         // Up arrow or P
            39 | 222 => self.right_held = true,     // Right arrow or '
            40 | 59 | 186 => self.down_held = true, // Down arrow or ; or :
            65 => self.button1_held = true,         // A
            83 => self.button2_held = true,         // S
            68 => self.button3_held = true,         // D
            70 => self.button4_held = true,         // F
            _ => {}
        }
    }

    /// Handle key up
    /// JSVecx Pattern: onkeyup(event)
    #[wasm_bindgen(js_name = onKeyUp)]
    pub fn on_key_up(&mut self, key_code: u32) {
        match key_code {
            37 | 76 => self.left_held = false,
            38 | 80 => self.up_held = false,
            39 | 222 => self.right_held = false,
            40 | 59 | 186 => self.down_held = false,
            65 => self.button1_held = false,
            83 => self.button2_held = false,
            68 => self.button3_held = false,
            70 => self.button4_held = false,
            _ => {}
        }
    }

    /// Set joystick position directly (-127 to 127)
    /// Extension: For programmatic control
    #[wasm_bindgen(js_name = setJoystick)]
    pub fn set_joystick(&mut self, x: i8, y: i8) {
        // Map to held states for consistency with keyboard input
        self.left_held = x < -64;
        self.right_held = x > 64;
        self.up_held = y > 64;
        self.down_held = y < -64;
    }

    /// Set button state
    /// Extension: For programmatic control
    #[wasm_bindgen(js_name = setButton)]
    pub fn set_button(&mut self, button: u8, pressed: bool) {
        match button {
            1 => self.button1_held = pressed,
            2 => self.button2_held = pressed,
            3 => self.button3_held = pressed,
            4 => self.button4_held = pressed,
            _ => {}
        }
    }

    // ===== Rendering Output =====

    // NOTE: Lines are now accumulated directly in render_context during step()
    // No separate get_lines/clear_lines needed - architecture matches Vectrexy 1:1

    // ===== CPU Register Access (for debugging) =====

    /// Get Program Counter
    #[wasm_bindgen(js_name = getPC)]
    pub fn get_pc(&mut self) -> u16 {
        self.emulator.get_cpu().registers.pc
    }

    /// Get register A
    #[wasm_bindgen(js_name = getA)]
    pub fn get_a(&mut self) -> u8 {
        self.emulator.get_cpu().registers.a
    }

    /// Get register B
    #[wasm_bindgen(js_name = getB)]
    pub fn get_b(&mut self) -> u8 {
        self.emulator.get_cpu().registers.b
    }

    /// Get register D (A:B concatenated)
    #[wasm_bindgen(js_name = getD)]
    pub fn get_d(&mut self) -> u16 {
        self.emulator.get_cpu().registers.d()
    }

    /// Get register X
    #[wasm_bindgen(js_name = getX)]
    pub fn get_x(&mut self) -> u16 {
        self.emulator.get_cpu().registers.x
    }

    /// Get register Y
    #[wasm_bindgen(js_name = getY)]
    pub fn get_y(&mut self) -> u16 {
        self.emulator.get_cpu().registers.y
    }

    /// Get register U (User stack pointer)
    #[wasm_bindgen(js_name = getU)]
    pub fn get_u(&mut self) -> u16 {
        self.emulator.get_cpu().registers.u
    }

    /// Get register S (System stack pointer)
    #[wasm_bindgen(js_name = getS)]
    pub fn get_s(&mut self) -> u16 {
        self.emulator.get_cpu().registers.s
    }

    /// Get Direct Page register
    #[wasm_bindgen(js_name = getDP)]
    pub fn get_dp(&mut self) -> u8 {
        self.emulator.get_cpu().registers.dp
    }

    /// Get Condition Codes register
    #[wasm_bindgen(js_name = getCC)]
    pub fn get_cc(&mut self) -> u8 {
        let cc = &self.emulator.get_cpu().registers.cc;
        let mut result = 0u8;
        if cc.c {
            result |= 0x01;
        }
        if cc.v {
            result |= 0x02;
        }
        if cc.z {
            result |= 0x04;
        }
        if cc.n {
            result |= 0x08;
        }
        if cc.i {
            result |= 0x10;
        }
        if cc.h {
            result |= 0x20;
        }
        if cc.f {
            result |= 0x40;
        }
        if cc.e {
            result |= 0x80;
        }
        result
    }

    /// Get total cycles executed
    #[wasm_bindgen(js_name = getTotalCycles)]
    pub fn get_total_cycles(&self) -> u64 {
        self.total_cycles
    }

    /// Read byte from memory (for debugging)
    #[wasm_bindgen(js_name = readMemory)]
    pub fn read_memory(&mut self, address: u16) -> u8 {
        self.emulator.get_memory_bus().read(address)
    }

    /// Execute single instruction (for step debugging)
    #[wasm_bindgen(js_name = step)]
    pub fn step(&mut self) {
        // ARCHITECTURE FIX: Direct memory_bus access, no RefCell borrows needed
        let cpu = self.emulator.get_cpu();
        let pc = cpu.registers.pc;
        let opcode = self.emulator.get_memory_bus().read(pc);

        // Store in fields
        self.last_pc = pc;
        self.last_opcode = opcode;

        // Add to history buffer (circular, keep last 100)
        if self.pc_history.len() >= 100 {
            self.pc_history.remove(0);
        }
        self.pc_history.push((pc, opcode));

        let cycles = match self.emulator.execute_instruction(
            &self.input,
            &mut self.render_context,
            &mut self.audio_context,
        ) {
            Ok(cycles) => cycles,
            Err(e) => {
                console::error_1(&format!("CPU Error: {}", e).into());
                self.last_error = format!("CPU Error: {}", e);
                self.running = false;
                0
            }
        };

        // Convert RenderContext lines to vectors for JavaScript
        if self.render_context.lines.len() > 0 {
            for line in &self.render_context.lines {
                if self.vector_draw_cnt < self.vectors_draw.capacity() {
                    self.vectors_draw.push(Vector {
                        x0: line.p0.x as i32,
                        y0: line.p0.y as i32,
                        x1: line.p1.x as i32,
                        y1: line.p1.y as i32,
                        color: (line.brightness * 128.0) as u8,
                    });
                    self.vector_draw_cnt += 1;
                }
            }

            // Clear after conversion (important!)
            self.render_context.lines.clear();
        }

        self.total_cycles += cycles;
        self.instruction_count += 1;
    }

    /// Get last error message (for debugging)
    #[wasm_bindgen(js_name = getLastError)]
    pub fn get_last_error(&self) -> String {
        self.last_error.clone()
    }

    /// Get last PC (for debugging panics)
    #[wasm_bindgen(js_name = getLastPC)]
    pub fn get_last_pc(&self) -> u16 {
        self.last_pc
    }

    /// Get last opcode (for debugging panics)
    #[wasm_bindgen(js_name = getLastOpcode)]
    pub fn get_last_opcode(&self) -> u8 {
        self.last_opcode
    }

    /// Get PC history as JSON string (last N instructions before current)
    #[wasm_bindgen(js_name = getPCHistory)]
    pub fn get_pc_history(&self) -> String {
        let history: Vec<String> = self
            .pc_history
            .iter()
            .map(|(pc, opcode)| {
                format!(
                    "{{\"pc\":\"0x{:04X}\",\"opcode\":\"0x{:02X}\"}}",
                    pc, opcode
                )
            })
            .collect();
        format!("[{}]", history.join(","))
    }
}

// Implement Default to match JSVecx constructor pattern
impl Default for VectrexEmulator {
    fn default() -> Self {
        Self::new()
    }
}
