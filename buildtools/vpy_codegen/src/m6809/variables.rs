use vpy_parser::{Module, Item, Expr};
use std::collections::HashSet;

pub fn generate_variables(module: &Module) -> Result<String, String> {
    let mut asm = String::new();
    let mut vars = HashSet::new();
    let mut arrays = Vec::new();  // Track arrays for data generation
    
    // Collect all variable names from module (GlobalLet items)
    for item in &module.items {
        if let Item::GlobalLet { name, value, .. } = item {
            vars.insert(name.clone());
            
            // Check if this is an array initialization
            if matches!(value, Expr::List(_)) {
                arrays.push((name.clone(), value.clone()));
            }
        }
    }
    
    // Generate RAM variable definitions
    if !vars.is_empty() {
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; USER VARIABLES\n");
        asm.push_str(";***************************************************************************\n");
        
        let mut offset = 0;
        for var in vars.iter() {
            // IMPORTANT: Variable name already comes uppercase from unifier
            asm.push_str(&format!("VAR_{} EQU $CF10+{}\n", var, offset));
            offset += 2;  // 16-bit variables
        }
        
        asm.push_str("\n");
    }
    
    // Generate array data sections
    if !arrays.is_empty() {
        asm.push_str(";***************************************************************************\n");
        asm.push_str("; ARRAY DATA\n");
        asm.push_str(";***************************************************************************\n");
        
        // Calculate starting address for array data (after all variable pointers)
        // Variables start at $CF10, each takes 2 bytes
        let array_data_start = 0xCF10 + (vars.len() * 2);
        let mut data_offset = 0;
        
        // First pass: Generate EQU definitions for array data addresses
        for (name, value) in arrays.iter() {
            if let Expr::List(elements) = value {
                // Define array data address as EQU (so assembler can resolve in first pass)
                asm.push_str(&format!("VAR_{}_DATA EQU ${:04X}\n", name, array_data_start + data_offset));
                data_offset += elements.len() * 2;  // 16-bit elements
            }
        }
        asm.push_str("\n");
        
        // Second pass: Emit actual array data
        asm.push_str("; Array data storage\n");
        asm.push_str(&format!("    ORG ${:04X}  ; Start of array data section\n", array_data_start));
        for (name, value) in arrays {
            if let Expr::List(elements) = value {
                asm.push_str(&format!("; Array: VAR_{}_DATA\n", name));
                
                // Emit array elements
                for (i, elem) in elements.iter().enumerate() {
                    if let Expr::Number(n) = elem {
                        asm.push_str(&format!("    FDB {}    ; Element {}\n", n, i));
                    } else {
                        asm.push_str(&format!("    FDB 0     ; Element {} (TODO: complex init)\n", i));
                    }
                }
            }
        }
        asm.push_str("\n");
    }
    
    // Always define ARG variables for function calls
    asm.push_str("; Function argument slots\n");
    for i in 0..5 {
        asm.push_str(&format!("VAR_ARG{} EQU $CFE0+{}\n", i, i * 2));
    }
    asm.push_str("\n");
    
    Ok(asm)
}
