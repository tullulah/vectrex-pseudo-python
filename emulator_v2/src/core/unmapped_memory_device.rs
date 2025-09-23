// C++ Original: vectrexy/libs/emulator/include/emulator/UnmappedMemoryDevice.h + UnmappedMemoryDevice.cpp
// #pragma once
// #include "MemoryBus.h"
//
// class UnmappedMemoryDevice : public IMemoryBusDevice {
// public:
//     void Init(MemoryBus& memoryBus);
//
// private:
//     uint8_t Read(uint16_t address) const override;
//     void Write(uint16_t address, uint8_t value) override;
// };

use crate::core::MemoryBusDevice;

pub struct UnmappedMemoryDevice;

impl UnmappedMemoryDevice {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryBusDevice for UnmappedMemoryDevice {
    // C++ Original: uint8_t Read(uint16_t address) const override
    fn read(&self, address: u16) -> u8 {
        // C++ Original: ErrorHandler::Undefined("Read from unmapped range at address $%04x\n", address);
        eprintln!("Read from unmapped range at address ${:04X}", address);
        
        // C++ Original: return 0;
        0
    }
    
    // C++ Original: void Write(uint16_t address, uint8_t value) override
    fn write(&mut self, address: u16, value: u8) {
        // C++ Original: ErrorHandler::Undefined("Write to unmappped range of value $%02x at address $%04x\n", value, address);
        eprintln!("Write to unmapped range of value ${:02X} at address ${:04X}", value, address);
    }
}

impl Default for UnmappedMemoryDevice {
    fn default() -> Self {
        Self::new()
    }
}