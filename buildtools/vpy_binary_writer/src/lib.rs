//! VPy Binary Writer: Phase 8 of buildtools compiler pipeline
//!
//! Writes final linked binary to .bin file for emulator/cartridge
//!
//! # Module Structure
//!
//! - `error.rs`: Error types
//! - `writer.rs`: File writing logic
//!
//! # Input
//! `LinkedBinary` (final binary from Phase 7)
//!
//! # Output
//! `.bin` file written to disk

pub mod error;
pub mod writer;

pub use error::{WriterError, WriterResult};

pub fn write_binary(_binary: &[u8], _path: &str) -> WriterResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(write_binary(&[], "").is_ok());
    }
}
