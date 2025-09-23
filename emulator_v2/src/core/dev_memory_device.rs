// C++ Original: vectrexy/libs/emulator/include/emulator/DevMemoryDevice.h + DevMemoryDevice.cpp
// Replaces the UnmappedMemoryDevice, exposing new memory-mapped registers useful for Vectrex game
// development purposes.

use crate::core::MemoryBusDevice;

pub struct DevMemoryDevice {
    // C++ Original: MemoryBus* m_memoryBus{};
    // memory_bus: Option<*mut MemoryBus>, // TODO: Add when full printf functionality needed
    
    // C++ Original: uint8_t m_opFirstByte{};
    op_first_byte: u8,
}

impl DevMemoryDevice {
    pub fn new() -> Self {
        Self {
            op_first_byte: 0,
        }
    }
}

impl MemoryBusDevice for DevMemoryDevice {
    // C++ Original: uint8_t Read(uint16_t address) const override
    fn read(&self, address: u16) -> u8 {
        // C++ Original: ErrorHandler::Undefined("Read from unmapped range at address $%04x\n", address);
        eprintln!("Read from unmapped range at address ${:04X}", address);
        
        // C++ Original: return 0;
        0
    }
    
    // C++ Original: void Write(uint16_t address, uint8_t value) override
    fn write(&mut self, address: u16, value: u8) {
        // C++ Original: if (HandleDevWrite(address, value)) return;
        if self.handle_dev_write(address, value) {
            return;
        }
        
        // C++ Original: ErrorHandler::Undefined("Write to unmappped range of value $%02x at address $%04x\n", value, address);
        eprintln!("Write to unmapped range of value ${:02X} at address ${:04X}", value, address);
    }
}

impl DevMemoryDevice {
    // C++ Original: bool HandleDevWrite(uint16_t address, uint8_t value)
    fn handle_dev_write(&mut self, address: u16, value: u8) -> bool {
        // C++ Original: Printf-registers constants
        const DEV_PRINTF_PUSH_ARG8: u16 = 0xC100;
        const DEV_PRINTF_PUSH_ARG16: [u16; 2] = [0xC101, 0xC102];
        const DEV_PRINTF_PUSH_CSTR: [u16; 2] = [0xC103, 0xC104];
        const DEV_PRINTF_FORMAT: [u16; 2] = [0xC105, 0xC106];
        
        match address {
            DEV_PRINTF_PUSH_ARG8 => {
                // C++ Original: Printf arg8 handling - simplified for now
                println!("DEV: Push ARG8: ${:02X}", value);
                true
            }
            
            addr if addr == DEV_PRINTF_PUSH_ARG16[0] => {
                // C++ Original: m_opFirstByte = value;
                self.op_first_byte = value;
                true
            }
            
            addr if addr == DEV_PRINTF_PUSH_ARG16[1] => {
                // C++ Original: 16-bit value construction and storage
                let v = (self.op_first_byte as u16) << 8 | (value as u16);
                println!("DEV: Push ARG16: ${:04X}", v);
                true
            }
            
            addr if addr == DEV_PRINTF_PUSH_CSTR[0] => {
                // C++ Original: m_opFirstByte = value;
                self.op_first_byte = value;
                true
            }
            
            addr if addr == DEV_PRINTF_PUSH_CSTR[1] => {
                // C++ Original: String address construction
                let string_address = (self.op_first_byte as u16) << 8 | (value as u16);
                println!("DEV: Push CSTR address: ${:04X}", string_address);
                true
            }
            
            addr if addr == DEV_PRINTF_FORMAT[0] => {
                // C++ Original: m_opFirstByte = value;
                self.op_first_byte = value;
                true
            }
            
            addr if addr == DEV_PRINTF_FORMAT[1] => {
                // C++ Original: Format string handling and printf execution
                let format_address = (self.op_first_byte as u16) << 8 | (value as u16);
                println!("DEV: Printf format at: ${:04X}", format_address);
                // TODO: Implement full printf functionality when needed
                true
            }
            
            _ => false,
        }
    }
}

// C++ Original: void DevMemoryDevice::Init(MemoryBus& memoryBus) {
//     m_memoryBus = &memoryBus;
//     memoryBus.ConnectDevice(*this, MemoryMap::Unmapped.range, EnableSync::False);
// }
impl DevMemoryDevice {
    pub fn init_memory_bus(self_ref: std::rc::Rc<std::cell::RefCell<Self>>, memory_bus: &mut crate::core::memory_bus::MemoryBus) {
        use crate::core::{memory_map::MemoryMap, memory_bus::EnableSync};
        // C++ Original: m_memoryBus = &memoryBus; - TODO: Add when full printf functionality needed
        memory_bus.connect_device(self_ref, MemoryMap::UNMAPPED.range, EnableSync::False);
    }
}

impl Default for DevMemoryDevice {
    fn default() -> Self {
        Self::new()
    }
}