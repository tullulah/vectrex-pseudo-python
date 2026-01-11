// Expressions - Expression code generation for M6809 backend
use crate::ast::{BinOp, CmpOp, Expr, LogicOp};
use crate::codegen::CodegenOptions;
use super::{FuncCtx, emit_builtin_call, fresh_label, power_of_two_const, format_expr_ref};

pub fn emit_expr(expr: &Expr, out: &mut String, fctx: &FuncCtx, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions) {
    emit_expr_depth(expr, out, fctx, string_map, opts, 0, 0, None);
}

pub fn emit_expr_depth(
    expr: &Expr,
    out: &mut String,
    fctx: &FuncCtx,
    string_map: &std::collections::BTreeMap<String,String>,
    opts: &CodegenOptions,
    depth: usize,
    stack_depth: usize,
    caller_func: Option<&str> // NEW: Function name for cross-bank wrapper detection
) {
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
            // Emit numbers as-is in decimal format (assembler interprets negatives as signed)
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
                let adjusted_offset = off + (stack_depth * 2) as i32;
                out.push_str(&format!("    LDD {} ,S\n    STD RESULT\n", adjusted_offset)); 
            } 
            // Check if it's a compile-time constant
            else if let Some(value) = opts.const_values.get(&upper_name) {
                out.push_str(&format!("    LDD #{}\n    STD RESULT\n", value));
            }
            // Check if it's a mutable array (need address of VAR_NAME_DATA, not value)
            else if opts.mutable_arrays.contains(&name.name) {
                // Load immediate address of array data (native assembler now supports EQU in immediate mode)
                out.push_str(&format!("    LDD #VAR_{}_DATA\n    STD RESULT\n", upper_name));
            }
            // Otherwise it's a regular global variable
            else { 
                out.push_str(&format!("    LDD VAR_{}\n    STD RESULT\n", upper_name)); 
            }
        }
        Expr::Call(ci) => {
            if emit_builtin_call(&ci.name, &ci.args, out, fctx, string_map, opts, Some(ci.source_line)) { 
                return; 
            }
            for (i, arg) in ci.args.iter().enumerate() {
                if i >= 5 { break; }
                emit_expr_depth(arg, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i));
            }
            
            // Special case: in auto_loop mode, loop() function calls should use LOOP_BODY
            let target_name = if opts.auto_loop && ci.name.to_lowercase() == "loop" {
                "LOOP_BODY".to_string()
            } else {
                ci.name.to_uppercase()
            };
            
            // Check if this is a cross-bank call and needs wrapper
            let final_target = if let Some(ref caller) = fctx.func_name {
                // Use global generator to check if wrapper needed
                if let Some(wrapper_name) = super::bank_wrappers::get_wrapper_for_call(caller, &ci.name) {
                    wrapper_name
                } else {
                    target_name
                }
            } else {
                target_name
            };
            
            if opts.force_extended_jsr { 
                out.push_str(&format!("    JSR >{}\n", final_target)); 
            } else { 
                out.push_str(&format!("    JSR {}\n", final_target)); 
            }
        }
        Expr::MethodCall(mc) => {
            // Method call: obj.method(args)
            // First argument (VAR_ARG0) = address of object (self parameter)
            // Subsequent arguments in VAR_ARG1, VAR_ARG2, etc.
            
            // Special handling for local struct variables: need ADDRESS not value
            if let Expr::Ident(info) = &*mc.target {
                if let Some(off) = fctx.offset_of(&info.name) {
                    // Local struct: compute address with LEAX
                    let adjusted_offset = off + (stack_depth * 2) as i32;
                    out.push_str(&format!("    LEAX {},S  ; Compute address of local struct\n", adjusted_offset));
                    out.push_str("    STX VAR_ARG0\n");
                } else {
                    // Global struct or parameter: load value and pass as pointer
                    emit_expr_depth(&mc.target, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                    out.push_str("    LDD RESULT\n");
                    out.push_str("    STD VAR_ARG0\n");
                }
            } else {
                // Complex expression: evaluate and pass result as pointer
                emit_expr_depth(&mc.target, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str("    LDD RESULT\n");
                out.push_str("    STD VAR_ARG0\n");
            }
            
            // Emit remaining arguments
            for (i, arg) in mc.args.iter().enumerate() {
                if i >= 4 { break; } // Only 5 args total (ARG0-ARG4), first is self
                emit_expr_depth(arg, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str("    LDD RESULT\n");
                out.push_str(&format!("    STD VAR_ARG{}\n", i + 1));
            }
            
            // Determine struct name from target expression using type context
            let struct_name = match &*mc.target {
                Expr::Ident(info) => {
                    // Look up variable name in type context
                    opts.type_context.get(&info.name)
                        .map(|s| s.as_str())
                        .unwrap_or("UNKNOWN")
                }
                _ => "UNKNOWN" // Complex expressions not yet supported
            };
            let mangled_name = format!("{}_{}", struct_name, mc.method_name).to_uppercase();
            
            out.push_str(&format!("; Method call: {}.{}()\n", struct_name, mc.method_name));
            if opts.force_extended_jsr {
                out.push_str(&format!("    JSR >{}\n", mangled_name));
            } else {
                out.push_str(&format!("    JSR {}\n", mangled_name));
            }
        }
        Expr::Binary { op, left, right } => {
            // x+x and x-x peepholes
            if matches!(op, BinOp::Add) && format_expr_ref(left) == format_expr_ref(right) {
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
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
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                } else if let Some(shift) = power_of_two_const(left) {
                    emit_expr_depth(right, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    ASLB\n    ROLA\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Generalized power-of-two division via shifts (only when RHS is const).
            if matches!(op, BinOp::Div) {
                if let Some(shift) = power_of_two_const(right) {
                    emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                    out.push_str("    LDD RESULT\n");
                    for _ in 0..shift { out.push_str("    LSRA\n    RORB\n"); }
                    out.push_str("    STD RESULT\n");
                    return;
                }
            }
            // Fallback general operations via temporaries / helpers.
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            // ALWAYS use stack to preserve left operand from corruption by right operand
            out.push_str("    PSHS D\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1, stack_depth + 1, caller_func);
            out.push_str("    LDD RESULT\n    STD TMPRIGHT\n");
            out.push_str("    PULS D\n    STD TMPLEFT\n");
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
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            out.push_str("    LDD RESULT\n    COMA\n    COMB\n    STD RESULT\n");
        }
        Expr::Compare { op, left, right } => {
            emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            out.push_str("    LDD RESULT\n    STD TMPLEFT\n");
            emit_expr_depth(right, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
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
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                let fl = fresh_label("AND_FALSE");
                let en = fresh_label("AND_END");
                out.push_str(&format!("    LDD RESULT\n    BEQ {}\n", fl));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str(&format!(
                    "    LDD RESULT\n    BEQ {}\n    LDD #1\n    STD RESULT\n    BRA {}\n{}:\n    LDD #0\n    STD RESULT\n{}:\n",
                    fl, en, fl, en
                ));
            }
            LogicOp::Or => {
                let tr = fresh_label("OR_TRUE");
                let en = fresh_label("OR_END");
                emit_expr_depth(left, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str(&format!("    LDD RESULT\n    BNE {}\n", tr));
                emit_expr_depth(right, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                out.push_str(&format!(
                    "    LDD RESULT\n    BNE {}\n    LDD #0\n    STD RESULT\n    BRA {}\n{}:\n    LDD #1\n    STD RESULT\n{}:\n",
                    tr, en, tr, en
                ));
            }
        },
        Expr::Not(inner) => {
            emit_expr_depth(inner, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            out.push_str(
                "    LDD RESULT\n    BEQ NOT_TRUE\n    LDD #0\n    STD RESULT\n    BRA NOT_END\nNOT_TRUE:\n    LDD #1\n    STD RESULT\nNOT_END:\n",
            );
        }
        Expr::List(elements) => {
            // Array literal: load address of array data
            // Find this array in opts.inline_arrays (by matching element count and values)
            let mut found_label: Option<String> = None;
            for (label, inline_elements) in &opts.inline_arrays {
                if inline_elements.len() == elements.len() {
                    // Simple heuristic: match by element count and first element
                    // TODO: More robust matching (by AST ptr comparison or unique ID)
                    let match_first = if elements.is_empty() {
                        true
                    } else {
                        format!("{:?}", elements[0]) == format!("{:?}", inline_elements[0])
                    };
                    if match_first {
                        found_label = Some(label.clone());
                        break;
                    }
                }
            }
            
            let array_label = found_label.unwrap_or_else(|| {
                // Fallback: generate fresh label (shouldn't happen if collector works)
                fresh_label("ARRAY")
            });
            
            // Register this array for later data generation
            // The label will be resolved when we emit the DATA section
            out.push_str(&format!("; Array literal: {} elements at {}\n", elements.len(), array_label));
            out.push_str(&format!("    LDX #{}\n", array_label));
            out.push_str("    STX RESULT\n");
        }
        Expr::Index { target, index } => {
            // Array indexing: arr[index]
            // Special handling for const arrays (ROM-only data)
            if let Expr::Ident(target_name) = target.as_ref() {
                if let Some(const_array_label_suffix) = opts.const_arrays.get(&target_name.name) {
                    // This is a const array in ROM - generate special code
                    out.push_str(&format!("    ; ===== Const array indexing: {} =====\n", target_name.name));
                    
                    // 1. Evaluate index expression
                    emit_expr_depth(index, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
                    
                    // 2. Index is in RESULT, multiply by 2 (each element is 2 bytes)
                    out.push_str("    LDD RESULT\n    ASLB\n    ROLA\n"); // D = index * 2
                    out.push_str("    STD TMPPTR\n"); // Save offset temporarily
                    
                    // 3. Load const array base address and add offset
                    let const_array_label = format!("CONST_ARRAY_{}", const_array_label_suffix);
                    out.push_str(&format!("    LDX #{}\n", const_array_label)); // X = CONST_ARRAY_NAME address
                    out.push_str("    LDD TMPPTR\n"); // D = offset (index * 2)
                    
                    // 4. Add offset to base address
                    out.push_str("    LEAX D,X\n"); // X += D (X = base + offset)
                    
                    // 5. For string arrays: Return pointer itself. For number arrays: Load value
                    if opts.const_string_arrays.contains(&target_name.name) {
                        // String array - return pointer (address in X)
                        out.push_str("    ; String array - load pointer from table\n");
                        out.push_str("    LDD ,X\n    STD RESULT\n"); // Load pointer from FDB table
                    } else {
                        // Number array - return value
                        out.push_str("    LDD ,X\n    STD RESULT\n");
                    }
                    return;
                }
            }
            
            // Regular array (variable in RAM)
            // 1. Evaluate array expression (should return address)
            emit_expr_depth(target, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            out.push_str("    LDD RESULT\n    STD TMPPTR\n"); // Save array base address
            
            // 2. Evaluate index expression
            emit_expr_depth(index, out, fctx, string_map, opts, depth + 1, stack_depth, caller_func);
            
            // 3. Multiply index by 2 (each element is 2 bytes)
            out.push_str("    LDD RESULT\n    ASLB\n    ROLA\n"); // D = index * 2
            
            // 4. Add to base address
            out.push_str("    ADDD TMPPTR\n"); // D = base + (index * 2)
            out.push_str("    TFR D,X\n"); // X = address of element
            
            // 5. Load value from computed address
            out.push_str("    LDD ,X\n    STD RESULT\n");
        }
        Expr::StructInit { struct_name, .. } => {
            // Phase 3 - struct initialization
            // For now, struct instance is stored as local variable (assigned via statement)
            // StructInit expression just returns a "pointer" (address) to where it will be stored
            // The actual storage allocation happens in Assign statement
            // For simplicity, we return address 0 here - actual address determined at assignment
            out.push_str(&format!("    ; StructInit({})", struct_name));
            out.push_str("\n    LDD #0\n    STD RESULT\n");
        }
        Expr::FieldAccess { target, field, source_line, .. } => {
            // Phase 3 - field access codegen
            // 1. Evaluate target expression to get struct instance address/variable
            // For local variables that are structs, we need to know:
            //   - Base offset in stack
            //   - Field offset within struct
            
            // Simple case: target is Ident (struct variable)
            if let Expr::Ident(name) = target.as_ref() {
                let var_name = &name.name;
                
                // ✅ NEW: Handle self.field read in struct methods
                if var_name == "self" {
                    // self is passed as VAR_ARG0 (pointer to struct)
                    if let Some(method_struct_type) = fctx.current_function_struct_type() {
                        if let Some(layout) = opts.structs.get(&method_struct_type) {
                            if let Some(field_layout) = layout.get_field(field) {
                                let field_offset = field_layout.offset as i32;
                                
                                // Load struct pointer and read field
                                out.push_str(&format!("    ; FieldAccess self.{} (struct {} field offset {})\n", field, method_struct_type, field_offset));
                                out.push_str("    LDX VAR_ARG0    ; Load struct pointer\n");
                                out.push_str(&format!("    LDD {},X        ; Read field value\n", field_offset));
                                out.push_str("    STD RESULT\n");
                            } else {
                                eprintln!("WARNING at line {}: Field '{}' not found in struct '{}'", source_line, field, method_struct_type);
                                out.push_str("    LDD #0\n    STD RESULT\n");
                            }
                        } else {
                            eprintln!("WARNING at line {}: Struct type '{}' not found for method", source_line, method_struct_type);
                            out.push_str("    LDD #0\n    STD RESULT\n");
                        }
                    } else {
                        eprintln!("WARNING at line {}: self.field access outside of struct method context", source_line);
                        out.push_str("    LDD #0\n    STD RESULT\n");
                    }
                    return; // Early return
                }
                
                // Check if it's a local variable
                if let Some(base_offset) = fctx.offset_of(var_name) {
                    // Get struct type from variable info
                    let struct_type = fctx.var_type(var_name);
                    
                    if let Some(type_name) = struct_type {
                        if !type_name.is_empty() {
                            // This is a struct variable - find field in its layout
                            if let Some(layout) = opts.structs.get(type_name) {
                                if let Some(field_layout) = layout.get_field(field) {
                                    let field_offset_bytes = field_layout.offset as i32; // offset is already in bytes
                                    let total_offset = base_offset + field_offset_bytes + (stack_depth * 2) as i32;
                                    out.push_str(&format!("    ; FieldAccess {}.{} (struct {} offset {})\n", var_name, field, type_name, total_offset));
                                    out.push_str(&format!("    LDD {},S\n    STD RESULT\n", total_offset));
                                } else {
                                    eprintln!("WARNING at line {}: Field '{}' not found in struct '{}'", source_line, field, type_name);
                                    out.push_str("    LDD #0\n    STD RESULT\n");
                                }
                            } else {
                                eprintln!("WARNING at line {}: Struct type '{}' not found", source_line, type_name);
                                out.push_str("    LDD #0\n    STD RESULT\n");
                            }
                        } else {
                            eprintln!("WARNING at line {}: Variable '{}' is not a struct", source_line, var_name);
                            out.push_str("    LDD #0\n    STD RESULT\n");
                        }
                    } else {
                        eprintln!("WARNING at line {}: Variable '{}' type unknown", source_line, var_name);
                        out.push_str("    LDD #0\n    STD RESULT\n");
                    }
                } else {
                    // Global variable or not found
                    out.push_str(&format!("    ; FieldAccess {}.{} (TODO: global structs)\n", var_name, field));
                    out.push_str("    LDD #0\n    STD RESULT\n");
                }
            } else {
                // Complex expression - not yet supported
                eprintln!("WARNING at line {}: FieldAccess on complex expression not yet supported", source_line);
                out.push_str("    LDD #0\n    STD RESULT\n");
            }
        }
    }
}

// power_of_two_const: return shift count if expression is a numeric power-of-two (>1).
