//! ASM emission logic - generates M6809 assembly strings

pub fn emit_function_prologue(name: &str) -> String {
    format!("{}:\n", name)
}

pub fn emit_function_epilogue() -> String {
    "    RTS\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_prologue() {
        let prologue = emit_function_prologue("main");
        assert!(prologue.contains("main"));
    }
}
