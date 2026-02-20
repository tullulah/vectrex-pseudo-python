//! PDB (Program Debug) format handling (JSON-based)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdbFile {
    pub version: String,
    pub symbols: HashMap<String, PdbSymbol>,
    pub source_lines: HashMap<usize, String>,
    pub variables: HashMap<String, PdbSymbol>,
    pub labels: HashMap<String, u32>,
    pub functions: HashMap<String, u32>,
    pub bios_symbols: HashMap<String, u32>,
    pub vpy_line_map: HashMap<usize, usize>,
    pub asm_line_map: HashMap<usize, usize>,
}

impl PdbFile {
    pub fn new() -> Self {
        PdbFile {
            version: "1.0".to_string(),
            symbols: HashMap::new(),
            source_lines: HashMap::new(),
            variables: HashMap::new(),
            labels: HashMap::new(),
            functions: HashMap::new(),
            bios_symbols: HashMap::new(),
            vpy_line_map: HashMap::new(),
            asm_line_map: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdbSymbol {
    pub address: u32,
    pub size: usize,
    pub source_line: usize,
    pub ty: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdb_serialization() {
        let pdb = PdbFile::new();
        let json = serde_json::to_string(&pdb).unwrap();
        assert!(json.contains("1.0"));
    }
}
