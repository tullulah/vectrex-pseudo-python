//! Input handling for joysticks and controllers
//! Port of vectrexy/libs/emulator/include/emulator/EngineTypes.h (Input class)

/* C++ Original:
class Input {
public:
    // Set by engine
    void SetButton(uint8_t joystickIndex, uint8_t buttonIndex, bool enable) {
        assert(joystickIndex < 2);
        assert(buttonIndex < 4);
        const uint8_t mask = 1 << (buttonIndex + joystickIndex * 4);
        SetBits(m_joystickButtonState, mask, enable == false);
    }
    void SetAnalogAxisX(int joystickIndex, int8_t value) {
        m_joystickAnalogState[joystickIndex * 2 + 0] = value;
    }

    void SetAnalogAxisY(int joystickIndex, int8_t value) {
        m_joystickAnalogState[joystickIndex * 2 + 1] = value;
    }

    // Read by emulator
    uint8_t ButtonStateMask() const { return m_joystickButtonState; }
    int8_t AnalogStateMask(int joyAxis) const { return m_joystickAnalogState[joyAxis]; }

    bool IsButtonDown(uint8_t joystickIndex, uint8_t buttonIndex) const {
        assert(joystickIndex < 2);
        assert(buttonIndex < 4);
        const uint8_t mask = 1 << (buttonIndex + joystickIndex * 4);
        return TestBits(m_joystickButtonState, mask) == false;
    }

private:
    // Buttons 4,3,2,1 for joy 0 in bottom bits, and for joy 1 in top bits
    uint8_t m_joystickButtonState = 0xFF; // Bits on if not pressed
    // X1, Y1, X2, Y2
    std::array<int8_t, 4> m_joystickAnalogState = {0};
};
*/

#[derive(Debug, Clone)]
pub struct Input {
    // C++ Original: uint8_t m_joystickButtonState = 0xFF; // Bits on if not pressed  
    // Buttons 4,3,2,1 for joy 0 in bottom bits, and for joy 1 in top bits
    joystick_button_state: u8,
    
    // C++ Original: std::array<int8_t, 4> m_joystickAnalogState = {0};
    // X1, Y1, X2, Y2
    joystick_analog_state: [i8; 4],
}

impl Default for Input {
    fn default() -> Self {
        Self {
            joystick_button_state: 0xFF, // Bits on if not pressed
            joystick_analog_state: [0; 4],
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    // C++ Original: void SetButton(uint8_t joystickIndex, uint8_t buttonIndex, bool enable)
    pub fn set_button(&mut self, joystick_index: u8, button_index: u8, enable: bool) {
        assert!(joystick_index < 2);
        assert!(button_index < 4);
        let mask = 1u8 << (button_index + joystick_index * 4);
        
        if !enable {
            self.joystick_button_state |= mask;  // Set bit (not pressed)
        } else {
            self.joystick_button_state &= !mask; // Clear bit (pressed)
        }
    }

    // C++ Original: void SetAnalogAxisX(int joystickIndex, int8_t value)
    pub fn set_analog_axis_x(&mut self, joystick_index: usize, value: i8) {
        self.joystick_analog_state[joystick_index * 2] = value;
    }

    // C++ Original: void SetAnalogAxisY(int joystickIndex, int8_t value)
    pub fn set_analog_axis_y(&mut self, joystick_index: usize, value: i8) {
        self.joystick_analog_state[joystick_index * 2 + 1] = value;
    }

    // C++ Original: uint8_t ButtonStateMask() const
    pub fn button_state_mask(&self) -> u8 {
        self.joystick_button_state
    }

    // C++ Original: int8_t AnalogStateMask(int joyAxis) const
    pub fn analog_state_mask(&self, joy_axis: usize) -> i8 {
        self.joystick_analog_state[joy_axis]
    }

    // C++ Original: bool IsButtonDown(uint8_t joystickIndex, uint8_t buttonIndex) const
    pub fn is_button_down(&self, joystick_index: u8, button_index: u8) -> bool {
        assert!(joystick_index < 2);
        assert!(button_index < 4);
        let mask = 1u8 << (button_index + joystick_index * 4);
        (self.joystick_button_state & mask) == 0 // Bit clear means pressed
    }
}