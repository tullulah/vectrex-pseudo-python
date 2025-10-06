// C++ Original: Timers.h
// Port 1:1 from Vectrexy libs/emulator/include/emulator/Timers.h

/// C++ Original: enum class TimerMode { FreeRunning, OneShot, PulseCounting };
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    FreeRunning,
    OneShot,
    PulseCounting,
}

impl TimerMode {
    /// C++ Original: inline const char* TimerModeToString(TimerMode mode)
    pub fn to_string(&self) -> &'static str {
        match self {
            TimerMode::FreeRunning => "FreeRunning",
            TimerMode::OneShot => "OneShot",
            TimerMode::PulseCounting => "PulseCounting",
        }
    }
}

/// Timer 1 is used mainly for drawing.
/// Supports timed interrupt each time t1 is loaded (one-shot), or continuous interrupts
/// (free-running) in which it auto-reloads initial count when it reaches 0.
/// C++ Original: class Timer1
#[derive(Debug, Clone)]
pub struct Timer1 {
    /// C++ Original: uint8_t m_latchLow = 0;
    latch_low: u8,
    /// C++ Original: uint8_t m_latchHigh = 0;
    latch_high: u8,
    /// C++ Original: uint16_t m_counter = 0;
    counter: u16,
    /// C++ Original: mutable bool m_interruptFlag = false;
    interrupt_flag: bool,
    /// C++ Original: bool m_pb7Flag = false;
    pb7_flag: bool,
    /// C++ Original: bool m_pb7SignalLow = false;
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

    /// C++ Original: void SetTimerMode(TimerMode mode)
    pub fn set_timer_mode(&self, mode: TimerMode) {
        // C++ Original: ASSERT_MSG(mode == TimerMode::OneShot, "Only supports one-shot mode for now");
        assert_eq!(
            mode,
            TimerMode::OneShot,
            "Only supports one-shot mode for now"
        );
    }

    /// C++ Original: TimerMode Mode() const
    pub fn mode(&self) -> TimerMode {
        TimerMode::OneShot
    }

    /// C++ Original: void WriteCounterLow(uint8_t value)
    pub fn write_counter_low(&mut self, value: u8) {
        self.latch_low = value;
    }

    /// C++ Original: void WriteCounterHigh(uint8_t value)
    pub fn write_counter_high(&mut self, value: u8) {
        self.latch_high = value;
        // Transfer contents of both latches to counter and reset interrupt flag
        self.counter = (self.latch_high as u16) << 8 | (self.latch_low as u16);
        self.interrupt_flag = false;

        //@TODO: This should happen 1 cycle later
        if self.pb7_flag {
            self.pb7_signal_low = true;
        }
    }

    /// C++ Original: uint8_t ReadCounterLow() const
    pub fn read_counter_low(&mut self) -> u8 {
        self.interrupt_flag = false;
        (self.counter & 0xFF) as u8
    }

    /// C++ Original: uint8_t ReadCounterHigh() const
    pub fn read_counter_high(&self) -> u8 {
        (self.counter >> 8) as u8
    }

    /// C++ Original: void WriteLatchLow(uint8_t value)
    pub fn write_latch_low(&mut self, value: u8) {
        self.write_counter_low(value);
    }

    /// C++ Original: void WriteLatchHigh(uint8_t value)
    pub fn write_latch_high(&mut self, value: u8) {
        self.latch_high = value;
    }

    /// C++ Original: uint8_t ReadLatchLow() const
    pub fn read_latch_low(&self) -> u8 {
        self.latch_low
    }

    /// C++ Original: uint8_t ReadLatchHigh() const
    pub fn read_latch_high(&self) -> u8 {
        self.latch_high
    }

    /// C++ Original: void Update(cycles_t cycles)
    pub fn update(&mut self, cycles: u64) {
        let expired = cycles >= self.counter as u64;
        self.counter = self.counter.saturating_sub(cycles as u16);
        if expired {
            self.interrupt_flag = true;
            //@TODO: When do we set this back to false? What is it used for?
            // self.interrupt_signal_low = true;
            self.pb7_signal_low = false;
        }
    }

    /// C++ Original: void SetInterruptFlag(bool enabled)
    pub fn set_interrupt_flag(&mut self, enabled: bool) {
        self.interrupt_flag = enabled;
    }

    /// C++ Original: bool InterruptFlag() const
    pub fn interrupt_flag(&self) -> bool {
        self.interrupt_flag
    }

    /// C++ Original: void SetPB7Flag(bool enabled)
    pub fn set_pb7_flag(&mut self, enabled: bool) {
        self.pb7_flag = enabled;
    }

    /// C++ Original: bool PB7Flag() const
    pub fn pb7_flag(&self) -> bool {
        self.pb7_flag
    }

    /// C++ Original: bool PB7SignalLow() const
    pub fn pb7_signal_low(&self) -> bool {
        self.pb7_signal_low
    }
}

impl Default for Timer1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer 2 is used mainly as a 50Hz game frame timer.
/// C++ Original: class Timer2
#[derive(Debug, Clone)]
pub struct Timer2 {
    /// Note: Timer2 has no high-order latch
    /// C++ Original: uint8_t m_latchLow = 0;
    latch_low: u8,
    /// C++ Original: uint16_t m_counter = 0;
    counter: u16,
    /// C++ Original: mutable bool m_interruptFlag = false;
    interrupt_flag: bool,
}

impl Timer2 {
    pub fn new() -> Self {
        Self {
            latch_low: 0,
            counter: 0,
            interrupt_flag: false,
        }
    }

    /// C++ Original: void SetTimerMode(TimerMode mode)
    pub fn set_timer_mode(&self, mode: TimerMode) {
        // C++ Original: ASSERT_MSG(mode == TimerMode::OneShot, "Only supports one-shot mode for now");
        assert_eq!(
            mode,
            TimerMode::OneShot,
            "Only supports one-shot mode for now"
        );
    }

    /// C++ Original: TimerMode Mode() const
    pub fn mode(&self) -> TimerMode {
        TimerMode::OneShot
    }

    /// C++ Original: void WriteCounterLow(uint8_t value)
    pub fn write_counter_low(&mut self, value: u8) {
        self.latch_low = value;
    }

    /// C++ Original: void WriteCounterHigh(uint8_t value)
    pub fn write_counter_high(&mut self, value: u8) {
        // Transfer contents of counter high and low-order latch to counter and reset interrupt flag
        self.counter = (value as u16) << 8 | (self.latch_low as u16);
        self.interrupt_flag = false;
    }

    /// C++ Original: uint8_t ReadCounterLow() const
    pub fn read_counter_low(&mut self) -> u8 {
        self.interrupt_flag = false;
        (self.counter & 0xFF) as u8
    }

    /// C++ Original: uint8_t ReadCounterHigh() const
    pub fn read_counter_high(&self) -> u8 {
        (self.counter >> 8) as u8
    }

    /// C++ Original: void Update(cycles_t cycles)
    pub fn update(&mut self, cycles: u64) {
        let expired = cycles >= self.counter as u64;
        self.counter = self.counter.saturating_sub(cycles as u16);
        if expired {
            self.interrupt_flag = true;
        }
    }

    /// C++ Original: void SetInterruptFlag(bool enabled)
    pub fn set_interrupt_flag(&mut self, enabled: bool) {
        self.interrupt_flag = enabled;
    }

    /// C++ Original: bool InterruptFlag() const
    pub fn interrupt_flag(&self) -> bool {
        self.interrupt_flag
    }
}

impl Default for Timer2 {
    fn default() -> Self {
        Self::new()
    }
}
