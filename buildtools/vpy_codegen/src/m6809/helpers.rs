//! Runtime Helper Functions
//!
//! Mathematical and utility functions

/// Get BIOS function address from VECTREX.I
/// Returns the address as a hex string (e.g., "$F1AA")
/// Falls back to hardcoded value if VECTREX.I cannot be read
fn get_bios_address(symbol_name: &str, fallback_address: &str) -> String {
    // Try to get from VECTREX.I
    let possible_paths = vec![
        "ide/frontend/public/include/VECTREX.I",
        "../ide/frontend/public/include/VECTREX.I",
        "../../ide/frontend/public/include/VECTREX.I",
        "./ide/frontend/public/include/VECTREX.I",
    ];
    
    for path in &possible_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            // Parse VECTREX.I to find the symbol
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with(';') {
                    continue;
                }
                
                // Parse lines like: "Wait_Recal  EQU     $F192"
                if let Some(equ_pos) = line.find("EQU") {
                    let name_part = line[..equ_pos].trim();
                    let value_part = line[equ_pos + 3..].trim();
                    
                    if name_part.eq_ignore_ascii_case(symbol_name) {
                        // Extract just the address (e.g., "$F1AA" or "$F1AA   ; comment")
                        if let Some(addr) = value_part.split_whitespace().next() {
                            if addr.starts_with('$') || addr.starts_with("0x") {
                                return addr.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to hardcoded value
    fallback_address.to_string()
}

pub fn generate_helpers() -> Result<String, String> {
    eprintln!("[DEBUG HELPERS] Generating runtime helpers...");
    let mut asm = String::new();
    
    // Get BIOS function addresses from VECTREX.I
    let dp_to_d0 = get_bios_address("DP_to_D0", "$F1AA");
    let dp_to_c8 = get_bios_address("DP_to_C8", "$F1AF");
    
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; RUNTIME HELPERS\n");
    asm.push_str(";***************************************************************************\n\n");
    
    // VECTREX_PRINT_TEXT: Call Print_Str_d with proper setup
    // Entry: VAR_ARG0=x, VAR_ARG1=y, VAR_ARG2=string pointer
    asm.push_str("VECTREX_PRINT_TEXT:\n");
    asm.push_str("    ; VPy signature: PRINT_TEXT(x, y, string)\n");
    asm.push_str("    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_D0 - set Direct Page for BIOS/VIA access\n", dp_to_d0));
    asm.push_str("    LDU VAR_ARG2   ; string pointer (third parameter)\n");
    asm.push_str("    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)\n");
    asm.push_str("    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)\n");
    asm.push_str("    JSR Print_Str_d ; Print string from U register\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_C8 - restore DP before return\n", dp_to_c8));
    asm.push_str("    RTS\n\n");
    
    // VECTREX_PRINT_NUMBER: Print number at position
    // Entry: VAR_ARG0=x, VAR_ARG1=y, VAR_ARG2=number
    asm.push_str("VECTREX_PRINT_NUMBER:\n");
    asm.push_str("    ; VPy signature: PRINT_NUMBER(x, y, num)\n");
    asm.push_str("    ; Convert number to hex string and print\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_D0 - set Direct Page for BIOS/VIA access\n", dp_to_d0));
    asm.push_str("    LDA VAR_ARG1+1   ; Y position\n");
    asm.push_str("    LDB VAR_ARG0+1   ; X position\n");
    asm.push_str("    JSR Moveto_d     ; Move to position\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert number to string (show low byte as hex)\n");
    asm.push_str("    LDA VAR_ARG2+1   ; Load number value\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert high nibble to ASCII\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    ANDA #$0F\n");
    asm.push_str("    CMPA #10\n");
    asm.push_str("    BLO PN_DIGIT1\n");
    asm.push_str("    ADDA #7          ; A-F\n");
    asm.push_str("PN_DIGIT1:\n");
    asm.push_str("    ADDA #'0'\n");
    asm.push_str("    STA NUM_STR      ; Store first digit\n");
    asm.push_str("    \n");
    asm.push_str("    ; Convert low nibble to ASCII  \n");
    asm.push_str("    LDA VAR_ARG2+1\n");
    asm.push_str("    ANDA #$0F\n");
    asm.push_str("    CMPA #10\n");
    asm.push_str("    BLO PN_DIGIT2\n");
    asm.push_str("    ADDA #7          ; A-F\n");
    asm.push_str("PN_DIGIT2:\n");
    asm.push_str("    ADDA #'0'\n");
    asm.push_str("    ORA #$80         ; Set high bit for string termination\n");
    asm.push_str("    STA NUM_STR+1    ; Store second digit with high bit\n");
    asm.push_str("    \n");
    asm.push_str("    ; Print the string\n");
    asm.push_str("    LDU #NUM_STR     ; Point to our number string\n");
    asm.push_str("    JSR Print_Str_d  ; Print using BIOS\n");
    asm.push_str(&format!("    JSR {}      ; DP_to_C8 - restore DP before return\n", dp_to_c8));
    asm.push_str("    RTS\n\n");
    
    // Call module-specific runtime helpers
    super::math::emit_runtime_helpers(&mut asm);
    super::joystick::emit_runtime_helpers(&mut asm);
    super::drawing::emit_runtime_helpers(&mut asm);
    super::level::emit_runtime_helpers(&mut asm);
    super::utilities::emit_runtime_helpers(&mut asm);
    
    eprintln!("[DEBUG HELPERS] ASM length after all helpers: {}", asm.len());
    
    Ok(asm)
}
