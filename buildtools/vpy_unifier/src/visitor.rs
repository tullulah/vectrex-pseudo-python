//! AST visitor pattern for traversing and analyzing modules
//!
//! This module provides the infrastructure for implementing various passes
//! over the AST without duplicating traversal logic.

use std::marker::PhantomData;

/// AST visitor trait for custom passes
///
/// Implement this trait to create a custom visitor that processes AST nodes
pub trait AstVisitor {
    fn visit_item(&mut self, _name: &str) {}
    fn visit_function(&mut self, _name: &str) {}
    fn visit_statement(&mut self, _stmt: &str) {}
}

/// Default no-op visitor
pub struct DefaultVisitor {
    _phantom: PhantomData<()>,
}

impl AstVisitor for DefaultVisitor {
    fn visit_item(&mut self, _name: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_creation() {
        let mut _visitor = DefaultVisitor {
            _phantom: PhantomData,
        };
        // Just verify it can be created
    }
}
