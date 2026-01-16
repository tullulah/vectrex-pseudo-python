// Utility Builtins for VPy
// Simple utility functions for common tasks

use vpy_parser::Expr;

/// Emit MOVE(x, y) - Move beam to position without drawing
/// 
/// Parameters:
/// - x: X coordinate (-127 to 127)
/// - y: Y coordinate (-127 to 127)
/// 
/// Uses BIOS Moveto_d_7F
pub fn emit_move(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== MOVE builtin =====\n");
    
    if args.len() != 2 {
        out.push_str("    ; ERROR: MOVE requires 2 arguments (x, y)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if arguments are constants (optimization)
    if let (Expr::Number(y), Expr::Number(x)) = (&args[1], &args[0]) {
        out.push_str(&format!("    LDA #{}                 ; Y coordinate\n", y));
        out.push_str(&format!("    LDB #{}                 ; X coordinate\n", x));
        out.push_str("    JSR Moveto_d_7F        ; Move beam\n");
    } else {
        out.push_str("    ; TODO: Support variable x, y (requires expressions)\n");
    }
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit LEN(array) - Get array length
/// 
/// For now, returns size from array metadata (stored at array_ptr - 2)
/// 
/// Note: This is a placeholder - proper implementation needs:
/// - Array metadata tracking
/// - String length calculation
/// - Variable length extraction
pub fn emit_len(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== LEN builtin =====\n");
    
    if args.len() != 1 {
        out.push_str("    ; ERROR: LEN requires 1 argument (array or string)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Placeholder - always return 0 for now
    out.push_str("    ; TODO: Implement array/string length extraction\n");
    out.push_str("    ; Needs metadata tracking in array system\n");
    out.push_str("    LDD #0                 ; Placeholder return\n");
    out.push_str("    STD RESULT\n");
}

/// Emit GET_TIME() - Get frame counter
/// 
/// Returns number of frames elapsed since boot.
/// Uses VIA timer or internal counter.
/// 
/// For now, returns placeholder 0 (needs timer integration)
pub fn emit_get_time(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== GET_TIME builtin =====\n");
    out.push_str("    ; TODO: Integrate with VIA timer or frame counter\n");
    out.push_str("    LDD FRAME_COUNTER      ; Placeholder - needs initialization\n");
    out.push_str("    STD RESULT\n");
}

/// Emit PEEK(addr) - Read byte from memory address
/// 
/// Parameters:
/// - addr: Memory address (0-65535)
/// 
/// Returns: Byte value at address (0-255)
pub fn emit_peek(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== PEEK builtin =====\n");
    
    if args.len() != 1 {
        out.push_str("    ; ERROR: PEEK requires 1 argument (address)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if address is constant
    if let Expr::Number(addr) = &args[0] {
        out.push_str(&format!("    LDA ${:04X}             ; Read from address\n", addr));
        out.push_str("    CLR RESULT             ; Clear high byte\n");
        out.push_str("    STA RESULT+1           ; Store low byte\n");
    } else {
        out.push_str("    ; TODO: Support variable address\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
    }
}

/// Emit POKE(addr, value) - Write byte to memory address
/// 
/// Parameters:
/// - addr: Memory address (0-65535)
/// - value: Byte value (0-255)
/// 
/// WARNING: Direct memory access - can corrupt system state!
pub fn emit_poke(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== POKE builtin =====\n");
    
    if args.len() != 2 {
        out.push_str("    ; ERROR: POKE requires 2 arguments (address, value)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if both arguments are constants
    if let (Expr::Number(addr), Expr::Number(value)) = (&args[0], &args[1]) {
        out.push_str(&format!("    LDA #{}                ; Value to write\n", value));
        out.push_str(&format!("    STA ${:04X}             ; Write to address\n", addr));
    } else {
        out.push_str("    ; TODO: Support variable address/value\n");
    }
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit WAIT(frames) - Wait for N frames
/// 
/// Parameters:
/// - frames: Number of frames to wait (1-255)
/// 
/// Calls WAIT_RECAL() N times
pub fn emit_wait(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== WAIT builtin =====\n");
    
    if args.len() != 1 {
        out.push_str("    ; ERROR: WAIT requires 1 argument (frames)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if frames is constant
    if let Expr::Number(frames) = &args[0] {
        if *frames <= 10 {
            // Inline for small counts
            for _ in 0..*frames {
                out.push_str("    JSR Wait_Recal\n");
            }
        } else {
            // Use loop for larger counts
            out.push_str(&format!("    LDA #{}                ; Frame counter\n", frames));
            out.push_str("WAIT_LOOP:\n");
            out.push_str("    PSHS A\n");
            out.push_str("    JSR Wait_Recal\n");
            out.push_str("    PULS A\n");
            out.push_str("    DECA\n");
            out.push_str("    BNE WAIT_LOOP\n");
        }
    } else {
        out.push_str("    ; TODO: Support variable frame count\n");
    }
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit BEEP(frequency, duration) - Generate sound
/// 
/// Parameters:
/// - frequency: Sound frequency (0-255, PSG period)
/// - duration: Duration in frames (1-255)
/// 
/// Uses PSG registers for tone generation
pub fn emit_beep(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== BEEP builtin =====\n");
    
    if args.len() != 2 {
        out.push_str("    ; ERROR: BEEP requires 2 arguments (frequency, duration)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if arguments are constants
    if let (Expr::Number(freq), Expr::Number(dur)) = (&args[0], &args[1]) {
        out.push_str("    ; Set PSG tone on channel A\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP               ; Set DP to PSG\n");
        out.push_str(&format!("    LDA #{}                ; Frequency low\n", freq & 0xFF));
        out.push_str("    STA <$00               ; PSG register 0\n");
        out.push_str("    LDA #0\n");
        out.push_str("    STA <$01               ; PSG register 1 (high)\n");
        out.push_str("    LDA #$0F               ; Volume max\n");
        out.push_str("    STA <$08               ; PSG register 8 (volume A)\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP               ; Restore DP\n");
        out.push_str("    \n");
        out.push_str(&format!("    ; Wait {} frames\n", dur));
        out.push_str(&format!("    LDA #{}                ; Duration\n", dur));
        out.push_str("BEEP_LOOP:\n");
        out.push_str("    PSHS A\n");
        out.push_str("    JSR Wait_Recal\n");
        out.push_str("    PULS A\n");
        out.push_str("    DECA\n");
        out.push_str("    BNE BEEP_LOOP\n");
        out.push_str("    \n");
        out.push_str("    ; Silence\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP\n");
        out.push_str("    LDA #0\n");
        out.push_str("    STA <$08               ; Volume off\n");
        out.push_str("    LDA #$C8\n");
        out.push_str("    TFR A,DP\n");
    } else {
        out.push_str("    ; TODO: Support variable frequency/duration\n");
    }
    
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit FADE_IN() - Gradual intensity increase
/// 
/// Gradually increases intensity from 0 to current SET_INTENSITY value.
/// Uses 8 steps with WAIT_RECAL between each.
pub fn emit_fade_in(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== FADE_IN builtin =====\n");
    out.push_str("    JSR FADE_IN_RUNTIME\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit FADE_OUT() - Gradual intensity decrease
/// 
/// Gradually decreases intensity from current value to 0.
/// Uses 8 steps with WAIT_RECAL between each.
pub fn emit_fade_out(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== FADE_OUT builtin =====\n");
    out.push_str("    JSR FADE_OUT_RUNTIME\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}
