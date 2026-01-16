//! Memory layout decisions - where code and data go in memory

pub struct MemoryLayout {
    pub code_start: u32,
    pub code_size: u32,
    pub data_start: u32,
    pub data_size: u32,
}

impl MemoryLayout {
    pub fn vectrex() -> Self {
        MemoryLayout {
            code_start: 0x0000,
            code_size: 0x8000, // 32KB cartridge window
            data_start: 0x8000,
            data_size: 0x0400, // 1KB RAM
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectrex_layout() {
        let layout = MemoryLayout::vectrex();
        assert_eq!(layout.code_start, 0x0000);
        assert_eq!(layout.code_size, 0x8000);
    }
}
