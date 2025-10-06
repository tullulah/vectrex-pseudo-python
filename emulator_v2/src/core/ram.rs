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

use crate::core::memory_bus::MemoryBus;
use crate::core::engine_types::RenderContext;
use crate::types::Cycles;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Ram {
    memory: [u8; Self::SIZE_BYTES], // Back to simple array
}

impl Ram {
    // C++ Original: static inline constexpr size_t k_sizeBytes = 1024;
    // VECTREXY FIX: Vectrex has 1KB internal RAM but the system supports up to 32KB
    // The memory map from 0xC800-0xFFFF is 14KB, so we use 32KB to cover all ranges safely
    pub const SIZE_BYTES: usize = 32768; // 32KB (0x8000 bytes)

    pub fn new() -> Self {
        let mut ram = Self {
            memory: [0; Self::SIZE_BYTES],
        };
        ram.randomize(0);
        ram
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr = (address as usize) % Self::SIZE_BYTES;
        self.memory[addr]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // Back to &mut self
        let addr = (address as usize) % Self::SIZE_BYTES;
        self.memory[addr] = value;
    }

    pub fn zero(&mut self) {
        // Back to &mut self
        self.memory.fill(0);
    }

    pub fn randomize(&mut self, seed: u32) {
        // Back to &mut self
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
    pub fn init_memory_bus(self_ref: Rc<UnsafeCell<Self>>, memory_bus: &mut MemoryBus) {
        use crate::core::{memory_bus::EnableSync, memory_map::MemoryMap};
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
        // Back to &mut self
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

        // Test address wrapping (RAM is 32KB = 0x8000 bytes)
        ram.write(0x8000, 0xBB); // Should wrap to 0x00
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
        assert!(
            has_non_zero,
            "RAM should have non-zero values after randomize"
        );
    }

    #[test]
    fn test_ram_memory_bus_device() {
        let mut ram = Ram::new();
        let mut render_context = RenderContext::new();

        // Test MemoryBusDevice trait implementation
        MemoryBusDevice::write(&mut ram, 0x123, 0x45);
        assert_eq!(MemoryBusDevice::read(&ram, 0x123), 0x45);

        // Test sync (should do nothing for RAM)
        MemoryBusDevice::sync(&mut ram, 100u64, &mut render_context);
    }
}
