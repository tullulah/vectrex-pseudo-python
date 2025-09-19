//! 6809 cycle timing tables & scaffold for future centralized timing logic.
//!
//! Fuentes (referencia):
//!  - Motorola MC6809/MC6809E 8-bit Microprocessor Programming Manual
//!  - Motorola M6809 Microprocessor Technical Data
//!  - Listados comunitarios contrastados
//!
//! Estado actual:
//!  - `CYCLES_BASE` y prefijos conservan una tabla densa (algunos valores provisionales / INVALID).
//!  - Aún no se usa de forma autoritativa en `CPU::step`; el núcleo mantiene semillas internas.
//!  - Este archivo añade una capa `CycleInfo` y helpers para migrar gradualmente.
//!
//! Próximo paso previsto: reemplazar los seeds del gran `match` en `step()` por consultas
//! a estas tablas + ajustes dinámicos (branch taken, long branch, indexados complejos, máscaras de stack).
//!
//! Nota: No alterar timings todavía (TDD actual depende de los valores en handlers). Este scaffold es pasivo.

#![allow(dead_code)]

pub const INVALID: u8 = 0xFF; // sentinel for undefined / unassigned opcode slot

// Base (no prefix) opcode cycles (256 entries)
// For unimplemented opcodes in the current emulator, we still store the real cycle count (when known)
// and mark with a trailing comment: // NOT IMPLEMENTED
pub static CYCLES_BASE: [u8;256] = [
    /*00*/ 2,6,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID, INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    /*10*/ INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID, INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    /*20*/ 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,
    /*30*/ 6,6,6,6,4,5,4,5, 2,6,3,8,20,11,19,19, // LEAX..LEAU | PSHS/PULS | PSHU/PULU | RTS/RTI | CWAI(20) | MUL(11) | WAI(19) | SWI(19)
    /*40*/ 2,INVALID,INVALID,2,2,2,INVALID,2, 2,2,2,2,2,2,2,2,
    /*50*/ INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID, INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,INVALID,
    /*60*/ 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
    /*70*/ 7,6,6,7,7,6,7,7, 7,7,7,6,7,7,3,7, // Extended RMW/JMP cluster: RMW ext=7, JMP ext=3, CLR ext=7
    /*80*/ 2,2,2,2,2,2,2,2, 2,2,2,2,4,5,5,5,
    /*90*/ 4,4,4,4,4,4,4,4, 4,4,4,4,6,6,6,6,
    /*A0*/ 6,6,6,6,6,6,6,6, 6,6,6,6,8,8,8,8,
    /*B0*/ 5,5,5,5,5,5,5,5, 5,5,5,5,7,8,7,7, // JSR extended corrected to 8 cycles
    /*C0*/ 2,2,2,2,2,2,2,2, 2,2,2,2,2,5,5,5,
    /*D0*/ 4,4,4,4,4,4,4,4, 4,4,4,4,5,5,5,5,
    /*E0*/ 6,6,6,6,6,6,6,6, 6,6,6,6,7,7,7,7,
    /*F0*/ 5,5,5,5,5,5,5,5, 5,5,5,5,6,6,6,6,
];

// NOT IMPLEMENTED (placeholders / INVALID):
// 0x01,0x02..0x0F (most), 0x40+ entries with INVALID, full 0x50 page, and any opcodes not yet decoded in CPU::step.
// These retain either INVALID (0xFF) or provisional cycle counts based on documented 6809 timings.

// Prefix 0x10 opcodes (extended / long branches). Unused slots = INVALID.
pub static CYCLES_PREFIX10: [u8;256] = {
    let mut t=[INVALID;256];
    // Long branches (LBRA..LBSR etc) relative long
    t[0x21]=5; t[0x22]=5; t[0x23]=5; t[0x24]=5; t[0x25]=5; t[0x26]=5; t[0x27]=5; t[0x28]=5;
    t[0x29]=5; t[0x2A]=5; t[0x2B]=5; t[0x2C]=5; t[0x2D]=5; t[0x2E]=5; t[0x2F]=5;
    t
};

// Prefix 0x11 opcodes
pub static CYCLES_PREFIX11: [u8;256] = {
    let mut t=[INVALID;256];
    t[0x3F] = 20; // SWI3 NOT IMPLEMENTED
    // CMPU
    t[0x83]=5; t[0x93]=6; t[0xA3]=8; t[0xB3]=7;
    // CMPS
    t[0x8C]=5; t[0x9C]=6; t[0xAC]=8; t[0xBC]=7;
    t
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AddrMode { Inherent, Immediate, Direct, Indexed, Extended, Relative, LongRelative, Stack, Unknown }

pub fn base_cycles(op:u8)->u8 { CYCLES_BASE[op as usize] }

pub fn pref10_cycles(sub:u8)->u8 { CYCLES_PREFIX10[sub as usize] }
pub fn pref11_cycles(sub:u8)->u8 { CYCLES_PREFIX11[sub as usize] }

// ---------------------------------------------------------------------------
// New scaffold: richer cycle metadata (optional for gradual migration)
// ---------------------------------------------------------------------------
#[derive(Clone,Copy,Debug)]
pub struct CycleInfo {
    pub base: u8,            // Documented base cycles (no dynamic adders)
    pub branch_short: bool,  // Short branch (taken adds +1)
    pub branch_long: bool,   // Long branch (taken adds +1 vs base 5 else base 5 w/out +1?)
    pub may_index_extra: bool, // Placeholder: some indexed forms add cycle; refine per postbyte
    pub variable: bool,      // True if inherently variable (stack mask, etc.)
}

impl CycleInfo {
    pub const fn simple(base:u8)->Self { Self { base, branch_short:false, branch_long:false, may_index_extra:false, variable:false } }
}

// Sparse map (subset) for opcodes already asserted in tests; more will be added incrementally.
// NOTE: We intentionally DO NOT enforce consistency yet—only provide a query surface.
use core::ops::Index;
pub struct CycleMap([Option<CycleInfo>;256]);
impl Default for CycleMap { fn default()->Self { Self([None;256]) } }
impl Index<u8> for CycleMap { type Output = Option<CycleInfo>; fn index(&self, i:u8)->&Self::Output { &self.0[i as usize] } }

pub static CYCLE_MAP: CycleMap = {
    let mut arr: [Option<CycleInfo>;256] = [None;256];
    arr[0x8E] = Some(CycleInfo{ base:3, branch_short:false, branch_long:false, may_index_extra:false, variable:false });
    arr[0xC0] = Some(CycleInfo::simple(2));
    arr[0xCB] = Some(CycleInfo::simple(2));
    arr[0x8D] = Some(CycleInfo::simple(7));
    arr[0x39] = Some(CycleInfo::simple(5));
    arr[0x20] = Some(CycleInfo{ base:2, branch_short:true, branch_long:false, may_index_extra:false, variable:false });
    CycleMap(arr)
};

pub fn lookup_cycle_info(op:u8) -> Option<CycleInfo> { CYCLE_MAP[op] }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cycle_map_subset_present() {
        let bra = lookup_cycle_info(0x20).unwrap();
        assert!(bra.branch_short);
        assert_eq!(bra.base, 2);
        let rts = lookup_cycle_info(0x39).unwrap();
        assert_eq!(rts.base, 5);
    }
}