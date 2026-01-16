//! Codegen context - maintains state during code generation

use std::collections::HashMap;

/// Context for code generation (variables, function scopes, etc.)
#[derive(Debug, Clone)]
pub struct CodegenContext {
    pub variables: HashMap<String, u32>,
    pub functions: HashMap<String, u32>,
    pub current_offset: u32,
}

impl CodegenContext {
    pub fn new() -> Self {
        CodegenContext {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_offset: 0,
        }
    }

    pub fn define_variable(&mut self, name: String, offset: u32) {
        self.variables.insert(name, offset);
    }

    pub fn allocate(&mut self, size: u32) -> u32 {
        let offset = self.current_offset;
        self.current_offset += size;
        offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = CodegenContext::new();
        assert_eq!(ctx.current_offset, 0);
    }

    #[test]
    fn test_allocation() {
        let mut ctx = CodegenContext::new();
        let offset1 = ctx.allocate(2);
        let offset2 = ctx.allocate(2);
        assert_eq!(offset1, 0);
        assert_eq!(offset2, 2);
    }
}
