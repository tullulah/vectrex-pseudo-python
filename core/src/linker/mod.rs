// VPy Linker - Professional linking system for modular compilation
//
// Architecture:
// 1. Object Files (.vo) - Compiled modules with unresolved symbols
// 2. Linker (vecld) - Resolves symbols, assigns banks, generates ROM
// 3. Linker Scripts (.ld) - Memory layout configuration

pub mod object;
pub mod script;
pub mod resolver;
pub mod bank_allocator;
pub mod rom_writer;
pub mod asm_parser;

pub use object::{VectrexObject, ObjectHeader, Section, Symbol, Relocation};
pub use script::LinkerScript;
pub use resolver::SymbolResolver;
pub use bank_allocator::BankAllocator;
pub use rom_writer::RomWriter;
pub use asm_parser::{extract_sections, build_symbol_table, collect_relocations};

/// Version of the object file format
pub const OBJECT_FORMAT_VERSION: u16 = 1;

/// Magic number for .vo files ("VObj")
pub const OBJECT_MAGIC: [u8; 4] = [0x56, 0x4F, 0x62, 0x6A];
