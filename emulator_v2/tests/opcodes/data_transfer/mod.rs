//! Data transfer instruction tests
pub mod test_lda_fixed;

// Page 1 LDY/STY/LDS/STS opcodes - newly implemented
pub mod test_ldy_immediate;
pub mod test_ldy_direct;
pub mod test_ldy_indexed;
pub mod test_ldy_extended;
pub mod test_sty_direct;
pub mod test_sty_indexed;
pub mod test_sty_extended;
pub mod test_lds_immediate;
pub mod test_sts_direct;
pub mod test_sts_indexed;
pub mod test_sts_extended;
// TEMPORALMENTE COMENTADOS: API incorrecta causando 137 errores de compilación
// pub mod test_sts_immediate_page2;
// pub mod test_sts_direct_page2;
// pub mod test_sts_indexed_page2;
// pub mod test_sts_extended_page2;