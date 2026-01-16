//! Math Builtin Functions
//!
//! Basic math operations for VPy

use vpy_parser::Expr;
use super::expressions;

pub fn emit_abs(args: &[Expr], out: &mut String) {
    if args.len() != 1 {
        out.push_str("    ; ERROR: ABS requires 1 argument\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        return;
    }
    
    out.push_str("    ; ABS: Absolute value\n");
    
    // Evaluate argument
    expressions::emit_simple_expr(&args[0], out);
    
    // Check if negative (test high bit of A)
    out.push_str("    LDD RESULT\n");
    out.push_str("    TSTA           ; Test sign bit\n");
    out.push_str("    BPL .ABS_POS   ; Branch if positive\n");
    
    // Negative: negate (two's complement)
    out.push_str("    COMA           ; Complement A\n");
    out.push_str("    COMB           ; Complement B\n");
    out.push_str("    ADDD #1        ; Add 1 for two's complement\n");
    
    out.push_str(".ABS_POS:\n");
    out.push_str("    STD RESULT\n");
}

pub fn emit_min(args: &[Expr], out: &mut String) {
    if args.len() != 2 {
        out.push_str("    ; ERROR: MIN requires 2 arguments\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        return;
    }
    
    out.push_str("    ; MIN: Return minimum of two values\n");
    
    // Evaluate first argument -> store in TMPPTR
    expressions::emit_simple_expr(&args[0], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR     ; Save first value\n");
    
    // Evaluate second argument -> RESULT
    expressions::emit_simple_expr(&args[1], out);
    
    // Compare: TMPPTR vs RESULT (signed comparison)
    out.push_str("    LDD TMPPTR     ; Load first value\n");
    out.push_str("    CMPD RESULT    ; Compare with second\n");
    out.push_str("    BLE .MIN_FIRST ; Branch if first <= second\n");
    
    // Second is smaller, already in RESULT
    out.push_str("    BRA .MIN_END\n");
    
    out.push_str(".MIN_FIRST:\n");
    out.push_str("    STD RESULT     ; First is smaller\n");
    
    out.push_str(".MIN_END:\n");
}

pub fn emit_max(args: &[Expr], out: &mut String) {
    if args.len() != 2 {
        out.push_str("    ; ERROR: MAX requires 2 arguments\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        return;
    }
    
    out.push_str("    ; MAX: Return maximum of two values\n");
    
    // Evaluate first argument -> store in TMPPTR
    expressions::emit_simple_expr(&args[0], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR     ; Save first value\n");
    
    // Evaluate second argument -> RESULT
    expressions::emit_simple_expr(&args[1], out);
    
    // Compare: TMPPTR vs RESULT (signed comparison)
    out.push_str("    LDD TMPPTR     ; Load first value\n");
    out.push_str("    CMPD RESULT    ; Compare with second\n");
    out.push_str("    BGE .MAX_FIRST ; Branch if first >= second\n");
    
    // Second is larger, already in RESULT
    out.push_str("    BRA .MAX_END\n");
    
    out.push_str(".MAX_FIRST:\n");
    out.push_str("    STD RESULT     ; First is larger\n");
    
    out.push_str(".MAX_END:\n");
}

pub fn emit_clamp(args: &[Expr], out: &mut String) {
    if args.len() != 3 {
        out.push_str("    ; ERROR: CLAMP requires 3 arguments (value, min, max)\n");
        out.push_str("    LDD #0\n    STD RESULT\n");
        return;
    }
    
    out.push_str("    ; CLAMP: Clamp value to range [min, max]\n");
    
    // Evaluate value (arg 0)
    expressions::emit_simple_expr(&args[0], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR     ; Save value\n");
    
    // Evaluate min (arg 1)
    expressions::emit_simple_expr(&args[1], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+2   ; Save min\n");
    
    // Evaluate max (arg 2)
    expressions::emit_simple_expr(&args[2], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+4   ; Save max\n");
    
    // Compare value with min
    out.push_str("    LDD TMPPTR     ; Load value\n");
    out.push_str("    CMPD TMPPTR+2  ; Compare with min\n");
    out.push_str("    BGE .CLAMP_CHK_MAX ; Branch if value >= min\n");
    
    // Value < min: return min
    out.push_str("    LDD TMPPTR+2\n");
    out.push_str("    STD RESULT\n");
    out.push_str("    BRA .CLAMP_END\n");
    
    out.push_str(".CLAMP_CHK_MAX:\n");
    // Compare value with max
    out.push_str("    LDD TMPPTR     ; Load value again\n");
    out.push_str("    CMPD TMPPTR+4  ; Compare with max\n");
    out.push_str("    BLE .CLAMP_OK  ; Branch if value <= max\n");
    
    // Value > max: return max
    out.push_str("    LDD TMPPTR+4\n");
    out.push_str("    STD RESULT\n");
    out.push_str("    BRA .CLAMP_END\n");
    
    out.push_str(".CLAMP_OK:\n");
    // Value is in range: return value
    out.push_str("    LDD TMPPTR\n");
    out.push_str("    STD RESULT\n");
    
    out.push_str(".CLAMP_END:\n");
}
