// C++ Original: EngineTypes.h
// Port 1:1 from Vectrexy libs/emulator/include/emulator/EngineTypes.h

use std::path::PathBuf;
use crate::types::Line;

/// Input system for joysticks and buttons
/// C++ Original: class Input
#[derive(Debug)]
pub struct Input {
    /// Buttons 4,3,2,1 for joy 0 in bottom bits, and for joy 1 in top bits
    /// Bits on if not pressed
    joystick_button_state: u8,
    /// X1, Y1, X2, Y2
    joystick_analog_state: [i8; 4],
}

impl Input {
    pub fn new() -> Self {
        Self {
            joystick_button_state: 0xFF, // Bits on if not pressed
            joystick_analog_state: [0; 4],
        }
    }

    /// Set by engine
    /// C++ Original: void SetButton(uint8_t joystickIndex, uint8_t buttonIndex, bool enable)
    pub fn set_button(&mut self, joystick_index: u8, button_index: u8, enable: bool) {
        assert!(joystick_index < 2);
        assert!(button_index < 4);
        let mask = 1 << (button_index + joystick_index * 4);
        self.set_bits(mask, !enable);
    }

    /// C++ Original: void SetAnalogAxisX(int joystickIndex, int8_t value)
    pub fn set_analog_axis_x(&mut self, joystick_index: usize, value: i8) {
        self.joystick_analog_state[joystick_index * 2 + 0] = value;
    }

    /// C++ Original: void SetAnalogAxisY(int joystickIndex, int8_t value)
    pub fn set_analog_axis_y(&mut self, joystick_index: usize, value: i8) {
        self.joystick_analog_state[joystick_index * 2 + 1] = value;
    }

    /// Read by emulator
    /// C++ Original: uint8_t ButtonStateMask() const
    pub fn button_state_mask(&self) -> u8 {
        self.joystick_button_state
    }

    /// C++ Original: int8_t AnalogStateMask(int joyAxis) const
    pub fn analog_state_mask(&self, joy_axis: usize) -> i8 {
        self.joystick_analog_state[joy_axis]
    }

    /// C++ Original: bool IsButtonDown(uint8_t joystickIndex, uint8_t buttonIndex) const
    pub fn is_button_down(&self, joystick_index: u8, button_index: u8) -> bool {
        assert!(joystick_index < 2);
        assert!(button_index < 4);
        let mask = 1 << (button_index + joystick_index * 4);
        self.test_bits(mask) == false
    }

    // Helper functions from BitOps.h equivalent
    fn set_bits(&mut self, mask: u8, enable: bool) {
        if enable {
            self.joystick_button_state |= mask;
        } else {
            self.joystick_button_state &= !mask;
        }
    }

    fn test_bits(&self, mask: u8) -> bool {
        (self.joystick_button_state & mask) != 0
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

/// C++ Original: struct RenderContext
#[derive(Debug)]
pub struct RenderContext {
    /// C++ Original: std::vector<Line> lines;
    pub lines: Vec<Line>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Add a line to the render context
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

/// C++ Original: struct AudioContext
#[derive(Debug)]
pub struct AudioContext {
    /// C++ Original: const float CpuCyclesPerAudioSample
    pub cpu_cycles_per_audio_sample: f32,
    /// Samples produced this frame
    pub samples: Vec<f32>,
}

impl AudioContext {
    /// C++ Original: AudioContext(float cpuCyclesPerAudioSample)
    pub fn new(cpu_cycles_per_audio_sample: f32) -> Self {
        Self {
            cpu_cycles_per_audio_sample,
            samples: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn add_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }
}

/// C++ Original: class EmuEvent
pub mod emu_event {
    use std::path::PathBuf;

    /// C++ Original: struct BreakIntoDebugger {}
    #[derive(Debug, Clone)]
    pub struct BreakIntoDebugger;

    /// C++ Original: struct Reset {}
    #[derive(Debug, Clone)]
    pub struct Reset;

    /// C++ Original: struct OpenBiosRomFile
    #[derive(Debug, Clone)]
    pub struct OpenBiosRomFile {
        pub path: Option<PathBuf>,
    }

    /// C++ Original: struct OpenRomFile
    #[derive(Debug, Clone)]
    pub struct OpenRomFile {
        /// If not set, use open file dialog
        pub path: Option<PathBuf>,
    }
}

/// C++ Original: using Type = std::variant<BreakIntoDebugger, Reset, OpenBiosRomFile, OpenRomFile>;
#[derive(Debug, Clone)]
pub enum EmuEventType {
    BreakIntoDebugger,
    Reset,
    OpenBiosRomFile { path: Option<PathBuf> },
    OpenRomFile { path: Option<PathBuf> },
}

/// C++ Original: class EmuEvent
#[derive(Debug, Clone)]
pub struct EmuEvent {
    pub event_type: EmuEventType,
}

impl EmuEvent {
    pub fn break_into_debugger() -> Self {
        Self {
            event_type: EmuEventType::BreakIntoDebugger,
        }
    }

    pub fn reset() -> Self {
        Self {
            event_type: EmuEventType::Reset,
        }
    }

    pub fn open_bios_rom_file(path: Option<PathBuf>) -> Self {
        Self {
            event_type: EmuEventType::OpenBiosRomFile { path },
        }
    }

    pub fn open_rom_file(path: Option<PathBuf>) -> Self {
        Self {
            event_type: EmuEventType::OpenRomFile { path },
        }
    }
}

/// C++ Original: using EmuEvents = std::vector<EmuEvent>;
pub type EmuEvents = Vec<EmuEvent>;

/// C++ Original: class IEngineService
pub trait IEngineService {
    fn set_focus_main_window(&self);
    fn set_focus_console(&self);
    fn reset_overlay(&self, message: &str);
}

/// Default implementation for testing
pub struct MockEngineService;

impl IEngineService for MockEngineService {
    fn set_focus_main_window(&self) {
        // Mock implementation
    }

    fn set_focus_console(&self) {
        // Mock implementation
    }

    fn reset_overlay(&self, _message: &str) {
        // Mock implementation
    }
}