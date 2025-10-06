// C++ Original: vectrexy/libs/emulator/include/emulator/Cartridge.h + Cartridge.cpp
// #pragma once
// #include "MemoryBus.h"
// #include <vector>
//
// class Cartridge : public IMemoryBusDevice {
// public:
//     void Init(MemoryBus& memoryBus);
//     void Reset() {}
//     bool LoadRom(const char* file);
//
// private:
//     uint8_t Read(uint16_t address) const override;
//     void Write(uint16_t address, uint8_t value) override;
//
// private:
//     std::vector<uint8_t> m_data;
// };

use crate::core::{MemoryBusDevice, MemoryMap};
use std::fs;

pub struct Cartridge {
    // C++ Original: std::vector<uint8_t> m_data;
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // C++ Original: void Reset() {}
    pub fn reset(&mut self) {
        // Empty - same as C++ original
    }

    // C++ Original: bool LoadRom(const char* file)
    pub fn load_rom(&mut self, file: &str) -> bool {
        // C++ Original: if (IsValidRom(file)) { ... }
        if self.is_valid_rom(file) {
            match self.read_stream_until_end(file) {
                Ok(data) => {
                    self.data = data;
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    // C++ Original: ReadStreamUntilEnd helper function
    fn read_stream_until_end(&self, file: &str) -> std::io::Result<Vec<u8>> {
        fs::read(file)
    }

    // C++ Original: IsValidRom helper function (simplified version)
    fn is_valid_rom(&self, file: &str) -> bool {
        // C++ Original: Complex ROM validation with copyright check, music location, title parsing
        // For now, simplified version - just check if file exists and has reasonable size
        match fs::metadata(file) {
            Ok(metadata) => {
                let size = metadata.len() as usize;
                size > 0 && size <= MemoryMap::CARTRIDGE.physical_size
            }
            Err(_) => false,
        }
    }
}

impl MemoryBusDevice for Cartridge {
    // C++ Original: uint8_t Read(uint16_t address) const override
    fn read(&self, address: u16) -> u8 {
        // C++ Original: auto mappedAddress = MemoryMap::Cartridge.MapAddress(address);
        let mapped_address = MemoryMap::CARTRIDGE.map_address(address);

        // C++ Original: if (mappedAddress >= m_data.size()) { ... return 1; }
        if mapped_address >= self.data.len() {
            // C++ Original: ErrorHandler::Undefined("Invalid Cartridge read at $%04x\n", address);
            eprintln!("Invalid Cartridge read at ${:04X}", address);

            // C++ Original: Some roms erroneously access cartridge space when trying to draw vector lists
            // (e.g. Mine Storm, Polar Rescue), so by returning $01 here, we help to hide/fix these bugs.
            1
        } else {
            // C++ Original: return m_data[mappedAddress];
            self.data[mapped_address]
        }
    }

    // C++ Original: void Write(uint16_t /*address*/, uint8_t /*value*/) override
    fn write(&mut self, _address: u16, _value: u8) {
        // Back to &mut self
        // C++ Original: ErrorHandler::Undefined("Writes to Cartridge ROM not allowed\n");
        eprintln!("Writes to Cartridge ROM not allowed");
    }
}

// C++ Original: void Cartridge::Init(MemoryBus& memoryBus) {
//     memoryBus.ConnectDevice(*this, MemoryMap::Cartridge.range, EnableSync::False);
//     m_data.resize(MemoryMap::Cartridge.physicalSize, 0);
// }
impl Cartridge {
    pub fn init_memory_bus(
        self_ref: std::rc::Rc<std::cell::UnsafeCell<Self>>,
        memory_bus: &mut crate::core::memory_bus::MemoryBus,
    ) {
        use crate::core::{memory_bus::EnableSync, memory_map::MemoryMap};
        memory_bus.connect_device(
            self_ref.clone(),
            MemoryMap::CARTRIDGE.range,
            EnableSync::False,
        );

        // C++ Original: m_data.resize(MemoryMap::Cartridge.physicalSize, 0);
        unsafe {
            (*self_ref.get())
                .data
                .resize(MemoryMap::CARTRIDGE.physical_size, 0);
        }
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self::new()
    }
}
