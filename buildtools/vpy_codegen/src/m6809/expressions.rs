//! Expression Compilation for M6809
//!
//! Compiles VPy expressions to M6809 assembly
//! Result stored in RESULT (2-byte RAM variable)

use vpy_parser::{Expr, BinOp, CmpOp};
use super::builtins;

/// Emit code for simple expression (numbers, vars, strings, calls)
pub fn emit_simple_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("    LDD #{}\n", n));
            out.push_str("    STD RESULT\n");
        }
        
        Expr::StringLit(s) => {
            // TODO: String literals need string table
            out.push_str(&format!("    ; STRING: \"{}\"\n", s));
            out.push_str("    LDD #0  ; TODO: String table\n");
            out.push_str("    STD RESULT\n");
        }
        
        Expr::Ident(id) => {
            // IMPORTANT: Name already comes uppercase from unifier (INPUT_INPUT_RESULT, not input_input_result)
            out.push_str(&format!("    LDD VAR_{}\n", id.name));
            out.push_str("    STD RESULT\n");
        }
        
        Expr::Call(call) => {
            // Check if builtin
            if builtins::emit_builtin(&call.name, &call.args, out) {
                return;
            }
            
            // User function call (name already uppercase from unifier)
            // Evaluate arguments and store in VAR_ARG0-4
            for (i, arg) in call.args.iter().enumerate().take(5) {
                emit_simple_expr(arg, out);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            
            // Call function
            out.push_str(&format!("    JSR {}\n", call.name));
        }
        
        Expr::Binary { left, op, right } => {
            emit_binop(left, *op, right, out);
        }
        
        Expr::Not(expr) => {
            emit_simple_expr(expr, out);
            out.push_str("    LDD RESULT\n");
            out.push_str("    BNE .NOT_ZERO\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .NOT_END\n");
            out.push_str(".NOT_ZERO:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".NOT_END:\n");
            out.push_str("    STD RESULT\n");
        }
        
        Expr::BitNot(expr) => {
            emit_simple_expr(expr, out);
            out.push_str("    LDD RESULT\n");
            out.push_str("    COMA\n");
            out.push_str("    COMB\n");
            out.push_str("    STD RESULT\n");
        }
        
        Expr::Compare { left, op, right } => {
            emit_compare(left, *op, right, out);
        }
        
        Expr::Index { target, index } => {
            emit_index(target, index, out);
        }
        
        _ => {
            out.push_str(&format!("    ; TODO: Expr {:?}\n", expr));
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
        }
    }
}

fn emit_binop(left: &Expr, op: BinOp, right: &Expr, out: &mut String) {
    // Evaluate left
    emit_simple_expr(left, out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    PSHS D\n");
    
    // Evaluate right
    emit_simple_expr(right, out);
    out.push_str("    LDD RESULT\n");
    
    // Perform operation
    match op {
        BinOp::Add => {
            out.push_str("    ADDD ,S++\n");
        }
        BinOp::Sub => {
            out.push_str("    SUBD ,S++\n");
        }
        BinOp::Mul => {
            out.push_str("    PULS X      ; Get left into X\n");
            out.push_str("    JSR MUL16   ; D = X * D\n");
        }
        BinOp::Div => {
            out.push_str("    PULS X      ; Get left into X\n");
            out.push_str("    JSR DIV16   ; D = X / D\n");
        }
        BinOp::Mod => {
            out.push_str("    PULS X      ; Get left into X\n");
            out.push_str("    JSR MOD16   ; D = X % D\n");
        }
        BinOp::BitAnd => {
            out.push_str("    PULS X\n");
            out.push_str("    ANDA X\n");
            out.push_str("    ANDB X+1\n");
        }
        BinOp::BitOr => {
            out.push_str("    PULS X\n");
            out.push_str("    ORA X\n");
            out.push_str("    ORB X+1\n");
        }
        BinOp::BitXor => {
            out.push_str("    PULS X\n");
            out.push_str("    EORA X\n");
            out.push_str("    EORB X+1\n");
        }
        _ => {
            out.push_str(&format!("    ; TODO: BinOp {:?}\n", op));
            out.push_str("    LEAS 2,S\n");
        }
    }
    
    out.push_str("    STD RESULT\n");
}

fn emit_compare(left: &Expr, op: CmpOp, right: &Expr, out: &mut String) {
    emit_simple_expr(left, out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    PSHS D\n");
    
    emit_simple_expr(right, out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    CMPD ,S++\n");
    
    let branch_true = match op {
        CmpOp::Eq => "BEQ",
        CmpOp::Ne => "BNE",
        CmpOp::Lt => "BLT",
        CmpOp::Le => "BLE",
        CmpOp::Gt => "BGT",
        CmpOp::Ge => "BGE",
    };
    
    out.push_str(&format!("    {} .CMP_TRUE\n", branch_true));
    out.push_str("    LDD #0\n");
    out.push_str("    BRA .CMP_END\n");
    out.push_str(".CMP_TRUE:\n");
    out.push_str("    LDD #1\n");
    out.push_str(".CMP_END:\n");
    out.push_str("    STD RESULT\n");
}

fn emit_index(array: &Expr, index: &Expr, out: &mut String) {
    // Evaluate array (gets address)
    emit_simple_expr(array, out);
    out.push_str("    LDX RESULT  ; Array base address\n");
    out.push_str("    PSHS X\n");
    
    // Evaluate index
    emit_simple_expr(index, out);
    out.push_str("    LDD RESULT  ; Index\n");
    out.push_str("    ASLB        ; Multiply by 2 (16-bit elements)\n");
    out.push_str("    ROLA\n");
    
    // Calculate address
    out.push_str("    PULS X      ; Array base\n");
    out.push_str("    LEAX D,X    ; X = base + (index * 2)\n");
    out.push_str("    LDD ,X      ; Load value\n");
    out.push_str("    STD RESULT\n");
}
