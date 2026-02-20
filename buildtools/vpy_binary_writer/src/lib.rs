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

pub fn write_binary(binary: &[u8], path: &str) -> WriterResult<()> {
    writer::write_to_file(path, binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_binary_to_tempfile() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output.bin");
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        write_binary(&data, path.to_str().unwrap()).unwrap();
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(read_back, data);
    }

    #[test]
    fn test_write_binary_invalid_path() {
        // Writing to a non-existent directory must fail
        assert!(write_binary(&[0x00], "/nonexistent_dir/out.bin").is_err());
    }
}
