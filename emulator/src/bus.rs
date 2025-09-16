use crate::via6522::Via6522;

// Basic Vectrex memory map (simplified):
//  0x0000-0x7FFF : RAM / cartridge window (cartridge typically 0x0000-0x7FFF)
//  0x8000-0xC7FF : (Often mirrors / open bus in simplified model)
//  0xC800-0xCFFF : (System RAM / BIOS work area in real hardware; simplified as RAM)
//  0xD000-0xD00F : VIA 6522 registers (mirrored every 16 bytes across D000-D0FF in hardware)
//  0xE000-0xEFFF : Optional 8K BIOS (if 8K image loaded)
//  0xF000-0xF7FF : 4K BIOS (if 4K image) or upper half of 8K BIOS
//  0xF800-0xFFFF : Vector / interrupt vectors (part of BIOS region if present)
// For enforcement we treat any non-VIA access >=0xD010 and < BIOS base as open/unmapped.

#[derive(Default)]
pub struct MemStats {
    pub reads_unmapped: u64,
    pub writes_unmapped: u64,
    pub writes_bios_ignored: u64,
    pub cart_oob_reads: u64,
}

pub struct Bus { pub mem: [u8;65536], pub via: Via6522, bios_read_only: bool, pub last_via_write: Option<(u16,u8)>, pub total_cycles: u64, pub stats: MemStats, bios_base: u16, cart_len: usize }
impl Default for Bus { fn default()->Self{ Self{ mem:[0;65536], via:Via6522::new(), bios_read_only:false, last_via_write:None, total_cycles:0, stats:MemStats::default(), bios_base:0xF000, cart_len:0 } } }
impl Bus {
    pub fn new()->Self{Self::default()}
    pub fn set_bios_read_only(&mut self, ro: bool){ self.bios_read_only = ro; }
    pub fn set_bios_base(&mut self, base: u16){ self.bios_base = base; }
    pub fn set_cart_len(&mut self, len: usize){ self.cart_len = len.min(0x8000); }
        pub fn test_set_cart_len(&mut self, len: usize){ self.cart_len = len; }
        pub fn test_cart_len(&self) -> usize { self.cart_len }
    pub fn load_block(&mut self, base:u16, data:&[u8], allow_bios_write: bool){
        for (i,b) in data.iter().enumerate(){
            let addr_u = base as usize + i; if addr_u>=65536 { break; }
            let addr = addr_u as u16;
            let bios_region = addr >= self.bios_base;
            if allow_bios_write || !(self.bios_read_only && bios_region){ self.mem[addr_u]=*b; }
        }
    }
    fn unmapped_read_fallback(&mut self, addr:u16)->u8 {
        // If within cartridge window but beyond loaded length, return 0x01 per spec goal.
        if (addr as usize) < 0x8000 && (addr as usize) >= self.cart_len && self.cart_len>0 {
            self.stats.cart_oob_reads = self.stats.cart_oob_reads.wrapping_add(1);
            return 0x01;
        }
        self.stats.reads_unmapped = self.stats.reads_unmapped.wrapping_add(1);
        0xFF
    }
    fn unmapped_write_record(&mut self, _addr:u16){ self.stats.writes_unmapped = self.stats.writes_unmapped.wrapping_add(1); }
    pub fn read8(&mut self, addr:u16)->u8 {
        // VIA window: treat D000-D00F explicitly. Future: mirror D000-D0FF if needed.
        if (addr & 0xFFF0)==0xD000 { return self.via.read((addr & 0x000F) as u8); }
        // BIOS region readable (ROM) if bios_read_only set and addr >= bios_base.
        // All other addresses < 0xD000 or between 0xD010 and bios_base are considered RAM or open.
        if addr < 0xD000 {
            // Cartridge window / RAM region; apply OOB cartridge semantics first.
            if (addr as usize) < 0x8000 && (addr as usize) >= self.cart_len && self.cart_len>0 {
                self.stats.cart_oob_reads = self.stats.cart_oob_reads.wrapping_add(1);
                return 0x01;
            }
            return self.mem[addr as usize];
        }
        if addr >= self.bios_base { return self.mem[addr as usize]; }
        // 0xD000-0xD00F handled above; 0xD010..bios_base-1 is open/unmapped in this simplified model.
        self.unmapped_read_fallback(addr)
    }
    pub fn write8(&mut self, addr:u16, val:u8){
        if (addr & 0xFFF0)==0xD000 { self.via.write((addr & 0x000F) as u8,val); self.last_via_write=Some((addr,val)); return; }
        let bios_region = addr >= self.bios_base;
        if self.bios_read_only && bios_region { self.stats.writes_bios_ignored = self.stats.writes_bios_ignored.wrapping_add(1); return; }
        if addr < 0xD000 { self.mem[addr as usize]=val; return; }
        // 0xD010..bios_base-1 => unmapped
        if !bios_region { self.unmapped_write_record(addr); return; }
        // If not read-only, allow (e.g., if dynamically loaded or no BIOS yet)
        self.mem[addr as usize]=val;
    }
    pub fn tick(&mut self, cycles:u32){
        if cycles>0 { self.total_cycles = self.total_cycles.wrapping_add(cycles as u64); }
        self.via.tick(cycles);
    }
    pub fn total_cycles(&self)->u64 { self.total_cycles }
    pub fn via_ifr(&self)->u8 { self.via.raw_ifr() }
    pub fn via_ier(&self)->u8 { self.via.raw_ier() }
}
