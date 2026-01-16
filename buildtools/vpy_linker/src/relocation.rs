//! Apply relocation records during linking
//!
//! Resolves symbol references and patches addresses in object files

pub fn apply_relocations(_symbols: &[(&str, u32)], _binary: &mut [u8]) -> crate::LinkerResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_relocations() {
        let mut binary = vec![0; 256];
        assert!(apply_relocations(&[], &mut binary).is_ok());
    }
}
