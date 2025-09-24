// C++ Original: ShiftRegister.cpp + ShiftRegister.h
// Port 1:1 from Vectrexy libs/emulator/src/ShiftRegister.cpp

/// C++ Original: enum class ShiftRegisterMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftRegisterMode {
    /// C++ Original: Disabled
    Disabled,
    /// C++ Original: ShiftOutUnder02
    ShiftOutUnder02,
}

/// The VIA's shift register, mainly responsible for driving the drawing of line patterns. It can be
/// loaded with an 8 bit mask that represents the pattern to be drawn, and although it's called a
/// "shift" register, it actually rotates its values so the pattern will repeat.
/// C++ Original: class ShiftRegister
#[derive(Debug)]
pub struct ShiftRegister {
    /// C++ Original: ShiftRegisterMode m_mode = ShiftRegisterMode::Disabled;
    mode: ShiftRegisterMode,
    /// C++ Original: uint8_t m_value = 0;
    value: u8,
    /// C++ Original: mutable int m_shiftCyclesLeft = 0;
    shift_cycles_left: i32,
    /// C++ Original: bool m_cb2Active = false;
    cb2_active: bool,
    /// C++ Original: mutable bool m_interruptFlag = false;
    interrupt_flag: bool,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            mode: ShiftRegisterMode::Disabled,
            value: 0,
            shift_cycles_left: 0,
            cb2_active: false,
            interrupt_flag: false,
        }
    }

    /// C++ Original: void SetMode(ShiftRegisterMode mode)
    pub fn set_mode(&mut self, mode: ShiftRegisterMode) {
        self.mode = mode;
    }

    /// C++ Original: ShiftRegisterMode Mode() const
    pub fn mode(&self) -> ShiftRegisterMode {
        self.mode
    }

    /// C++ Original: void ShiftRegister::SetValue(uint8_t value)
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.shift_cycles_left = 18;
        self.interrupt_flag = false;
        self.update(2);
    }

    /// C++ Original: uint8_t ShiftRegister::ReadValue() const
    pub fn read_value(&mut self) -> u8 {
        self.shift_cycles_left = 18;
        self.interrupt_flag = false;
        self.value
    }

    /// C++ Original: bool CB2Active() const
    pub fn cb2_active(&self) -> bool {
        self.cb2_active
    }

    /// C++ Original: void ShiftRegister::Update(cycles_t cycles)
    pub fn update(&mut self, cycles: u64) {
        // C++ Original: for (int i = 0; i < cycles; ++i)
        for _ in 0..cycles {
            if self.shift_cycles_left > 0 {
                // C++ Original: if (m_shiftCyclesLeft % 2 == 1)
                if self.shift_cycles_left % 2 == 1 {
                    // C++ Original: bool isLastShiftCycle = m_shiftCyclesLeft == 1;
                    let is_last_shift_cycle = self.shift_cycles_left == 1;
                    if is_last_shift_cycle {
                        // For the last (9th) shift cycle, we output the same bit that was output for
                        // the 8th, which is now in bit position 0. We also don't shift (is that
                        // correct?)
                        // C++ Original: uint8_t bit = TestBits01(m_value, BITS(0));
                        let bit = test_bits_01(self.value, bits!(0));
                        // C++ Original: m_cb2Active = bit == 0;
                        self.cb2_active = bit == 0;
                    } else {
                        // C++ Original: uint8_t bit = TestBits01(m_value, BITS(7));
                        let bit = test_bits_01(self.value, bits!(7));
                        // C++ Original: m_cb2Active = bit == 0;
                        self.cb2_active = bit == 0;
                        // C++ Original: m_value = (m_value << 1) | bit;
                        self.value = (self.value << 1) | bit;
                    }
                }
                // C++ Original: --m_shiftCyclesLeft;
                self.shift_cycles_left -= 1;

                // Interrupt enable once we're done shifting
                // C++ Original: if (m_shiftCyclesLeft == 0) m_interruptFlag = true;
                if self.shift_cycles_left == 0 {
                    self.interrupt_flag = true;
                }
            }
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

    // Getters for testing
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn shift_cycles_left(&self) -> i32 {
        self.shift_cycles_left
    }
}

impl Default for ShiftRegister {
    fn default() -> Self {
        Self::new()
    }
}

// C++ Original: BitOps.h functions
/// C++ Original: inline T TestBits01(T target, U mask)
fn test_bits_01(target: u8, mask: u8) -> u8 {
    if (target & mask) != 0 { 1 } else { 0 }
}

/// C++ Original: BITS macro - creates bit mask
/// BITS(0) = 0x01, BITS(7) = 0x80, etc.
macro_rules! bits {
    (0) => { 0x01 };
    (1) => { 0x02 };
    (2) => { 0x04 };
    (3) => { 0x08 };
    (4) => { 0x10 };
    (5) => { 0x20 };
    (6) => { 0x40 };
    (7) => { 0x80 };
}

// Make macro available for use
use bits;