//! Relocation handling in assembler

#[derive(Debug, Clone)]
pub struct RelocationRecord {
    pub offset: u32,
    pub symbol: String,
    pub ty: RelocationTy,
}

#[derive(Debug, Clone, Copy)]
pub enum RelocationTy {
    Absolute8,
    Absolute16,
    Relative16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_creation() {
        let _reloc = RelocationRecord {
            offset: 0,
            symbol: "main".to_string(),
            ty: RelocationTy::Absolute16,
        };
        // Just verify it can be created
    }
}
