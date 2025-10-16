// debug_info.rs - Estructuras para debug symbols (.pdb file generation)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Debug information collected during compilation for mapping VPy source to binary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Version of the debug format
    pub version: String,
    
    /// Source file name (e.g., "main.vpy")
    pub source: String,
    
    /// Binary file name (e.g., "main.bin")
    pub binary: String,
    
    /// Entry point address in hex (e.g., "0xC880")
    pub entry_point: String,
    
    /// Symbol table: symbol name -> address in hex
    pub symbols: HashMap<String, String>,
    
    /// Line mapping: VPy line number (as string) -> address in hex
    #[serde(rename = "lineMap")]
    pub line_map: HashMap<String, String>,
}

impl DebugInfo {
    pub fn new(source: String, binary: String) -> Self {
        Self {
            version: "1.0".to_string(),
            source,
            binary,
            entry_point: "0x0000".to_string(),
            symbols: HashMap::new(),
            line_map: HashMap::new(),
        }
    }
    
    /// Add a symbol (function name, label, etc.) with its address
    pub fn add_symbol(&mut self, name: String, address: u16) {
        self.symbols.insert(name, format!("0x{:04X}", address));
    }
    
    /// Add a line mapping from VPy source line to binary address
    pub fn add_line_mapping(&mut self, line: usize, address: u16) {
        self.line_map.insert(line.to_string(), format!("0x{:04X}", address));
    }
    
    /// Set the entry point address
    pub fn set_entry_point(&mut self, address: u16) {
        self.entry_point = format!("0x{:04X}", address);
    }
    
    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Context for tracking line information during code generation
#[derive(Debug, Clone)]
pub struct LineTracker {
    /// Current address being generated (relative to ORG)
    pub current_address: u16,
    
    /// Current VPy source line (if known)
    pub current_line: Option<usize>,
    
    /// Accumulated debug info
    pub debug_info: DebugInfo,
}

impl LineTracker {
    pub fn new(source: String, binary: String, org: u16) -> Self {
        Self {
            current_address: org,
            current_line: None,
            debug_info: DebugInfo::new(source, binary),
        }
    }
    
    /// Update current source line
    pub fn set_line(&mut self, line: usize) {
        self.current_line = Some(line);
        // Record mapping when we first encounter this line
        self.debug_info.add_line_mapping(line, self.current_address);
    }
    
    /// Add bytes to current address (track code generation progress)
    pub fn advance(&mut self, bytes: u16) {
        self.current_address = self.current_address.wrapping_add(bytes);
    }
    
    /// Add a symbol at current address
    pub fn add_symbol(&mut self, name: String) {
        self.debug_info.add_symbol(name, self.current_address);
    }
}
