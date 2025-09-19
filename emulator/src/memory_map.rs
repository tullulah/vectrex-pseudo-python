//! Canonical Vectrex memory map aligned with vectrexy `MemoryMap.h`.
//! Ranges (inclusive):
//! 0000-BFFF : Cartridge (up to 48K)
//! C000-C7FF : Unmapped gap
//! C800-CFFF : 2K window -> 1K RAM shadowed twice
//! D000-D7FF : VIA 6522 (16 bytes shadowed 128x)
//! D800-DDFF : Illegal (VIA+RAM select) (here: treated as illegal/unmapped)
//! E000-FFFF : 8K BIOS region (Mine Storm + BIOS)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region { Cartridge, Gap, Ram, Via, Illegal, Bios, Unmapped }

pub const CART_START: u16 = 0x0000; pub const CART_END: u16 = 0xBFFF;
pub const GAP_START: u16 = 0xC000; pub const GAP_END: u16 = 0xC7FF;
pub const RAM_START: u16 = 0xC800; pub const RAM_END: u16 = 0xCFFF; // 2K physical, 1K logical
pub const VIA_START: u16 = 0xD000; pub const VIA_END: u16 = 0xD7FF; // mirrored
pub const ILLEGAL_START: u16 = 0xD800; pub const ILLEGAL_END: u16 = 0xDFFF;
pub const BIOS_START: u16 = 0xE000; pub const BIOS_END: u16 = 0xFFFF; // 8K

pub fn classify(addr: u16) -> Region {
    // Cobertura total 0000-FFFF por rangos definidos; no se necesita fallback.
    if addr <= CART_END { return Region::Cartridge; }
    if addr <= GAP_END { return Region::Gap; }
    if addr <= RAM_END { return Region::Ram; }
    if addr <= VIA_END { return Region::Via; }
    if addr <= ILLEGAL_END { return Region::Illegal; }
    // Resto hasta FFFF es BIOS
    Region::Bios
}

#[inline] pub fn ram_offset(addr: u16) -> usize { ((addr - RAM_START) % 0x0400) as usize } // 1K logical
#[inline] pub fn via_reg(addr: u16) -> u8 { ((addr - VIA_START) % 0x10) as u8 } // 16 regs
#[inline] pub fn cart_offset(addr: u16) -> usize { (addr - CART_START) as usize }
#[inline] pub fn bios_offset(addr: u16) -> usize { (addr - BIOS_START) as usize }

/// Decide BIOS placement: 4K image loads at 0xF000, 8K image spans 0xE000-0xFFFF.
pub fn bios_load_base(size: usize) -> u16 { if size <= 0x1000 { 0xF000 } else { BIOS_START } }
