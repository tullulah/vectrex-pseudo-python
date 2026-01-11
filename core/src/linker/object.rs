// Object File Format (.vo)
//
// Binary format for compiled VPy modules with unresolved symbols.
// Similar to ELF .o files but simpler and Vectrex-specific.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Read, Write};

/// Complete object file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectrexObject {
    pub header: ObjectHeader,
    pub sections: Vec<Section>,
    pub symbols: SymbolTable,
    pub relocations: Vec<Relocation>,
    pub debug_info: DebugInfo,
}

/// Object file header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectHeader {
    pub magic: [u8; 4],           // "VObj" signature
    pub version: u16,             // Format version
    pub target: TargetArch,       // M6809, etc.
    pub flags: ObjectFlags,       // Position-independent, etc.
    pub source_file: String,      // Original .vpy file
}

/// Target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetArch {
    M6809,
    // Future: M6809E, HD6309, etc.
}

/// Object file flags
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ObjectFlags {
    pub position_independent: bool,
    pub contains_bank_hints: bool,
}

/// Code/data section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,             // ".text.main", ".data.player_x", ".rodata.STR_0"
    pub section_type: SectionType,
    pub bank_hint: Option<u8>,    // Preferred bank (None = linker decides)
    pub alignment: u16,           // Required alignment (1, 2, 256, etc.)
    pub data: Vec<u8>,            // Raw bytes (empty for BSS)
}

/// Section type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionType {
    Text,       // Executable code
    Data,       // Initialized data
    Bss,        // Uninitialized data (zero-filled)
    ReadOnly,   // Constants (strings, const arrays)
}

impl Section {
    pub fn size(&self) -> usize {
        match self.section_type {
            SectionType::Bss => self.alignment as usize, // Size stored in alignment field for BSS
            _ => self.data.len(),
        }
    }
}

/// Symbol table (exports + imports)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolTable {
    pub exports: Vec<Symbol>,     // Symbols defined in this object
    pub imports: Vec<Symbol>,     // External symbols needed
}

/// Symbol definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,             // "LEVEL1_INIT", "player_x", "VECTREX_SET_INTENSITY"
    pub section: Option<usize>,   // Section index (None for extern)
    pub offset: u16,              // Offset within section
    pub scope: SymbolScope,       // Local, Global, Weak
    pub symbol_type: SymbolType,  // Function, Variable, Constant
}

/// Symbol scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolScope {
    Local,      // Visible only in this file
    Global,     // Exported to other files
    Weak,       // Can be overridden
}

/// Symbol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolType {
    Function,
    Variable,
    Constant,
}

/// Relocation entry (reference to external symbol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relocation {
    pub section: usize,           // Section containing reference
    pub offset: u16,              // Offset of reference within section
    pub reloc_type: RelocationType,
    pub symbol: String,           // Referenced symbol name
    pub addend: i32,              // Additional offset
}

/// Relocation type (how to patch address)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelocationType {
    Absolute16,    // Full 16-bit address (JSR, LDX #addr)
    Relative8,     // PC-relative branch (BRA, BEQ) ±127 bytes
    Relative16,    // Long branch (LBRA, LBEQ) ±32K
    Direct,        // Direct page addressing (8-bit)
    High8,         // High byte of address
    Low8,          // Low byte of address
    CrossBank,     // Cross-bank call (needs wrapper)
}

/// Debug information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DebugInfo {
    pub line_map: HashMap<u16, usize>,  // Address → source line
    pub source_lines: Vec<String>,       // Original source code
}

impl VectrexObject {
    /// Create new empty object
    pub fn new(source_file: String) -> Self {
        Self {
            header: ObjectHeader {
                magic: crate::linker::OBJECT_MAGIC,
                version: crate::linker::OBJECT_FORMAT_VERSION,
                target: TargetArch::M6809,
                flags: ObjectFlags {
                    position_independent: false,
                    contains_bank_hints: false,
                },
                source_file,
            },
            sections: Vec::new(),
            symbols: SymbolTable::default(),
            relocations: Vec::new(),
            debug_info: DebugInfo::default(),
        }
    }

    /// Write object to binary format
    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Serialize to bincode (compact binary format)
        let bytes = bincode::serialize(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writer.write_all(&bytes)?;
        Ok(())
    }
    
    /// Save object to file (.vo)
    pub fn save(&self, path: &std::path::Path) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.write(&mut file)
    }

    /// Read object from binary format
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        
        let obj: VectrexObject = bincode::deserialize(&bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // Validate magic number
        if obj.header.magic != crate::linker::OBJECT_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid magic number: expected VObj, got {:?}", obj.header.magic),
            ));
        }
        
        // Check version compatibility
        if obj.header.version != crate::linker::OBJECT_FORMAT_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Incompatible object version: {} (expected {})", 
                    obj.header.version, crate::linker::OBJECT_FORMAT_VERSION),
            ));
        }
        
        Ok(obj)
    }

    /// Get total size of all sections
    pub fn total_size(&self) -> usize {
        self.sections.iter().map(|s| s.size()).sum()
    }

    /// Find symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.exports.iter().find(|s| s.name == name)
            .or_else(|| self.symbols.imports.iter().find(|s| s.name == name))
    }

    /// Check if symbol is exported
    pub fn exports_symbol(&self, name: &str) -> bool {
        self.symbols.exports.iter().any(|s| s.name == name)
    }

    /// Check if symbol is imported (external)
    pub fn imports_symbol(&self, name: &str) -> bool {
        self.symbols.imports.iter().any(|s| s.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_object_roundtrip() {
        // Create test object
        let mut obj = VectrexObject::new("test.vpy".to_string());
        
        // Add section
        obj.sections.push(Section {
            name: ".text.main".to_string(),
            section_type: SectionType::Text,
            bank_hint: Some(31),
            alignment: 1,
            data: vec![0xBD, 0xF3, 0x73], // JSR Print_Str_d
        });
        
        // Add exported symbol
        obj.symbols.exports.push(Symbol {
            name: "main".to_string(),
            section: Some(0),
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        // Add imported symbol
        obj.symbols.imports.push(Symbol {
            name: "VECTREX_SET_INTENSITY".to_string(),
            section: None,
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        // Add relocation
        obj.relocations.push(Relocation {
            section: 0,
            offset: 1,
            reloc_type: RelocationType::Absolute16,
            symbol: "VECTREX_SET_INTENSITY".to_string(),
            addend: 0,
        });
        
        // Write to bytes
        let mut buffer = Vec::new();
        obj.write(&mut buffer).unwrap();
        
        // Read back
        let mut cursor = Cursor::new(buffer);
        let obj2 = VectrexObject::read(&mut cursor).unwrap();
        
        // Verify
        assert_eq!(obj2.header.source_file, "test.vpy");
        assert_eq!(obj2.sections.len(), 1);
        assert_eq!(obj2.sections[0].name, ".text.main");
        assert_eq!(obj2.symbols.exports.len(), 1);
        assert_eq!(obj2.symbols.exports[0].name, "main");
        assert_eq!(obj2.symbols.imports.len(), 1);
        assert_eq!(obj2.relocations.len(), 1);
    }

    #[test]
    fn test_symbol_lookup() {
        let mut obj = VectrexObject::new("test.vpy".to_string());
        
        obj.symbols.exports.push(Symbol {
            name: "main".to_string(),
            section: Some(0),
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        obj.symbols.imports.push(Symbol {
            name: "helper".to_string(),
            section: None,
            offset: 0,
            scope: SymbolScope::Global,
            symbol_type: SymbolType::Function,
        });
        
        assert!(obj.exports_symbol("main"));
        assert!(!obj.exports_symbol("helper"));
        assert!(obj.imports_symbol("helper"));
        assert!(!obj.imports_symbol("main"));
    }
}
