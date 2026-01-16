//! Write final binary to .bin file

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_to_file(path: impl AsRef<Path>, data: &[u8]) -> crate::WriterResult<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_write_binary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.bin");
        let data = vec![0xAA, 0xBB, 0xCC];
        assert!(write_to_file(&path, &data).is_ok());
        let read_data = fs::read(&path).unwrap();
        assert_eq!(read_data, data);
    }
}
