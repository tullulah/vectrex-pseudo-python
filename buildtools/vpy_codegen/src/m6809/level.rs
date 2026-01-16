// Level System Builtins for VPy
// Tile-based level loading and management

use vpy_parser::Expr;

/// Emit LOAD_LEVEL(level_name) - Load level data from ROM
/// 
/// Parameters:
/// - level_name: String literal name of level asset
/// 
/// Memory Layout:
/// - LEVEL_PTR ($CF20): Pointer to current level data
/// - LEVEL_WIDTH ($CF22): Level width in tiles
/// - LEVEL_HEIGHT ($CF23): Level height in tiles
/// 
/// Level Data Format (in ROM):
/// - FDB width, height
/// - FCB tile_data... (width * height bytes)
pub fn emit_load_level(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== LOAD_LEVEL builtin =====\n");
    
    if args.len() != 1 {
        out.push_str("    ; ERROR: LOAD_LEVEL requires 1 argument (level_name)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // Check if argument is string literal (level name)
    if let Expr::StringLit(level_name) = &args[0] {
        out.push_str(&format!("    ; Load level: '{}'\n", level_name));
        
        // Load pointer to level data (asset must exist)
        let label = format!("LEVEL_{}", level_name.to_uppercase().replace(" ", "_"));
        out.push_str(&format!("    LDX #{}\n", label));
        out.push_str("    STX LEVEL_PTR          ; Store level data pointer\n");
        
        // Load width and height from level header
        out.push_str("    LDA ,X+                ; Load width (byte)\n");
        out.push_str("    STA LEVEL_WIDTH\n");
        out.push_str("    LDA ,X+                ; Load height (byte)\n");
        out.push_str("    STA LEVEL_HEIGHT\n");
        
        out.push_str("    LDD #1                 ; Return success\n");
        out.push_str("    STD RESULT\n");
    } else {
        out.push_str("    ; ERROR: LOAD_LEVEL requires string literal (level name)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
    }
}

/// Emit SHOW_LEVEL() - Render current level
/// 
/// Iterates through all tiles and draws them based on tile type.
/// Uses LEVEL_PTR to access tile data.
/// 
/// Simple implementation: Draw rectangles for non-empty tiles
pub fn emit_show_level(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== SHOW_LEVEL builtin =====\n");
    out.push_str("    JSR SHOW_LEVEL_RUNTIME\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit UPDATE_LEVEL() - Update level state (placeholder for game logic)
/// 
/// Can be extended for:
/// - Animated tiles
/// - Destructible tiles
/// - Tile state changes
pub fn emit_update_level(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== UPDATE_LEVEL builtin =====\n");
    out.push_str("    ; Placeholder - extend for animated/destructible tiles\n");
    out.push_str("    LDD #0\n");
    out.push_str("    STD RESULT\n");
}

/// Emit GET_LEVEL_WIDTH() - Return level width in tiles
pub fn emit_get_level_width(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== GET_LEVEL_WIDTH builtin =====\n");
    out.push_str("    CLR RESULT             ; Clear high byte\n");
    out.push_str("    LDA LEVEL_WIDTH        ; Load width (byte)\n");
    out.push_str("    STA RESULT+1           ; Store in low byte\n");
}

/// Emit GET_LEVEL_HEIGHT() - Return level height in tiles
pub fn emit_get_level_height(_args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== GET_LEVEL_HEIGHT builtin =====\n");
    out.push_str("    CLR RESULT             ; Clear high byte\n");
    out.push_str("    LDA LEVEL_HEIGHT       ; Load height (byte)\n");
    out.push_str("    STA RESULT+1           ; Store in low byte\n");
}

/// Emit GET_LEVEL_TILE(x, y) - Get tile at position
/// 
/// Parameters:
/// - x: Tile X coordinate (0-based)
/// - y: Tile Y coordinate (0-based)
/// 
/// Returns: Tile index/type (0 = empty, 1+ = tile types)
/// 
/// Calculation: offset = y * width + x
/// Address = LEVEL_PTR + 2 (skip header) + offset
pub fn emit_get_level_tile(args: &[Expr], out: &mut String) {
    out.push_str("    ; ===== GET_LEVEL_TILE builtin =====\n");
    
    if args.len() != 2 {
        out.push_str("    ; ERROR: GET_LEVEL_TILE requires 2 arguments (x, y)\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
        return;
    }
    
    // For now, only support constant arguments (optimization)
    if let (Expr::Number(x), Expr::Number(y)) = (&args[0], &args[1]) {
        out.push_str(&format!("    ; Get tile at ({}, {})\n", x, y));
        
        // Calculate offset: y * width + x
        out.push_str(&format!("    LDA #{}                 ; Y coordinate\n", y));
        out.push_str("    LDB LEVEL_WIDTH        ; Multiply by width\n");
        out.push_str("    MUL                    ; D = y * width\n");
        out.push_str(&format!("    ADDD #{}               ; Add X coordinate\n", x));
        
        // Add to level data pointer (skip 2-byte header)
        out.push_str("    ADDD #2                ; Skip width/height header\n");
        out.push_str("    ADDD LEVEL_PTR         ; Add base pointer\n");
        out.push_str("    TFR D,X                ; X = tile address\n");
        
        // Load tile value
        out.push_str("    CLR RESULT             ; Clear high byte\n");
        out.push_str("    LDA ,X                 ; Load tile value\n");
        out.push_str("    STA RESULT+1           ; Store in low byte\n");
    } else {
        out.push_str("    ; ERROR: GET_LEVEL_TILE currently requires constant arguments\n");
        out.push_str("    ; TODO: Support variable x, y coordinates\n");
        out.push_str("    LDD #0\n");
        out.push_str("    STD RESULT\n");
    }
}
