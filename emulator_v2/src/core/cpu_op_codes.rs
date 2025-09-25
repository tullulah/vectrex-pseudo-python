//! CPU opcodes definitions and lookup tables
//! Port of vectrexy/libs/emulator/include/emulator/CpuOpCodes.h

// Helper functions for opcode pages
// C++ Original: inline bool IsOpcodePage1(uint8_t opCode) { return opCode == 0x10; }
pub fn is_opcode_page1(opcode: u8) -> bool {
    opcode == 0x10
}

// C++ Original: inline bool IsOpcodePage2(uint8_t opCode) { return opCode == 0x11; }  
pub fn is_opcode_page2(opcode: u8) -> bool {
    opcode == 0x11
}

// C++ Original: enum class AddressingMode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    Relative,  // Used for branch instructions, involves addding signed constant to PC if branch is
               // taken (+/- 7 or 15 bits).
    Inherent,  // Opcode contains all addressing info (no EA). Also known as "Register" addressing.
    Immediate, // Data follows opcode byte immediately, e.g. 'LDA #$20' loads $20 into A ('#'
               // signifies immediate addressing)
    Direct,    // EA of data is made up of DP value (high) and byte following opcode byte (low):
               // EA = DP:(PC). So there are 256 pages of 256 values.
    Indexed,   // EA is computed using one of the pointer registers (X, Y, U, S, PC). The "postbyte"
               // (byte following opcode byte) specifies variation of computation of EA.
    Extended,  // EA of data is 16 bits following opcode byte: EA = (PC):(PC+1). Always 3 byte
               // instruction.
    Illegal,   // Not an addressing mode; used to denote an illegal addressing.
    Variant,   // Not an addressing mode; used for Page1/Page2 byte
}

// C++ Original: struct CpuOp
#[derive(Debug, Clone)]
pub struct CpuOp {
    pub op_code: u8,
    pub name: &'static str,
    pub addr_mode: AddressingMode,
    pub cycles: u8,
    pub size: u8,
    pub description: &'static str,
}

// Default illegal opcode entry
const ILLEGAL_OP: CpuOp = CpuOp {
    op_code: 0x00,
    name: "ILLEGAL", 
    addr_mode: AddressingMode::Illegal,
    cycles: 1,
    size: 1,
    description: "Illegal instruction"
};

// Runtime lookup function for all opcodes
// C++ Original: inline constexpr const CpuOp& LookupCpuOp(uint8_t cpuOpPage, uint8_t opCode)
pub fn lookup_cpu_op_runtime(cpu_op_page: u8, op_code: u8) -> CpuOp {
    match cpu_op_page {
        0 => lookup_cpu_op_page0(op_code),
        1 => lookup_cpu_op_page1(op_code),
        2 => lookup_cpu_op_page2(op_code),
        _ => ILLEGAL_OP,
    }
}

// Page 0 opcodes lookup
fn lookup_cpu_op_page0(op_code: u8) -> CpuOp {
    match op_code {
        // NEG instructions
        0x00 => CpuOp { op_code: 0x00, name: "NEG", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Negate memory" },
        0x40 => CpuOp { op_code: 0x40, name: "NEGA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Negate Accumulator A" },
        0x50 => CpuOp { op_code: 0x50, name: "NEGB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Negate Accumulator B" },
        0x60 => CpuOp { op_code: 0x60, name: "NEG", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Negate memory indexed" },
        0x70 => CpuOp { op_code: 0x70, name: "NEG", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Negate memory extended" },

        // COM instructions
        0x03 => CpuOp { op_code: 0x03, name: "COM", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Complement memory" },
        0x43 => CpuOp { op_code: 0x43, name: "COMA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Complement Accumulator A" },
        0x53 => CpuOp { op_code: 0x53, name: "COMB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Complement Accumulator B" },
        0x63 => CpuOp { op_code: 0x63, name: "COM", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Complement memory indexed" },
        0x73 => CpuOp { op_code: 0x73, name: "COM", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Complement memory extended" },

        // LSR instructions - Logical Shift Right
        0x04 => CpuOp { op_code: 0x04, name: "LSR", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Logical Shift Right memory" },
        0x44 => CpuOp { op_code: 0x44, name: "LSRA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Logical Shift Right A" },
        0x54 => CpuOp { op_code: 0x54, name: "LSRB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Logical Shift Right B" },
        0x64 => CpuOp { op_code: 0x64, name: "LSR", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Logical Shift Right indexed" },
        0x74 => CpuOp { op_code: 0x74, name: "LSR", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Logical Shift Right extended" },

        // ROR instructions - Rotate Right through Carry
        0x06 => CpuOp { op_code: 0x06, name: "ROR", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Rotate Right memory" },
        0x46 => CpuOp { op_code: 0x46, name: "RORA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Rotate Right A" },
        0x56 => CpuOp { op_code: 0x56, name: "RORB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Rotate Right B" },
        0x66 => CpuOp { op_code: 0x66, name: "ROR", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Rotate Right indexed" },
        0x76 => CpuOp { op_code: 0x76, name: "ROR", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Rotate Right extended" },

        // ASR instructions - Arithmetic Shift Right
        0x07 => CpuOp { op_code: 0x07, name: "ASR", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Arithmetic Shift Right memory" },
        0x47 => CpuOp { op_code: 0x47, name: "ASRA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Arithmetic Shift Right A" },
        0x57 => CpuOp { op_code: 0x57, name: "ASRB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Arithmetic Shift Right B" },
        0x67 => CpuOp { op_code: 0x67, name: "ASR", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Arithmetic Shift Right indexed" },
        0x77 => CpuOp { op_code: 0x77, name: "ASR", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Arithmetic Shift Right extended" },

        // ASL/LSL instructions - Arithmetic/Logical Shift Left
        0x08 => CpuOp { op_code: 0x08, name: "ASL", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Arithmetic Shift Left memory" },
        0x48 => CpuOp { op_code: 0x48, name: "ASLA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Arithmetic Shift Left A" },
        0x58 => CpuOp { op_code: 0x58, name: "ASLB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Arithmetic Shift Left B" },
        0x68 => CpuOp { op_code: 0x68, name: "ASL", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Arithmetic Shift Left indexed" },
        0x78 => CpuOp { op_code: 0x78, name: "ASL", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Arithmetic Shift Left extended" },

        // ROL instructions - Rotate Left through Carry
        0x09 => CpuOp { op_code: 0x09, name: "ROL", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Rotate Left memory" },
        0x49 => CpuOp { op_code: 0x49, name: "ROLA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Rotate Left A" },
        0x59 => CpuOp { op_code: 0x59, name: "ROLB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Rotate Left B" },
        0x69 => CpuOp { op_code: 0x69, name: "ROL", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Rotate Left indexed" },
        0x79 => CpuOp { op_code: 0x79, name: "ROL", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Rotate Left extended" },

        // Long Branch Always - C++ Original: OpLBRA()
        0x16 => CpuOp { op_code: 0x16, name: "LBRA", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Always" },

        // Jump and Subroutine instructions - C++ Original: OpJmp/OpJsr
        0x0E => CpuOp { op_code: 0x0E, name: "JMP", addr_mode: AddressingMode::Direct, cycles: 3, size: 2, description: "Jump direct" },
        0x6E => CpuOp { op_code: 0x6E, name: "JMP", addr_mode: AddressingMode::Indexed, cycles: 3, size: 2, description: "Jump indexed" },
        0x7E => CpuOp { op_code: 0x7E, name: "JMP", addr_mode: AddressingMode::Extended, cycles: 4, size: 3, description: "Jump extended" },
        
        0x17 => CpuOp { op_code: 0x17, name: "LBSR", addr_mode: AddressingMode::Relative, cycles: 9, size: 3, description: "Long Branch to Subroutine" },
        0x8D => CpuOp { op_code: 0x8D, name: "BSR", addr_mode: AddressingMode::Relative, cycles: 7, size: 2, description: "Branch to Subroutine" },
        0x9D => CpuOp { op_code: 0x9D, name: "JSR", addr_mode: AddressingMode::Direct, cycles: 7, size: 2, description: "Jump to Subroutine direct" },
        0xAD => CpuOp { op_code: 0xAD, name: "JSR", addr_mode: AddressingMode::Indexed, cycles: 7, size: 2, description: "Jump to Subroutine indexed" },
        0xBD => CpuOp { op_code: 0xBD, name: "JSR", addr_mode: AddressingMode::Extended, cycles: 8, size: 3, description: "Jump to Subroutine extended" },

        // Branch instructions - C++ Original: OpBranch(condition_lambda)
        // All branch instructions are 3 cycles, 2 bytes (no cycle adjustment for taken/not taken)
        0x20 => CpuOp { op_code: 0x20, name: "BRA", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch always" },
        0x21 => CpuOp { op_code: 0x21, name: "BRN", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch never" },
        0x22 => CpuOp { op_code: 0x22, name: "BHI", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch higher" },
        0x23 => CpuOp { op_code: 0x23, name: "BLS", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch lower or same" },
        0x24 => CpuOp { op_code: 0x24, name: "BCC", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch carry clear" },
        0x25 => CpuOp { op_code: 0x25, name: "BCS", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch carry set" },
        0x26 => CpuOp { op_code: 0x26, name: "BNE", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch not equal" },
        0x27 => CpuOp { op_code: 0x27, name: "BEQ", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch equal" },
        0x28 => CpuOp { op_code: 0x28, name: "BVC", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch overflow clear" },
        0x29 => CpuOp { op_code: 0x29, name: "BVS", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch overflow set" },
        0x2A => CpuOp { op_code: 0x2A, name: "BPL", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch plus" },
        0x2B => CpuOp { op_code: 0x2B, name: "BMI", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch minus" },
        0x2C => CpuOp { op_code: 0x2C, name: "BGE", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch greater or equal" },
        0x2D => CpuOp { op_code: 0x2D, name: "BLT", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch less than" },
        0x2E => CpuOp { op_code: 0x2E, name: "BGT", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch greater than" },
        0x2F => CpuOp { op_code: 0x2F, name: "BLE", addr_mode: AddressingMode::Relative, cycles: 3, size: 2, description: "Branch less or equal" },

        // LEA instructions - C++ Original: OpLEA<0, opCode>(register)
        // All LEA instructions are 4 cycles, 2 bytes, indexed addressing
        0x30 => CpuOp { op_code: 0x30, name: "LEAX", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Effective Address X" },
        0x31 => CpuOp { op_code: 0x31, name: "LEAY", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Effective Address Y" },
        0x32 => CpuOp { op_code: 0x32, name: "LEAS", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Effective Address S" },
        0x33 => CpuOp { op_code: 0x33, name: "LEAU", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Effective Address U" },

        // DEC instructions
        0x0A => CpuOp { op_code: 0x0A, name: "DEC", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Decrement memory" },
        0x4A => CpuOp { op_code: 0x4A, name: "DECA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Decrement Accumulator A" },
        0x5A => CpuOp { op_code: 0x5A, name: "DECB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Decrement Accumulator B" },
        0x6A => CpuOp { op_code: 0x6A, name: "DEC", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Decrement memory indexed" },
        0x7A => CpuOp { op_code: 0x7A, name: "DEC", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Decrement memory extended" },

        // INC instructions
        0x0C => CpuOp { op_code: 0x0C, name: "INC", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Increment memory" },
        0x4C => CpuOp { op_code: 0x4C, name: "INCA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Increment Accumulator A" },
        0x5C => CpuOp { op_code: 0x5C, name: "INCB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Increment Accumulator B" },
        0x6C => CpuOp { op_code: 0x6C, name: "INC", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Increment memory indexed" },
        0x7C => CpuOp { op_code: 0x7C, name: "INC", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Increment memory extended" },

        // TST instructions
        0x0D => CpuOp { op_code: 0x0D, name: "TST", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Test memory" },
        0x4D => CpuOp { op_code: 0x4D, name: "TSTA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Test Accumulator A" },
        0x5D => CpuOp { op_code: 0x5D, name: "TSTB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Test Accumulator B" },
        0x6D => CpuOp { op_code: 0x6D, name: "TST", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Test memory indexed" },

        // SUB instructions - SUBA
        0x80 => CpuOp { op_code: 0x80, name: "SUBA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Subtract from A immediate" },
        0x90 => CpuOp { op_code: 0x90, name: "SUBA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Subtract from A direct" },
        0xA0 => CpuOp { op_code: 0xA0, name: "SUBA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Subtract from A indexed" },
        0xB0 => CpuOp { op_code: 0xB0, name: "SUBA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Subtract from A extended" },

        // CMP instructions - CMPA (Compare A) - C++ Original: OpCMP<0, opCode>(A)
        0x81 => CpuOp { op_code: 0x81, name: "CMPA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Compare A immediate" },
        0x91 => CpuOp { op_code: 0x91, name: "CMPA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Compare A direct" },
        0xA1 => CpuOp { op_code: 0xA1, name: "CMPA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Compare A indexed" },
        0xB1 => CpuOp { op_code: 0xB1, name: "CMPA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Compare A extended" },

        // CMP instructions - CMPB (Compare B) - C++ Original: OpCMP<0, opCode>(B)
        0xC1 => CpuOp { op_code: 0xC1, name: "CMPB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Compare B immediate" },
        0xD1 => CpuOp { op_code: 0xD1, name: "CMPB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Compare B direct" },
        0xE1 => CpuOp { op_code: 0xE1, name: "CMPB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Compare B indexed" },
        0xF1 => CpuOp { op_code: 0xF1, name: "CMPB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Compare B extended" },

        // CMP instructions - CMPX (Compare X) - C++ Original: OpCMP<0, opCode>(X)
        0x8C => CpuOp { op_code: 0x8C, name: "CMPX", addr_mode: AddressingMode::Immediate, cycles: 4, size: 3, description: "Compare X immediate" },
        0x9C => CpuOp { op_code: 0x9C, name: "CMPX", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Compare X direct" },
        0xAC => CpuOp { op_code: 0xAC, name: "CMPX", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Compare X indexed" },
        0xBC => CpuOp { op_code: 0xBC, name: "CMPX", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Compare X extended" },

        // OR instructions - ORAA/ORAB
        // C++ Original: case 0x8A: OpOR<0, 0x8A>(A);
        0x8A => CpuOp { op_code: 0x8A, name: "ORAA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "OR Accumulator A immediate" },
        0x9A => CpuOp { op_code: 0x9A, name: "ORAA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "OR Accumulator A direct" },
        0xAA => CpuOp { op_code: 0xAA, name: "ORAA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "OR Accumulator A indexed" },
        0xBA => CpuOp { op_code: 0xBA, name: "ORAA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "OR Accumulator A extended" },
        0xDA => CpuOp { op_code: 0xDA, name: "ORAB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "OR Accumulator B direct" },
        0xEA => CpuOp { op_code: 0xEA, name: "ORAB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "OR Accumulator B indexed" },
        0xFA => CpuOp { op_code: 0xFA, name: "ORAB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "OR Accumulator B extended" },

        // AND instructions - ANDA/ANDB
        // C++ Original: case 0x84: OpAND<0, 0x84>(A);
        0x84 => CpuOp { op_code: 0x84, name: "ANDA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "AND Accumulator A immediate" },
        0x94 => CpuOp { op_code: 0x94, name: "ANDA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "AND Accumulator A direct" },
        0xA4 => CpuOp { op_code: 0xA4, name: "ANDA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "AND Accumulator A indexed" },
        0xB4 => CpuOp { op_code: 0xB4, name: "ANDA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "AND Accumulator A extended" },
        0xD4 => CpuOp { op_code: 0xD4, name: "ANDB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "AND Accumulator B direct" },
        0xE4 => CpuOp { op_code: 0xE4, name: "ANDB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "AND Accumulator B indexed" },
        0xF4 => CpuOp { op_code: 0xF4, name: "ANDB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "AND Accumulator B extended" },

        // EOR instructions - EORA/EORB
        // C++ Original: case 0x88: OpEOR<0, 0x88>(A);
        0x88 => CpuOp { op_code: 0x88, name: "EORA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "EOR Accumulator A immediate" },
        0x98 => CpuOp { op_code: 0x98, name: "EORA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "EOR Accumulator A direct" },
        0xA8 => CpuOp { op_code: 0xA8, name: "EORA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "EOR Accumulator A indexed" },
        0xB8 => CpuOp { op_code: 0xB8, name: "EORA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "EOR Accumulator A extended" },
        0xD8 => CpuOp { op_code: 0xD8, name: "EORB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "EOR Accumulator B direct" },
        0xE8 => CpuOp { op_code: 0xE8, name: "EORB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "EOR Accumulator B indexed" },
        0xF8 => CpuOp { op_code: 0xF8, name: "EORB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "EOR Accumulator B extended" },

        // ADD instructions - ADDA
        0x8B => CpuOp { op_code: 0x8B, name: "ADDA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Add to A immediate" },
        0x9B => CpuOp { op_code: 0x9B, name: "ADDA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Add to A direct" },
        0xAB => CpuOp { op_code: 0xAB, name: "ADDA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Add to A indexed" },
        0xBB => CpuOp { op_code: 0xBB, name: "ADDA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Add to A extended" },

        // SUB instructions - SUBB
        0xC0 => CpuOp { op_code: 0xC0, name: "SUBB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Subtract from B immediate" },
        0xD0 => CpuOp { op_code: 0xD0, name: "SUBB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Subtract from B direct" },
        0xE0 => CpuOp { op_code: 0xE0, name: "SUBB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Subtract from B indexed" },
        0xF0 => CpuOp { op_code: 0xF0, name: "SUBB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Subtract from B extended" },

        // OR/AND/EOR immediate for B register (already included above in groups)
        // C++ Original: case 0xCA: OpOR<0, 0xCA>(B);
        0xCA => CpuOp { op_code: 0xCA, name: "ORAB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "OR Accumulator B immediate" },
        0xC4 => CpuOp { op_code: 0xC4, name: "ANDB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "AND Accumulator B immediate" },
        0xC8 => CpuOp { op_code: 0xC8, name: "EORB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "EOR Accumulator B immediate" },

        // ADD instructions - ADDB
        0xCB => CpuOp { op_code: 0xCB, name: "ADDB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Add to B immediate" },
        0xDB => CpuOp { op_code: 0xDB, name: "ADDB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Add to B direct" },
        0xEB => CpuOp { op_code: 0xEB, name: "ADDB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Add to B indexed" },
        0xFB => CpuOp { op_code: 0xFB, name: "ADDB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Add to B extended" },

        // ADD/SUB instructions - 16-bit D (ADDD/SUBD)
        0xC3 => CpuOp { op_code: 0xC3, name: "ADDD", addr_mode: AddressingMode::Immediate, cycles: 4, size: 3, description: "Add to Double Accumulator immediate" },
        0xD3 => CpuOp { op_code: 0xD3, name: "ADDD", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Add to Double Accumulator direct" },
        0xE3 => CpuOp { op_code: 0xE3, name: "ADDD", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Add to Double Accumulator indexed" },
        0xF3 => CpuOp { op_code: 0xF3, name: "ADDD", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Add to Double Accumulator extended" },

        0x83 => CpuOp { op_code: 0x83, name: "SUBD", addr_mode: AddressingMode::Immediate, cycles: 4, size: 3, description: "Subtract from Double Accumulator immediate" },
        0x93 => CpuOp { op_code: 0x93, name: "SUBD", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Subtract from Double Accumulator direct" },
        0xA3 => CpuOp { op_code: 0xA3, name: "SUBD", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Subtract from Double Accumulator indexed" },
        0xB3 => CpuOp { op_code: 0xB3, name: "SUBD", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Subtract from Double Accumulator extended" },

        // ADC instructions - Add with Carry A
        0x89 => CpuOp { op_code: 0x89, name: "ADCA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Add with Carry A immediate" },
        0x99 => CpuOp { op_code: 0x99, name: "ADCA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Add with Carry A direct" },
        0xA9 => CpuOp { op_code: 0xA9, name: "ADCA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Add with Carry A indexed" },
        0xB9 => CpuOp { op_code: 0xB9, name: "ADCA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Add with Carry A extended" },

        // ADC instructions - Add with Carry B
        0xC9 => CpuOp { op_code: 0xC9, name: "ADCB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Add with Carry B immediate" },
        0xD9 => CpuOp { op_code: 0xD9, name: "ADCB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Add with Carry B direct" },
        0xE9 => CpuOp { op_code: 0xE9, name: "ADCB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Add with Carry B indexed" },
        0xF9 => CpuOp { op_code: 0xF9, name: "ADCB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Add with Carry B extended" },

        // SBC instructions - Subtract with Carry A
        0x82 => CpuOp { op_code: 0x82, name: "SBCA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Subtract with Carry A immediate" },
        0x92 => CpuOp { op_code: 0x92, name: "SBCA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Subtract with Carry A direct" },
        0xA2 => CpuOp { op_code: 0xA2, name: "SBCA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Subtract with Carry A indexed" },
        0xB2 => CpuOp { op_code: 0xB2, name: "SBCA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Subtract with Carry A extended" },

        // SBC instructions - Subtract with Carry B
        0xC2 => CpuOp { op_code: 0xC2, name: "SBCB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Subtract with Carry B immediate" },
        0xD2 => CpuOp { op_code: 0xD2, name: "SBCB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Subtract with Carry B direct" },
        0xE2 => CpuOp { op_code: 0xE2, name: "SBCB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Subtract with Carry B indexed" },
        0xF2 => CpuOp { op_code: 0xF2, name: "SBCB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Subtract with Carry B extended" },

        // BIT test instructions - A
        0x85 => CpuOp { op_code: 0x85, name: "BITA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Bit Test A immediate" },
        0x95 => CpuOp { op_code: 0x95, name: "BITA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Bit Test A direct" },
        0xA5 => CpuOp { op_code: 0xA5, name: "BITA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Bit Test A indexed" },
        0xB5 => CpuOp { op_code: 0xB5, name: "BITA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Bit Test A extended" },

        // BIT test instructions - B
        0xC5 => CpuOp { op_code: 0xC5, name: "BITB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Bit Test B immediate" },
        0xD5 => CpuOp { op_code: 0xD5, name: "BITB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Bit Test B direct" },
        0xE5 => CpuOp { op_code: 0xE5, name: "BITB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Bit Test B indexed" },
        0xF5 => CpuOp { op_code: 0xF5, name: "BITB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Bit Test B extended" },
        0x7D => CpuOp { op_code: 0x7D, name: "TST", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Test memory extended" },

        // CLR instructions
        0x0F => CpuOp { op_code: 0x0F, name: "CLR", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Clear memory" },
        0x4F => CpuOp { op_code: 0x4F, name: "CLRA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Clear Accumulator A" },
        0x5F => CpuOp { op_code: 0x5F, name: "CLRB", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Clear Accumulator B" },
        0x6F => CpuOp { op_code: 0x6F, name: "CLR", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Clear memory indexed" },
        0x7F => CpuOp { op_code: 0x7F, name: "CLR", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Clear memory extended" },

        // Stack operations - C++ Original: OpPSH/OpPUL
        0x34 => CpuOp { op_code: 0x34, name: "PSHS", addr_mode: AddressingMode::Immediate, cycles: 5, size: 2, description: "Push System Stack" },
        0x35 => CpuOp { op_code: 0x35, name: "PULS", addr_mode: AddressingMode::Immediate, cycles: 5, size: 2, description: "Pull System Stack" },
        0x36 => CpuOp { op_code: 0x36, name: "PSHU", addr_mode: AddressingMode::Immediate, cycles: 5, size: 2, description: "Push User Stack" },
        0x37 => CpuOp { op_code: 0x37, name: "PULU", addr_mode: AddressingMode::Immediate, cycles: 5, size: 2, description: "Pull User Stack" },

        // System operations
        0x39 => CpuOp { op_code: 0x39, name: "RTS", addr_mode: AddressingMode::Inherent, cycles: 5, size: 1, description: "Return from Subroutine" },
        0x3B => CpuOp { op_code: 0x3B, name: "RTI", addr_mode: AddressingMode::Inherent, cycles: 0, size: 1, description: "Return from Interrupt - cycles handled dynamically" },
        0x3C => CpuOp { op_code: 0x3C, name: "CWAI", addr_mode: AddressingMode::Immediate, cycles: 20, size: 2, description: "Clear and Wait for Interrupt" },
        0x3D => CpuOp { op_code: 0x3D, name: "MUL", addr_mode: AddressingMode::Inherent, cycles: 11, size: 1, description: "Multiply A by B" },
        0x3F => CpuOp { op_code: 0x3F, name: "SWI", addr_mode: AddressingMode::Inherent, cycles: 19, size: 1, description: "Software Interrupt" },

        // Register operations
        0x1E => CpuOp { op_code: 0x1E, name: "EXG", addr_mode: AddressingMode::Immediate, cycles: 8, size: 2, description: "Exchange Registers" },
        0x1F => CpuOp { op_code: 0x1F, name: "TFR", addr_mode: AddressingMode::Immediate, cycles: 6, size: 2, description: "Transfer Registers" },

        // Condition code operations
        0x1A => CpuOp { op_code: 0x1A, name: "ORCC", addr_mode: AddressingMode::Immediate, cycles: 3, size: 2, description: "OR Condition Code" },
        0x1C => CpuOp { op_code: 0x1C, name: "ANDCC", addr_mode: AddressingMode::Immediate, cycles: 3, size: 2, description: "AND Condition Code" },

        // Miscellaneous
        0x19 => CpuOp { op_code: 0x19, name: "DAA", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Decimal Adjust A" },
        0x1D => CpuOp { op_code: 0x1D, name: "SEX", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "Sign Extend B to D" },

        // Page prefix opcodes
        0x10 => CpuOp { op_code: 0x10, name: "PAGE1", addr_mode: AddressingMode::Variant, cycles: 0, size: 0, description: "Page 1 prefix" },
        0x11 => CpuOp { op_code: 0x11, name: "PAGE2", addr_mode: AddressingMode::Variant, cycles: 0, size: 0, description: "Page 2 prefix" },

        // NOP
        0x12 => CpuOp { op_code: 0x12, name: "NOP", addr_mode: AddressingMode::Inherent, cycles: 2, size: 1, description: "No Operation" },

        // LD family - 8-bit
        0x86 => CpuOp { op_code: 0x86, name: "LDA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Load Accumulator A" },
        0x96 => CpuOp { op_code: 0x96, name: "LDA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Load Accumulator A" },
        0xA6 => CpuOp { op_code: 0xA6, name: "LDA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Accumulator A" },
        0xB6 => CpuOp { op_code: 0xB6, name: "LDA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Load Accumulator A" },
        
        0xC6 => CpuOp { op_code: 0xC6, name: "LDB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Load Accumulator B" },
        0xD6 => CpuOp { op_code: 0xD6, name: "LDB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Load Accumulator B" },
        0xE6 => CpuOp { op_code: 0xE6, name: "LDB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Accumulator B" },
        0xF6 => CpuOp { op_code: 0xF6, name: "LDB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Load Accumulator B" },

        // LD family - 16-bit  
        0x8E => CpuOp { op_code: 0x8E, name: "LDX", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load Index Register X" },
        0x9E => CpuOp { op_code: 0x9E, name: "LDX", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load Index Register X" },
        0xAE => CpuOp { op_code: 0xAE, name: "LDX", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load Index Register X" },
        0xBE => CpuOp { op_code: 0xBE, name: "LDX", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load Index Register X" },

        0xCC => CpuOp { op_code: 0xCC, name: "LDD", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load Double Accumulator" },
        0xDC => CpuOp { op_code: 0xDC, name: "LDD", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load Double Accumulator" },
        0xEC => CpuOp { op_code: 0xEC, name: "LDD", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load Double Accumulator" },
        0xFC => CpuOp { op_code: 0xFC, name: "LDD", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load Double Accumulator" },

        0xCE => CpuOp { op_code: 0xCE, name: "LDU", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load User Stack Pointer" },
        0xDE => CpuOp { op_code: 0xDE, name: "LDU", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load User Stack Pointer" },
        0xEE => CpuOp { op_code: 0xEE, name: "LDU", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load User Stack Pointer" },
        0xFE => CpuOp { op_code: 0xFE, name: "LDU", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load User Stack Pointer" },

        // ST family - 8-bit
        0x97 => CpuOp { op_code: 0x97, name: "STA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Store Accumulator A" },
        0xA7 => CpuOp { op_code: 0xA7, name: "STA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Store Accumulator A" },
        0xB7 => CpuOp { op_code: 0xB7, name: "STA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Store Accumulator A" },

        0xD7 => CpuOp { op_code: 0xD7, name: "STB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Store Accumulator B" },
        0xE7 => CpuOp { op_code: 0xE7, name: "STB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Store Accumulator B" },
        0xF7 => CpuOp { op_code: 0xF7, name: "STB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Store Accumulator B" },

        // ST family - 16-bit
        0x9F => CpuOp { op_code: 0x9F, name: "STX", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Store Index Register X" },
        0xAF => CpuOp { op_code: 0xAF, name: "STX", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Store Index Register X" },
        0xBF => CpuOp { op_code: 0xBF, name: "STX", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Store Index Register X" },

        0xDD => CpuOp { op_code: 0xDD, name: "STD", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Store Double Accumulator" },
        0xED => CpuOp { op_code: 0xED, name: "STD", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Store Double Accumulator" },
        0xFD => CpuOp { op_code: 0xFD, name: "STD", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Store Double Accumulator" },

        0xDF => CpuOp { op_code: 0xDF, name: "STU", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Store User Stack Pointer" },
        0xEF => CpuOp { op_code: 0xEF, name: "STU", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Store User Stack Pointer" },
        0xFF => CpuOp { op_code: 0xFF, name: "STU", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Store User Stack Pointer" },

        // Default case - illegal instruction
        _ => ILLEGAL_OP,
    }
}

// Page 1 opcodes (0x10xx prefix)
fn lookup_cpu_op_page1(op_code: u8) -> CpuOp {
    match op_code {
        // Long Branch instructions - C++ Original: OpLongBranch(condition_lambda)  
        // All long branches are 5 cycles, 3 bytes (16-bit signed offset)
        0x21 => CpuOp { op_code: 0x21, name: "LBRN", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Never" },
        0x22 => CpuOp { op_code: 0x22, name: "LBHI", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Higher" },
        0x23 => CpuOp { op_code: 0x23, name: "LBLS", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Lower or Same" },
        0x24 => CpuOp { op_code: 0x24, name: "LBCC", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Carry Clear" },
        0x25 => CpuOp { op_code: 0x25, name: "LBCS", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Carry Set" },
        0x26 => CpuOp { op_code: 0x26, name: "LBNE", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Not Equal" },
        0x27 => CpuOp { op_code: 0x27, name: "LBEQ", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Equal" },
        0x28 => CpuOp { op_code: 0x28, name: "LBVC", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Overflow Clear" },
        0x29 => CpuOp { op_code: 0x29, name: "LBVS", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Overflow Set" },
        0x2A => CpuOp { op_code: 0x2A, name: "LBPL", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Plus" },
        0x2B => CpuOp { op_code: 0x2B, name: "LBMI", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Minus" },
        0x2C => CpuOp { op_code: 0x2C, name: "LBGE", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Greater or Equal" },
        0x2D => CpuOp { op_code: 0x2D, name: "LBLT", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Less Than" },
        0x2E => CpuOp { op_code: 0x2E, name: "LBGT", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Greater Than" },
        0x2F => CpuOp { op_code: 0x2F, name: "LBLE", addr_mode: AddressingMode::Relative, cycles: 5, size: 3, description: "Long Branch Less or Equal" },

        // SWI2 instruction - C++ Original: OpSWI(InterruptVector::Swi2)
        0x3F => CpuOp { op_code: 0x3F, name: "SWI2", addr_mode: AddressingMode::Inherent, cycles: 20, size: 2, description: "Software Interrupt 2" },

        // LDY instructions - C++ Original: OpLD<1, opCode>(Y)
        0x8E => CpuOp { op_code: 0x8E, name: "LDY", addr_mode: AddressingMode::Immediate, cycles: 4, size: 4, description: "Load Y immediate" },
        0x9E => CpuOp { op_code: 0x9E, name: "LDY", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Load Y direct" },
        0xAE => CpuOp { op_code: 0xAE, name: "LDY", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Load Y indexed" },
        0xBE => CpuOp { op_code: 0xBE, name: "LDY", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Load Y extended" },

        // LDS instructions - C++ Original: OpLD<1, opCode>(S)
        0xCE => CpuOp { op_code: 0xCE, name: "LDS", addr_mode: AddressingMode::Immediate, cycles: 4, size: 4, description: "Load S immediate" },
        0xDE => CpuOp { op_code: 0xDE, name: "LDS", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Load S direct" },
        0xEE => CpuOp { op_code: 0xEE, name: "LDS", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Load S indexed" },
        0xFE => CpuOp { op_code: 0xFE, name: "LDS", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Load S extended" },

        // STY instructions - C++ Original: OpST<1, opCode>(Y)
        0x9F => CpuOp { op_code: 0x9F, name: "STY", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Store Y direct" },
        0xAF => CpuOp { op_code: 0xAF, name: "STY", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Store Y indexed" },
        0xBF => CpuOp { op_code: 0xBF, name: "STY", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Store Y extended" },

        // STS instructions - C++ Original: OpST<1, opCode>(S)
        0xDF => CpuOp { op_code: 0xDF, name: "STS", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Store S direct" },
        0xEF => CpuOp { op_code: 0xEF, name: "STS", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Store S indexed" },
        0xFF => CpuOp { op_code: 0xFF, name: "STS", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Store S extended" },

        // CMPD instructions - C++ Original: OpCMP<1, opCode>(D)
        0x83 => CpuOp { op_code: 0x83, name: "CMPD", addr_mode: AddressingMode::Immediate, cycles: 5, size: 4, description: "Compare D immediate" },
        0x93 => CpuOp { op_code: 0x93, name: "CMPD", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Compare D direct" },
        0xA3 => CpuOp { op_code: 0xA3, name: "CMPD", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Compare D indexed" },
        0xB3 => CpuOp { op_code: 0xB3, name: "CMPD", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Compare D extended" },

        // CMPY instructions - C++ Original: OpCMP<1, opCode>(Y)
        0x8C => CpuOp { op_code: 0x8C, name: "CMPY", addr_mode: AddressingMode::Immediate, cycles: 5, size: 4, description: "Compare Y immediate" },
        0x9C => CpuOp { op_code: 0x9C, name: "CMPY", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Compare Y direct" },
        0xAC => CpuOp { op_code: 0xAC, name: "CMPY", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Compare Y indexed" },
        0xBC => CpuOp { op_code: 0xBC, name: "CMPY", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Compare Y extended" },
        
        _ => ILLEGAL_OP,
    }
}

// Placeholder for Page 2 opcodes  
fn lookup_cpu_op_page2(op_code: u8) -> CpuOp {
    // C++ Original: Page 2 instructions (0x11xx) - compare opcodes only for now
    match op_code {
        // SWI3 instruction - C++ Original: OpSWI(InterruptVector::Swi3)
        0x3F => CpuOp { op_code: 0x3F, name: "SWI3", addr_mode: AddressingMode::Inherent, cycles: 20, size: 2, description: "Software Interrupt 3" },

        // CMPU - Compare U (16-bit) - C++ Original: OpCMP<2, opCode>(U)
        0x83 => CpuOp { op_code: 0x83, name: "CMPU", addr_mode: AddressingMode::Immediate, cycles: 5, size: 4, description: "Compare U register immediate" },
        0x93 => CpuOp { op_code: 0x93, name: "CMPU", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Compare U register direct" },
        0xA3 => CpuOp { op_code: 0xA3, name: "CMPU", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Compare U register indexed" },
        0xB3 => CpuOp { op_code: 0xB3, name: "CMPU", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Compare U register extended" },
        
        // CMPS - Compare S (16-bit) - C++ Original: OpCMP<2, opCode>(S)
        0x8C => CpuOp { op_code: 0x8C, name: "CMPS", addr_mode: AddressingMode::Immediate, cycles: 5, size: 4, description: "Compare S register immediate" },
        0x9C => CpuOp { op_code: 0x9C, name: "CMPS", addr_mode: AddressingMode::Direct, cycles: 6, size: 3, description: "Compare S register direct" },
        0xAC => CpuOp { op_code: 0xAC, name: "CMPS", addr_mode: AddressingMode::Indexed, cycles: 6, size: 3, description: "Compare S register indexed" },
        0xBC => CpuOp { op_code: 0xBC, name: "CMPS", addr_mode: AddressingMode::Extended, cycles: 7, size: 4, description: "Compare S register extended" },
        
        _ => ILLEGAL_OP
    }
}