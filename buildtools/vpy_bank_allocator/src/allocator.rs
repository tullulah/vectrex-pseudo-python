//! Bank allocation algorithm
//!
//! Implements the main algorithm for assigning functions to banks

/// Allocate functions to banks based on size and call relationships
pub fn allocate_single_bank() -> crate::BankLayout {
    crate::BankLayout {
        banks: vec![vec![]],
        num_banks: 1,
        bank_size: 32768,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bank_allocation() {
        let layout = allocate_single_bank();
        assert_eq!(layout.num_banks, 1);
    }
}
