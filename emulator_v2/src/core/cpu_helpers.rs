//! CPU helper functions
//! Port of vectrexy/libs/emulator/include/emulator/CpuHelpers.h

// C++ Original: This header mainly contains functions needed by both Cpu and Debugger

// C++ Original: Convenience cast functions
// template <typename T> constexpr int16_t S16(T v)
pub fn s16(v: i16) -> i16 {
    v
}

// Helper for sign extension from u8 to i16
pub fn s16_from_u8(v: u8) -> i16 {
    v as i8 as i16
}

// C++ Original: template <typename T> constexpr uint16_t U16(T v)  
pub fn u16(v: u16) -> u16 {
    v
}

// C++ Original: template <typename T> constexpr uint32_t U32(T v)
pub fn u32(v: u32) -> u32 {
    v
}

// C++ Original: template <typename T> constexpr uint8_t U8(T v)
pub fn u8(v: u16) -> u8 {
    v as u8
}

// C++ Original: Combine two 8-bit values into a 16-bit value
// constexpr uint16_t CombineToU16(uint8_t msb, uint8_t lsb)
pub fn combine_to_u16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

// C++ Original: constexpr int16_t CombineToS16(uint8_t msb, uint8_t lsb)
pub fn combine_to_s16(msb: u8, lsb: u8) -> i16 {
    combine_to_u16(msb, lsb) as i16
}