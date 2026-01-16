//! Builtin Functions for M6809
//!
//! Essential builtins:
//! - PRINT_TEXT: Print text at position
//! - DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
//! - WAIT_RECAL: Wait for screen refresh
//! - SET_INTENSITY: Set drawing intensity

use vpy_parser::{Expr, Module};
use super::expressions;

/// Check if function is a builtin and emit code
pub fn emit_builtin(
    name: &str,
    args: &[Expr],
    out: &mut String,
) -> bool {
    let up = name.to_ascii_uppercase();
    
    match up.as_str() {
        // ===== Core Display Builtins =====
        "WAIT_RECAL" => {
            emit_wait_recal(out);
            true
        }
        "SET_INTENSITY" => {
            emit_set_intensity(args, out);
            true
        }
        "PRINT_TEXT" => {
            emit_print_text(args, out);
            true
        }
        "DRAW_LINE" => {
            emit_draw_line(args, out);
            true
        }
        
        // ===== Joystick Input =====
        "J1_X" => {
            out.push_str("    JSR J1X_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J1_Y" => {
            out.push_str("    JSR J1Y_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "UPDATE_BUTTONS" => {
            out.push_str("    JSR $F1AA     ; DP_to_D0\n");
            out.push_str("    JSR $F1BA     ; Read_Btns\n");
            out.push_str("    JSR $F1AF     ; DP_to_C8\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J1_BUTTON_1" => {
            out.push_str("    LDA $C811      ; Vec_Button_1_1 (transition bits)\n");
            out.push_str("    ANDA #$01      ; Test bit 0\n");
            out.push_str("    BEQ .J1B1_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J1B1_END\n");
            out.push_str(".J1B1_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J1B1_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J1_BUTTON_2" => {
            out.push_str("    LDA $C811      ; Vec_Button_1_1 (transition bits)\n");
            out.push_str("    ANDA #$02      ; Test bit 1\n");
            out.push_str("    BEQ .J1B2_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J1B2_END\n");
            out.push_str(".J1B2_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J1B2_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J1_BUTTON_3" => {
            out.push_str("    LDA $C811      ; Vec_Button_1_1 (transition bits)\n");
            out.push_str("    ANDA #$04      ; Test bit 2\n");
            out.push_str("    BEQ .J1B3_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J1B3_END\n");
            out.push_str(".J1B3_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J1B3_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J1_BUTTON_4" => {
            out.push_str("    LDA $C811      ; Vec_Button_1_1 (transition bits)\n");
            out.push_str("    ANDA #$08      ; Test bit 3\n");
            out.push_str("    BEQ .J1B4_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J1B4_END\n");
            out.push_str(".J1B4_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J1B4_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        
        // ===== Joystick 2 Input (Player 2) =====
        "J2_X" => {
            out.push_str("    JSR J2X_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_Y" => {
            out.push_str("    JSR J2Y_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_1" => {
            out.push_str("    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)\n");
            out.push_str("    ANDA #$01      ; Test bit 0\n");
            out.push_str("    BEQ .J2B1_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J2B1_END\n");
            out.push_str(".J2B1_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J2B1_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_2" => {
            out.push_str("    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)\n");
            out.push_str("    ANDA #$02      ; Test bit 1\n");
            out.push_str("    BEQ .J2B2_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J2B2_END\n");
            out.push_str(".J2B2_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J2B2_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_3" => {
            out.push_str("    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)\n");
            out.push_str("    ANDA #$04      ; Test bit 2\n");
            out.push_str("    BEQ .J2B3_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J2B3_END\n");
            out.push_str(".J2B3_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J2B3_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_4" => {
            out.push_str("    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)\n");
            out.push_str("    ANDA #$08      ; Test bit 3\n");
            out.push_str("    BEQ .J2B4_OFF\n");
            out.push_str("    LDD #1\n");
            out.push_str("    BRA .J2B4_END\n");
            out.push_str(".J2B4_OFF:\n");
            out.push_str("    LDD #0\n");
            out.push_str(".J2B4_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_ANALOG_X" => {
            out.push_str("    ; J2_ANALOG_X: Read raw Player 2 X axis (0-255)\n");
            out.push_str("    LDB $CF02      ; Joy_2_X (unsigned byte)\n");
            out.push_str("    CLRA           ; Zero extend to 16-bit\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_ANALOG_Y" => {
            out.push_str("    ; J2_ANALOG_Y: Read raw Player 2 Y axis (0-255)\n");
            out.push_str("    LDB $CF03      ; Joy_2_Y (unsigned byte)\n");
            out.push_str("    CLRA           ; Zero extend to 16-bit\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_DIGITAL_X" => {
            out.push_str("    ; J2_DIGITAL_X: Player 2 X axis as -1/0/+1\n");
            out.push_str("    JSR J2X_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_DIGITAL_Y" => {
            out.push_str("    ; J2_DIGITAL_Y: Player 2 Y axis as -1/0/+1\n");
            out.push_str("    JSR J2Y_BUILTIN\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_UP" => {
            out.push_str("    ; J2_BUTTON_UP: Player 2 D-pad UP\n");
            out.push_str("    LDB $CF03      ; Joy_2_Y\n");
            out.push_str("    CMPB #149      ; Threshold for UP (>148)\n");
            out.push_str("    BHI .J2UP_ON\n");
            out.push_str("    LDD #0\n");
            out.push_str("    BRA .J2UP_END\n");
            out.push_str(".J2UP_ON:\n");
            out.push_str("    LDD #1\n");
            out.push_str(".J2UP_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_DOWN" => {
            out.push_str("    ; J2_BUTTON_DOWN: Player 2 D-pad DOWN\n");
            out.push_str("    LDB $CF03      ; Joy_2_Y\n");
            out.push_str("    CMPB #108      ; Threshold for DOWN (<108)\n");
            out.push_str("    BLO .J2DN_ON\n");
            out.push_str("    LDD #0\n");
            out.push_str("    BRA .J2DN_END\n");
            out.push_str(".J2DN_ON:\n");
            out.push_str("    LDD #1\n");
            out.push_str(".J2DN_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_LEFT" => {
            out.push_str("    ; J2_BUTTON_LEFT: Player 2 D-pad LEFT\n");
            out.push_str("    LDB $CF02      ; Joy_2_X\n");
            out.push_str("    CMPB #108      ; Threshold for LEFT (<108)\n");
            out.push_str("    BLO .J2LFT_ON\n");
            out.push_str("    LDD #0\n");
            out.push_str("    BRA .J2LFT_END\n");
            out.push_str(".J2LFT_ON:\n");
            out.push_str("    LDD #1\n");
            out.push_str(".J2LFT_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "J2_BUTTON_RIGHT" => {
            out.push_str("    ; J2_BUTTON_RIGHT: Player 2 D-pad RIGHT\n");
            out.push_str("    LDB $CF02      ; Joy_2_X\n");
            out.push_str("    CMPB #149      ; Threshold for RIGHT (>148)\n");
            out.push_str("    BHI .J2RGT_ON\n");
            out.push_str("    LDD #0\n");
            out.push_str("    BRA .J2RGT_END\n");
            out.push_str(".J2RGT_ON:\n");
            out.push_str("    LDD #1\n");
            out.push_str(".J2RGT_END:\n");
            out.push_str("    STD RESULT\n");
            true
        }
        
        // ===== Audio/Music =====
        "PLAY_MUSIC" => {
            out.push_str("    ; PLAY_MUSIC: Play music from asset\n");
            out.push_str("    JSR PLAY_MUSIC_RUNTIME\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "AUDIO_UPDATE" => {
            out.push_str("    ; AUDIO_UPDATE: Update audio/music\n");
            out.push_str("    JSR AUDIO_UPDATE\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "MUSIC_UPDATE" => {
            out.push_str("    ; MUSIC_UPDATE: Update music playback\n");
            out.push_str("    JSR MUSIC_UPDATE\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "STOP_MUSIC" => {
            out.push_str("    ; STOP_MUSIC: Stop music playback\n");
            out.push_str("    JSR STOP_MUSIC_RUNTIME\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "PLAY_SFX" => {
            out.push_str("    ; PLAY_SFX: Play sound effect\n");
            out.push_str("    JSR PLAY_SFX_RUNTIME\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        
        // ===== Vector Assets =====
        "DRAW_VECTOR" => {
            emit_draw_vector(args, out);
            true
        }
        "DRAW_VECTOR_EX" => {
            emit_draw_vector_ex(args, out);
            true
        }
        
        
        // ===== Math Functions (TODO: Complete implementation) =====
        "ABS" | "MATH_ABS" => {
            out.push_str("    ; TODO: ABS implementation\n");
            out.push_str("    LDD RESULT\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "MIN" | "MATH_MIN" => {
            out.push_str("    ; TODO: MIN implementation\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "MAX" | "MATH_MAX" => {
            out.push_str("    ; TODO: MAX implementation\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "LEN" => {
            out.push_str("    ; LEN: Get array/string length\n");
            expressions::emit_simple_expr(&args[0], out);
            out.push_str("    ; TODO: LEN implementation\n");
            out.push_str("    STD RESULT\n");
            true
        }
        
        // ===== Drawing Functions =====
        "DRAW_CIRCLE" => {
            out.push_str("    ; DRAW_CIRCLE: Draw circle\n");
            out.push_str("    JSR DRAW_CIRCLE_RUNTIME\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        "DRAW_POLYGON" => {
            out.push_str("    ; DRAW_POLYGON: Draw polygon\n");
            out.push_str("    JSR DRAW_POLYGON_RUNTIME\n");
            out.push_str("    LDD #0\n");
            out.push_str("    STD RESULT\n");
            true
        }
        
        // ===== Default: Not a builtin =====
        _ => false,
    }
}

fn emit_wait_recal(out: &mut String) {
    out.push_str("    ; WAIT_RECAL: Wait for screen refresh\n");
    out.push_str("    JSR Wait_Recal\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

fn emit_set_intensity(args: &[Expr], out: &mut String) {
    if args.len() != 1 {
        out.push_str("    ; ERROR: SET_INTENSITY requires 1 argument\n");
        return;
    }
    
    out.push_str("    ; SET_INTENSITY: Set drawing intensity\n");
    
    // Evaluate intensity argument
    expressions::emit_simple_expr(&args[0], out);
    
    // Load result into A and call BIOS
    out.push_str("    LDA RESULT+1    ; Load intensity (8-bit)\n");
    out.push_str("    JSR Intensity_a\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

fn hash_string(s: &str) -> u64 {
    let mut hash: u64 = 0;
    for b in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u64);
    }
    hash
}

fn emit_print_text(args: &[Expr], out: &mut String) {
    if args.len() != 3 {
        out.push_str("    ; ERROR: PRINT_TEXT requires 3 arguments (x, y, text)\n");
        return;
    }
    
    out.push_str("    ; PRINT_TEXT: Print text at position\n");
    
    // Store all 3 arguments in VAR_ARG0, VAR_ARG1, VAR_ARG2 (like core implementation)
    // Arg 0: x coordinate
    expressions::emit_simple_expr(&args[0], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD VAR_ARG0\n");
    
    // Arg 1: y coordinate
    expressions::emit_simple_expr(&args[1], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD VAR_ARG1\n");
    
    // Arg 2: text string
    match &args[2] {
        Expr::StringLit(s) => {
            // Load pointer to string in helpers bank
            let str_label = format!("PRINT_TEXT_STR_{}", hash_string(s));
            out.push_str(&format!("    LDX #{}      ; Pointer to string in helpers bank\n", str_label));
            out.push_str("    STX VAR_ARG2\n");
        }
        _ => {
            // Variable or expression - evaluate to pointer
            expressions::emit_simple_expr(&args[2], out);
            out.push_str("    LDD RESULT\n");
            out.push_str("    STD VAR_ARG2\n");
        }
    }
    
    // Call the helper which reads x, y, string from VAR_ARG0-2
    out.push_str("    JSR VECTREX_PRINT_TEXT\n");
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Escape special characters in strings for FCC directive
fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\"\""),  // Double quotes to escape in FCC
            '\\' => result.push_str("\\\\"), // Escape backslash
            '\n' => result.push_str("\\n"),  // Newline
            '\r' => result.push_str("\\r"),  // Carriage return
            '\t' => result.push_str("\\t"),  // Tab
            _ => result.push(ch),
        }
    }
    result
}

fn emit_draw_line(args: &[Expr], out: &mut String) {
    if args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_LINE requires 5 arguments (x0, y0, x1, y1, intensity)\n");
        return;
    }
    
    out.push_str("    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)\n");
    
    // Store all arguments in TMPPTR area (RESULT+0 to RESULT+8)
    // Arg 0: x0
    expressions::emit_simple_expr(&args[0], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+0    ; x0\n");
    
    // Arg 1: y0
    expressions::emit_simple_expr(&args[1], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+2    ; y0\n");
    
    // Arg 2: x1
    expressions::emit_simple_expr(&args[2], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+4    ; x1\n");
    
    // Arg 3: y1
    expressions::emit_simple_expr(&args[3], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+6    ; y1\n");
    
    // Arg 4: intensity
    expressions::emit_simple_expr(&args[4], out);
    out.push_str("    LDD RESULT\n");
    out.push_str("    STD TMPPTR+8    ; intensity\n");
    
    // Call DRAW_LINE_WRAPPER which handles DP switching and segmentation
    out.push_str("    JSR DRAW_LINE_WRAPPER\n");
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

fn emit_draw_vector(args: &[Expr], out: &mut String) {
    if args.len() != 3 {
        out.push_str("    ; ERROR: DRAW_VECTOR requires 3 arguments (asset_name, x, y)\n");
        return;
    }
    
    out.push_str("    ; DRAW_VECTOR: Draw vector asset at position\n");
    
    // For buildtools, we generate a call to the asset label directly
    // The asset must exist in the ROM (checked during compilation)
    match &args[0] {
        Expr::StringLit(asset_name) => {
            let symbol = format!("_{}", asset_name.to_uppercase().replace("-", "_").replace(" ", "_"));
            
            out.push_str(&format!("    ; Asset: {}\n", asset_name));
            
            // Evaluate x position (arg 1) - save immediately to avoid overwrite
            expressions::emit_simple_expr(&args[1], out);
            out.push_str("    LDA RESULT+1  ; X position (low byte)\n");
            out.push_str("    STA TMPPTR    ; Save X to temporary storage\n");
            
            // Evaluate y position (arg 2)
            expressions::emit_simple_expr(&args[2], out);
            out.push_str("    LDA RESULT+1  ; Y position (low byte)\n");
            out.push_str("    STA TMPPTR+1  ; Save Y to temporary storage\n");
            
            // Restore X and Y from temporary storage and set positions
            out.push_str("    LDA TMPPTR    ; X position\n");
            out.push_str("    STA DRAW_VEC_X\n");
            out.push_str("    LDA TMPPTR+1  ; Y position\n");
            out.push_str("    STA DRAW_VEC_Y\n");
            
            // Clear mirror flags (DRAW_VECTOR uses no mirroring)
            out.push_str("    CLR MIRROR_X\n");
            out.push_str("    CLR MIRROR_Y\n");
            out.push_str("    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data\n");
            
            // Single DP switch for all paths (CRITICAL PATTERN FROM CORE)
            out.push_str("    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)\n");
            
            // Load vector data pointer and call drawing function
            // Note: In buildtools, we expect the vector to be compiled with multiple _PATH0, _PATH1, etc.
            // For now, we'll generate a call to the first path as a placeholder
            // The actual path count should be determined by the asset compilation phase
            out.push_str(&format!("    LDX #{}_PATH0  ; Load first path\n", symbol));
            out.push_str("    JSR Draw_Sync_List_At_With_Mirrors\n");
            
            // Restore DP (CRITICAL PATTERN FROM CORE)
            out.push_str("    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)\n");
            
            out.push_str("    LDD #0\n    STD RESULT\n");
        }
        _ => {
            out.push_str("    ; ERROR: DRAW_VECTOR first argument must be string literal\n");
            out.push_str("    LDD #0\n    STD RESULT\n");
        }
    }
}

fn emit_draw_vector_ex(args: &[Expr], out: &mut String) {
    if args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_VECTOR_EX requires 5 arguments (asset_name, x, y, mirror, intensity)\n");
        return;
    }
    
    out.push_str("    ; DRAW_VECTOR_EX: Draw vector asset with transformations\n");
    
    match &args[0] {
        Expr::StringLit(asset_name) => {
            let symbol = format!("_{}", asset_name.to_uppercase().replace("-", "_").replace(" ", "_"));
            
            out.push_str(&format!("    ; Asset: {} (with mirror + intensity)\n", asset_name));
            
            // Evaluate x position (arg 1)
            expressions::emit_simple_expr(&args[1], out);
            out.push_str("    LDA RESULT+1  ; X position (low byte)\n");
            out.push_str("    STA DRAW_VEC_X\n");
            
            // Evaluate y position (arg 2)
            expressions::emit_simple_expr(&args[2], out);
            out.push_str("    LDA RESULT+1  ; Y position (low byte)\n");
            out.push_str("    STA DRAW_VEC_Y\n");
            
            // Evaluate mirror flag (arg 3)
            expressions::emit_simple_expr(&args[3], out);
            out.push_str("    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)\n");
            
            // Decode mirror mode into separate MIRROR_X and MIRROR_Y flags
            out.push_str("    ; Decode mirror mode into separate flags:\n");
            out.push_str("    CLR MIRROR_X  ; Clear X flag\n");
            out.push_str("    CLR MIRROR_Y  ; Clear Y flag\n");
            out.push_str("    CMPB #1       ; Check if X-mirror (mode 1)\n");
            out.push_str("    BNE .DSVEX_CHK_Y\n");
            out.push_str("    LDA #1\n");
            out.push_str("    STA MIRROR_X\n");
            out.push_str(".DSVEX_CHK_Y:\n");
            out.push_str("    CMPB #2       ; Check if Y-mirror (mode 2)\n");
            out.push_str("    BNE .DSVEX_CHK_XY\n");
            out.push_str("    LDA #1\n");
            out.push_str("    STA MIRROR_Y\n");
            out.push_str(".DSVEX_CHK_XY:\n");
            out.push_str("    CMPB #3       ; Check if both-mirror (mode 3)\n");
            out.push_str("    BNE .DSVEX_CALL\n");
            out.push_str("    LDA #1\n");
            out.push_str("    STA MIRROR_X\n");
            out.push_str("    STA MIRROR_Y\n");
            out.push_str(".DSVEX_CALL:\n");
            
            // Evaluate and set intensity override (arg 4)
            out.push_str("    ; Set intensity override for drawing\n");
            expressions::emit_simple_expr(&args[4], out);
            out.push_str("    LDA RESULT+1  ; Intensity (0-127)\n");
            out.push_str("    STA DRAW_VEC_INTENSITY  ; Store intensity override\n");
            
            // Single DP switch for all paths (CRITICAL PATTERN FROM CORE)
            out.push_str("    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)\n");
            
            // Load vector data and draw
            out.push_str(&format!("    LDX #{}_PATH0  ; Load first path\n", symbol));
            out.push_str("    JSR Draw_Sync_List_At_With_Mirrors\n");
            
            // Restore DP (CRITICAL PATTERN FROM CORE)
            out.push_str("    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)\n");
            
            out.push_str("    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw\n");
            out.push_str("    LDD #0\n    STD RESULT\n");
        }
        _ => {
            out.push_str("    ; ERROR: DRAW_VECTOR_EX first argument must be string literal\n");
            out.push_str("    LDD #0\n    STD RESULT\n");
        }
    }
}

/// Generate helper function implementations
pub fn generate_helper_functions() -> String {
    let mut asm = String::new();
    
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; HELPER FUNCTIONS\n");
    asm.push_str(";***************************************************************************\n\n");
    
    // Add any runtime helpers here (MUL16, DIV16, etc.)
    
    asm
}
/// Collect all PRINT_TEXT string literals from the module
/// Returns a map of hash -> string
pub fn collect_print_text_strings(module: &Module) -> std::collections::BTreeMap<u64, String> {
    let mut strings = std::collections::BTreeMap::new();
    
    // Visit statements and collect PRINT_TEXT strings
    for item in &module.items {
        if let vpy_parser::Item::Function(func) = item {
            for stmt in &func.body {
                collect_strings_from_stmt(stmt, &mut strings);
            }
        }
    }
    
    strings
}

fn collect_strings_from_stmt(stmt: &vpy_parser::Stmt, strings: &mut std::collections::BTreeMap<u64, String>) {
    match stmt {
        vpy_parser::Stmt::Expr(expr, _) => collect_strings_from_expr(expr, strings),
        vpy_parser::Stmt::Assign { value, .. } => collect_strings_from_expr(value, strings),
        vpy_parser::Stmt::Let { value, .. } => collect_strings_from_expr(value, strings),
        vpy_parser::Stmt::For { start, end, step, body, .. } => {
            collect_strings_from_expr(start, strings);
            collect_strings_from_expr(end, strings);
            if let Some(s) = step {
                collect_strings_from_expr(s, strings);
            }
            for s in body {
                collect_strings_from_stmt(s, strings);
            }
        }
        vpy_parser::Stmt::ForIn { iterable, body, .. } => {
            collect_strings_from_expr(iterable, strings);
            for s in body {
                collect_strings_from_stmt(s, strings);
            }
        }
        vpy_parser::Stmt::While { cond, body, .. } => {
            collect_strings_from_expr(cond, strings);
            for s in body {
                collect_strings_from_stmt(s, strings);
            }
        }
        vpy_parser::Stmt::If { cond, body, elifs, else_body, .. } => {
            collect_strings_from_expr(cond, strings);
            for s in body {
                collect_strings_from_stmt(s, strings);
            }
            for (e, b) in elifs {
                collect_strings_from_expr(e, strings);
                for s in b {
                    collect_strings_from_stmt(s, strings);
                }
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    collect_strings_from_stmt(s, strings);
                }
            }
        }
        vpy_parser::Stmt::CompoundAssign { value, .. } => collect_strings_from_expr(value, strings),
        _ => {}
    }
}

fn collect_strings_from_expr(expr: &Expr, strings: &mut std::collections::BTreeMap<u64, String>) {
    match expr {
        Expr::Call(call) => {
            if call.name.to_uppercase() == "PRINT_TEXT" && call.args.len() >= 3 {
                if let Expr::StringLit(s) = &call.args[2] {
                    let hash = hash_string(s);
                    strings.insert(hash, s.clone());
                }
            }
            for arg in &call.args {
                collect_strings_from_expr(arg, strings);
            }
        }
        Expr::Binary { left, right, .. } => {
            collect_strings_from_expr(left, strings);
            collect_strings_from_expr(right, strings);
        }
        Expr::Compare { left, right, .. } => {
            collect_strings_from_expr(left, strings);
            collect_strings_from_expr(right, strings);
        }
        Expr::Logic { left, right, .. } => {
            collect_strings_from_expr(left, strings);
            collect_strings_from_expr(right, strings);
        }
        Expr::Not(expr) | Expr::BitNot(expr) => collect_strings_from_expr(expr, strings),
        Expr::Index { target, index } => {
            collect_strings_from_expr(target, strings);
            collect_strings_from_expr(index, strings);
        }
        Expr::FieldAccess { target, .. } => collect_strings_from_expr(target, strings),
        Expr::MethodCall(call) => {
            for arg in &call.args {
                collect_strings_from_expr(arg, strings);
            }
        }
        Expr::List(items) => {
            for item in items {
                collect_strings_from_expr(item, strings);
            }
        }
        _ => {}
    }
}

/// Emit all PRINT_TEXT string data in helpers bank
pub fn emit_print_text_strings(strings: &std::collections::BTreeMap<u64, String>, out: &mut String) {
    if strings.is_empty() {
        return;
    }
    
    out.push_str(";**** PRINT_TEXT String Data ****\n");
    for (hash, s) in strings {
        let label = format!("PRINT_TEXT_STR_{}", hash);
        out.push_str(&format!("{}:\n", label));
        out.push_str(&format!("    FCC \"{}\"\n", escape_string(s)));
        out.push_str("    FCB $80          ; Vectrex string terminator\n\n");
    }
}