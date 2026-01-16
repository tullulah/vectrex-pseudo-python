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
    
    // SQRT_HELPER: Square root (Newton-Raphson with DIV16)
    asm.push_str("SQRT_HELPER:\n");
    asm.push_str("    ; Input: D = x, Output: D = sqrt(x)\n");
    asm.push_str("    ; Newton-Raphson: guess_new = (guess + x/guess) / 2\n");
    asm.push_str("    ; Iterate 4 times for convergence\n");
    asm.push_str("    \n");
    asm.push_str("    ; Handle edge cases\n");
    asm.push_str("    CMPD #0\n");
    asm.push_str("    BEQ .SQRT_DONE  ; sqrt(0) = 0\n");
    asm.push_str("    CMPD #1\n");
    asm.push_str("    BEQ .SQRT_DONE  ; sqrt(1) = 1\n");
    asm.push_str("    \n");
    asm.push_str("    STD TMPPTR      ; Save x\n");
    asm.push_str("    ; Initial guess = (x + 1) / 2\n");
    asm.push_str("    ADDD #1\n");
    asm.push_str("    ASRA            ; Divide by 2\n");
    asm.push_str("    RORB\n");
    asm.push_str("    STD TMPPTR2     ; guess\n");
    asm.push_str("    \n");
    asm.push_str("    ; Iterate 4 times\n");
    asm.push_str("    LDB #4\n");
    asm.push_str("    STB RESULT+1    ; Counter\n");
    asm.push_str(".SQRT_LOOP:\n");
    asm.push_str("    ; Calculate x/guess using DIV16\n");
    asm.push_str("    LDX TMPPTR      ; X = x (dividend)\n");
    asm.push_str("    LDD TMPPTR2     ; D = guess (divisor)\n");
    asm.push_str("    JSR DIV16       ; D = x/guess\n");
    asm.push_str("    \n");
    asm.push_str("    ; guess_new = (guess + x/guess) / 2\n");
    asm.push_str("    ADDD TMPPTR2    ; D = guess + x/guess\n");
    asm.push_str("    ASRA            ; Divide by 2\n");
    asm.push_str("    RORB\n");
    asm.push_str("    STD TMPPTR2     ; Update guess\n");
    asm.push_str("    \n");
    asm.push_str("    DEC RESULT+1    ; Decrement counter\n");
    asm.push_str("    BNE .SQRT_LOOP\n");
    asm.push_str("    \n");
    asm.push_str("    LDD TMPPTR2     ; Return final guess\n");
    asm.push_str(".SQRT_DONE:\n");
    asm.push_str("    RTS\n\n");
    
    // POW_HELPER: Power (base ^ exp)
    asm.push_str("POW_HELPER:\n");
    asm.push_str("    ; Input: TMPPTR = base, TMPPTR2 = exp, Output: D = result\n");
    asm.push_str("    LDD #1         ; result = 1\n");
    asm.push_str("    STD RESULT\n");
    asm.push_str(".POW_LOOP:\n");
    asm.push_str("    LDD TMPPTR2    ; Load exponent\n");
    asm.push_str("    BEQ .POW_DONE  ; If exp == 0, done\n");
    asm.push_str("    SUBD #1        ; exp--\n");
    asm.push_str("    STD TMPPTR2\n");
    asm.push_str("    ; result = result * base (simplified: assumes small values)\n");
    asm.push_str("    LDD RESULT\n");
    asm.push_str("    LDX TMPPTR     ; Load base\n");
    asm.push_str("    ; Simple multiplication loop\n");
    asm.push_str("    PSHS D\n");
    asm.push_str("    LDD #0\n");
    asm.push_str(".POW_MUL_LOOP:\n");
    asm.push_str("    LEAX -1,X\n");
    asm.push_str("    BEQ .POW_MUL_DONE\n");
    asm.push_str("    ADDD ,S\n");
    asm.push_str("    BRA .POW_MUL_LOOP\n");
    asm.push_str(".POW_MUL_DONE:\n");
    asm.push_str("    LEAS 2,S\n");
    asm.push_str("    STD RESULT\n");
    asm.push_str("    BRA .POW_LOOP\n");
    asm.push_str(".POW_DONE:\n");
    asm.push_str("    LDD RESULT\n");
    asm.push_str("    RTS\n\n");
    
    // ATAN2_HELPER: Arctangent (y, x)
    asm.push_str("ATAN2_HELPER:\n");
    asm.push_str("    ; Input: TMPPTR = y, TMPPTR2 = x, Output: D = angle (0-127)\n");
    asm.push_str("    ; Simplified: return approximate angle based on quadrant\n");
    asm.push_str("    LDD TMPPTR2    ; Load x\n");
    asm.push_str("    BEQ .ATAN2_X_ZERO\n");
    asm.push_str("    ; TODO: Full CORDIC implementation\n");
    asm.push_str("    ; For now return 0 (placeholder)\n");
    asm.push_str("    LDD #0\n");
    asm.push_str("    RTS\n");
    asm.push_str(".ATAN2_X_ZERO:\n");
    asm.push_str("    LDD TMPPTR     ; Load y\n");
    asm.push_str("    BPL .ATAN2_Y_POS\n");
    asm.push_str("    LDD #96        ; -90 degrees (3/4 of 128)\n");
    asm.push_str("    RTS\n");
    asm.push_str(".ATAN2_Y_POS:\n");
    asm.push_str("    LDD #32        ; +90 degrees (1/4 of 128)\n");
    asm.push_str("    RTS\n\n");
    
    // RAND_HELPER: Random number generator (Linear Congruential)
    asm.push_str("RAND_HELPER:\n");
    asm.push_str("    ; LCG: seed = (seed * 1103515245 + 12345) & 0x7FFF\n");
    asm.push_str("    ; Simplified for 6809: seed = (seed * 25 + 13) & 0x7FFF\n");
    asm.push_str("    LDD RAND_SEED\n");
    asm.push_str("    LDX #25\n");
    asm.push_str("    ; Multiply by 25 (simple loop)\n");
    asm.push_str("    PSHS D\n");
    asm.push_str("    LDD #0\n");
    asm.push_str(".RAND_MUL_LOOP:\n");
    asm.push_str("    LEAX -1,X\n");
    asm.push_str("    BEQ .RAND_MUL_DONE\n");
    asm.push_str("    ADDD ,S\n");
    asm.push_str("    BRA .RAND_MUL_LOOP\n");
    asm.push_str(".RAND_MUL_DONE:\n");
    asm.push_str("    LEAS 2,S\n");
    asm.push_str("    ADDD #13       ; Add constant\n");
    asm.push_str("    ANDA #$7F      ; Mask to positive 15-bit\n");
    asm.push_str("    STD RAND_SEED  ; Update seed\n");
    asm.push_str("    RTS\n\n");
    
    // RAND_RANGE_HELPER: Random in range [min, max]
    asm.push_str("RAND_RANGE_HELPER:\n");
    asm.push_str("    ; Input: TMPPTR = min, TMPPTR2 = max\n");
    asm.push_str("    JSR RAND_HELPER\n");
    asm.push_str("    ; D = rand()\n");
    asm.push_str("    ; range = max - min\n");
    asm.push_str("    PSHS D         ; Save random value\n");
    asm.push_str("    LDD TMPPTR2    ; max\n");
    asm.push_str("    SUBD TMPPTR    ; max - min\n");
    asm.push_str("    STD TMPPTR2    ; Save range\n");
    asm.push_str("    ; result = (rand % range) + min\n");
    asm.push_str("    PULS D         ; Restore random\n");
    asm.push_str("    ; Simple modulo: D = D % TMPPTR2 (TODO: proper modulo)\n");
    asm.push_str("    ; For now: mask to range (works for power-of-2 ranges)\n");
    asm.push_str("    ; result = min + (rand & (range-1))\n");
    asm.push_str("    ADDD TMPPTR    ; Add min\n");
    asm.push_str("    RTS\n\n");
    
    // DRAW_CIRCLE_RUNTIME: Draw circle with runtime parameters
    asm.push_str("DRAW_CIRCLE_RUNTIME:\n");
    asm.push_str("    ; Input: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY\n");
    asm.push_str("    ; Draw 16-sided polygon approximation\n");
    asm.push_str("    \n");
    asm.push_str("    ; Read parameters BEFORE DP change\n");
    asm.push_str("    LDB DRAW_CIRCLE_INTENSITY\n");
    asm.push_str("    PSHS B              ; Save intensity\n");
    asm.push_str("    LDB DRAW_CIRCLE_DIAM\n");
    asm.push_str("    SEX                 ; Sign-extend to 16-bit\n");
    asm.push_str("    LSRA                ; Divide by 2 = radius\n");
    asm.push_str("    RORB\n");
    asm.push_str("    STD DRAW_CIRCLE_TEMP   ; Save radius\n");
    asm.push_str("    LDB DRAW_CIRCLE_XC\n");
    asm.push_str("    SEX\n");
    asm.push_str("    STD DRAW_CIRCLE_TEMP+2 ; Save xc\n");
    asm.push_str("    LDB DRAW_CIRCLE_YC\n");
    asm.push_str("    SEX\n");
    asm.push_str("    STD DRAW_CIRCLE_TEMP+4 ; Save yc\n");
    asm.push_str("    \n");
    asm.push_str("    ; Setup BIOS\n");
    asm.push_str("    LDA #$D0\n");
    asm.push_str("    TFR A,DP\n");
    asm.push_str("    JSR Reset0Ref\n");
    asm.push_str("    \n");
    asm.push_str("    ; Set intensity\n");
    asm.push_str("    PULS A\n");
    asm.push_str("    CMPA #$5F\n");
    asm.push_str("    BEQ .DCR_INT_5F\n");
    asm.push_str("    JSR Intensity_a\n");
    asm.push_str("    BRA .DCR_AFTER_INT\n");
    asm.push_str(".DCR_INT_5F:\n");
    asm.push_str("    JSR Intensity_5F\n");
    asm.push_str(".DCR_AFTER_INT:\n");
    asm.push_str("    \n");
    asm.push_str("    ; TODO: Generate 16 vertices with trig (simplified version uses 8-gon)\n");
    asm.push_str("    ; For now, draw octagon approximation\n");
    asm.push_str("    ; Move to start position (xc + radius, yc)\n");
    asm.push_str("    LDD DRAW_CIRCLE_TEMP   ; radius\n");
    asm.push_str("    ADDD DRAW_CIRCLE_TEMP+2 ; xc + radius\n");
    asm.push_str("    TFR B,B\n");
    asm.push_str("    PSHS B              ; Save X\n");
    asm.push_str("    LDD DRAW_CIRCLE_TEMP+4 ; yc\n");
    asm.push_str("    TFR B,A             ; Y to A\n");
    asm.push_str("    PULS B              ; X to B\n");
    asm.push_str("    JSR Moveto_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; Simple octagon: 8 segments with fixed deltas\n");
    asm.push_str("    ; This is simplified - full implementation would use SIN_TABLE\n");
    asm.push_str("    LDD DRAW_CIRCLE_TEMP   ; radius\n");
    asm.push_str("    TFR B,A             ; Use low byte only\n");
    asm.push_str("    \n");
    asm.push_str("    ; Segment 1: move (0, -r)\n");
    asm.push_str("    CLR Vec_Misc_Count\n");
    asm.push_str("    NEGA                ; -radius\n");
    asm.push_str("    LDB #0\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; ... (simplified - full version would iterate all 16 segments)\n");
    asm.push_str("    ; For now return (minimal octagon)\n");
    asm.push_str("    RTS\n\n");
    
    // DRAW_RECT_RUNTIME: Draw rectangle with runtime parameters
    asm.push_str("DRAW_RECT_RUNTIME:\n");
    asm.push_str("    ; Input: DRAW_RECT_X, DRAW_RECT_Y, DRAW_RECT_WIDTH, DRAW_RECT_HEIGHT, DRAW_RECT_INTENSITY\n");
    asm.push_str("    \n");
    asm.push_str("    ; Read parameters BEFORE DP change\n");
    asm.push_str("    LDB DRAW_RECT_INTENSITY\n");
    asm.push_str("    PSHS B              ; Save intensity\n");
    asm.push_str("    LDB DRAW_RECT_WIDTH\n");
    asm.push_str("    PSHS B              ; Save width\n");
    asm.push_str("    LDB DRAW_RECT_HEIGHT\n");
    asm.push_str("    PSHS B              ; Save height\n");
    asm.push_str("    LDB DRAW_RECT_X\n");
    asm.push_str("    PSHS B              ; Save x\n");
    asm.push_str("    LDB DRAW_RECT_Y\n");
    asm.push_str("    PSHS B              ; Save y\n");
    asm.push_str("    \n");
    asm.push_str("    ; Setup BIOS\n");
    asm.push_str("    LDA #$D0\n");
    asm.push_str("    TFR A,DP\n");
    asm.push_str("    JSR Reset0Ref\n");
    asm.push_str("    \n");
    asm.push_str("    ; Set intensity\n");
    asm.push_str("    LDA 4,S             ; Get intensity from stack\n");
    asm.push_str("    CMPA #$5F\n");
    asm.push_str("    BEQ .DRR_INT_5F\n");
    asm.push_str("    JSR Intensity_a\n");
    asm.push_str("    BRA .DRR_AFTER_INT\n");
    asm.push_str(".DRR_INT_5F:\n");
    asm.push_str("    JSR Intensity_5F\n");
    asm.push_str(".DRR_AFTER_INT:\n");
    asm.push_str("    \n");
    asm.push_str("    ; Move to start position (x, y)\n");
    asm.push_str("    LDA ,S              ; y from stack\n");
    asm.push_str("    LDB 1,S             ; x from stack\n");
    asm.push_str("    JSR Moveto_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; Draw 4 sides\n");
    asm.push_str("    ; Side 1: Right (width, 0)\n");
    asm.push_str("    CLR Vec_Misc_Count\n");
    asm.push_str("    LDA #0\n");
    asm.push_str("    LDB 3,S             ; width\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; Side 2: Down (0, height)\n");
    asm.push_str("    CLR Vec_Misc_Count\n");
    asm.push_str("    LDA 2,S             ; height\n");
    asm.push_str("    LDB #0\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; Side 3: Left (-width, 0)\n");
    asm.push_str("    CLR Vec_Misc_Count\n");
    asm.push_str("    LDA #0\n");
    asm.push_str("    LDB 3,S             ; width\n");
    asm.push_str("    NEGB                ; -width\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("    ; Side 4: Up (0, -height)\n");
    asm.push_str("    CLR Vec_Misc_Count\n");
    asm.push_str("    LDA 2,S             ; height\n");
    asm.push_str("    NEGA                ; -height\n");
    asm.push_str("    LDB #0\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("    LEAS 5,S            ; Clean stack\n");
    asm.push_str("    RTS\n\n");
    
    // SHOW_LEVEL_RUNTIME - Render current level
    asm.push_str("; === SHOW_LEVEL_RUNTIME - Render level tiles ===\n");
    asm.push_str("SHOW_LEVEL_RUNTIME:\n");
    asm.push_str("    LDA #8\n");
    asm.push_str("    STA LEVEL_TILE_SIZE    ; 8x8 pixel tiles\n");
    asm.push_str("    \n");
    asm.push_str("    ; Calculate starting position (top-left of screen)\n");
    asm.push_str("    LDA LEVEL_HEIGHT       ; Y loop counter\n");
    asm.push_str("    STA TMPPTR+1           ; Store Y counter\n");
    asm.push_str("    \n");
    asm.push_str("SLR_Y_LOOP:\n");
    asm.push_str("    LDA LEVEL_WIDTH        ; X loop counter\n");
    asm.push_str("    STA TMPPTR             ; Store X counter\n");
    asm.push_str("    \n");
    asm.push_str("SLR_X_LOOP:\n");
    asm.push_str("    ; Calculate tile offset\n");
    asm.push_str("    LDA LEVEL_HEIGHT\n");
    asm.push_str("    SUBA TMPPTR+1          ; Y index\n");
    asm.push_str("    LDB LEVEL_WIDTH\n");
    asm.push_str("    MUL                    ; D = y * width\n");
    asm.push_str("    PSHS D\n");
    asm.push_str("    LDA LEVEL_WIDTH\n");
    asm.push_str("    SUBA TMPPTR            ; X index\n");
    asm.push_str("    CLRB\n");
    asm.push_str("    TFR A,B\n");
    asm.push_str("    ADDD ,S++              ; D = y*width + x\n");
    asm.push_str("    ADDD #2                ; Skip header\n");
    asm.push_str("    ADDD LEVEL_PTR\n");
    asm.push_str("    TFR D,X\n");
    asm.push_str("    \n");
    asm.push_str("    ; Load tile value\n");
    asm.push_str("    LDA ,X\n");
    asm.push_str("    BEQ SLR_SKIP_TILE      ; Skip if empty\n");
    asm.push_str("    \n");
    asm.push_str("    ; Draw tile rectangle\n");
    asm.push_str("    LDA LEVEL_WIDTH\n");
    asm.push_str("    SUBA TMPPTR\n");
    asm.push_str("    LDB LEVEL_TILE_SIZE\n");
    asm.push_str("    MUL\n");
    asm.push_str("    SUBD #64\n");
    asm.push_str("    TFR B,A\n");
    asm.push_str("    PSHS A\n");
    asm.push_str("    \n");
    asm.push_str("    LDA LEVEL_HEIGHT\n");
    asm.push_str("    SUBA TMPPTR+1\n");
    asm.push_str("    LDB LEVEL_TILE_SIZE\n");
    asm.push_str("    MUL\n");
    asm.push_str("    SUBD #64\n");
    asm.push_str("    TFR B,A\n");
    asm.push_str("    PULS B\n");
    asm.push_str("    \n");
    asm.push_str("    JSR Intensity_5F\n");
    asm.push_str("    JSR Moveto_d_7F\n");
    asm.push_str("    LDA LEVEL_TILE_SIZE\n");
    asm.push_str("    CLRB\n");
    asm.push_str("    JSR Draw_Line_d\n");
    asm.push_str("    \n");
    asm.push_str("SLR_SKIP_TILE:\n");
    asm.push_str("    DEC TMPPTR\n");
    asm.push_str("    BNE SLR_X_LOOP\n");
    asm.push_str("    \n");
    asm.push_str("    DEC TMPPTR+1\n");
    asm.push_str("    BNE SLR_Y_LOOP\n");
    asm.push_str("    \n");
    asm.push_str("    RTS\n\n");
    
    // FADE_IN_RUNTIME - Gradual intensity increase
    asm.push_str("; === FADE_IN_RUNTIME - Gradual intensity increase ===\n");
    asm.push_str("FADE_IN_RUNTIME:\n");
    asm.push_str("    LDA CURRENT_INTENSITY\n");
    asm.push_str("    PSHS A                 ; Save target intensity\n");
    asm.push_str("    LDA #8                 ; 8 steps\n");
    asm.push_str("    STA TMPPTR\n");
    asm.push_str("FADE_IN_LOOP:\n");
    asm.push_str("    LDA ,S                 ; Get target\n");
    asm.push_str("    LDB TMPPTR             ; Steps remaining\n");
    asm.push_str("    MUL                    ; D = target * steps / 8\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB                   ; Divide by 8\n");
    asm.push_str("    TFR B,A\n");
    asm.push_str("    JSR Intensity_a\n");
    asm.push_str("    JSR Wait_Recal\n");
    asm.push_str("    DEC TMPPTR\n");
    asm.push_str("    BNE FADE_IN_LOOP\n");
    asm.push_str("    PULS A                 ; Clean stack\n");
    asm.push_str("    RTS\n\n");
    
    // FADE_OUT_RUNTIME - Gradual intensity decrease
    asm.push_str("; === FADE_OUT_RUNTIME - Gradual intensity decrease ===\n");
    asm.push_str("FADE_OUT_RUNTIME:\n");
    asm.push_str("    LDA CURRENT_INTENSITY\n");
    asm.push_str("    PSHS A\n");
    asm.push_str("    LDA #8\n");
    asm.push_str("    STA TMPPTR\n");
    asm.push_str("FADE_OUT_LOOP:\n");
    asm.push_str("    LDA ,S\n");
    asm.push_str("    LDB #8\n");
    asm.push_str("    SUBB TMPPTR            ; 8 - steps_remaining\n");
    asm.push_str("    MUL\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB\n");
    asm.push_str("    LSRA\n");
    asm.push_str("    RORB\n");
    asm.push_str("    TFR B,A\n");
    asm.push_str("    JSR Intensity_a\n");
    asm.push_str("    JSR Wait_Recal\n");
    asm.push_str("    DEC TMPPTR\n");
    asm.push_str("    BNE FADE_OUT_LOOP\n");
    asm.push_str("    PULS A\n");
    asm.push_str("    RTS\n\n");
    
    eprintln!("[DEBUG HELPERS] ASM length after MOD16: {}", asm.len());
    
    Ok(asm)
}
