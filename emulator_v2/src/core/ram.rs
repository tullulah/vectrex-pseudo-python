// C++ Original:
// #pragma once
// #include <array>
// class Ram { 
//   static inline constexpr size_t k_sizeBytes = 1024;
//   std::array<uint8_t, k_sizeBytes> m_memory;
//   public:
//     Ram();
//     uint8_t Read(uint16_t address) const;
//     void Write(uint16_t address, uint8_t value);
//     void Zero();
//     void Randomize();
//     void Init(); };

use crate::types::Cycles;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::memory_bus::MemoryBus;

pub struct Ram {
    memory: [u8; Self::SIZE_BYTES],
}

impl Ram {
    pub const SIZE_BYTES: usize = 1024;

    pub fn new() -> Self {
        let mut ram = Self {
            memory: [0; Self::SIZE_BYTES],
        };
        // C++ Original: void Ram::Init() { Randomize(); }
        // Note: In C++ this would use a default/random seed, we use 0 for deterministic behavior
        ram.randomize(0);
        ram
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr = (address as usize) % Self::SIZE_BYTES;
        self.memory[addr]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let addr = (address as usize) % Self::SIZE_BYTES;
        self.memory[addr] = value;
    }

    pub fn zero(&mut self) {
        self.memory.fill(0);
    }

    // C++ Original: void Randomize(unsigned int seed)
    pub fn randomize(&mut self, seed: u32) {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        for i in 0..Self::SIZE_BYTES {
            self.memory[i] = rng.gen_range(0..=255);
        }
    }

    pub fn init(&mut self) {
        // C++ Original: void Ram::Init() { Randomize(); }
        // Note: In C++ this would use a default/random seed, we use 0 for deterministic behavior
        self.randomize(0);
    }

    // C++ Original: void Init(MemoryBus& memoryBus) {
    //     memoryBus.ConnectDevice(*this, MemoryMap::Ram.range, EnableSync::False);
    // }
    pub fn init_memory_bus(self_ref: Rc<RefCell<Self>>, memory_bus: &mut MemoryBus) {
        use crate::core::{memory_map::MemoryMap, memory_bus::EnableSync};
        memory_bus.connect_device(self_ref, MemoryMap::RAM.range, EnableSync::False);
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

use crate::core::memory_bus::MemoryBusDevice;

impl MemoryBusDevice for Ram {
    fn read(&self, address: u16) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }

    fn sync(&mut self, _cycles: Cycles) {
        // RAM no necesita sincronización por ciclos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_basic_operations() {
        let mut ram = Ram::new();
        
        // Test write and read
        ram.write(0x00, 0xAA);
        assert_eq!(ram.read(0x00), 0xAA);
        
        // Test address wrapping (RAM is 1024 bytes)
        ram.write(0x400, 0xBB); // Should wrap to 0x00
        assert_eq!(ram.read(0x00), 0xBB);
        
        // Test zero
        ram.zero();
        assert_eq!(ram.read(0x00), 0x00);
        for i in 0..Ram::SIZE_BYTES {
            assert_eq!(ram.read(i as u16), 0x00);
        }
    }

    #[test]
    fn test_ram_randomize() {
        let mut ram = Ram::new();
        ram.zero();
        
        // After randomize, should have some non-zero values
        ram.randomize(42); // Use a seed for deterministic test
        let mut has_non_zero = false;
        for i in 0..Ram::SIZE_BYTES {
            if ram.read(i as u16) != 0 {
                has_non_zero = true;
                break;
            }
        }
        assert!(has_non_zero, "RAM should have non-zero values after randomize");
    }

    #[test] 
    fn test_ram_memory_bus_device() {
        let mut ram = Ram::new();
        
        // Test MemoryBusDevice trait implementation
        MemoryBusDevice::write(&mut ram, 0x123, 0x45);
        assert_eq!(MemoryBusDevice::read(&ram, 0x123), 0x45);
        
        // Test sync (should do nothing for RAM)
        MemoryBusDevice::sync(&mut ram, 100u64); // Cycles is u64
    }
}