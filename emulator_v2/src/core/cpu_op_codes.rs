//! CPU opcodes definitions and lookup tables
//! Port of vectrexy/libs/emulator/include/emulator/CpuOpCodes.h

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

// C++ Original: inline constexpr CpuOp CpuOpsPage0[]
// Note: Only implementing the opcodes we actually use for now
// Full table would be 256 entries with gaps filled with Illegal entries
pub static CPU_OPS_PAGE0_LD_ST: [CpuOp; 35] = [
    // LD family - 8-bit
    CpuOp { op_code: 0x86, name: "LDA", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Load Accumulator A" },
    CpuOp { op_code: 0x96, name: "LDA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Load Accumulator A" },
    CpuOp { op_code: 0xA6, name: "LDA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Accumulator A" },
    CpuOp { op_code: 0xB6, name: "LDA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Load Accumulator A" },
    
    CpuOp { op_code: 0xC6, name: "LDB", addr_mode: AddressingMode::Immediate, cycles: 2, size: 2, description: "Load Accumulator B" },
    CpuOp { op_code: 0xD6, name: "LDB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Load Accumulator B" },
    CpuOp { op_code: 0xE6, name: "LDB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Load Accumulator B" },
    CpuOp { op_code: 0xF6, name: "LDB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Load Accumulator B" },

    // LD family - 16-bit  
    CpuOp { op_code: 0x8E, name: "LDX", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load Index Register X" },
    CpuOp { op_code: 0x9E, name: "LDX", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load Index Register X" },
    CpuOp { op_code: 0xAE, name: "LDX", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load Index Register X" },
    CpuOp { op_code: 0xBE, name: "LDX", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load Index Register X" },
    
    CpuOp { op_code: 0xCC, name: "LDD", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load Double Accumulator" },
    CpuOp { op_code: 0xDC, name: "LDD", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load Double Accumulator" },
    CpuOp { op_code: 0xEC, name: "LDD", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load Double Accumulator" },
    CpuOp { op_code: 0xFC, name: "LDD", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load Double Accumulator" },
    
    CpuOp { op_code: 0xCE, name: "LDU", addr_mode: AddressingMode::Immediate, cycles: 3, size: 3, description: "Load User Stack Pointer" },
    CpuOp { op_code: 0xDE, name: "LDU", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Load User Stack Pointer" },
    CpuOp { op_code: 0xEE, name: "LDU", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Load User Stack Pointer" },
    CpuOp { op_code: 0xFE, name: "LDU", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Load User Stack Pointer" },

    // ST family - 8-bit
    CpuOp { op_code: 0x97, name: "STA", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Store Accumulator A" },
    CpuOp { op_code: 0xA7, name: "STA", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Store Accumulator A" },
    CpuOp { op_code: 0xB7, name: "STA", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Store Accumulator A" },
    
    CpuOp { op_code: 0xD7, name: "STB", addr_mode: AddressingMode::Direct, cycles: 4, size: 2, description: "Store Accumulator B" },
    CpuOp { op_code: 0xE7, name: "STB", addr_mode: AddressingMode::Indexed, cycles: 4, size: 2, description: "Store Accumulator B" },
    CpuOp { op_code: 0xF7, name: "STB", addr_mode: AddressingMode::Extended, cycles: 5, size: 3, description: "Store Accumulator B" },

    // ST family - 16-bit
    CpuOp { op_code: 0x9F, name: "STX", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Store Index Register X" },
    CpuOp { op_code: 0xAF, name: "STX", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Store Index Register X" },
    CpuOp { op_code: 0xBF, name: "STX", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Store Index Register X" },
    
    CpuOp { op_code: 0xDD, name: "STD", addr_mode: AddressingMode::Direct, cycles: 5, size: 2, description: "Store Double Accumulator" },
    CpuOp { op_code: 0xED, name: "STD", addr_mode: AddressingMode::Indexed, cycles: 5, size: 2, description: "Store Double Accumulator" },
    CpuOp { op_code: 0xFD, name: "STD", addr_mode: AddressingMode::Extended, cycles: 6, size: 3, description: "Store Double Accumulator" },
    
    CpuOp { op_code: 0xDF, name: "STU", addr_mode: AddressingMode::Direct, cycles: 6, size: 2, description: "Store User Stack Pointer" },
    CpuOp { op_code: 0xEF, name: "STU", addr_mode: AddressingMode::Indexed, cycles: 6, size: 2, description: "Store User Stack Pointer" },
    CpuOp { op_code: 0xFF, name: "STU", addr_mode: AddressingMode::Extended, cycles: 7, size: 3, description: "Store User Stack Pointer" },
];

// C++ Original: constexpr bool IsOpCodePage1(uint8_t firstByte)
pub fn is_opcode_page1(first_byte: u8) -> bool {
    first_byte == 0x10
}

// C++ Original: constexpr bool IsOpCodePage2(uint8_t firstByte) 
pub fn is_opcode_page2(first_byte: u8) -> bool {
    first_byte == 0x11
}

// C++ Original: inline const CpuOp& LookupCpuOpRuntime(int page, uint8_t opCode)
// Simplified version for implemented opcodes only
pub fn lookup_cpu_op_runtime(page: u8, op_code: u8) -> Option<&'static CpuOp> {
    if page != 0 {
        // Pages 1 and 2 not implemented yet
        return None;
    }
    
    // Find opcode in our implemented LD/ST table
    CPU_OPS_PAGE0_LD_ST.iter().find(|op| op.op_code == op_code)
}