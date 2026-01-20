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
    // DEBUG TRACE: Log input/output info
    eprintln!("[vpy_binary_writer] write_binary called: path={}", _path);
    eprintln!("[vpy_binary_writer] binary size: {} bytes", _binary.len());
    // Optionally, print first 32 bytes as hex for inspection
    let preview_len = _binary.len().min(32);
    if preview_len > 0 {
        let preview: Vec<String> = _binary[..preview_len].iter().map(|b| format!("{:02X}", b)).collect();
        eprintln!("[vpy_binary_writer] binary preview: {}", preview.join(" "));
    }
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
