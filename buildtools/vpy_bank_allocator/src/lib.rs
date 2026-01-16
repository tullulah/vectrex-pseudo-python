//! VPy Bank Allocator: Phase 4 of buildtools compiler pipeline
//!
//! Assigns functions to banks for multibank cartridges
//!
//! # Module Structure
//!
//! - `error.rs`: Error types
//! - `allocator.rs`: Main allocation algorithm
//! - `graph.rs`: Call graph analysis
//!
//! # Input
//! `UnifiedModule` (unified module from Phase 3)
//!
//! # Output
//! `BankLayout` (function-to-bank mapping for multibank or single bank)

pub mod allocator;
pub mod error;
pub mod graph;

pub use error::{BankAllocatorError, BankAllocatorResult};

/// Layout of functions in banks
#[derive(Debug, Clone)]
pub struct BankLayout {
    /// Functions assigned to each bank
    pub banks: Vec<Vec<String>>,
    /// Total banks needed
    pub num_banks: usize,
    /// Bank size limit
    pub bank_size: usize,
}

/// Allocate functions to banks
///
/// # Arguments
/// * `module` - Unified module from Phase 3
/// * `multibank` - Whether to use multibank mode
///
/// # Returns
/// * `BankAllocatorResult<BankLayout>` - Bank layout or error
///
/// # TODO
/// Phase 4 implementation: function-to-bank assignment with call graph analysis
pub fn allocate_banks(_module: &str, _multibank: bool) -> BankAllocatorResult<BankLayout> {
    Ok(BankLayout {
        banks: vec![vec![]],
        num_banks: 1,
        bank_size: 32768,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bank() {
        let layout = allocate_banks("dummy", false).unwrap();
        assert_eq!(layout.num_banks, 1);
    }
}
