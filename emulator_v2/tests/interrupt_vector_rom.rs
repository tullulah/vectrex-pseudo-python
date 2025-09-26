// C++ Original: ROM-like device para mapear vectores de interrupción en tests
// Basado en el patrón de Vectrexy para manejar interrupt vectors

use vectrex_emulator_v2::core::memory_bus::MemoryBusDevice;
use vectrex_emulator_v2::types::Cycles;

pub struct InterruptVectorRom {
    data: [u8; 16], // 0xFFF0-0xFFFF (16 bytes)
}

impl InterruptVectorRom {
    pub fn new() -> Self {
        let mut rom = Self {
            data: [0; 16],
        };
        
        // C++ Original: Default interrupt vectors pointing to test addresses
        // FIRQ vector (0xFFF6): Default to 0xE020 (offset 0x06 from 0xFFF0)
        rom.write_vector_at_offset(0x06, 0xE020);
        
        // IRQ vector (0xFFF8): Default to 0xE030 (offset 0x08 from 0xFFF0)
        rom.write_vector_at_offset(0x08, 0xE030);
        
        // SWI vector (0xFFFA): Default to 0xE040 (offset 0x0A from 0xFFF0)
        rom.write_vector_at_offset(0x0A, 0xE040);
        
        // NMI vector (0xFFFC): Default to 0xE050 (offset 0x0C from 0xFFF0)
        rom.write_vector_at_offset(0x0C, 0xE050);
        
        // RESET vector (0xFFFE): Default to 0xE060 (offset 0x0E from 0xFFF0)
        rom.write_vector_at_offset(0x0E, 0xE060);
        
        // SWI2 vector (0xFFF2): Default to 0xE010 (offset 0x02 from 0xFFF0)
        rom.write_vector_at_offset(0x02, 0xE010);
        
        // SWI3 vector (0xFFF4): Default to 0xE000 (offset 0x04 from 0xFFF0)
        rom.write_vector_at_offset(0x04, 0xE000);
        
        rom
    }
    
    pub fn set_swi_vector(&mut self, address: u16) {
        // SWI vector at 0xFFFA (offset 0x0A from 0xFFF0)
        self.write_vector_at_offset(0x0A, address);
    }
    
    pub fn set_swi2_vector(&mut self, address: u16) {
        // SWI2 vector at 0xFFF2 (offset 0x02 from 0xFFF0)
        self.write_vector_at_offset(0x02, address);
    }
    
    pub fn set_swi3_vector(&mut self, address: u16) {
        // SWI3 vector at 0xFFF4 (offset 0x04 from 0xFFF0)  
        self.write_vector_at_offset(0x04, address);
    }
    
    fn write_vector_at_offset(&mut self, offset: usize, address: u16) {
        self.data[offset] = (address >> 8) as u8;     // High byte
        self.data[offset + 1] = (address & 0xFF) as u8; // Low byte
    }
}

impl MemoryBusDevice for InterruptVectorRom {
    fn read(&self, address: u16) -> u8 {
        if address < 0xFFF0 || address > 0xFFFF {
            return 0; // Return 0 for out of range addresses
        }
        
        let offset = (address - 0xFFF0) as usize;
        self.data[offset]
    }
    
    fn write(&mut self, _address: u16, _value: u8) {
        // ROM is read-only, silently ignore writes for test compatibility
    }
    
    fn sync(&mut self, _cycles: Cycles) {
        // ROM doesn't need sync
    }
}