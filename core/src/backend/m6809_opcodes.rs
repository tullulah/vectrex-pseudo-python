// M6809 Opcode table for accurate instruction size calculation
// Reference: Motorola M6809 Programming Reference Guide

/// Get the size in bytes of an M6809 instruction starting with the given opcode
pub fn get_instruction_size(opcode: u8, next_byte: Option<u8>) -> u16 {
    match opcode {
        // Page 2 prefix (10) - adds 1 byte, then check next opcode
        0x10 => {
            if let Some(op2) = next_byte {
                1 + get_page2_size(op2)
            } else {
                1 // Just the prefix
            }
        }
        
        // Page 3 prefix (11) - adds 1 byte, then check next opcode
        0x11 => {
            if let Some(op2) = next_byte {
                1 + get_page3_size(op2)
            } else {
                1 // Just the prefix
            }
        }
        
        // Inherent mode (1 byte instructions)
        0x12 | 0x13 |  // NOP, SYNC
        0x19 | 0x1D |  // DAA, SEX
        0x39 | 0x3A | 0x3B | 0x3D | 0x3E | 0x3F |  // RTS, ABX, RTI, MUL, RESET, SWI
        0x40 | 0x43 | 0x44 | 0x46 | 0x47 | 0x48 | 0x49 | 0x4A | 0x4C | 0x4D | 0x4F |  // NEG, COM, LSR, ROR, ASR, ASL, ROL, DEC, INC, TST, CLR (A)
        0x50 | 0x53 | 0x54 | 0x56 | 0x57 | 0x58 | 0x59 | 0x5A | 0x5C | 0x5D | 0x5F |  // NEG, COM, LSR, ROR, ASR, ASL, ROL, DEC, INC, TST, CLR (B)
        0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x88 | 0x89 | 0x8A | 0x8B |  // Immediate 8-bit (sin 0x8C, 0x8D, 0x8E)
        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 | 0x98 | 0x99 | 0x9A | 0x9B | 0x9C => 1,  // Direct (sin 0x9D, 0x9E, 0x9F)
        
        // Immediate mode (2 bytes: opcode + immediate value)
        0x86 | 0xC6 |  // LDA/LDB (8-bit immediate)
        0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 | 0x26 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2B | 0x2C | 0x2D | 0x2E | 0x2F |  // Short branches
        0x1F | 0x34 | 0x35 | 0x36 | 0x37 => 2,  // TFR, PSHS, PULS, PSHU, PULU
        
        // Direct mode (2 bytes: opcode + address low byte)
        0x96 | 0xD6 | 0xDC => 2,  // LDA/LDB/LDD direct
        
        // Indexed mode (2-5 bytes depending on post-byte)
        0x0E | 0x0F |  // JMP indexed (base + postbyte)
        0x6E | 0x6F | 0x7E |  // JMP variants
        0xAD |  // JSR indexed
        0xA0..=0xAF | 0xE0..=0xEF | 0x60..=0x6F => {
            // Indexed addressing - need to parse post-byte
            // For now, estimate 2 bytes (most common case)
            2
        }
        
        // Extended mode (3 bytes: opcode + 16-bit address)
        0x97 | 0xB6 | 0xB7 | 0xBD | 0xBE | 0xBF | 0xD7 | 0xF6 | 0xF7 | 0xFC | 0xFD | 0xFE | 0xFF |
        0x7F |  // CLR extended
        0x9D | 0x9E | 0x9F => 3,  // JSR/JMP extended
        
        // 16-bit immediate (3 bytes: opcode + 16-bit value)
        0x8C | 0x8D | 0x8E | 0xCC | 0xCE => 3,  // CMPX, CPX, LDX, LDD, LDU immediate
        
        // Long branches (3 bytes: opcode + 16-bit offset)
        0x16 | 0x17 => 3,  // LBRA, LBSR
        
        // Default fallback (most common)
        _ => 1,
    }
}

fn get_page2_size(opcode: u8) -> u16 {
    match opcode {
        0x3F | 0x83 | 0x8C | 0x8E | 0x93 | 0xA3 | 0xAC | 0xAE | 0xB3 | 0xBC | 0xBE => 3,
        _ => 2,
    }
}

fn get_page3_size(opcode: u8) -> u16 {
    match opcode {
        0x3F | 0x83 | 0x8C | 0x93 | 0xA3 | 0xAC | 0xB3 | 0xBC => 3,
        _ => 2,
    }
}
