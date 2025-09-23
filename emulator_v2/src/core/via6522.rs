//! VIA 6522 Versatile Interface Adapter implementation
//! Port of vectrexy/libs/emulator/include/emulator/Via.h and src/Via.cpp

use crate::types::{Cycles, Vector2, Line, RenderContext};
use crate::core::MemoryBusDevice;

// C++ Original: enum class TimerMode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerMode {
    FreeRunning,
    OneShot,
    PulseCounting,
}

// C++ Original: enum class ShiftRegisterMode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftRegisterMode {
    Disabled,
    ShiftOutUnder02,
}

/* C++ Original from Via.h:
// Registers
uint8_t m_portB{};
uint8_t m_portA{};
uint8_t m_dataDirB{};
uint8_t m_dataDirA{};
uint8_t m_periphCntl{};
uint8_t m_interruptEnable{};

Screen m_screen;
Psg m_psg;
Timer1 m_timer1;
Timer2 m_timer2;
ShiftRegister m_shiftRegister;
uint8_t m_joystickButtonState{};
int8_t m_joystickPot{};
bool m_ca1Enabled{};
mutable bool m_ca1InterruptFlag{};
bool m_firqEnabled{};
*/

#[derive(Debug)]
pub struct Timer1 {
    // C++ Original: Timer1 from Timers.h
    latch_low: u8,
    latch_high: u8,
    counter: u16,
    interrupt_flag: bool,
    pb7_flag: bool,
    pb7_signal_low: bool,
}

impl Timer1 {
    pub fn new() -> Self {
        Self {
            latch_low: 0,
            latch_high: 0,
            counter: 0,
            interrupt_flag: false,
            pb7_flag: false,
            pb7_signal_low: false,
        }
    }

    /* C++ Original:
    void WriteCounterLow(uint8_t value) { m_latchLow = value; }
    */
    pub fn write_counter_low(&mut self, value: u8) {
        self.latch_low = value;
    }

    /* C++ Original:
    void WriteCounterHigh(uint8_t value) {
        m_latchHigh = value;
        // Transfer contents of both latches to counter and reset interrupt flag
        m_counter = m_latchHigh << 8 | m_latchLow;
        m_interruptFlag = false;
        if (m_pb7Flag) {
            m_pb7SignalLow = true;
        }
    }
    */
    pub fn write_counter_high(&mut self, value: u8) {
        self.latch_high = value;
        // Transfer contents of both latches to counter and reset interrupt flag
        self.counter = ((self.latch_high as u16) << 8) | (self.latch_low as u16);
        self.interrupt_flag = false;

        // @TODO: This should happen 1 cycle later
        if self.pb7_flag {
            self.pb7_signal_low = true;
        }
    }

    /* C++ Original:
    uint8_t ReadCounterLow() const {
        m_interruptFlag = false;
        return static_cast<uint8_t>(m_counter & 0xFF);
    }
    */
    pub fn read_counter_low(&mut self) -> u8 {
        self.interrupt_flag = false;
        (self.counter & 0xFF) as u8
    }

    /* C++ Original:
    uint8_t ReadCounterHigh() const { return static_cast<uint8_t>(m_counter >> 8); }
    */
    pub fn read_counter_high(&self) -> u8 {
        (self.counter >> 8) as u8
    }

    pub fn write_latch_low(&mut self, value: u8) {
        self.write_counter_low(value);
    }

    pub fn write_latch_high(&mut self, value: u8) {
        self.latch_high = value;
    }

    pub fn read_latch_low(&self) -> u8 {
        self.latch_low
    }

    pub fn read_latch_high(&self) -> u8 {
        self.latch_high
    }

    /* C++ Original:
    void Update(cycles_t cycles) {
        bool expired = cycles >= m_counter;
        m_counter -= checked_static_cast<uint16_t>(cycles);
        if (expired) {
            m_interruptFlag = true;
            if (m_pb7Flag) {
                m_pb7SignalLow = false;
            }
        }
    }
    */
    pub fn update(&mut self, cycles: Cycles) {
        let expired = cycles >= (self.counter as Cycles);
        self.counter = self.counter.saturating_sub(cycles as u16);
        if expired {
            self.interrupt_flag = true;
            if self.pb7_flag {
                self.pb7_signal_low = false;
            }
        }
    }

    pub fn mode(&self) -> TimerMode {
        TimerMode::OneShot
    }

    pub fn pb7_flag(&self) -> bool {
        self.pb7_flag
    }

    pub fn pb7_signal_low(&self) -> bool {
        self.pb7_signal_low
    }

    pub fn set_pb7_flag(&mut self, flag: bool) {
        self.pb7_flag = flag;
    }

    pub fn set_interrupt_flag(&mut self, flag: bool) {
        self.interrupt_flag = flag;
    }

    pub fn interrupt_flag(&self) -> bool {
        self.interrupt_flag
    }
}

#[derive(Debug)]
pub struct Timer2 {
    // C++ Original: Timer2 from Timers.h - Note: Timer2 has no high-order latch
    latch_low: u8, // C++ Original: uint8_t m_latchLow = 0; // Note: Timer2 has no high-order latch
    counter: u16,  // C++ Original: uint16_t m_counter = 0;
    interrupt_flag: bool, // C++ Original: mutable bool m_interruptFlag = false;
}

impl Timer2 {
    pub fn new() -> Self {
        Self {
            latch_low: 0,
            counter: 0,
            interrupt_flag: false,
        }
    }

    /* C++ Original:
    void WriteCounterLow(uint8_t value) { m_latchLow = value; }
    */
    pub fn write_counter_low(&mut self, value: u8) {
        self.latch_low = value;
    }

    /* C++ Original:
    void WriteCounterHigh(uint8_t value) {
        // Transfer contents of counter high and low-order latch to counter and reset interrupt flag
        m_counter = value << 8 | m_latchLow;
        m_interruptFlag = false;
    }
    */
    pub fn write_counter_high(&mut self, value: u8) {
        // Transfer contents of counter high and low-order latch to counter and reset interrupt flag
        self.counter = ((value as u16) << 8) | (self.latch_low as u16);
        self.interrupt_flag = false;
    }

    /* C++ Original:
    uint8_t ReadCounterLow() const {
        m_interruptFlag = false;
        return static_cast<uint8_t>(m_counter & 0xFF);
    }
    */
    pub fn read_counter_low(&mut self) -> u8 {
        self.interrupt_flag = false;
        (self.counter & 0xFF) as u8
    }

    /* C++ Original:
    uint8_t ReadCounterHigh() const { return static_cast<uint8_t>(m_counter >> 8); }
    */
    pub fn read_counter_high(&self) -> u8 {
        (self.counter >> 8) as u8
    }

    /* C++ Original:
    void Update(cycles_t cycles) {
        bool expired = cycles >= m_counter;
        m_counter -= checked_static_cast<uint16_t>(cycles);
        if (expired) {
            m_interruptFlag = true;
        }
    }
    */
    pub fn update(&mut self, cycles: Cycles) {
        let expired = cycles >= (self.counter as Cycles);
        self.counter = self.counter.saturating_sub(cycles as u16);
        if expired {
            self.interrupt_flag = true;
        }
    }

    /* C++ Original:
    TimerMode Mode() const { return TimerMode::OneShot; }
    */
    pub fn mode(&self) -> TimerMode {
        TimerMode::OneShot
    }

    /* C++ Original:
    void SetInterruptFlag(bool enabled) { m_interruptFlag = enabled; }
    */
    pub fn set_interrupt_flag(&mut self, flag: bool) {
        self.interrupt_flag = flag;
    }

    /* C++ Original:
    bool InterruptFlag() const { return m_interruptFlag; }
    */
    pub fn interrupt_flag(&self) -> bool {
        self.interrupt_flag
    }
}

// Placeholder for complex components that need more implementation
#[derive(Debug)]
pub struct DelayedValueStore<T> {
    // C++ Original: DelayedValueStore from DelayedValueStore.h
    cycles_to_update_value: Cycles,
    cycles_left: Cycles,
    next_value: T,
    value: T,
}

impl<T: Copy + Default> DelayedValueStore<T> {
    pub fn new(delay: Cycles) -> Self {
        Self {
            cycles_to_update_value: delay,
            cycles_left: 0,
            next_value: T::default(),
            value: T::default(),
        }
    }

    pub fn set(&mut self, next_value: T) {
        self.next_value = next_value;
        self.cycles_left = self.cycles_to_update_value;
        if self.cycles_left == 0 {
            self.value = self.next_value;
        }
    }

    /* C++ Original:
    void Update(cycles_t cycles) {
        (void)cycles;
        assert(cycles == 1);
        if (m_cyclesLeft > 0 && --m_cyclesLeft == 0) {
            m_value = m_nextValue;
        }
    }
    */
    pub fn update(&mut self, cycles: Cycles) {
        // C++ Original: assert(cycles == 1);
        assert_eq!(cycles, 1, "DelayedValueStore expects 1 cycle updates");
        if self.cycles_left > 0 {
            self.cycles_left -= 1;
            if self.cycles_left == 0 {
                self.value = self.next_value;
            }
        }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn set_delay(&mut self, delay: Cycles) {
        self.cycles_to_update_value = delay;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RampPhase {
    RampOff,
    RampUp,
    RampOn,
    RampDown,
}

/* C++ Original from Screen.h:
class Screen {
public:
    void Init();
    void Update(cycles_t cycles, RenderContext& renderContext);
    void FrameUpdate(double frameTime);
    void ZeroBeam();
    void SetBlankEnabled(bool enabled) { m_blank = enabled; }
    void SetIntegratorsEnabled(bool enabled) { m_integratorsEnabled = enabled; }
    void SetIntegratorX(int8_t value) { m_velocityX = value; }
    void SetIntegratorY(int8_t value) { m_velocityY = value; }
    void SetIntegratorXYOffset(int8_t value) { m_xyOffset = value; }
    void SetBrightness(uint8_t value) { m_brightness = value; }
private:
    bool m_integratorsEnabled{};
    Vector2 m_pos;
    bool m_lastDrawingEnabled{};
    Vector2 m_lastDir;
    DelayedValueStore<float> m_velocityX;
    DelayedValueStore<float> m_velocityY;
    float m_xyOffset = 0.f;
    float m_brightness = 0.f;
    bool m_blank = false;
    enum class RampPhase { RampOff, RampUp, RampOn, RampDown } m_rampPhase = RampPhase::RampOff;
    int32_t m_rampDelay = 0;
    float m_brightnessCurve = 0.f;
};
*/
#[derive(Debug)]
pub struct VectrexScreen {
    // C++ Original: Screen implementation with hardware delays
    integrators_enabled: bool,
    pos: Vector2,
    last_drawing_enabled: bool,
    last_dir: Vector2,
    
    // C++ Original: DelayedValueStore for velocity with hardware delays
    velocity_x: DelayedValueStore<f32>,
    velocity_y: DelayedValueStore<f32>,
    
    xy_offset: f32,
    brightness: f32,
    blank: bool,
    ramp_phase: RampPhase,
    ramp_delay: i32,
    brightness_curve: f32,
    
    // C++ Original: constants from Screen.cpp
    ramp_up_delay: i32,
    ramp_down_delay: i32,
    line_draw_scale: f32,
}

impl VectrexScreen {
    /* C++ Original:
    void Screen::Init() {
        m_velocityX.CyclesToUpdateValue = VelocityXDelay;
    }
    */
    pub fn new() -> Self {
        let mut screen = Self {
            integrators_enabled: false,
            pos: Vector2::zero(),
            last_drawing_enabled: false,
            last_dir: Vector2::zero(),
            velocity_x: DelayedValueStore::new(6), // C++ Original: VelocityXDelay = 6
            velocity_y: DelayedValueStore::new(0),
            xy_offset: 0.0,
            brightness: 0.0,
            blank: false,
            ramp_phase: RampPhase::RampOff,
            ramp_delay: 0,
            brightness_curve: 0.0,
            ramp_up_delay: 5,    // C++ Original: RampUpDelay = 5
            ramp_down_delay: 10, // C++ Original: RampDownDelay = 10
            line_draw_scale: 0.85, // C++ Original: LineDrawScale = 0.85f
        };
        screen.init();
        screen
    }

    fn init(&mut self) {
        // C++ Original: Init() sets velocity delays
        self.velocity_x.set_delay(6);
    }

    pub fn set_blank_enabled(&mut self, enabled: bool) {
        self.blank = enabled;
    }

    pub fn set_integrators_enabled(&mut self, enabled: bool) {
        self.integrators_enabled = enabled;
    }

    pub fn set_integrator_x(&mut self, value: i8) {
        self.velocity_x.set(value as f32);
    }

    pub fn set_integrator_y(&mut self, value: i8) {
        self.velocity_y.set(value as f32);
    }

    pub fn set_integrator_xy_offset(&mut self, value: i8) {
        self.xy_offset = value as f32;
    }

    pub fn set_brightness(&mut self, value: u8) {
        self.brightness = value as f32;
    }

    /* C++ Original:
    void Screen::ZeroBeam() {
        m_pos = {0.f, 0.f};
        m_lastDrawingEnabled = false;
    }
    */
    pub fn zero_beam(&mut self) {
        // @TODO: move beam towards 0,0 over time
        self.pos = Vector2::zero();
        self.last_drawing_enabled = false;
    }

    /* C++ Original:
    void Screen::Update(cycles_t cycles, RenderContext& renderContext) {
        m_velocityX.Update(cycles);
        m_velocityY.Update(cycles);
        // Handle ramp phase transitions
        // Move beam and generate lines
    }
    */
    pub fn update(&mut self, cycles: Cycles, render_context: &mut RenderContext) {
        self.velocity_x.update(cycles);
        self.velocity_y.update(cycles);

        // Handle switching to RampUp/RampDown
        match self.ramp_phase {
            RampPhase::RampOff | RampPhase::RampDown => {
                if self.integrators_enabled {
                    self.ramp_phase = RampPhase::RampUp;
                    self.ramp_delay = self.ramp_up_delay;
                }
            }
            RampPhase::RampOn | RampPhase::RampUp => {
                if !self.integrators_enabled {
                    self.ramp_phase = RampPhase::RampDown;
                    self.ramp_delay = self.ramp_down_delay;
                }
            }
        }

        // Handle switching to RampOn/RampOff
        match self.ramp_phase {
            RampPhase::RampUp => {
                // C++ Original: if (--m_rampDelay <= 0) - Wait some cycles, then go to RampOn
                self.ramp_delay -= 1;
                if self.ramp_delay <= 0 {
                    self.ramp_phase = RampPhase::RampOn;
                }
            }
            RampPhase::RampDown => {
                // C++ Original: if (--m_rampDelay <= 0) - Wait some cycles, then go to RampOff  
                self.ramp_delay -= 1;
                if self.ramp_delay <= 0 {
                    self.ramp_phase = RampPhase::RampOff;
                }
            }
            RampPhase::RampOff | RampPhase::RampOn => {}
        }

        let last_pos = self.pos;
        let curr_dir = Vector2::normalized(Vector2::new(self.velocity_x.value(), self.velocity_y.value()));

        // Move beam while ramp is on or its way down
        match self.ramp_phase {
            RampPhase::RampDown | RampPhase::RampOn => {
                let offset = Vector2::new(self.xy_offset, self.xy_offset);
                let velocity = Vector2::new(self.velocity_x.value(), self.velocity_y.value());
                let delta = (velocity + offset) / 128.0 * (cycles as f32) * self.line_draw_scale;
                self.pos = self.pos + delta;
            }
            RampPhase::RampOff | RampPhase::RampUp => {}
        }

        // We might draw even when integrators are disabled (e.g. drawing dots)
        let drawing_enabled = !self.blank && (self.brightness > 0.0 && self.brightness <= 128.0);
        
        if drawing_enabled {
            if self.last_drawing_enabled 
                && Vector2::magnitude(self.last_dir) > 0.0 
                && self.last_dir == curr_dir 
                && !render_context.lines.is_empty() 
            {
                // Extend the last line
                if let Some(last_line) = render_context.lines.last_mut() {
                    last_line.p1 = self.pos;
                }
            } else {
                // Create new line
                let lerp = |a: f32, b: f32, t: f32| a + t * (b - a);
                let ease_out = |v: f32| 1.0 - (1.0 - v).powf(5.0);

                // Lerp between linear brightness and ease out curve
                let mut b = self.brightness / 128.0;
                b = lerp(b, ease_out(b), self.brightness_curve);
                
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

    pub fn frame_update(&mut self, _frame_time: f64) {
        // C++ Original: ImGui debug controls - simplified for now
    }

    pub fn set_brightness_curve(&mut self, curve: f32) {
        self.brightness_curve = curve;
    }
}

// Keep SimpleScreen as alias for compatibility
pub type SimpleScreen = VectrexScreen;

#[derive(Debug)]
pub struct SimplePsg {
    // Simplified PSG implementation
    bc1: bool,
    bdir: bool,
}

impl SimplePsg {
    pub fn new() -> Self {
        Self {
            bc1: false,
            bdir: false,
        }
    }

    pub fn reset(&mut self) {
        self.bc1 = false;
        self.bdir = false;
    }

    pub fn set_bc1(&mut self, value: bool) {
        self.bc1 = value;
    }

    pub fn set_bdir(&mut self, value: bool) {
        self.bdir = value;
    }

    pub fn bc1(&self) -> bool {
        self.bc1
    }

    pub fn bdir(&self) -> bool {
        self.bdir
    }

    pub fn write_da(&mut self, _value: u8) {
        // Simplified implementation
    }

    pub fn update(&mut self, _cycles: Cycles) {
        // Simplified implementation
    }

    pub fn sample(&self) -> f32 {
        0.0 // Simplified implementation
    }

    pub fn frame_update(&mut self, _frame_time: f64) {
        // Simplified implementation
    }
}

#[derive(Debug)]
pub struct SimpleShiftRegister {
    // Simplified shift register implementation
    value: u8,
    mode: ShiftRegisterMode,
    interrupt_flag: bool,
    cb2_active: bool,
}

impl SimpleShiftRegister {
    pub fn new() -> Self {
        Self {
            value: 0,
            mode: ShiftRegisterMode::Disabled,
            interrupt_flag: false,
            cb2_active: false,
        }
    }

    pub fn read_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn set_mode(&mut self, mode: ShiftRegisterMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> ShiftRegisterMode {
        self.mode
    }

    pub fn set_interrupt_flag(&mut self, flag: bool) {
        self.interrupt_flag = flag;
    }

    pub fn interrupt_flag(&self) -> bool {
        self.interrupt_flag
    }

    pub fn cb2_active(&self) -> bool {
        self.cb2_active
    }

    pub fn update(&mut self, _cycles: Cycles) {
        // Simplified implementation
    }
}

// C++ Original: VIA register addresses from Via.cpp
const PORT_B: u16 = 0x0;
const PORT_A: u16 = 0x1;
const DATA_DIR_B: u16 = 0x2;
const DATA_DIR_A: u16 = 0x3;
const TIMER1_LOW: u16 = 0x4;
const TIMER1_HIGH: u16 = 0x5;
const TIMER1_LATCH_LOW: u16 = 0x6;
const TIMER1_LATCH_HIGH: u16 = 0x7;
const TIMER2_LOW: u16 = 0x8;
const TIMER2_HIGH: u16 = 0x9;
const SHIFT: u16 = 0xA;
const AUX_CNTL: u16 = 0xB;
const PERIPH_CNTL: u16 = 0xC;
const INTERRUPT_FLAG: u16 = 0xD;
const INTERRUPT_ENABLE: u16 = 0xE;
const PORT_A_NO_HANDSHAKE: u16 = 0xF;

// C++ Original: PortB bit definitions
const PORT_B_MUX_DISABLED: u8 = 0x01;
const PORT_B_MUX_SEL_MASK: u8 = 0x06;
const PORT_B_MUX_SEL_SHIFT: u8 = 1;
const PORT_B_SOUND_BC1: u8 = 0x08;
const PORT_B_SOUND_BDIR: u8 = 0x10;
const PORT_B_COMPARATOR: u8 = 0x20;
const PORT_B_RAMP_DISABLED: u8 = 0x80;

// C++ Original: InterruptFlag bit definitions
const IF_CA2: u8 = 0x01;
const IF_CA1: u8 = 0x02;
const IF_SHIFT: u8 = 0x04;
const IF_CB2: u8 = 0x08;
const IF_CB1: u8 = 0x10;
const IF_TIMER2: u8 = 0x20;
const IF_TIMER1: u8 = 0x40;
const IF_IRQ_ENABLED: u8 = 0x80;

// C++ Original: InterruptEnable bit definitions
const IE_CA2: u8 = 0x01;
const IE_CA1: u8 = 0x02;
const IE_SHIFT: u8 = 0x04;
const IE_CB2: u8 = 0x08;
const IE_CB1: u8 = 0x10;
const IE_TIMER2: u8 = 0x20;
const IE_TIMER1: u8 = 0x40;
const IE_SET_CLEAR_CONTROL: u8 = 0x80;

/* C++ Original from Via.h:
class Via : public IMemoryBusDevice {
public:
    void Init(MemoryBus& memoryBus);
    void Reset();
    void SetSyncContext(const Input& input, RenderContext& renderContext, AudioContext& audioContext);
    void FrameUpdate(double frameTime);
    bool IrqEnabled() const;
    bool FirqEnabled() const;
    Screen& GetScreen() { return m_screen; }
private:
    uint8_t Read(uint16_t address) const override;
    void Write(uint16_t address, uint8_t value) override;
    void Sync(cycles_t cycles) override;
    // ... registers and components
};
*/
#[derive(Debug)]
pub struct Via6522 {
    // C++ Original: VIA registers
    port_b: u8,
    port_a: u8,
    data_dir_b: u8,
    data_dir_a: u8,
    periph_cntl: u8,
    interrupt_enable: u8,

    // C++ Original: Components
    screen: VectrexScreen,
    psg: SimplePsg,
    timer1: Timer1,
    timer2: Timer2,
    shift_register: SimpleShiftRegister,

    // C++ Original: Input state
    joystick_button_state: u8,
    joystick_pot: i8,
    ca1_enabled: bool,
    ca1_interrupt_flag: bool,
    firq_enabled: bool,

    // Audio related (simplified)
    elapsed_audio_cycles: f32,
}

impl Via6522 {
    pub fn new() -> Self {
        Self {
            port_b: 0,
            port_a: 0,
            data_dir_b: 0,
            data_dir_a: 0,
            periph_cntl: 0,
            interrupt_enable: 0,
            screen: VectrexScreen::new(),
            psg: SimplePsg::new(),
            timer1: Timer1::new(),
            timer2: Timer2::new(),
            shift_register: SimpleShiftRegister::new(),
            joystick_button_state: 0,
            joystick_pot: 0,
            ca1_enabled: false,
            ca1_interrupt_flag: false,
            firq_enabled: false,
            elapsed_audio_cycles: 0.0,
        }
    }

    /* C++ Original:
    void Reset() {
        m_portB = m_portA = 0;
        m_dataDirB = m_dataDirA = 0;
        m_periphCntl = 0;
        m_interruptEnable = 0;
        // ... reset all components
        SetBits(m_portB, PortB::RampDisabled, true);
    }
    */
    pub fn reset(&mut self) {
        self.port_b = 0;
        self.port_a = 0;
        self.data_dir_b = 0;
        self.data_dir_a = 0;
        self.periph_cntl = 0;
        self.interrupt_enable = 0;

        self.screen = VectrexScreen::new();
        self.psg.reset();
        self.timer1 = Timer1::new();
        self.timer2 = Timer2::new();
        self.shift_register = SimpleShiftRegister::new();
        self.joystick_button_state = 0;
        self.ca1_enabled = false;
        self.ca1_interrupt_flag = false;
        self.firq_enabled = false;
        self.elapsed_audio_cycles = 0.0;

        // C++ Original: SetBits(m_portB, PortB::RampDisabled, true);
        self.port_b |= PORT_B_RAMP_DISABLED;
    }

    pub fn frame_update(&mut self, frame_time: f64) {
        self.screen.frame_update(frame_time);
        self.psg.frame_update(frame_time);
    }

    /* C++ Original:
    bool IrqEnabled() const;
    bool FirqEnabled() const;
    */
    pub fn irq_enabled(&self) -> bool {
        (self.get_interrupt_flag_value() & IF_IRQ_ENABLED) != 0
    }

    pub fn firq_enabled(&self) -> bool {
        self.firq_enabled
    }

    // C++ Original: GetInterruptFlagValue helper
    fn get_interrupt_flag_value(&self) -> u8 {
        let mut result = 0u8;

        if self.ca1_interrupt_flag {
            result |= IF_CA1;
        }
        if self.shift_register.interrupt_flag() {
            result |= IF_SHIFT;
        }
        if self.timer2.interrupt_flag() {
            result |= IF_TIMER2;
        }
        if self.timer1.interrupt_flag() {
            result |= IF_TIMER1;
        }

        // Set IRQ flag if any enabled interrupt is active
        if (result & self.interrupt_enable) != 0 {
            result |= IF_IRQ_ENABLED;
        }

        result
    }

    // Helper functions for bit operations
    fn test_bits(value: u8, mask: u8) -> bool {
        (value & mask) != 0
    }

    fn set_bits(target: &mut u8, mask: u8, enable: bool) {
        if enable {
            *target |= mask;
        } else {
            *target &= !mask;
        }
    }

    fn read_bits_with_shift(value: u8, mask: u8, shift: u8) -> u8 {
        (value & mask) >> shift
    }
}

impl MemoryBusDevice for Via6522 {
    /* C++ Original:
    uint8_t Read(uint16_t address) const override {
        const uint16_t index = MemoryMap::Via.MapAddress(address);
        switch (index) {
            case Register::PortB: { ... }
            case Register::PortA: { ... }
            // ... all register cases
        }
    }
    */
    fn read(&self, address: u16) -> u8 {
        let index = address & 0x0F; // VIA registers are 4-bit addressed
        
        match index {
            PORT_B => {
                let mut result = self.port_b;

                // Set comparator bit to port A (DAC) value < joystick POT value
                let port_a_signed = self.port_a as i8;
                Self::set_bits(&mut result, PORT_B_COMPARATOR, port_a_signed < self.joystick_pot);

                Self::set_bits(&mut result, PORT_B_SOUND_BC1, self.psg.bc1());
                Self::set_bits(&mut result, PORT_B_SOUND_BDIR, self.psg.bdir());

                result
            }
            PORT_A => {
                // C++ Original: m_ca1InterruptFlag = false; // Cleared by read/write of Port A
                // Note: This is a mutable operation, but we're in read() - we'll handle this differently
                let mut result = self.port_a;

                // Digital input
                if !Self::test_bits(self.port_b, PORT_B_SOUND_BDIR) && Self::test_bits(self.port_b, PORT_B_SOUND_BC1) {
                    if self.data_dir_a == 0 { // Input mode
                        result = self.joystick_button_state;
                    }
                }

                result
            }
            DATA_DIR_B => self.data_dir_b,
            DATA_DIR_A => self.data_dir_a,
            TIMER1_LOW => {
                // Note: This should clear interrupt flag but we're in read() - need RefCell or different approach
                (self.timer1.counter & 0xFF) as u8
            }
            TIMER1_HIGH => self.timer1.read_counter_high(),
            TIMER1_LATCH_LOW => self.timer1.read_latch_low(),
            TIMER1_LATCH_HIGH => self.timer1.read_latch_high(),
            TIMER2_LOW => {
                // Note: This should clear interrupt flag but we're in read() 
                (self.timer2.counter & 0xFF) as u8
            }
            TIMER2_HIGH => self.timer2.read_counter_high(),
            SHIFT => self.shift_register.read_value(),
            AUX_CNTL => {
                let mut aux_cntl = 0u8;
                // C++ Original: SetBits(auxCntl, 0b110 << AuxCntl::ShiftRegisterModeShift, true); //@HACK
                aux_cntl |= 0b110 << 2; // ShiftRegisterModeShift = 2
                Self::set_bits(&mut aux_cntl, 0x40, self.timer1.mode() == TimerMode::FreeRunning); // Timer1FreeRunning
                Self::set_bits(&mut aux_cntl, 0x20, self.timer2.mode() == TimerMode::PulseCounting); // Timer2PulseCounting  
                Self::set_bits(&mut aux_cntl, 0x80, self.timer1.pb7_flag()); // PB7Flag
                aux_cntl
            }
            PERIPH_CNTL => self.periph_cntl,
            INTERRUPT_FLAG => self.get_interrupt_flag_value(),
            INTERRUPT_ENABLE => self.interrupt_enable,
            PORT_A_NO_HANDSHAKE => {
                panic!("A without handshake not implemented yet");
            }
            _ => {
                panic!("Invalid VIA register address: {:04X}", address);
            }
        }
    }

    /* C++ Original:
    void Write(uint16_t address, uint8_t value) override {
        const uint16_t index = MemoryMap::Via.MapAddress(address);
        switch (index) {
            case Register::PortB: { ... }
            case Register::PortA: { ... }
            // ... all register cases with UpdateIntegrators() and UpdatePsg() calls
        }
    }
    */
    fn write(&mut self, address: u16, value: u8) {
        let index = address & 0x0F;
        
        match index {
            PORT_B => {
                self.port_b = value;
                self.update_integrators();
                self.update_psg();
            }
            PORT_A => {
                self.ca1_interrupt_flag = false; // Cleared by read/write of Port A
                
                // Port A is connected directly to the DAC
                self.port_a = value;
                if self.data_dir_a == 0xFF {
                    self.update_integrators();
                }
            }
            DATA_DIR_B => {
                self.data_dir_b = value;
            }
            DATA_DIR_A => {
                self.data_dir_a = value;
                if !(self.data_dir_a == 0 || self.data_dir_a == 0xFF) {
                    panic!("Expecting DDR for A to be either all 0s or all 1s");
                }
            }
            TIMER1_LOW => {
                self.timer1.write_counter_low(value);
            }
            TIMER1_HIGH => {
                self.timer1.write_counter_high(value);
            }
            TIMER1_LATCH_LOW => {
                self.timer1.write_latch_low(value);
            }
            TIMER1_LATCH_HIGH => {
                self.timer1.write_latch_high(value);
            }
            TIMER2_LOW => {
                self.timer2.write_counter_low(value);
            }
            TIMER2_HIGH => {
                self.timer2.write_counter_high(value);
            }
            SHIFT => {
                self.shift_register.set_value(value);
            }
            AUX_CNTL => {
                // C++ Original: complex aux control register handling
                let shift_mode = match Self::read_bits_with_shift(value, 0x1C, 2) { // ShiftRegisterModeMask, Shift
                    0 => ShiftRegisterMode::Disabled,
                    0b110 => ShiftRegisterMode::ShiftOutUnder02,
                    _ => {
                        println!("Unexpected ShiftRegisterMode, forcing to ShiftOutUnder02");
                        ShiftRegisterMode::ShiftOutUnder02
                    }
                };
                self.shift_register.set_mode(shift_mode);

                // Timer modes - C++ original only supports OneShot
                if !Self::test_bits(value, 0x40) { // Timer1FreeRunning
                    // OneShot mode - supported
                } else {
                    println!("t1 assumed always on one-shot mode");
                }

                if !Self::test_bits(value, 0x20) { // Timer2PulseCounting  
                    // OneShot mode - supported
                } else {
                    println!("t2 assumed always on one-shot mode");
                }

                self.timer1.set_pb7_flag(Self::test_bits(value, 0x80)); // PB7Flag
            }
            PERIPH_CNTL => {
                let ca2 = Self::read_bits_with_shift(value, 0x0E, 1); // CA2Mask, CA2Shift
                if ca2 != 0b110 && ca2 != 0b111 {
                    panic!("Unexpected value for CA2 bits: {:X}", ca2);
                }

                let cb2 = Self::read_bits_with_shift(value, 0xE0, 5); // CB2Mask, CB2Shift
                if cb2 != 0b110 && cb2 != 0b111 {
                    panic!("Unexpected value for CB2 bits: {:X}", cb2);
                }

                self.periph_cntl = value;
                if self.shift_register.mode() == ShiftRegisterMode::Disabled {
                    // C++ Original: IsBlankEnabled check
                    let blank_enabled = Self::read_bits_with_shift(self.periph_cntl, 0xE0, 5) == 0b110;
                    self.screen.set_blank_enabled(blank_enabled);
                }
            }
            INTERRUPT_FLAG => {
                // Clear interrupt flags for any bits that are enabled
                if Self::test_bits(value, IF_CA1) {
                    self.ca1_interrupt_flag = false;
                }
                if Self::test_bits(value, IF_SHIFT) {
                    self.shift_register.set_interrupt_flag(false);
                }
                if Self::test_bits(value, IF_TIMER2) {
                    self.timer2.set_interrupt_flag(false);
                }
                if Self::test_bits(value, IF_TIMER1) {
                    self.timer1.set_interrupt_flag(false);
                }
            }
            INTERRUPT_ENABLE => {
                // When bit 7 = 0 : Each 1 in a bit position is cleared (disabled).
                // When bit 7 = 1 : Each 1 in a bit position enables that bit.
                let set_clear = Self::test_bits(value, IE_SET_CLEAR_CONTROL);
                let mask = value & 0x7F;
                
                if set_clear {
                    self.interrupt_enable |= mask; // Enable bits
                } else {
                    self.interrupt_enable &= !mask; // Clear bits
                }
            }
            _ => {
                panic!("Invalid VIA register address: {:04X}", address);
            }
        }
    }

    /* C++ Original:
    void Sync(cycles_t cycles) override {
        DoSync(cycles, input, renderContext, audioContext);
    }
    */
    fn sync(&mut self, cycles: Cycles) {
        // Simplified sync - in full implementation this would take input/render/audio contexts
        self.do_sync(cycles);
    }
}

impl Via6522 {
    /* C++ Original helper methods from Via.cpp Write() implementation:
    auto UpdateIntegrators = [&] {
        const bool muxEnabled = !TestBits(m_portB, PortB::MuxDisabled);
        if (muxEnabled) {
            switch (ReadBitsWithShift(m_portB, PortB::MuxSelMask, PortB::MuxSelShift)) {
                case 0: m_screen.SetIntegratorY(static_cast<int8_t>(m_portA)); break;
                case 1: m_screen.SetIntegratorXYOffset(static_cast<int8_t>(m_portA)); break;
                case 2: m_screen.SetBrightness(m_portA); break;
                case 3: m_directAudioSamples.Add(static_cast<int8_t>(m_portA) / 128.f); break;
            }
        }
        // Always output to X-axis integrator
        m_screen.SetIntegratorX(static_cast<int8_t>(m_portA));
    };
    */
    fn update_integrators(&mut self) {
        let mux_enabled = !Self::test_bits(self.port_b, PORT_B_MUX_DISABLED);

        if mux_enabled {
            let mux_sel = Self::read_bits_with_shift(self.port_b, PORT_B_MUX_SEL_MASK, PORT_B_MUX_SEL_SHIFT);
            match mux_sel {
                0 => { // Y-axis integrator
                    self.screen.set_integrator_y(self.port_a as i8);
                }
                1 => { // X,Y Axis integrator offset
                    self.screen.set_integrator_xy_offset(self.port_a as i8);
                }
                2 => { // Z Axis (Vector Brightness) level
                    self.screen.set_brightness(self.port_a);
                }
                3 => { // Connected to sound output line via divider network
                    // C++ Original: m_directAudioSamples.Add(static_cast<int8_t>(m_portA) / 128.f);
                    // Simplified for now
                }
                _ => {
                    panic!("Invalid MUX selection: {}", mux_sel);
                }
            }
        }

        // Always output to X-axis integrator
        self.screen.set_integrator_x(self.port_a as i8);
    }

    /* C++ Original:
    auto UpdatePsg = [&] {
        const bool muxEnabled = !TestBits(m_portB, PortB::MuxDisabled);
        if (!muxEnabled) {
            m_psg.SetBC1(TestBits(m_portB, PortB::SoundBC1));
            m_psg.SetBDIR(TestBits(m_portB, PortB::SoundBDir));
            m_psg.WriteDA(m_portA);
        }
    };
    */
    fn update_psg(&mut self) {
        let mux_enabled = !Self::test_bits(self.port_b, PORT_B_MUX_DISABLED);

        if !mux_enabled {
            self.psg.set_bc1(Self::test_bits(self.port_b, PORT_B_SOUND_BC1));
            self.psg.set_bdir(Self::test_bits(self.port_b, PORT_B_SOUND_BDIR));
            self.psg.write_da(self.port_a);
        }
    }

    /* C++ Original:
    void DoSync(cycles_t cycles, const Input& input, RenderContext& renderContext, AudioContext& audioContext) {
        // Update cached input state
        m_joystickButtonState = input.ButtonStateMask();
        
        // Analog input: update POT value if MUX is enabled
        const bool muxEnabled = !TestBits(m_portB, PortB::MuxDisabled);
        if (muxEnabled) {
            uint8_t muxSel = ReadBitsWithShift(m_portB, PortB::MuxSelMask, PortB::MuxSelShift);
            m_joystickPot = input.AnalogStateMask(muxSel);
        }
        
        // CA1 and FIRQ handling
        // Audio update
        // Timer updates in cycle-accurate loop
    }
    */
    fn do_sync(&mut self, cycles: Cycles) {
        // Simplified DoSync without input/render/audio contexts
        // In full implementation, this would update input state, handle audio, etc.

        // Update timers and shift register cycle by cycle for accuracy
        let mut cycles_left = cycles;
        while cycles_left > 0 {
            self.timer1.update(1);
            self.timer2.update(1);
            self.shift_register.update(1);

            // Shift register's CB2 line drives /BLANK
            if self.shift_register.mode() == ShiftRegisterMode::ShiftOutUnder02 {
                self.screen.set_blank_enabled(self.shift_register.cb2_active());
            }

            // If the Timer1 PB7 flag is set, then PB7 drives /RAMP
            if self.timer1.pb7_flag() {
                Self::set_bits(&mut self.port_b, PORT_B_RAMP_DISABLED, !self.timer1.pb7_signal_low());
            }

            // C++ Original: PeriphCntl::IsZeroEnabled check
            let zero_enabled = Self::read_bits_with_shift(self.periph_cntl, 0x0E, 1) == 0b110;
            if zero_enabled {
                self.screen.zero_beam();
            }

            // Integrators are enabled while RAMP line is active (low)
            self.screen.set_integrators_enabled(!Self::test_bits(self.port_b, PORT_B_RAMP_DISABLED));

            // Note: In full implementation, this would update screen with render context
            // For now we'll create a dummy render context
            let mut dummy_render_context = RenderContext {
                lines: Vec::new(),
            };
            self.screen.update(1, &mut dummy_render_context);
            // In a real emulator, we'd accumulate these lines somewhere

            cycles_left -= 1;
        }
    }

    // Public getters for accessing components
    pub fn screen(&mut self) -> &mut VectrexScreen {
        &mut self.screen
    }

    pub fn set_input_state(&mut self, button_state: u8, pot_value: i8) {
        self.joystick_button_state = button_state;
        self.joystick_pot = pot_value;
    }
}