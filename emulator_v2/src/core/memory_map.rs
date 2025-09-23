// C++ Original: vectrexy/libs/emulator/include/emulator/MemoryMap.h

pub struct Mapping {
    pub range: (u16, u16),
    pub physical_size: usize,
    pub logical_size: usize,
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

    pub fn map_address(&self, address: u16) -> usize {
        assert!(
            address >= self.range.0 && address <= self.range.1,
            "Mapping address out of range! Value: ${:04X}, Range: [${:04X}, ${:04X}]",
            address, self.range.0, self.range.1
        );
        ((address - self.range.0) % self.logical_size as u16) as usize
    }
}

pub struct MemoryMap;

impl MemoryMap {
    pub const CARTRIDGE: Mapping = Mapping::new(0x0000, 0xBFFF, 1);
    pub const UNMAPPED: Mapping = Mapping::new(0xC000, 0xC7FF, 1);
    pub const RAM: Mapping = Mapping::new(0xC800, 0xCFFF, 2);
    pub const VIA: Mapping = Mapping::new(0xD000, 0xD7FF, 128);
    pub const ILLEGAL: Mapping = Mapping::new(0xD800, 0xDFFF, 1);
    pub const BIOS: Mapping = Mapping::new(0xE000, 0xFFFF, 1);
}