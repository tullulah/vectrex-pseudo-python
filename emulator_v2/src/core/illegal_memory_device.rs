// C++ Original: vectrexy/libs/emulator/include/emulator/IllegalMemoryDevice.h
// #pragma once
// #include "MemoryBus.h"
// #include "MemoryMap.h"
// #include "core/ConsoleOutput.h"
// #include "core/ErrorHandler.h"
//
// class IllegalMemoryDevice : public IMemoryBusDevice {
// public:
//     void Init(MemoryBus& memoryBus) {
//         memoryBus.ConnectDevice(*this, MemoryMap::Illegal.range, EnableSync::False);
//     }
//
// private:
//     uint8_t Read(uint16_t address) const override {
//         ErrorHandler::Undefined("Read from illegal range at address $%04x\n", address);
//         return 0;
//     }
//     void Write(uint16_t address, uint8_t value) override {
//         ErrorHandler::Undefined("Write to illegal range of value $%02x at address $%04x\n", value,
//                                 address);
//     }
// };

use crate::core::MemoryBusDevice;

pub struct IllegalMemoryDevice;

impl IllegalMemoryDevice {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryBusDevice for IllegalMemoryDevice {
    // C++ Original: uint8_t Read(uint16_t address) const override
    fn read(&self, address: u16) -> u8 {
        // C++ Original: ErrorHandler::Undefined("Read from illegal range at address $%04x\n", address);
        eprintln!("Read from illegal range at address ${:04X}", address);
        
        // C++ Original: return 0;
        0
    }
    
    // C++ Original: void Write(uint16_t address, uint8_t value) override
    fn write(&mut self, address: u16, value: u8) {
        // C++ Original: ErrorHandler::Undefined("Write to illegal range of value $%02x at address $%04x\n", value, address);
        eprintln!("Write to illegal range of value ${:02X} at address ${:04X}", value, address);
    }
}

impl Default for IllegalMemoryDevice {
    fn default() -> Self {
        Self::new()
    }
}