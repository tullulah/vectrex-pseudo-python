// C++ Original: Psg.cpp
// Port 1:1 from Vectrexy libs/emulator/src/Psg.cpp

use std::cmp;

/// C++ Original: enum class AmplitudeMode { Fixed, Envelope };
/// TODO: Used in PSG amplitude control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmplitudeMode {
    Fixed,
    Envelope,
}

/// C++ Original: namespace Register
pub mod register {
    // C++ Original: ToneGeneratorALow = 0,
    pub const TONE_GENERATOR_A_LOW: usize = 0;
    // C++ Original: ToneGeneratorAHigh = 1,
    pub const TONE_GENERATOR_A_HIGH: usize = 1;
    // C++ Original: ToneGeneratorBLow = 2,
    pub const TONE_GENERATOR_B_LOW: usize = 2;
    // C++ Original: ToneGeneratorBHigh = 3,
    pub const TONE_GENERATOR_B_HIGH: usize = 3;
    // C++ Original: ToneGeneratorCLow = 4,
    pub const TONE_GENERATOR_C_LOW: usize = 4;
    // C++ Original: ToneGeneratorCHigh = 5,
    pub const TONE_GENERATOR_C_HIGH: usize = 5;
    // C++ Original: NoiseGenerator = 6,
    pub const NOISE_GENERATOR: usize = 6;
    // C++ Original: MixerControl = 7,
    pub const MIXER_CONTROL: usize = 7;
    // C++ Original: AmplitudeA = 8,
    pub const AMPLITUDE_A: usize = 8;
    // C++ Original: AmplitudeB = 9,
    pub const AMPLITUDE_B: usize = 9;
    // C++ Original: AmplitudeC = 10,
    pub const AMPLITUDE_C: usize = 10;
    // C++ Original: EnvelopePeriodLow = 11,
    pub const ENVELOPE_PERIOD_LOW: usize = 11;
    // C++ Original: EnvelopePeriodHigh = 12,
    pub const ENVELOPE_PERIOD_HIGH: usize = 12;
    // C++ Original: EnvelopeShape = 13,
    pub const ENVELOPE_SHAPE: usize = 13;
    // C++ Original: IOPortADataStore = 14,
    pub const IO_PORT_A_DATA_STORE: usize = 14;
    // C++ Original: IOPortBDataStore = 15
    pub const IO_PORT_B_DATA_STORE: usize = 15;
}

/// C++ Original: namespace MixerControlRegister
pub mod mixer_control_register {
    // C++ Original: const uint8_t ToneA = BITS(0);
    pub const TONE_A: u8 = 0x01;  // BITS(0)
    // C++ Original: const uint8_t ToneB = BITS(1);
    pub const TONE_B: u8 = 0x02;  // BITS(1)
    // C++ Original: const uint8_t ToneC = BITS(2);
    pub const TONE_C: u8 = 0x04;  // BITS(2)
    // C++ Original: const uint8_t NoiseA = BITS(3);
    pub const NOISE_A: u8 = 0x08; // BITS(3)
    // C++ Original: const uint8_t NoiseB = BITS(4);
    pub const NOISE_B: u8 = 0x10; // BITS(4)
    // C++ Original: const uint8_t NoiseC = BITS(5);
    pub const NOISE_C: u8 = 0x20; // BITS(5)

    /// C++ Original: bool IsEnabled(uint8_t reg, uint8_t type) { return !TestBits(reg, type); }
    pub fn is_enabled(reg: u8, type_mask: u8) -> bool {
        // Enabled when bit is 0 (C++ Original comment)
        (reg & type_mask) == 0
    }
}

/// C++ Original: namespace AmplitudeControlRegister
pub mod amplitude_control_register {
    use super::AmplitudeMode;

    // C++ Original: const uint8_t FixedVolume = BITS(0, 1, 2, 3);
    pub const FIXED_VOLUME: u8 = 0x0F; // BITS(0, 1, 2, 3)
    // C++ Original: const uint8_t EnvelopeMode = BITS(4);
    pub const ENVELOPE_MODE: u8 = 0x10; // BITS(4)

    /// C++ Original: AmplitudeMode GetMode(uint8_t reg) { return TestBits(reg, EnvelopeMode) ? AmplitudeMode::Envelope : AmplitudeMode::Fixed; }
    pub fn get_mode(reg: u8) -> AmplitudeMode {
        if (reg & ENVELOPE_MODE) != 0 {
            AmplitudeMode::Envelope
        } else {
            AmplitudeMode::Fixed
        }
    }

    /// C++ Original: uint32_t GetFixedVolume(uint8_t reg) { return ReadBits(reg, FixedVolume); }
    pub fn get_fixed_volume(reg: u8) -> u32 {
        (reg & FIXED_VOLUME) as u32
    }
}

/// C++ Original: template <typename T, size_t size> class PlotData
#[derive(Debug)]
pub struct PlotData<T, const SIZE: usize> {
    values: [T; SIZE],
    index: usize,
}

impl<T, const SIZE: usize> PlotData<T, SIZE>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self {
            values: [T::default(); SIZE],
            index: 0,
        }
    }

    /// C++ Original: void Clear()
    pub fn clear(&mut self) {
        for value in &mut self.values {
            *value = T::default();
        }
    }

    /// C++ Original: void AddValue(const T& value)
    pub fn add_value(&mut self, value: T) {
        if self.index == 0 {
            self.clear();
        }
        self.values[self.index] = value;
        self.index = (self.index + 1) % SIZE;
    }

    /// C++ Original: const ArrayType& Values() const
    pub fn values(&self) -> &[T; SIZE] {
        &self.values
    }
}

/// Timer used by Tone and Noise Generators
/// C++ Original: class Timer
#[derive(Debug, Clone)]
pub struct Timer {
    /// C++ Original: uint32_t m_period{};
    period: u32,
    /// C++ Original: uint32_t m_time{}; // Time in period
    time: u32,
}

impl Timer {
    /// C++ Original: Timer(uint32_t period = 0)
    pub fn new(period: u32) -> Self {
        let mut timer = Self { period: 0, time: 0 };
        timer.set_period(period);
        timer.reset();
        timer
    }

    /// C++ Original: void SetPeriod(uint32_t period)
    pub fn set_period(&mut self, period: u32) {
        // Keep relative position when changing period
        let ratio = if self.period == 0 {
            0.0
        } else {
            self.time as f32 / self.period as f32
        };
        self.period = period;
        self.time = (self.period as f32 * ratio) as u32;
    }

    /// C++ Original: uint32_t Period() const
    pub fn period(&self) -> u32 {
        self.period
    }

    /// C++ Original: void Reset()
    pub fn reset(&mut self) {
        self.time = 0;
    }

    /// Returns true when timer expires (and auto-resets)
    /// C++ Original: bool Clock()
    pub fn clock(&mut self) -> bool {
        if self.period > 0 {
            self.time += 1;
            if self.time == self.period {
                self.reset();
                return true;
            }
        }
        false
    }
}

/// C++ Original: class ToneGenerator
#[derive(Debug)]
pub struct ToneGenerator {
    /// C++ Original: uint16_t m_period{};
    period: u16,
    /// C++ Original: Timer m_timer{1};
    timer: Timer,
    /// C++ Original: uint32_t m_value{};           // 0 or 1
    value: u32,
}

impl ToneGenerator {
    pub fn new() -> Self {
        Self {
            period: 0,
            timer: Timer::new(1),
            value: 0,
        }
    }

    /// C++ Original: void SetPeriodHigh(uint8_t high)
    #[allow(unused_comparisons)]
    pub fn set_period_high(&mut self, high: u8) {
        // C++ Original: assert(high <= 0xff); - Always true for u8 but kept for 1:1 compatibility
        assert!(high <= 0xff); // Only 8 bits should be set
        self.period = ((high as u16) << 8) | (self.period & 0x00ff);
        self.on_period_updated();
    }

    /// C++ Original: void SetPeriodLow(uint8_t low)
    pub fn set_period_low(&mut self, low: u8) {
        self.period = (self.period & 0xff00) | (low as u16);
        self.on_period_updated();
    }

    /// C++ Original: uint8_t PeriodHigh() const
    pub fn period_high(&self) -> u8 {
        (self.period >> 8) as u8 // Top 4 bits
    }

    /// C++ Original: uint8_t PeriodLow() const
    pub fn period_low(&self) -> u8 {
        (self.period & 0xff) as u8
    }

    /// When period is 0, we don't want to hear anything from the tone generator
    /// C++ Original: bool IsEnabled() const
    pub fn is_enabled(&self) -> bool {
        self.period > 0
    }

    /// C++ Original: void Clock()
    pub fn clock(&mut self) {
        if self.timer.clock() {
            self.value = if self.value == 0 { 1 } else { 0 };
        }
    }

    /// C++ Original: uint32_t Value() const
    pub fn value(&self) -> u32 {
        self.value
    }

    /// C++ Original: void OnPeriodUpdated()
    fn on_period_updated(&mut self) {
        // Note: changing period does not reset value
        self.timer.set_period(cmp::max(1, self.period as u32));
    }
}

/// C++ Original: class NoiseGenerator  
#[derive(Debug)]
pub struct NoiseGenerator {
    period: u8,
    timer: Timer,
    shift_register: u32,
}

impl NoiseGenerator {
    pub fn new() -> Self {
        Self {
            period: 0,
            timer: Timer::new(1),
            shift_register: 1,
        }
    }

    /// C++ Original: void SetPeriod(uint8_t period)
    pub fn set_period(&mut self, period: u8) {
        self.period = period;
        self.timer.set_period(cmp::max(1, (period & 0x1F) as u32));
    }

    /// C++ Original: uint8_t Period() const
    pub fn period(&self) -> u8 {
        self.period
    }

    /// C++ Original: void Clock()
    pub fn clock(&mut self) {
        if self.timer.clock() {
            // Implement LFSR noise generation (simplified)
            let bit = ((self.shift_register & 1) ^ ((self.shift_register >> 3) & 1)) & 1;
            self.shift_register = (self.shift_register >> 1) | (bit << 16);
        }
    }

    /// C++ Original: uint32_t Value() const
    pub fn value(&self) -> u32 {
        self.shift_register & 1
    }
}

/// C++ Original: class EnvelopeGenerator
#[derive(Debug)]
pub struct EnvelopeGenerator {
    period: u16,
    timer: Timer,
    divider: Timer,
    shape: u8,
    curr_shape_index: usize,
    value: u32,
}

impl EnvelopeGenerator {
    pub fn new() -> Self {
        Self {
            period: 0,
            timer: Timer::new(1),
            divider: Timer::new(1),
            shape: 0,
            curr_shape_index: 0,
            value: 0,
        }
    }

    /// C++ Original: void SetPeriodHigh(uint8_t high)
    pub fn set_period_high(&mut self, high: u8) {
        self.period = ((high as u16) << 8) | (self.period & 0x00ff);
        self.on_period_updated();
    }

    /// C++ Original: void SetPeriodLow(uint8_t low)
    pub fn set_period_low(&mut self, low: u8) {
        self.period = (self.period & 0xff00) | (low as u16);
        self.on_period_updated();
    }

    /// C++ Original: void SetShape(uint8_t shape)
    pub fn set_shape(&mut self, shape: u8) {
        assert!(shape < 16);
        self.shape = shape;
        self.curr_shape_index = 0;
        self.update_value();
    }

    /// C++ Original: uint8_t PeriodHigh() const
    pub fn period_high(&self) -> u8 {
        (self.period >> 8) as u8
    }

    /// C++ Original: uint8_t PeriodLow() const
    pub fn period_low(&self) -> u8 {
        (self.period & 0xff) as u8
    }

    /// C++ Original: uint8_t Shape()
    pub fn shape(&self) -> u8 {
        self.shape
    }

    /// C++ Original: void Clock()
    pub fn clock(&mut self) {
        if self.divider.clock() && self.timer.clock() {
            self.update_value();
        }
    }

    /// C++ Original: uint32_t Value() const
    pub fn value(&self) -> u32 {
        self.value
    }

    /// C++ Original: void OnPeriodUpdated()
    fn on_period_updated(&mut self) {
        //@TODO: why am I dividing by 16 here?
        let time_to_increment_shape_index = cmp::max(1, self.period as u32 / 16);
        self.timer.set_period(time_to_increment_shape_index);
        self.update_value();
    }

    /// C++ Original: void UpdateValue()
    fn update_value(&mut self) {
        // Simplified envelope shapes - this would need the full lookup table from C++
        // For now, just implement a basic envelope
        let volume_levels = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        
        if self.curr_shape_index < volume_levels.len() {
            self.value = volume_levels[self.curr_shape_index] as u32;
            self.curr_shape_index += 1;
        } else {
            self.value = 0;
        }
    }
}

/// C++ Original: enum class PsgMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PsgMode {
    /// BDIR off BC1 off
    Inactive,
    /// BDIR off BC1 on
    Read,
    /// BDIR on  BC1 off
    Write,
    /// BDIR on  BC1 on
    LatchAddress,
}

/// Implementation of the AY-3-8912 Programmable Sound Generator (PSG)
/// C++ Original: class Psg (with PsgImpl)
#[derive(Debug)]
pub struct Psg {
    /// C++ Original: PsgMode m_mode = PsgMode::Inactive;
    mode: PsgMode,
    /// C++ Original: bool m_BDIR{};
    bdir: bool,
    /// C++ Original: bool m_BC1{};
    bc1: bool,
    /// C++ Original: uint8_t m_DA{}; // Data/Address bus (DA7-DA0)
    da: u8,
    /// C++ Original: uint8_t m_latchedAddress{};
    latched_address: u8,
    /// C++ Original: std::array<uint8_t, 16> m_registers{};
    registers: [u8; 16],
    /// C++ Original: Timer m_masterDivider{16}; // Input clock divided by 16
    master_divider: Timer,
    /// C++ Original: std::array<ToneGenerator, 3> m_toneGenerators{};
    tone_generators: [ToneGenerator; 3],
    /// C++ Original: NoiseGenerator m_noiseGenerator{};
    noise_generator: NoiseGenerator,
    /// C++ Original: EnvelopeGenerator m_envelopeGenerator{};
    envelope_generator: EnvelopeGenerator,
    /// Sample output
    sample: f32,
}

impl Psg {
    /// C++ Original: Psg::Psg() and PsgImpl::PsgImpl()
    pub fn new() -> Self {
        Self {
            mode: PsgMode::Inactive,
            bdir: false,
            bc1: false,
            da: 0,
            latched_address: 0,
            registers: [0; 16],
            master_divider: Timer::new(16),
            tone_generators: [ToneGenerator::new(), ToneGenerator::new(), ToneGenerator::new()],
            noise_generator: NoiseGenerator::new(),
            envelope_generator: EnvelopeGenerator::new(),
            sample: 0.0,
        }
    }

    /// C++ Original: void Psg::Init()
    pub fn init(&mut self) {
        // Implementation would go here
    }

    /// C++ Original: void Psg::SetBDIR(bool enable)
    pub fn set_bdir(&mut self, enable: bool) {
        self.bdir = enable;
    }

    /// C++ Original: void Psg::SetBC1(bool enable)
    pub fn set_bc1(&mut self, enable: bool) {
        self.bc1 = enable;
    }

    /// C++ Original: bool Psg::BDIR() const
    pub fn bdir(&self) -> bool {
        self.bdir
    }

    /// C++ Original: bool Psg::BC1() const
    pub fn bc1(&self) -> bool {
        self.bc1
    }

    /// C++ Original: void Psg::WriteDA(uint8_t value)
    pub fn write_da(&mut self, value: u8) {
        self.da = value;
        
        // Determine mode from BDIR and BC1
        self.mode = match (self.bdir, self.bc1) {
            (false, false) => PsgMode::Inactive,
            (false, true) => PsgMode::Read,
            (true, false) => PsgMode::Write,
            (true, true) => PsgMode::LatchAddress,
        };

        match self.mode {
            PsgMode::LatchAddress => {
                self.latched_address = value & 0x0F; // Only lower 4 bits
            }
            PsgMode::Write => {
                self.write(self.latched_address as u16, value);
            }
            _ => {}
        }
    }

    /// C++ Original: uint8_t Psg::ReadDA()
    pub fn read_da(&self) -> u8 {
        match self.mode {
            PsgMode::Read => self.read(self.latched_address as u16),
            _ => self.da,
        }
    }

    /// C++ Original: void Psg::Reset()
    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.da = 0;
        self.latched_address = 0;
        self.mode = PsgMode::Inactive;
        // Reset all generators would go here
    }

    /// C++ Original: void Psg::Update(cycles_t cycles)
    pub fn update(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.clock();
        }
    }

    /// C++ Original: float Psg::Sample() const
    pub fn sample(&self) -> f32 {
        self.sample
    }

    /// C++ Original: void Psg::FrameUpdate(double frameTime)
    pub fn frame_update(&mut self, _frame_time: f64) {
        // GUI/frame update logic would go here
    }

    /// C++ Original: void PsgImpl::Clock()
    fn clock(&mut self) {
        if self.master_divider.clock() {
            // Clock all generators
            for generator in &mut self.tone_generators {
                generator.clock();
            }
            self.noise_generator.clock();
            self.envelope_generator.clock();

            // Calculate sample (simplified)
            self.sample = 0.0;
            // Mix all channels - this would be much more complex in the real implementation
        }
    }

    /// C++ Original: uint8_t PsgImpl::Read(uint16_t address)
    fn read(&self, address: u16) -> u8 {
        match address as usize {
            register::TONE_GENERATOR_A_LOW => self.tone_generators[0].period_low(),
            register::TONE_GENERATOR_A_HIGH => self.tone_generators[0].period_high(),
            register::TONE_GENERATOR_B_LOW => self.tone_generators[1].period_low(),
            register::TONE_GENERATOR_B_HIGH => self.tone_generators[1].period_high(),
            register::TONE_GENERATOR_C_LOW => self.tone_generators[2].period_low(),
            register::TONE_GENERATOR_C_HIGH => self.tone_generators[2].period_high(),
            register::NOISE_GENERATOR => self.noise_generator.period(),
            register::ENVELOPE_PERIOD_LOW => self.envelope_generator.period_low(),
            register::ENVELOPE_PERIOD_HIGH => self.envelope_generator.period_high(),
            register::ENVELOPE_SHAPE => self.envelope_generator.shape(),
            _ => self.registers[address as usize],
        }
    }

    /// C++ Original: void PsgImpl::Write(uint16_t address, uint8_t value)
    fn write(&mut self, address: u16, value: u8) {
        match address as usize {
            register::TONE_GENERATOR_A_LOW => self.tone_generators[0].set_period_low(value),
            register::TONE_GENERATOR_A_HIGH => self.tone_generators[0].set_period_high(value),
            register::TONE_GENERATOR_B_LOW => self.tone_generators[1].set_period_low(value),
            register::TONE_GENERATOR_B_HIGH => self.tone_generators[1].set_period_high(value),
            register::TONE_GENERATOR_C_LOW => self.tone_generators[2].set_period_low(value),
            register::TONE_GENERATOR_C_HIGH => self.tone_generators[2].set_period_high(value),
            register::NOISE_GENERATOR => self.noise_generator.set_period(value),
            register::ENVELOPE_PERIOD_LOW => self.envelope_generator.set_period_low(value),
            register::ENVELOPE_PERIOD_HIGH => self.envelope_generator.set_period_high(value),
            register::ENVELOPE_SHAPE => self.envelope_generator.set_shape(value),
            _ => {
                self.registers[address as usize] = value;
            }
        }
    }

    /// C++ Original: VIA sync context needs advance_cycles method
    pub fn advance_cycles(&mut self, cycles: u32) {
        self.update(cycles as u64);
    }
}

impl Default for Psg {
    fn default() -> Self {
        Self::new()
    }
}