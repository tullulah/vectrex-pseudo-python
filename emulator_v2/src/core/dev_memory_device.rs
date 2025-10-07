// C++ Original: vectrexy/libs/emulator/include/emulator/DevMemoryDevice.h + DevMemoryDevice.cpp
// Replaces the UnmappedMemoryDevice, exposing new memory-mapped registers useful for Vectrex game
// development purposes.

use crate::core::MemoryBusDevice;
use std::cell::UnsafeCell;
use std::rc::Weak;

// C++ Original: Printf-registers constants - made public for testing
pub const DEV_PRINTF_PUSH_ARG8: u16 = 0xC100;
pub const DEV_PRINTF_PUSH_ARG16: [u16; 2] = [0xC101, 0xC102];
pub const DEV_PRINTF_PUSH_CSTR: [u16; 2] = [0xC103, 0xC104];
pub const DEV_PRINTF_FORMAT: [u16; 2] = [0xC105, 0xC106];

pub struct DevMemoryDevice {
    // C++ Original: MemoryBus* m_memoryBus{};
    // NOTA: memory_bus no es usable con UnsafeCell ya que necesita Weak<>
    // Por ahora dejarlo como None (printf no disponible)
    memory_bus: Option<Weak<UnsafeCell<crate::core::memory_bus::MemoryBus>>>,

    // C++ Original: uint8_t m_opFirstByte{};
    op_first_byte: u8,
}

impl DevMemoryDevice {
    pub fn new() -> Self {
        Self {
            memory_bus: None,
            op_first_byte: 0,
        }
    }

    // Helper method to read null-terminated string from memory
    fn read_string_from_memory(
        &self,
        memory_bus: &mut crate::core::memory_bus::MemoryBus,
        mut address: u16,
    ) -> String {
        let mut result = String::new();
        let mut max_len = 256; // Prevent infinite loops

        loop {
            if max_len == 0 {
                result.push_str("...[truncated]");
                break;
            }

            let byte = memory_bus.read(address);
            if byte == 0 {
                break; // Null terminator
            }

            // Only add printable ASCII characters
            if byte >= 0x20 && byte < 0x7F {
                result.push(byte as char);
            } else if byte == 0x0A {
                result.push('\n');
            } else if byte == 0x0D {
                result.push('\r');
            } else {
                result.push_str(&format!("\\x{:02X}", byte));
            }

            address = address.wrapping_add(1);
            max_len -= 1;
        }

        result
    }
}

impl MemoryBusDevice for DevMemoryDevice {
    // C++ Original: uint8_t Read(uint16_t address) const override
    fn read(&mut self, address: u16) -> u8 {
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
        eprintln!(
            "Write to unmapped range of value ${:02X} at address ${:04X}",
            value, address
        );
    }
}

impl DevMemoryDevice {
    // C++ Original: bool HandleDevWrite(uint16_t address, uint8_t value)
    fn handle_dev_write(&mut self, address: u16, value: u8) -> bool {
        // C++ Original: Printf-registers handling
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

                // Try to read format string from memory bus if available
                if let Some(weak_bus) = &self.memory_bus {
                    if let Some(bus_rc) = weak_bus.upgrade() {
                        let bus_ptr = bus_rc.get();
                        let format_string = unsafe {
                            self.read_string_from_memory(&mut *bus_ptr, format_address)
                        };
                        println!("DEV: Printf: {}", format_string);
                    } else {
                        println!(
                            "DEV: Printf format at: ${:04X} (memory bus dropped)",
                            format_address
                        );
                    }
                } else {
                    println!(
                        "DEV: Printf format at: ${:04X} (memory bus not connected)",
                        format_address
                    );
                }
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
    pub fn init_memory_bus(
        self_ref: std::rc::Rc<std::cell::UnsafeCell<Self>>,
        memory_bus: &mut crate::core::memory_bus::MemoryBus,
        memory_bus_weak: std::rc::Weak<std::cell::UnsafeCell<crate::core::memory_bus::MemoryBus>>,
    ) {
        use crate::core::{memory_bus::EnableSync, memory_map::MemoryMap};

        // Store weak reference to memory bus for printf functionality
        unsafe {
            (*self_ref.get()).memory_bus = Some(memory_bus_weak);
        }

        // C++ Original: memoryBus.ConnectDevice(*this, MemoryMap::Unmapped.range, EnableSync::False);
        memory_bus.connect_device(self_ref, MemoryMap::UNMAPPED.range, EnableSync::False);
    }
}

impl Default for DevMemoryDevice {
    fn default() -> Self {
        Self::new()
    }
}
