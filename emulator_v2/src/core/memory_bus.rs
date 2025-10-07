//! Memory Bus for connecting devices and handling memory-mapped I/O
//! Port of vectrexy/libs/emulator/include/emulator/MemoryBus.h

use crate::types::Cycles;
use std::cell::{Cell, UnsafeCell};
use std::rc::Rc;

// C++ Original: using MemoryRange = std::pair<uint16_t, uint16_t>;
pub type MemoryRange = (u16, u16);

/* C++ Original:
struct IMemoryBusDevice {
    virtual uint8_t Read(uint16_t address) const = 0;
    virtual void Write(uint16_t address, uint8_t value) = 0;
    virtual void Sync(cycles_t cycles) { (void)cycles; }
};
*/
pub trait MemoryBusDevice {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    // C++ Original: virtual void Sync(cycles_t cycles) { (void)cycles; }
    // 1:1 Port: Only takes cycles, devices get context from their stored state (like Via::m_syncContext)
    fn sync(&mut self, _cycles: Cycles) {
        // Default implementation does nothing
    }
}

// C++ Original: enum class EnableSync { False, True };
#[derive(Debug, Clone, Copy)]
pub enum EnableSync {
    False,
    True,
}

// C++ Original: struct DeviceInfo in MemoryBus private section
struct DeviceInfo {
    device: Rc<UnsafeCell<dyn MemoryBusDevice>>,
    memory_range: MemoryRange,
    sync_enabled: bool,
    sync_cycles: Cell<Cycles>,
}

impl DeviceInfo {
    fn new(device: Rc<UnsafeCell<dyn MemoryBusDevice>>, start: u16, end: u16, enable_sync: EnableSync) -> Self {
        Self {
            device,
            memory_range: (start, end),
            sync_enabled: matches!(enable_sync, EnableSync::True),
            sync_cycles: Cell::new(0),
        }
    }
}

pub struct MemoryBus {
    device_infos: Vec<DeviceInfo>,
    on_read_callback: Option<Box<dyn Fn(u16, u8)>>,
    on_write_callback: Option<Box<dyn Fn(u16, u8)>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            device_infos: Vec::new(),
            on_read_callback: None,
            on_write_callback: None,
        }
    }

    pub fn connect_device(
        &mut self,
        device: Rc<UnsafeCell<dyn MemoryBusDevice>>,
        range: MemoryRange,
        enable_sync: EnableSync,
    ) {
        self.device_infos
            .push(DeviceInfo::new(device, range.0, range.1, enable_sync));
        self.device_infos
            .sort_by(|a, b| a.memory_range.0.cmp(&b.memory_range.0));
    }

    pub fn register_callbacks(
        &mut self,
        on_read: Option<Box<dyn Fn(u16, u8)>>,
        on_write: Option<Box<dyn Fn(u16, u8)>>,
    ) {
        self.on_read_callback = on_read;
        self.on_write_callback = on_write;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let device_info = self.find_device_info_mut(address);
        let device_ptr = device_info.device.get();

        let cycles = device_info.sync_cycles.get();
        if cycles > 0 {
            unsafe { (*device_ptr).sync(cycles) };
            device_info.sync_cycles.set(0);
        }

        let value = unsafe { (*device_ptr).read(address) };

        if let Some(callback) = &self.on_read_callback {
            callback(address, value);
        }

        value
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(callback) = &self.on_write_callback {
            callback(address, value);
        }

        let device_info = self.find_device_info_mut(address);
        let device_ptr = device_info.device.get();
        
        let cycles = device_info.sync_cycles.get();
        if cycles > 0 {
            unsafe { (*device_ptr).sync(cycles) };
            device_info.sync_cycles.set(0);
        }

        unsafe { (*device_ptr).write(address, value) };
    }

    pub fn read_raw(&mut self, address: u16) -> u8 {
        let device_info = self.find_device_info_mut(address);
        unsafe { (*device_info.device.get()).read(address) }
    }

    pub fn read16(&mut self, address: u16) -> u16 {
        let high = self.read(address) as u16;
        let low = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn add_sync_cycles(&self, cycles: Cycles) {
        for device_info in &self.device_infos {
            if device_info.sync_enabled {
                let current = device_info.sync_cycles.get();
                device_info.sync_cycles.set(current + cycles);
            }
        }
    }

    pub fn sync(&mut self) {
        for device_info in &mut self.device_infos {
            let cycles = device_info.sync_cycles.get();
            if cycles > 0 {
                unsafe { (*device_info.device.get()).sync(cycles) };
                device_info.sync_cycles.set(0);
            }
        }
    }

    fn find_device_info_mut(&mut self, address: u16) -> &mut DeviceInfo {
        // C++ Original: if (!m_deviceInfos.empty() && address >= m_deviceInfos[0].m_memoryRange.first)
        if !self.device_infos.is_empty() && address >= self.device_infos[0].memory_range.0 {
            for info in &mut self.device_infos {
                // C++ Original: if (address <= it->m_memoryRange.second)
                if address <= info.memory_range.1 {
                    return info;
                }
            }
        }
        panic!("Unmapped address: ${:04X}", address);
    }

    #[cfg(test)]
    pub fn device_count(&self) -> usize {
        self.device_infos.len()
    }

    #[cfg(test)]
    pub fn device_info(&self, index: usize) -> Option<(u16, u16)> {
        self.device_infos.get(index).map(|info| info.memory_range)
    }
}
