//! Scope management for variables, functions, and imports
//!
//! Tracks variable definitions, function scopes, and import namespaces

use std::collections::HashMap;

/// Represents a scope level (module, function, block)
#[derive(Debug, Clone)]
pub struct Scope {
    /// Variables defined in this scope
    pub variables: HashMap<String, String>,
    /// Functions defined in this scope
    pub functions: HashMap<String, String>,
    /// Parent scope (if nested)
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    /// Create a new empty scope
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child scope with parent reference
    pub fn child(&self) -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    /// Check if a symbol is defined in this or parent scopes
    pub fn lookup(&self, name: &str) -> Option<String> {
        if let Some(var) = self.variables.get(name) {
            return Some(var.clone());
        }
        if let Some(func) = self.functions.get(name) {
            return Some(func.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.lookup(name);
        }
        None
    }

    /// Define a variable in this scope
    pub fn define_var(&mut self, name: String, type_info: String) {
        self.variables.insert(name, type_info);
    }

    /// Define a function in this scope
    pub fn define_func(&mut self, name: String, signature: String) {
        self.functions.insert(name, signature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_creation() {
        let scope = Scope::new();
        assert!(scope.variables.is_empty());
        assert!(scope.functions.is_empty());
    }

    #[test]
    fn test_variable_definition() {
        let mut scope = Scope::new();
        scope.define_var("x".to_string(), "int".to_string());
        assert!(scope.lookup("x").is_some());
    }

    #[test]
    fn test_scope_nesting() {
        let mut parent = Scope::new();
        parent.define_var("x".to_string(), "int".to_string());

        let child = parent.child();
        assert!(child.lookup("x").is_some());
    }
}
