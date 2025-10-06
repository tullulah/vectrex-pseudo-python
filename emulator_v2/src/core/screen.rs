// C++ Original: Screen.cpp
// Port 1:1 from Vectrexy libs/emulator/src/Screen.cpp

use crate::core::delayed_value_store::DelayedValueStore;
use crate::core::engine_types::RenderContext;
use crate::types::{magnitude, normalized, Line, Vector2};

/// C++ Original: enum class RampPhase { RampOff, RampUp, RampOn, RampDown }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RampPhase {
    RampOff,
    RampUp,
    RampOn,
    RampDown,
}

/// Models the actual 9" screen that comes with a Vectrex, including hardware delays when moving the
/// beam, etc.
/// C++ Original: class Screen
#[derive(Debug)]
pub struct Screen {
    /// C++ Original: bool m_integratorsEnabled{};
    integrators_enabled: bool,
    /// C++ Original: Vector2 m_pos;
    pos: Vector2,
    /// C++ Original: bool m_lastDrawingEnabled{};
    last_drawing_enabled: bool,
    /// C++ Original: Vector2 m_lastDir;
    last_dir: Vector2,
    /// C++ Original: DelayedValueStore<float> m_velocityX;
    velocity_x: DelayedValueStore<f32>,
    /// C++ Original: DelayedValueStore<float> m_velocityY;
    velocity_y: DelayedValueStore<f32>,
    /// C++ Original: float m_xyOffset = 0.f;
    xy_offset: f32,
    /// C++ Original: float m_brightness = 0.f;
    brightness: f32,
    /// C++ Original: bool m_blank = false;
    blank: bool,
    /// C++ Original: enum class RampPhase { RampOff, RampUp, RampOn, RampDown } m_rampPhase = RampPhase::RampOff;
    ramp_phase: RampPhase,
    /// C++ Original: int32_t m_rampDelay = 0;
    ramp_delay: i32,
    /// C++ Original: float m_brightnessCurve = 0.f; // Set externally
    brightness_curve: f32,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            integrators_enabled: false,
            pos: Vector2::zero(),
            last_drawing_enabled: false,
            last_dir: Vector2::zero(),
            velocity_x: DelayedValueStore::new(),
            velocity_y: DelayedValueStore::new(),
            xy_offset: 0.0,
            brightness: 0.0,
            blank: false,
            ramp_phase: RampPhase::RampOff,
            ramp_delay: 0,
            brightness_curve: 0.0,
        }
    }

    /// C++ Original: void Screen::Init()
    pub fn init(&mut self) {
        // C++ Original: m_velocityX.CyclesToUpdateValue = VelocityXDelay;
        self.velocity_x.cycles_to_update_value = VELOCITY_X_DELAY;
    }

    /// C++ Original: void Screen::Update(cycles_t cycles, RenderContext& renderContext)
    pub fn update(&mut self, cycles: u64, render_context: &mut RenderContext) {
        // C++ Original: m_velocityX.Update(cycles); m_velocityY.Update(cycles);
        // Note: Vectrexy's Via::DoSync calls this with cycles=1 in a loop.
        // We expect cycles=1 here, matching Vectrexy's cycle-accurate architecture.
        self.velocity_x.update(cycles);
        self.velocity_y.update(cycles);

        // Handle switching to RampUp/RampDown
        match self.ramp_phase {
            RampPhase::RampOff | RampPhase::RampDown => {
                if self.integrators_enabled {
                    self.ramp_phase = RampPhase::RampUp;
                    self.ramp_delay = RAMP_UP_DELAY;
                }
            }
            RampPhase::RampOn | RampPhase::RampUp => {
                if !self.integrators_enabled {
                    self.ramp_phase = RampPhase::RampDown;
                    self.ramp_delay = RAMP_DOWN_DELAY;
                }
            }
        }

        // Handle switching to RampOn/RampOff
        match self.ramp_phase {
            RampPhase::RampUp => {
                // Wait some cycles, then go to RampOn
                // C++ Original: if (--m_rampDelay <= 0)
                self.ramp_delay -= 1;
                if self.ramp_delay <= 0 {
                    self.ramp_phase = RampPhase::RampOn;
                }
            }
            RampPhase::RampDown => {
                // Wait some cycles, then go to RampOff
                // C++ Original: if (--m_rampDelay <= 0)
                self.ramp_delay -= 1;
                if self.ramp_delay <= 0 {
                    self.ramp_phase = RampPhase::RampOff;
                }
            }
            RampPhase::RampOff | RampPhase::RampOn => {}
        }

        let last_pos = self.pos;
        let curr_dir = normalized(Vector2::new(
            *self.velocity_x.value(),
            *self.velocity_y.value(),
        ));

        // Move beam while ramp is on or its way down
        match self.ramp_phase {
            RampPhase::RampDown | RampPhase::RampOn => {
                // C++ Original: const auto offset = Vector2{m_xyOffset, m_xyOffset};
                let offset = Vector2::new(self.xy_offset, self.xy_offset);
                let velocity = Vector2::new(*self.velocity_x.value(), *self.velocity_y.value());
                // C++ Original: Vector2 delta = (velocity + offset) / 128.f * static_cast<float>(cycles) * LineDrawScale;
                let delta = (velocity + offset) / 128.0 * (cycles as f32) * LINE_DRAW_SCALE;
                self.pos += delta;
            }
            RampPhase::RampOff | RampPhase::RampUp => {}
        }

        // We might draw even when integrators are disabled (e.g. drawing dots)
        let drawing_enabled = !self.blank && (self.brightness > 0.0 && self.brightness <= 128.0);

        if drawing_enabled {
            if self.last_drawing_enabled
                && magnitude(self.last_dir) > 0.0
                && self.last_dir == curr_dir
                && !render_context.lines.is_empty()
            {
                // Extend the last line - update the end point
                if let Some(last_line) = render_context.lines.last_mut() {
                    last_line.p1 = self.pos;
                }
            } else {
                // Lerp between the linear brightness value and an ease out curve based on the user-set
                // brightness curve value.
                let mut b = self.brightness / 128.0;

                // C++ Original: auto easeOut = [](float v) { return 1.f - powf(1.f - v, 5); };
                let ease_out_b = 1.0 - (1.0 - b).powf(5.0);

                // C++ Original: auto lerp = [](float a, float b, float t) { return a + t * (b - a); };
                b = b + self.brightness_curve * (ease_out_b - b);

                // C++ Original: Create new Line with Vector2 p0, p1, brightness
                render_context.lines.push(Line {
                    p0: last_pos,
                    p1: self.pos,
                    brightness: b,
                });
            }
        }

        self.last_drawing_enabled = drawing_enabled;
        self.last_dir = curr_dir;
    }

    /// C++ Original: void Screen::FrameUpdate(double /*frameTime*/)
    pub fn frame_update(&mut self, _frame_time: f64) {
        // GUI logic would go here - simplified for this implementation
        self.velocity_x.cycles_to_update_value = VELOCITY_X_DELAY;
    }

    /// C++ Original: void Screen::ZeroBeam()
    pub fn zero_beam(&mut self) {
        //@TODO: move beam towards 0,0 over time
        self.pos = Vector2::zero();
        self.last_drawing_enabled = false;
    }

    /// C++ Original: void SetBlankEnabled(bool enabled)
    pub fn set_blank_enabled(&mut self, enabled: bool) {
        self.blank = enabled;
    }

    /// C++ Original: void SetIntegratorsEnabled(bool enabled)
    pub fn set_integrators_enabled(&mut self, enabled: bool) {
        self.integrators_enabled = enabled;
    }

    /// C++ Original: void SetIntegratorX(int8_t value)
    pub fn set_integrator_x(&mut self, value: i8) {
        self.velocity_x.assign(value as f32);
    }

    /// C++ Original: void SetIntegratorY(int8_t value)
    pub fn set_integrator_y(&mut self, value: i8) {
        self.velocity_y.assign(value as f32);
    }

    /// C++ Original: void SetIntegratorXYOffset(int8_t value)
    pub fn set_integrator_xy_offset(&mut self, value: i8) {
        self.xy_offset = value as f32;
    }

    /// C++ Original: void SetBrightness(uint8_t value)
    pub fn set_brightness(&mut self, value: u8) {
        self.brightness = value as f32;
    }

    /// C++ Original: void SetBrightnessCurve(float v)
    pub fn set_brightness_curve(&mut self, v: f32) {
        self.brightness_curve = v;
    }

    // Getters for testing
    pub fn pos(&self) -> Vector2 {
        self.pos
    }

    pub fn ramp_phase(&self) -> RampPhase {
        self.ramp_phase
    }

    pub fn integrators_enabled(&self) -> bool {
        self.integrators_enabled
    }

    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    pub fn blank(&self) -> bool {
        self.blank
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

// C++ Original: namespace { ... } constants
// //@TODO: make these conditionally const for "shipping" build
/// C++ Original: int32_t RampUpDelay = 5;
const RAMP_UP_DELAY: i32 = 5;
/// C++ Original: int32_t RampDownDelay = 10;
const RAMP_DOWN_DELAY: i32 = 10;
/// C++ Original: int32_t VelocityXDelay = 6;
const VELOCITY_X_DELAY: u64 = 6;
/// LineDrawScale is required because introducing ramp and velX delays means we now create lines
/// that go outside the 256x256 grid. So we scale down the line drawing values a little to make
/// it fit within the grid again.
/// C++ Original: float LineDrawScale = 0.85f;
const LINE_DRAW_SCALE: f32 = 0.85;
