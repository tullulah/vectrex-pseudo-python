//! Compilation Context for M6809 Codegen
//!
//! Provides thread-local context for sharing information across expression compilation
//! without needing to pass parameters through every function call.

use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    /// Set of array names that are mutable (GlobalLet, stored in RAM)
    /// Const arrays are not in this set (stored in ROM)
    static MUTABLE_ARRAYS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Initialize the mutable arrays context
/// Call this before compiling expressions
pub fn set_mutable_arrays(arrays: HashSet<String>) {
    MUTABLE_ARRAYS.with(|ma| {
        *ma.borrow_mut() = arrays;
    });
}

/// Check if an array name is mutable (stored in RAM)
/// Returns true if the array was defined with 'let' (GlobalLet)
/// Returns false if it's const (stored in ROM)
pub fn is_mutable_array(name: &str) -> bool {
    MUTABLE_ARRAYS.with(|ma| {
        ma.borrow().contains(name)
    })
}

/// Clear the mutable arrays context
pub fn clear_context() {
    MUTABLE_ARRAYS.with(|ma| {
        ma.borrow_mut().clear();
    });
}
