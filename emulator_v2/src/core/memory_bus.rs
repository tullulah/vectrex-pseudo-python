//! Memory Bus for connecting devices and handling memory-mapped I/O
//! Port of vectrexy/libs/emulator/include/emulator/MemoryBus.h

use crate::types::Cycles;
use std::cell::{Cell, UnsafeCell}; // UnsafeCell para evitar RefCell panic
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
    fn read(&self, address: u16) -> u8;
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
    // C++ Original: IMemoryBusDevice* device = nullptr;
    // UNSAFE FIX: Use UnsafeCell to avoid RefCell borrow panic
    device: Rc<UnsafeCell<dyn MemoryBusDevice>>,

    // C++ Original: MemoryRange memoryRange;
    memory_range: MemoryRange,

    // C++ Original: bool syncEnabled = false;
    sync_enabled: bool,

    // C++ Original: mutable cycles_t syncCycles = 0;
    sync_cycles: Cell<Cycles>,
}

/* C++ Original:
class MemoryBus {
public:
    void ConnectDevice(IMemoryBusDevice& device, MemoryRange range, EnableSync enableSync);
    uint8_t Read(uint16_t address) const;
    void Write(uint16_t address, uint8_t value);
    uint8_t ReadRaw(uint16_t address) const;
    uint16_t Read16(uint16_t address) const;
    void AddSyncCycles(cycles_t cycles);
    void Sync();
    // ... callbacks and private methods
};
*/
pub struct MemoryBus {
    // C++ Original: std::vector<DeviceInfo> m_devices;
    device_infos: Vec<DeviceInfo>,

    // C++ Original: OnReadCallback m_onReadCallback; OnWriteCallback m_onWriteCallback;
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

    /* C++ Original:
    void ConnectDevice(IMemoryBusDevice& device, MemoryRange range, EnableSync enableSync) {
        m_devices.push_back(DeviceInfo{&device, range, enableSync == EnableSync::True});

        std::sort(m_devices.begin(), m_devices.end(),
                  [](const DeviceInfo& info1, const DeviceInfo& info2) {
                      return info1.memoryRange.first < info2.memoryRange.first;
                  });
    }
    */
    pub fn connect_device(
        &mut self,
        device: Rc<UnsafeCell<dyn MemoryBusDevice>>,
        range: MemoryRange,
        enable_sync: EnableSync,
    ) {
        self.device_infos.push(DeviceInfo {
            device,
            memory_range: range,
            sync_enabled: matches!(enable_sync, EnableSync::True),
            sync_cycles: Cell::new(0),
        });

        // Sort by first address in range (equivalent to C++ sort)
        self.device_infos
            .sort_by(|a, b| a.memory_range.0.cmp(&b.memory_range.0));
    }

    /* C++ Original:
    using OnReadCallback = std::function<void(uint16_t, uint8_t)>;
    using OnWriteCallback = std::function<void(uint16_t, uint8_t)>;
    void RegisterCallbacks(OnReadCallback onReadCallback, OnWriteCallback onWriteCallback);
    */
    pub fn register_callbacks(
        &mut self,
        on_read: Option<Box<dyn Fn(u16, u8)>>,
        on_write: Option<Box<dyn Fn(u16, u8)>>,
    ) {
        self.on_read_callback = on_read;
        self.on_write_callback = on_write;
    }

    /* C++ Original:
    uint8_t Read(uint16_t address) const {
        auto& deviceInfo = FindDeviceInfo(address);
        SyncDevice(deviceInfo);

        uint8_t value = deviceInfo.device->Read(address);

        if (m_onReadCallback)
            m_onReadCallback(address, value);

        return value;
    }
    */
    pub fn read(&self, address: u16) -> u8 {
        let device_info = self.find_device_info(address);

        // C++ Original: SyncDevice(deviceInfo) is called BEFORE reading in Vectrexy
        // 1:1 Port: Always sync before reading (devices get context from stored state)
        self.sync_device_for_read(device_info);

        // UNSAFE FIX: Use UnsafeCell to get mutable access without RefCell
        // SAFETY: We control all access through MemoryBus, no concurrent mutation
        let value = unsafe { (*device_info.device.get()).read(address) };

        if let Some(callback) = &self.on_read_callback {
            callback(address, value);
        }

        value
    }

    /* C++ Original:
    void Write(uint16_t address, uint8_t value) {
        if (m_onWriteCallback)
            m_onWriteCallback(address, value);

        auto& deviceInfo = FindDeviceInfo(address);
        SyncDevice(deviceInfo);

        deviceInfo.device->Write(address, value);
    }
    */
    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(callback) = &self.on_write_callback {
            callback(address, value);
        }

        let device_info = self.find_device_info(address);

        // UNSAFE FIX: Use UnsafeCell to get mutable access without RefCell panic
        // SAFETY: We control all access through MemoryBus, no concurrent mutation possible
        unsafe {
            (*device_info.device.get()).write(address, value);
        }
    }

    /* C++ Original:
    uint8_t ReadRaw(uint16_t address) const {
        auto& deviceInfo = FindDeviceInfo(address);
        return deviceInfo.device->Read(address);
    }
    */
    pub fn read_raw(&self, address: u16) -> u8 {
        let device_info = self.find_device_info(address);
        // UNSAFE FIX: Use UnsafeCell
        unsafe { (*device_info.device.get()).read(address) }
    }

    /* C++ Original:
    uint16_t Read16(uint16_t address) const {
        // Big endian
        auto high = Read(address++);
        auto low = Read(address);
        return static_cast<uint16_t>(high) << 8 | static_cast<uint16_t>(low);
    }
    */
    pub fn read16(&self, address: u16) -> u16 {
        // Big endian
        let high = self.read(address) as u16;
        let low = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    /* C++ Original:
    void AddSyncCycles(cycles_t cycles) {
        for (auto& deviceInfo : m_devices) {
            if (deviceInfo.syncEnabled)
                deviceInfo.syncCycles += cycles;
        }
    }
    */
    pub fn add_sync_cycles(&self, cycles: Cycles) {
        for device_info in &self.device_infos {
            if device_info.sync_enabled {
                let current = device_info.sync_cycles.get();
                device_info.sync_cycles.set(current + cycles);
            }
        }
    }

    /* C++ Original:
    void Sync() {
        for (auto& deviceInfo : m_devices) {
            SyncDevice(deviceInfo);
        }
    }
    */
    // 1:1 Port: No render_context parameter - devices get context from their stored state
    pub fn sync(&mut self) {
        // UNSAFE FIX: Use UnsafeCell for device access
        for device_info in &self.device_infos {
            let cycles = device_info.sync_cycles.get();
            if cycles > 0 {
                unsafe {
                    (*device_info.device.get()).sync(cycles);
                }
                device_info.sync_cycles.set(0);
            }
        }
    }

    // Private methods

    /* C++ Original:
    const DeviceInfo& FindDeviceInfo(uint16_t address) const {
        if (address >= m_devices[0].memoryRange.first) {
            for (const auto& info : m_devices) {
                if (address <= info.memoryRange.second) {
                    return info;
                }
            }
        }
        // ... error handling
    }
    */
    fn find_device_info(&self, address: u16) -> &DeviceInfo {
        if !self.device_infos.is_empty() && address >= self.device_infos[0].memory_range.0 {
            for info in &self.device_infos {
                if address <= info.memory_range.1 {
                    return info;
                }
            }
        }

        // C++ uses ErrorHandler::Undefined, we'll panic for now
        panic!("Unmapped address: ${:04X}", address);
    }

    /* C++ Original:
    void SyncDevice(const DeviceInfo& deviceInfo) const {
        if (deviceInfo.syncCycles > 0) {
            deviceInfo.device->Sync(deviceInfo.syncCycles);
            deviceInfo.syncCycles = 0;
        }
    }
    */
    // C++ Original: SyncDevice called during Read() to sync cycles before reading device
    // 1:1 Port: Devices get context from their stored state (Via::m_syncContext), not passed as parameter
    fn sync_device_for_read(&self, device_info: &DeviceInfo) {
        let cycles = device_info.sync_cycles.get();
        if cycles > 0 {
            // UNSAFE FIX: Use UnsafeCell
            unsafe {
                (*device_info.device.get()).sync(cycles);
            }
            // C++ Original: deviceInfo.syncCycles = 0; (clears after sync)
            device_info.sync_cycles.set(0);
        }
    }

    // Public methods for testing
    #[cfg(test)]
    pub fn device_count(&self) -> usize {
        self.device_infos.len()
    }

    #[cfg(test)]
    pub fn device_info(&self, index: usize) -> Option<(u16, u16)> {
        self.device_infos.get(index).map(|info| info.memory_range)
    }
}
