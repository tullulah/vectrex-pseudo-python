//! Opcodes tests module
//! Comprehensive 1:1 tests for all 6809 CPU opcodes
//! Based on Vectrexy implementation

// Categorized opcode test modules
pub mod data_transfer;  // LD, ST, LEA, TFR, EXG, PSH, PUL
pub mod arithmetic;     // ADD, SUB, MUL, DIV, CMP, INC, DEC
pub mod misc;          // NOP, SYNC, JMP, ORCC, ANDCC
pub mod branch;        // BRA, BEQ, JSR, RTS, LBRA, LBSR

// Special opcode categories
pub mod illegal;       // Illegal opcodes (should panic)
pub mod reserved;      // Reserved opcodes (should panic)
pub mod interrupt;     // RTI, SWI, CWAI