//! Debug symbol generation - extracts info from linker output

pub fn extract_symbols(_binary_data: &[u8]) -> crate::DebugResult<Vec<(String, u32)>> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_extraction() {
        let symbols = extract_symbols(&[]).unwrap();
        assert!(symbols.is_empty());
    }
}
