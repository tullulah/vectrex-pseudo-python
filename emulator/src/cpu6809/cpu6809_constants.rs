// Canonical 6809 vector addresses (high, low)
// Standard 6809 vector map (big-endian: high byte at base, low at base+1)
// Reference: Vectrex hardware and Motorola 6809 datasheet.
pub const VEC_SWI2: u16 = 0xFFF2; // SWI2
pub const VEC_SWI3: u16 = 0xFFF4; // SWI3
pub const VEC_FIRQ: u16 = 0xFFF6; // FIRQ
pub const VEC_IRQ:  u16 = 0xFFF8; // IRQ
pub const VEC_SWI:  u16 = 0xFFFA; // SWI (SWI1)
pub const VEC_NMI:  u16 = 0xFFFC; // NMI
pub const VEC_RESET:u16 = 0xFFFE; // RESET

// Centralized list of illegal / undefined base opcodes for MC6809 treated as
// 1-cycle NOPs. Includes placeholders 0x7B, 0x8F currently handled as NOP to
// suppress noise. Any modification here MUST be reflected in documentation
// (SUPER_SUMMARY.md section 24) and tests using is_illegal_base_opcode.
pub const ILLEGAL_BASE_OPCODES: &[u8] = &[
    0x01,0x02,0x05,0x14,0x15,0x38,0x45,0x4E,0x52,0x61,0x7B,0x8F,0xCF,
    0x41,0x42,0x4B,0x51,0x55,0x5B,0x5E,0x62,0x65,0x6B,0x71,0x72,0x75,0x87,0xC7,0xCD
];

#[inline]
pub fn is_illegal_base_opcode(op: u8) -> bool { ILLEGAL_BASE_OPCODES.contains(&op) }

// Valid extended prefixes (page 2 & 3)
pub const VALID_PREFIX10: &[u8] = &[
    // Long branches (all conditional forms) 0x21-0x2F
    0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2A,0x2B,0x2C,0x2D,0x2E,0x2F,
    // SWI2
    0x3F,
    // CMPD & CMPY families
    0x83,0x93,0xA3,0xB3, // CMPD imm/dir/idx/ext
    0x8C,0x9C,0xAC,0xBC, // CMPY imm/dir/idx/ext
    // LDY/STY
    0x8E, // LDY immediate
    0x9E,0xAE,0xBE, // LDY direct/indexed/extended
    0x9F,0xAF,0xBF, // STY direct/indexed/extended
    // LDS/STS
    0xCE, // LDS immediate
    0xDE,0xEE,0xFE, // LDS direct/indexed/extended
    0xDF,0xEF,0xFF, // STS direct/indexed/extended
];

pub const VALID_PREFIX11: &[u8] = &[
    // SWI3
    0x3F,
    // CMPU & CMPS families
    0x83,0x93,0xA3,0xB3, // CMPU imm/dir/idx/ext
    0x8C,0x9C,0xAC,0xBC, // CMPS imm/dir/idx/ext
];
