//! Opcodes tests module
//! Comprehensive 1:1 tests for all 6809 CPU opcodes
//! Based on Vectrexy implementation

// LD (Load) family - COMPLETED
pub mod test_lda;
pub mod test_ldb;
pub mod test_ldx;
pub mod test_ldd;
pub mod test_ldu;

// ST (Store) family - IN PROGRESS
pub mod test_sta;
pub mod test_stb;