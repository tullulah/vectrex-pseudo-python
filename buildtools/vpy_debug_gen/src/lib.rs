//! VPy Debug Gen: Phase 9 of buildtools compiler pipeline
//!
//! Generates debug symbols (.pdb) from linker output
//!
//! **Depends on Phase 7 (linker) for correct addresses**
//!
//! # Module Structure
//!
//! - `error.rs`: Error types
//! - `format.rs`: PDB format handling
//! - `generator.rs`: Symbol extraction and PDB creation
//!
//! # Input
//! `LinkedBinary` + original ASM (with final addresses from Phase 7)
//!
//! # Output
//! `.pdb` JSON file for IDE debugging

pub mod error;
pub mod format;
pub mod generator;

pub use error::{DebugError, DebugResult};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugInfo {
    pub symbols: std::collections::HashMap<String, SymbolInfo>,
    pub source_lines: std::collections::HashMap<usize, String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SymbolInfo {
    pub address: u32,
    pub size: usize,
    pub source_line: usize,
}

pub fn generate_debug_info(_binary: &str, _asm: &str) -> DebugResult<DebugInfo> {
    Ok(DebugInfo {
        symbols: std::collections::HashMap::new(),
        source_lines: std::collections::HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        let info = generate_debug_info("", "").unwrap();
        assert!(info.symbols.is_empty());
    }
}
