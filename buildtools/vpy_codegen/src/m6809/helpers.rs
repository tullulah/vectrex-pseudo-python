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
    
    // MUL16: Multiply X * D -> D
    asm.push_str("MUL16:\n");
    asm.push_str("    ; Multiply 16-bit X * D -> D\n");
    asm.push_str("    ; Simple implementation (can be optimized)\n");
    asm.push_str("    PSHS X,B,A\n");
    asm.push_str("    LDD #0         ; Result accumulator\n");
    asm.push_str("    LDX 2,S        ; Multiplier\n");
    asm.push_str(".MUL16_LOOP:\n");
    asm.push_str("    BEQ .MUL16_END\n");
    asm.push_str("    ADDD ,S        ; Add multiplicand\n");
    asm.push_str("    LEAX -1,X\n");
    asm.push_str("    BRA .MUL16_LOOP\n");
    asm.push_str(".MUL16_END:\n");
    asm.push_str("    LEAS 4,S\n");
    asm.push_str("    RTS\n\n");
    
    // DIV16: Divide X / D -> D (quotient)
    asm.push_str("DIV16:\n");
    asm.push_str("    ; Divide 16-bit X / D -> D\n");
    asm.push_str("    ; Simple implementation\n");
    asm.push_str("    PSHS X,D\n");
    asm.push_str("    LDD #0         ; Quotient\n");
    asm.push_str(".DIV16_LOOP:\n");
    asm.push_str("    PSHS D         ; Save quotient\n");
    asm.push_str("    LDD 4,S        ; Load dividend (after PSHS D)\n");
    asm.push_str("    CMPD 2,S       ; Compare with divisor (after PSHS D)\n");
    asm.push_str("    PULS D         ; Restore quotient\n");
    asm.push_str("    BLT .DIV16_END\n");
    asm.push_str("    ADDD #1        ; Increment quotient\n");
    asm.push_str("    LDX 2,S\n");
    asm.push_str("    PSHS D\n");
    asm.push_str("    LDD 2,S        ; Divisor\n");
    asm.push_str("    LEAX D,X       ; Subtract divisor\n");
    asm.push_str("    STX 4,S\n");
    asm.push_str("    PULS D\n");
    asm.push_str("    BRA .DIV16_LOOP\n");
    asm.push_str(".DIV16_END:\n");
    asm.push_str("    LEAS 4,S\n");
    asm.push_str("    RTS\n\n");
    
    // MOD16: Modulo X % D -> D (remainder)
    asm.push_str("MOD16:\n");
    asm.push_str("    ; Modulo 16-bit X % D -> D\n");
    asm.push_str("    PSHS X,D\n");
    asm.push_str(".MOD16_LOOP:\n");
    asm.push_str("    PSHS D         ; Save D\n");
    asm.push_str("    LDD 4,S        ; Load dividend (after PSHS D)\n");
    asm.push_str("    CMPD 2,S       ; Compare with divisor (after PSHS D)\n");
    asm.push_str("    PULS D         ; Restore D\n");
    asm.push_str("    BLT .MOD16_END\n");
    asm.push_str("    LDX 2,S\n");
    asm.push_str("    LDD ,S\n");
    asm.push_str("    LEAX D,X\n");
    asm.push_str("    STX 2,S\n");
    asm.push_str("    BRA .MOD16_LOOP\n");
    asm.push_str(".MOD16_END:\n");
    asm.push_str("    LDD 2,S        ; Remainder\n");
    asm.push_str("    LEAS 4,S\n");
    asm.push_str("    RTS\n\n");
    
    // J1X_BUILTIN: Joystick 1 X axis (-1, 0, +1)
    asm.push_str("J1X_BUILTIN:\n");
    asm.push_str("    ; Read J1_X from $CF00 and return -1/0/+1\n");
    asm.push_str("    LDB $CF00      ; Joy_1_X (unsigned byte 0-255)\n");
    asm.push_str("    CMPB #108      ; Compare with lower threshold\n");
    asm.push_str("    BLO .J1X_LEFT  ; Branch if <108 (left)\n");
    asm.push_str("    CMPB #148      ; Compare with upper threshold\n");
    asm.push_str("    BHI .J1X_RIGHT ; Branch if >148 (right)\n");
    asm.push_str("    ; Center (108-148)\n");
    asm.push_str("    LDD #0\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J1X_LEFT:\n");
    asm.push_str("    LDD #-1\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J1X_RIGHT:\n");
    asm.push_str("    LDD #1\n");
    asm.push_str("    RTS\n\n");
    
    // J1Y_BUILTIN: Joystick 1 Y axis (-1, 0, +1)
    asm.push_str("J1Y_BUILTIN:\n");
    asm.push_str("    ; Read J1_Y from $CF01 and return -1/0/+1\n");
    asm.push_str("    LDB $CF01      ; Joy_1_Y (unsigned byte 0-255)\n");
    asm.push_str("    CMPB #108      ; Compare with lower threshold\n");
    asm.push_str("    BLO .J1Y_DOWN  ; Branch if <108 (down)\n");
    asm.push_str("    CMPB #148      ; Compare with upper threshold\n");
    asm.push_str("    BHI .J1Y_UP    ; Branch if >148 (up)\n");
    asm.push_str("    ; Center (108-148)\n");
    asm.push_str("    LDD #0\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J1Y_DOWN:\n");
    asm.push_str("    LDD #-1\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J1Y_UP:\n");
    asm.push_str("    LDD #1\n");
    asm.push_str("    RTS\n\n");
    
    // J2X_BUILTIN: Joystick 2 X axis (-1, 0, +1)
    asm.push_str("J2X_BUILTIN:\n");
    asm.push_str("    ; Read J2_X from $CF02 and return -1/0/+1\n");
    asm.push_str("    LDB $CF02      ; Joy_2_X (unsigned byte 0-255)\n");
    asm.push_str("    CMPB #108      ; Compare with lower threshold\n");
    asm.push_str("    BLO .J2X_LEFT  ; Branch if <108 (left)\n");
    asm.push_str("    CMPB #148      ; Compare with upper threshold\n");
    asm.push_str("    BHI .J2X_RIGHT ; Branch if >148 (right)\n");
    asm.push_str("    ; Center (108-148)\n");
    asm.push_str("    LDD #0\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J2X_LEFT:\n");
    asm.push_str("    LDD #-1\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J2X_RIGHT:\n");
    asm.push_str("    LDD #1\n");
    asm.push_str("    RTS\n\n");
    
    // J2Y_BUILTIN: Joystick 2 Y axis (-1, 0, +1)
    asm.push_str("J2Y_BUILTIN:\n");
    asm.push_str("    ; Read J2_Y from $CF03 and return -1/0/+1\n");
    asm.push_str("    LDB $CF03      ; Joy_2_Y (unsigned byte 0-255)\n");
    asm.push_str("    CMPB #108      ; Compare with lower threshold\n");
    asm.push_str("    BLO .J2Y_DOWN  ; Branch if <108 (down)\n");
    asm.push_str("    CMPB #148      ; Compare with upper threshold\n");
    asm.push_str("    BHI .J2Y_UP    ; Branch if >148 (up)\n");
    asm.push_str("    ; Center (108-148)\n");
    asm.push_str("    LDD #0\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J2Y_DOWN:\n");
    asm.push_str("    LDD #-1\n");
    asm.push_str("    RTS\n");
    asm.push_str(".J2Y_UP:\n");
    asm.push_str("    LDD #1\n");
    asm.push_str("    RTS\n\n");
    
    eprintln!("[DEBUG HELPERS] ASM length after MOD16: {}", asm.len());
    
    Ok(asm)
}
