// C++ Original:
// namespace MemoryMap {
//     template <typename T1, typename T2, typename T3>
//     constexpr inline bool IsInRange(T1 value, const std::pair<T2, T3>& range) {
//         return value >= range.first && value <= range.second;
//     }
//     struct Mapping {
//         std::pair<uint16_t, uint16_t> range;
//         const size_t physicalSize;
//         const size_t logicalSize;
//         constexpr Mapping(uint16_t first, uint16_t last, size_t shadowDivisor = 1)
//             : range{first, last}
//             , physicalSize{last - first + 1u}
//             , logicalSize{(last - first + 1u) / shadowDivisor} {}
//         uint16_t MapAddress(uint16_t address) const {
//             ASSERT_MSG(IsInRange(address, range),
//                        "Mapping address out of range! Value: $%04x, Range: [$%04x, $%04x]", address,
//                        range.first, range.second);
//             return (address - range.first) % logicalSize;
//         }
//     };

pub struct Mapping {
    pub range: (u16, u16), // (first_address, last_address)
    pub physical_size: usize, // size in bytes of address range, including shadowed
    pub logical_size: usize,  // size in bytes of unshadowed address range
}

impl Mapping {
    pub const fn new(first: u16, last: u16, shadow_divisor: usize) -> Self {
        let physical_size = (last - first + 1) as usize;
        let logical_size = physical_size / shadow_divisor;
        Self {
            range: (first, last),
            physical_size,
            logical_size,
        }
    }

    pub fn is_in_range(&self, address: u16) -> bool {
        address >= self.range.0 && address <= self.range.1
    }

    pub fn map_address(&self, address: u16) -> u16 {
        // C++ Original: return (address - range.first) % logicalSize;
        assert!(
            self.is_in_range(address),
            "Mapping address out of range! Value: ${:04X}, Range: [${:04X}, ${:04X}]",
            address,
            self.range.0,
            self.range.1
        );
        ((address - self.range.0) as usize % self.logical_size) as u16
    }
}

// C++ Original: constexpr auto Cartridge = Mapping(0x0000, 0xBFFF);
pub const CARTRIDGE: Mapping = Mapping::new(0x0000, 0xBFFF, 1);
// static_assert(Cartridge.physicalSize == 32768 + 16384, "");
const _: () = assert!(CARTRIDGE.physical_size == 32768 + 16384);

// C++ Original: constexpr auto Unmapped = Mapping(0xC000, 0xC7FF);
pub const UNMAPPED: Mapping = Mapping::new(0xC000, 0xC7FF, 1);
// static_assert(Unmapped.physicalSize == 2048);
const _: () = assert!(UNMAPPED.physical_size == 2048);

// C++ Original: constexpr auto Ram = Mapping(0xC800, 0xCFFF, 2);
// RAM 1 KB shadowed twice
pub const RAM: Mapping = Mapping::new(0xC800, 0xCFFF, 2);
// static_assert(Ram.physicalSize == 2048, "");
const _: () = assert!(RAM.physical_size == 2048);

// C++ Original: constexpr auto Via = Mapping(0xD000, 0xD7FF, 128);
// 6522 VIA 16 bytes shadowed 128 times
pub const VIA: Mapping = Mapping::new(0xD000, 0xD7FF, 128);
// static_assert(Via.physicalSize == 2048, "");
const _: () = assert!(VIA.physical_size == 2048);

// C++ Original: constexpr auto Illegal = Mapping(0xD800, 0xDFFF);
// Both VIA + RAM selected
pub const ILLEGAL: Mapping = Mapping::new(0xD800, 0xDFFF, 1);
// static_assert(Illegal.physicalSize == 2048, "");
const _: () = assert!(ILLEGAL.physical_size == 2048);

// C++ Original: constexpr auto Bios = Mapping(0xE000, 0xFFFF);
// Mine Storm (first half: 0xE000-0xEFFF) + BIOS (second half: 0xF000-0xFFFF)
pub const BIOS: Mapping = Mapping::new(0xE000, 0xFFFF, 1);
// static_assert(Bios.physicalSize == 8192, "");
const _: () = assert!(BIOS.physical_size == 8192);

pub fn is_in_range<T>(value: T, range: (T, T)) -> bool
where
    T: PartialOrd,
{
    // C++ Original: constexpr inline bool IsInRange(T1 value, const std::pair<T2, T3>& range) {
    //     return value >= range.first && value <= range.second;
    // }
    value >= range.0 && value <= range.1
}