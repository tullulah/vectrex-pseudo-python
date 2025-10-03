//! Basic types and platform-specific definitions
//! Port of vectrexy/libs/core/include/core/Base.h

// C++ Original:
// using cycles_t = uint64_t;
pub type Cycles = u64;

// C++ Original from Base.h lines 1-50:
// Platform and endianness detection, build configs
// (Rust handles most of this automatically)

#[cfg(debug_assertions)]
pub const CONFIG_DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const CONFIG_DEBUG: bool = false;

// C++ Original: #define ENDIANESS_LITTLE 1
pub const ENDIANNESS_LITTLE: bool = cfg!(target_endian = "little");

// C++ Original: #define BITFIELDS_MSB_TO_LSB 0 (for MSVC)
// Rust bitfields are more explicit, this is less relevant
pub const BITFIELDS_MSB_TO_LSB: bool = false;