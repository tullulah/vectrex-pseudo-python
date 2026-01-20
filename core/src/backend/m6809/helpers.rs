// Helpers - Runtime helper functions (MUL, DIV, etc.)

/// Emit 16-bit multiply helper routine
pub fn emit_mul_helper(out: &mut String) {
    out.push_str(
        "MUL16:\n    LDD MUL_A\n    STD MUL_RES\n    LDD #0\n    STD MUL_TMP\n    LDD MUL_B\n    STD MUL_CNT\nMUL16_LOOP:\n    LDD MUL_CNT\n    BEQ MUL16_DONE\n    LDD MUL_CNT\n    ANDA #1\n    BEQ MUL16_SKIP\n    LDD MUL_RES\n    ADDD MUL_TMP\n    STD MUL_TMP\nMUL16_SKIP:\n    LDD MUL_RES\n    ASLB\n    ROLA\n    STD MUL_RES\n    LDD MUL_CNT\n    LSRA\n    RORB\n    STD MUL_CNT\n    BRA MUL16_LOOP\nMUL16_DONE:\n    LDD MUL_TMP\n    STD RESULT\n    RTS\n\n",
    );
}

/// Emit 16-bit division helper routine
pub fn emit_div_helper(out: &mut String) {
    out.push_str(
        "DIV16:\n    LDD #0\n    STD DIV_Q\n    LDD DIV_A\n    STD DIV_R\n    LDD DIV_B\n    BEQ DIV16_DONE\nDIV16_LOOP:\n    LDD DIV_R\n    SUBD DIV_B\n    BLO DIV16_DONE\n    STD DIV_R\n    LDD DIV_Q\n    ADDD #1\n    STD DIV_Q\n    BRA DIV16_LOOP\nDIV16_DONE:\n    LDD DIV_Q\n    STD RESULT\n    RTS\n\n",
    );
}
