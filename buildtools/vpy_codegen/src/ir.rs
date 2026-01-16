//! Intermediate Representation (IR) for M6809 code generation
//!
//! Bridges the gap between high-level AST and low-level assembly

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
    pub globals: Vec<IRGlobal>,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub instructions: Vec<IRInstruction>,
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    Load { register: String, value: i32 },
    Store { register: String, address: u32 },
    Call { function: String },
    Return,
}

#[derive(Debug, Clone)]
pub struct IRGlobal {
    pub name: String,
    pub value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_creation() {
        let program = IRProgram {
            functions: vec![],
            globals: vec![],
        };
        assert!(program.functions.is_empty());
    }
}
