//! Object file structures with relocation tables

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ObjectFile {
    pub name: String,
    pub sections: Vec<Section>,
    pub symbol_table: HashMap<String, Symbol>,
    pub relocations: Vec<Relocation>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub data: Vec<u8>,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub address: u32,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Relocation {
    pub offset: u32,
    pub symbol: String,
    pub ty: RelocType,
}

#[derive(Debug, Clone, Copy)]
pub enum RelocType {
    Absolute,
    Relative,
    Bank,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_file_creation() {
        let obj = ObjectFile {
            name: "test.o".to_string(),
            sections: vec![],
            symbol_table: HashMap::new(),
            relocations: vec![],
        };
        assert_eq!(obj.name, "test.o");
    }
}
