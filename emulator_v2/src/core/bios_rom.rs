// C++ Original:
// class BiosRom {
//   static inline constexpr size_t k_sizeBytes = 8192;
//   std::array<uint8_t, k_sizeBytes> m_memory;
//   public:
//     BiosRom();
//     uint8_t Read(uint16_t address) const;
//     void Write(uint16_t address, uint8_t value);
//     bool LoadBiosRom(const uint8_t *data, size_t size); };

use crate::types::Cycles;
use std::fs;

pub struct BiosRom {
    memory: [u8; Self::SIZE_BYTES],
}

impl BiosRom {
    pub const SIZE_BYTES: usize = 8192;

    pub fn new() -> Self {
        Self {
            memory: [0; Self::SIZE_BYTES],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        // C++ Original: uint8_t BiosRom::Read(uint16_t address) const {
        //     return m_memory.at(MemoryMap::Bios.MapAddress(address)); }
        let mapped_addr = self.map_address(address);
        self.memory[mapped_addr as usize]
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        // C++ Original: void BiosRom::Write(uint16_t, uint8_t) {
        //     // ROM is read-only
        // }
        // ROM es de solo lectura, no hacer nada
    }

    // C++ Original: bool BiosRom::LoadBiosRom(const uint8_t *data, size_t size)
    // Vectrexy expects EXACTLY 8KB - no duplication, no size flexibility
    pub fn load_bios_rom(&mut self, data: &[u8]) -> bool {
        // C++ Original: fs.Read(&m_data[0], m_data.size());
        // Handle both 4KB and 8KB BIOS data. If 4KB, duplicate it to fill 8KB.
        if data.len() == 4096 {
            // Duplicate 4KB data into the 8KB buffer
            self.memory[..4096].copy_from_slice(data);
            self.memory[4096..].copy_from_slice(data);
            true
        } else if data.len() == Self::SIZE_BYTES {
            // Copy full 8KB directly
            self.memory.copy_from_slice(data);
            true
        } else {
            eprintln!(
                "BIOS size mismatch: expected 4096 or {} bytes, got {}",
                Self::SIZE_BYTES,
                data.len()
            );
            false
        }
    }

    pub fn load_bios_rom_from_file(
        &mut self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        if !self.load_bios_rom(&data) {
            return Err(format!(
                "BIOS ROM size mismatch: expected {} bytes, got {}",
                Self::SIZE_BYTES,
                data.len()
            )
            .into());
        }
        Ok(())
    }

    // C++ Original: return (address - range.first) % logicalSize;
    // Para BIOS: range = [0xE000, 0xFFFF], logicalSize = 8192
    fn map_address(&self, address: u16) -> u16 {
        const BIOS_BASE: u16 = 0xE000;
        (address - BIOS_BASE) % (Self::SIZE_BYTES as u16)
    }

    // C++ Original: void BiosRom::Init(MemoryBus& memoryBus) {
    //     memoryBus.ConnectDevice(*this, MemoryMap::Bios.range, EnableSync::False);
    // }
    pub fn init_memory_bus(
        self_ref: std::rc::Rc<std::cell::UnsafeCell<Self>>,
        memory_bus: &mut crate::core::memory_bus::MemoryBus,
    ) {
        use crate::core::{memory_bus::EnableSync, memory_map::MemoryMap};
        memory_bus.connect_device(self_ref, MemoryMap::BIOS.range, EnableSync::False);
    }
}

impl Default for BiosRom {
    fn default() -> Self {
        Self::new()
    }
}

use crate::core::memory_bus::MemoryBusDevice;

impl MemoryBusDevice for BiosRom {
    fn read(&mut self, address: u16) -> u8 {
        let mapped_addr = self.map_address(address);
        self.memory[mapped_addr as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }
}
