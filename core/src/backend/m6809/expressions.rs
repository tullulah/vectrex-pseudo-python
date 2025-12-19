// Expressions - Expression code generation for M6809 backend
use crate::ast::{BinOp, CmpOp, Expr, LogicOp};
use crate::codegen::CodegenOptions;
use super::{FuncCtx, emit_builtin_call, fresh_label, power_of_two_const, format_expr_ref};

pub fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
    emit_expr_depth(expr, out, fctx, string_map, opts, 0);
}

pub fn emit_expr_depth(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, depth: usize) {
    // Safety: Prevent stack overflow with deep recursion
    const MAX_DEPTH: usize = 500;
    if depth > MAX_DEPTH {
        panic!("Maximum expression nesting depth ({}) exceeded. Please simplify your expressions or split into smaller parts.", MAX_DEPTH);
    }
    if depth > 450 {
        eprintln!("WARNING: Deep recursion at depth {} in emit_expr", depth);
    }
    
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("    LDD #{}\n    STD RESULT\n", *n));
        }
        Expr::StringLit(s) => {
            if let Some(label) = string_map.get(s) {
                out.push_str(&format!("    LDX #{}\n    STX RESULT\n", label));
            } else {
                out.push_str("    LDD #0\n    STD RESULT\n");
            }
        }
        Expr::Ident(name) => {
            let upper_name = name.name.to_uppercase();
            // Check if it's a local variable first
            if let Some(off) = fctx.offset_of(&name.name) { 
                out.push_str(&format!("    LDD {} ,S\n    STD RESULT\n", off)); 
            } 
            // Check if it's a compile-time constant
            else if let Some(value) = opts.const_values.get(&upper_name) {
                out.push_str(&format!("    LDD #{}\n    STD RESULT\n", value));
            }
            // Otherwise it's a global variable
            else { 
                out.push_str(&format!("    LDD VAR_{}\n    STD RESULT\n", upper_name)); 
            }
        }
        Expr::Call(ci) => {
            if emit_builtin_call(&ci.name, &ci.args, out, fctx, string_map, opts, Some(ci.source_line)) { return; }
            for (i, arg) in ci.args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr_depth(arg, out, fctx, string_map, opts, depth + 1);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            
            // Special case: in auto_loop mode, loop() function calls should use LOOP_BODY
            let target_name = if opts.auto_loop && ci.name.to_lowercase() == "loop" {
                "LOOP_BODY".to_string()
            } else {
                ci.name.to_uppercase()
            };
            
            if opts.force_extended_jsr { 
                out.push_str(&format!("    JSR >{}\n", target_name)); 
            } else { 
                out.push_str(&format!("    JSR {}\n", target_name)); 
            }
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                out.push_str("    LDD RESULT\n    ADDD RESULT\n    STD RESULT\n");
                return;
            }
            if matches!(op, BinOp::Sub) && format_expr_ref(left) == format_expr_ref(right) {
                out.push_str("    CLRA\n    CLRB\n    STD RESULT\n");
                return;
            }
            // Generalized power-of-two multiply via shifts (any 2^k) using ASLB/ROLA.
            if matches!(op, BinOp::Mul) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
            match op {
                BinOp::Add => out.push_str("    LDD TMPLEFT\n    ADDD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Sub => out.push_str("    LDD TMPLEFT\n    SUBD TMPRIGHT\n    STD RESULT\n"),
                BinOp::Mul => out.push_str("    LDD TMPLEFT\n    STD MUL_A\n    LDD TMPRIGHT\n    STD MUL_B\n    JSR MUL16\n"),
                BinOp::Div => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n"),
                BinOp::FloorDiv => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n"), // División entera igual que Div en enteros
                BinOp::Mod => out.push_str("    LDD TMPLEFT\n    STD DIV_A\n    LDD TMPRIGHT\n    STD DIV_B\n    JSR DIV16\n    ; quotient in RESULT, need remainder: A - Q*B\n    LDD DIV_A\n    STD TMPLEFT\n    LDD RESULT\n    STD MUL_A\n    LDD DIV_B\n    STD MUL_B\n    JSR MUL16\n    ; product in RESULT, subtract from original A (TMPLEFT)\n    LDD TMPLEFT\n    SUBD RESULT\n    STD RESULT\n"),
                BinOp::Shl => out.push_str("    LDD TMPLEFT\nSHL_LOOP: LDA TMPRIGHT+1\n    BEQ SHL_DONE\n    ASLB\n    ROLA\n    DEC TMPRIGHT+1\n    BRA SHL_LOOP\nSHL_DONE: STD RESULT\n"),
                BinOp::Shr => out.push_str("    LDD TMPLEFT\nSHR_LOOP: LDA TMPRIGHT+1\n    BEQ SHR_DONE\n    LSRA\n    RORB\n    DEC TMPRIGHT+1\n    BRA SHR_LOOP\nSHR_DONE: STD RESULT\n"),
                BinOp::BitAnd => out.push_str("    LDD TMPLEFT\n    ANDA TMPRIGHT+1\n    ANDB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitOr  => out.push_str("    LDD TMPLEFT\n    ORA TMPRIGHT+1\n    ORB TMPRIGHT\n    STD RESULT\n"),
                BinOp::BitXor => out.push_str("    LDD TMPLEFT\n    EORA TMPRIGHT+1\n    EORB TMPRIGHT\n    STD RESULT\n"),
            }
        }
        Expr::BitNot(inner) => {
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n    LDD TMPLEFT\n    SUBD TMPRIGHT\n");
            // DON'T overwrite result before branch - execute branch immediately after SUBD
            let lt = fresh_label("CT");
            let end = fresh_label("CE");
            let br = match op { CmpOp::Eq => "BEQ", CmpOp::Ne => "BNE", CmpOp::Lt => "BLT", CmpOp::Le => "BLE", CmpOp::Gt => "BGT", CmpOp::Ge => "BGE" };
            out.push_str(&format!(
                "    {} {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                br, lt, end, lt, end
            ));
        }
        Expr::Logic { op, left, right } => match op {
            LogicOp::And => {
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1);
            out.push_str(
                "    LDD RESULT\n    BEQ NOT_TRUE\n    LDD #0\n    STD RESULT\n    BRA NOT_END\nNOT_TRUE:\n    LDD #1\n    STD RESULT\nNOT_END:\n",
            );
        }
        Expr::List(elements) => {
            // Array literal: load address of array data
            // The array data is generated in the DATA section at the end
            let array_label = fresh_label("ARRAY");
            
            // Register this array for later data generation
            // The label will be resolved when we emit the DATA section
            out.push_str(&format!("; Array literal: {} elements at {}\n", elements.len(), array_label));
            out.push_str(&format!("    LDX #{}\n", array_label));
            out.push_str("    STX RESULT\n");
            
            // Store array info in global context for DATA section generation
            // This will be handled by collect_array_literals function
        }
        Expr::Index { target, index } => {
            // Array indexing: arr[index]
            // 1. Evaluate array expression (should return address)
            emit_expr_depth(target, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n    STD TMPPTR\n"); // Save array base address
            
            // 2. Evaluate index expression
            emit_expr_depth(index, out, fctx, string_map, opts, depth + 1);
            
            // 3. Multiply index by 2 (each element is 2 bytes)
            out.push_str("    LDD RESULT\n    ASLB\n    ROLA\n"); // D = index * 2
            
            // 4. Add to base address
            out.push_str("    ADDD TMPPTR\n"); // D = base + (index * 2)
            out.push_str("    TFR D,X\n"); // X = address of element
            
            // 5. Load value from computed address
            out.push_str("    LDD ,X\n    STD RESULT\n");
        }
    }
}

// power_of_two_const: return shift count if expression is a numeric power-of-two (>1).
