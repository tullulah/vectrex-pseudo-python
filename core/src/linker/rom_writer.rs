// ROM Writer
//
// Writes final ROM file from linked sections.

use std::io::{self, Write};

pub struct RomWriter {
    rom_size: usize,
    data: Vec<u8>,
}

impl RomWriter {
    pub fn new(rom_size: usize) -> Self {
        Self {
            rom_size,
            data: vec![0xFF; rom_size], // Pad with 0xFF
        }
    }

    pub fn write_bank(&mut self, bank_id: u8, offset: u16, data: &[u8]) -> Result<(), String> {
        let bank_size = if bank_id == 31 { 8192 } else { 16384 };
        let start = (bank_id as usize) * bank_size + (offset as usize);
        
        if start + data.len() > self.rom_size {
            return Err(format!("ROM overflow: bank {} offset {} size {}", bank_id, offset, data.len()));
        }
        
        self.data[start..start + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn finalize<W: Write>(self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.data)?;
        Ok(())
    }
}
