//! PDB (Program Debug) format handling (JSON-based)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdbFile {
    pub version: String,
    pub symbols: HashMap<String, PdbSymbol>,
    pub source_lines: HashMap<usize, String>,
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
        let pdb = PdbFile {
            version: "1.0".to_string(),
            symbols: HashMap::new(),
            source_lines: HashMap::new(),
        };
        let json = serde_json::to_string(&pdb).unwrap();
        assert!(json.contains("1.0"));
    }
}
