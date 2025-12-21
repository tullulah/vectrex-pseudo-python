// Emission - High-level code emission functions for M6809 backend
use crate::ast::{Function, Stmt};
use crate::codegen::CodegenOptions;
use super::{LoopCtx, FuncCtx, emit_stmt, collect_locals, collect_locals_with_params, RuntimeUsage, LineTracker, DebugInfo};
use super::analyze_var_types; // Import the new function
use std::sync::atomic::{AtomicBool, Ordering};

// Tracking for last END position
static LAST_END_SET: AtomicBool = AtomicBool::new(false);

pub fn emit_function(f: &Function, out: &mut String, string_map: &std::collections::BTreeMap<String,String>, opts: &CodegenOptions, tracker: &mut LineTracker, global_names: &[String]) {
    // Reset end position tracking for each function
    LAST_END_SET.store(false, Ordering::Relaxed);
    
    // Map special VPy functions to proper ASM labels
    let label_name = if f.name == "main" {
        "MAIN".to_string()
    } else {
        f.name.to_uppercase()
    };
    
    out.push_str(&format!("{}: ; function\n", label_name));
    out.push_str(&format!("; --- function {} ---\n", f.name));
    let locals = collect_locals_with_params(&f.body, global_names, &f.params);
    
    // Analyze variable types to determine struct instances and their sizes
    let var_info = analyze_var_types(&f.body, &locals, &opts.structs);
    
    // Calculate frame size based on actual variable sizes
    let mut frame_size = 0;
    for var_name in &locals {
        let size = var_info.get(var_name)
            .map(|(_, s)| *s as i32)
            .unwrap_or(2); // Default to 2 bytes for simple variables
        frame_size += size;
    }
    
    if frame_size > 0 { out.push_str(&format!("    LEAS -{},S ; allocate locals\n", frame_size)); }
    // Copy parameters from VAR_ARG to stack locals (parameters are first N locals)
    for (i, p) in f.params.iter().enumerate().take(4) {
        if let Some(idx) = locals.iter().position(|l| l.eq_ignore_ascii_case(p)) {
            // Calculate offset for this parameter
            let mut offset = 0;
            for j in 0..idx {
                let size = var_info.get(&locals[j])
                    .map(|(_, s)| *s as i32)
                    .unwrap_or(2);
                offset += size;
            }
            out.push_str(&format!("    LDD VAR_ARG{}\n    STD {},S ; param {}\n", i, offset, p));
        }
    }
    let fctx = FuncCtx { 
        locals: locals.clone(), 
        frame_size, 
        var_info,
        // Detect if this is a struct method by checking if name contains underscore
        // Format: STRUCTNAME_methodname (e.g., POINT_MOVE, ENTITY_GET_NEW_X)
        struct_type: if f.name.contains('_') {
            // Extract struct name (part before first underscore)
            f.name.split('_').next().map(|s| s.to_string())
        } else {
            None
        }
    };
    for stmt in &f.body { emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map, opts, tracker, 0); }
    if !matches!(f.body.last(), Some(Stmt::Return(_, _))) {
    if frame_size > 0 { out.push_str(&format!("    LEAS {},S ; free locals\n", frame_size)); }
        out.push_str("    RTS\n");
    }
    out.push('\n');
}

// emit_builtin_helpers: simple placeholder wrappers for Vectrex intrinsics.
pub fn emit_builtin_helpers(out: &mut String, usage: &RuntimeUsage, opts: &CodegenOptions, debug_info: &mut DebugInfo) {
    let w = &usage.wrappers_used;
    // Only emit vector phase helper if referenced
    if w.contains("VECTREX_VECTOR_PHASE_BEGIN") {
        if opts.fast_wait {
            out.push_str("VECTREX_VECTOR_PHASE_BEGIN:\n    JSR DP_to_C8\n    JSR VECTREX_RESET0_FAST\n    RTS\n");
        } else {
            out.push_str("VECTREX_VECTOR_PHASE_BEGIN:\n    JSR DP_to_C8\n    JSR Reset0Ref\n    RTS\n");
        }
    }
    if w.contains("VECTREX_DBG_STATIC_VL") {
        out.push_str("VECTREX_DBG_STATIC_VL:\n    JSR DP_to_C8\n    LDU #DBG_STATIC_LIST\n    LDA #$5F\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDBG_STATIC_LIST:\n    FCB $80,$20\n");
    }
    if opts.blink_intensity {
        out.push_str("VECTREX_BLINK_INT:\n    LDA BLINK_STATE\n    EORA #$01\n    STA BLINK_STATE\n    BEQ BLINK_LOW\nBLINK_HIGH: LDA #$5F\n    BRA BLINK_SET\nBLINK_LOW:  LDA #$10\nBLINK_SET:  JSR Intensity_a\n    RTS\n");
    }
    if opts.debug_init_draw {
        out.push_str("VECTREX_DEBUG_DRAW:\n    JSR DP_to_C8\n    LDU #DEBUG_DRAW_LIST\n    LDA #$40\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\nDEBUG_DRAW_LIST:\n    FCB $80,$40\n");
    }
    if opts.per_frame_silence {
        out.push_str("VECTREX_SILENCE:\n    LDA #0\n    STA $D001\n    CLR $D000\n    LDA #1\n    STA $D001\n    CLR $D000\n    LDA #2\n    STA $D001\n    CLR $D000\n    LDA #3\n    STA $D001\n    CLR $D000\n    LDA #4\n    STA $D001\n    CLR $D000\n    LDA #5\n    STA $D001\n    CLR $D000\n    LDA #6\n    STA $D001\n    CLR $D000\n    LDA #7\n    STA $D001\n    LDA #$3F\n    STA $D000\n    LDA #8\n    STA $D001\n    CLR $D000\n    LDA #9\n    STA $D001\n    CLR $D000\n    LDA #10\n    STA $D001\n    CLR $D000\n    RTS\n");
    }
    if w.contains("VECTREX_PRINT_TEXT") {
        let start_line = out.lines().count() + 1;
        let function_code = "VECTREX_PRINT_TEXT:\n    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)\n    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)\n    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP       ; Set Direct Page to $D0 for BIOS\n    LDU VAR_ARG2   ; string pointer (ARG2 = third param)\n    LDA VAR_ARG1+1 ; Y (ARG1 = second param)\n    LDB VAR_ARG0+1 ; X (ARG0 = first param)\n    JSR Print_Str_d\n    LDA #$C8       ; Restore DP to $C8 for our code\n    TFR A,DP\n    RTS\n";
        out.push_str(function_code);
        let end_line = out.lines().count();
        
        // Register ASM function location for debugging
        debug_info.add_asm_function(
            "VECTREX_PRINT_TEXT".to_string(),
            debug_info.asm.clone(),
            start_line,
            end_line,
            "native"
        );
    }
    if w.contains("VECTREX_DEBUG_PRINT") {
        let start_line = out.lines().count() + 1;
        let function_code = "VECTREX_DEBUG_PRINT:\n    ; Debug print to console - writes to gap area (C000-C7FF)\n    LDA VAR_ARG0+1   ; Load value to debug print\n    STA $C000        ; Debug output value in unmapped gap\n    LDA #$42         ; Debug marker\n    STA $C001        ; Debug marker to indicate new output\n    RTS\n";
        out.push_str(function_code);
        let end_line = out.lines().count();
        
        // Register ASM function location for debugging  
        debug_info.add_asm_function(
            "VECTREX_DEBUG_PRINT".to_string(),
            debug_info.asm.clone(),
            start_line,
            end_line,
            "native"
        );
    }
    if w.contains("VECTREX_DEBUG_PRINT_LABELED") {
        out.push_str(
            "VECTREX_DEBUG_PRINT_LABELED:\n    ; Debug print with label - writes to gap area (C000-C7FF)\n    ; Write label string pointer to unmapped gap\n    LDA VAR_ARG0     ; Label string pointer high byte\n    STA $C002        ; Label pointer high in gap\n    LDA VAR_ARG0+1   ; Label string pointer low byte  \n    STA $C003        ; Label pointer low in gap\n    ; Write value to debug output\n    LDA VAR_ARG1+1   ; Load value to debug print\n    STA $C000        ; Debug output value in gap\n    LDA #$FE         ; Labeled debug marker\n    STA $C001        ; Debug marker to indicate labeled output\n    RTS\n"
        );
    }
    if w.contains("VECTREX_POKE") {
        out.push_str(
            "VECTREX_POKE:\n    ; Write byte to memory address\n    ; ARG0 = address (16-bit), ARG1 = value (8-bit)\n    LDX VAR_ARG0     ; Load address into X\n    LDA VAR_ARG1+1   ; Load value (low byte)\n    STA ,X           ; Store value to address\n    RTS\n"
        );
    }
    if w.contains("VECTREX_PEEK") {
        out.push_str(
            "VECTREX_PEEK:\n    ; Read byte from memory address\n    ; ARG0 = address (16-bit), returns value in VAR_ARG0+1\n    LDX VAR_ARG0     ; Load address into X\n    LDA ,X           ; Load value from address\n    STA VAR_ARG0+1   ; Store result in low byte of ARG0\n    RTS\n"
        );
    }
    if w.contains("VECTREX_PRINT_NUMBER") {
        out.push_str(
            "VECTREX_PRINT_NUMBER:\n    ; Print number at position\n    ; ARG0 = X position, ARG1 = Y position, ARG2 = number value\n    ; Simple implementation: convert number to string and print\n    LDA VAR_ARG1+1   ; Y position\n    LDB VAR_ARG0+1   ; X position\n    JSR Moveto_d     ; Move to position\n    \n    ; Convert number to string (simple: just show low byte as hex)\n    LDA VAR_ARG2+1   ; Load number value\n    \n    ; Convert high nibble to ASCII\n    LSRA\n    LSRA\n    LSRA\n    LSRA\n    ANDA #$0F\n    CMPA #10\n    BLO PN_DIGIT1\n    ADDA #7          ; A-F\nPN_DIGIT1:\n    ADDA #'0'\n    STA NUM_STR      ; Store first digit\n    \n    ; Convert low nibble to ASCII  \n    LDA VAR_ARG2+1\n    ANDA #$0F\n    CMPA #10\n    BLO PN_DIGIT2\n    ADDA #7          ; A-F\nPN_DIGIT2:\n    ADDA #'0'\n    ORA #$80         ; Set high bit for string termination\n    STA NUM_STR+1    ; Store second digit with high bit\n    \n    ; Print the string\n    LDU #NUM_STR     ; Point to our number string\n    JSR Print_Str_d  ; Print using BIOS\n    RTS\n\nNUM_STR: RMB 2      ; Space for 2-digit hex number\n"
        );
    }
    if w.contains("VECTREX_MOVE_TO") {
        out.push_str(
            "VECTREX_MOVE_TO:\n    LDA VAR_ARG1+1 ; Y\n    LDB VAR_ARG0+1 ; X\n    JSR Moveto_d\n    ; store new current position\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("VECTREX_DRAW_TO") {
        out.push_str(
            "; Draw from current (VCUR_X,VCUR_Y) to new (x,y) provided in low bytes VAR_ARG0/1.\n; Semántica: igual a MOVE_TO seguido de línea, pero preserva origen previo como punto inicial.\n; Deltas pueden ser ±127 (hardware Vectrex soporta rango completo).\nVECTREX_DRAW_TO:\n    ; Cargar destino (x,y)\n    LDA VAR_ARG0+1  ; Xdest en A temporalmente\n    STA VLINE_DX    ; reutilizar buffer temporal (bajo) para Xdest\n    LDA VAR_ARG1+1  ; Ydest en A\n    STA VLINE_DY    ; reutilizar buffer temporal para Ydest\n    ; Calcular dx = Xdest - VCUR_X\n    LDA VLINE_DX\n    SUBA VCUR_X\n    STA VLINE_DX\n    ; Calcular dy = Ydest - VCUR_Y\n    LDA VLINE_DY\n    SUBA VCUR_Y\n    STA VLINE_DY\n    ; No clamping needed - signed byte arithmetic handles ±127 correctly\n    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)\n    LDA VCUR_Y\n    LDB VCUR_X\n    JSR Moveto_d\n    ; Dibujar línea usando deltas (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    ; Actualizar posición actual al destino exacto original\n    LDA VAR_ARG0+1\n    STA VCUR_X\n    LDA VAR_ARG1+1\n    STA VCUR_Y\n    RTS\n"
        );
    }
    if w.contains("DRAW_LINE_WRAPPER") {
        out.push_str(
            "; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly\n; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.\n; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).\nDRAW_LINE_WRAPPER:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)\n    STA >$D00C     ; VIA_cntl\n    ; Set DP to hardware registers\n    LDA #$D0\n    TFR A,DP\n    ; ALWAYS set intensity (no optimization)\n    LDA VAR_ARG4+1\n    JSR Intensity_a\n    ; Move to start (y in A, x in B) - use signed byte values\n    LDA VAR_ARG1+1  ; Y start (signed byte)\n    LDB VAR_ARG0+1  ; X start (signed byte)\n    JSR Moveto_d\n    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes\n    ; dx = x1 - x0 (treating as signed)\n    LDD VAR_ARG2    ; x1 (16-bit)\n    SUBD VAR_ARG0   ; subtract x0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DX_CLAMP_HI_OK\n    LDD #127\nDLW_DX_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DX_CLAMP_LO_OK\n    LDD #-128\nDLW_DX_CLAMP_LO_OK:\n    STB VLINE_DX    ; Store dx as signed byte\n    ; dy = y1 - y0 (treating as signed)\n    LDD VAR_ARG3    ; y1 (16-bit)\n    SUBD VAR_ARG1   ; subtract y0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLW_DY_CLAMP_HI_OK\n    LDD #127\nDLW_DY_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLW_DY_CLAMP_LO_OK\n    LDD #-128\nDLW_DY_CLAMP_LO_OK:\n    STB VLINE_DY    ; Store dy as signed byte\n    ; dx and dy are already clamped to ±127 - no further clamping needed\n    ; Vectrex hardware supports full ±127 delta range\n    LDA VLINE_DX\n    STA VLINE_DX    ; Keep full range\n    LDA VLINE_DY\n    STA VLINE_DY    ; Keep full range\n    ; Clear Vec_Misc_Count for proper timing\n    CLR Vec_Misc_Count\n    ; Draw line (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code\n    TFR A,DP\n    RTS\n\n; DRAW_LINE_FAST - optimized version that skips redundant setup\n; Use this for multiple consecutive draws with same intensity\n; Args: (x0,y0,x1,y1) only - intensity must be set beforehand\nDRAW_LINE_FAST:\n    ; Move to start (y in A, x in B) - use signed byte values\n    LDA VAR_ARG1+1  ; Y start (signed byte)\n    LDB VAR_ARG0+1  ; X start (signed byte)\n    JSR Moveto_d\n    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes\n    ; dx = x1 - x0 (treating as signed)\n    LDD VAR_ARG2    ; x1 (16-bit)\n    SUBD VAR_ARG0   ; subtract x0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLF_DX_CLAMP_HI_OK\n    LDD #127\nDLF_DX_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLF_DX_CLAMP_LO_OK\n    LDD #-128\nDLF_DX_CLAMP_LO_OK:\n    STB VLINE_DX    ; Store dx as signed byte\n    ; dy = y1 - y0 (treating as signed)\n    LDD VAR_ARG3    ; y1 (16-bit)\n    SUBD VAR_ARG1   ; subtract y0 (16-bit)\n    ; Clamp D to signed byte range (-128 to +127)\n    CMPD #127\n    BLE DLF_DY_CLAMP_HI_OK\n    LDD #127\nDLF_DY_CLAMP_HI_OK:\n    CMPD #-128\n    BGE DLF_DY_CLAMP_LO_OK\n    LDD #-128\nDLF_DY_CLAMP_LO_OK:\n    STB VLINE_DY    ; Store dy as signed byte\n    ; dx and dy are already clamped to ±127 - no further clamping needed\n    ; Vectrex hardware supports full ±127 delta range\n    LDA VLINE_DX\n    STA VLINE_DX    ; Keep full range\n    LDA VLINE_DY\n    STA VLINE_DY    ; Keep full range\n    ; Clear Vec_Misc_Count for proper timing\n    CLR Vec_Misc_Count\n    ; Draw line (A=dy, B=dx)\n    LDA VLINE_DY\n    LDB VLINE_DX\n    JSR Draw_Line_d\n    RTS\n"
        );
    }
    if w.contains("VECTREX_FRAME_BEGIN") {
        if opts.fast_wait {
            out.push_str(
                "VECTREX_FRAME_BEGIN:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    JSR VECTREX_RESET0_FAST\n    RTS\n"
            );
        } else {
            out.push_str(
                "VECTREX_FRAME_BEGIN:\n    LDA VAR_ARG0+1\n    JSR Intensity_a\n    JSR Reset0Ref\n    RTS\n"
            );
        }
    }
    if w.contains("VECTREX_DRAW_VL") {
        out.push_str(
            "VECTREX_DRAW_VL:\n    LDU VAR_ARG0\n    LDA VAR_ARG1+1\n    JSR Intensity_a\n    JSR Draw_VL\n    RTS\n"
        );
    }
    if w.contains("VECTREX_SET_ORIGIN") {
        if opts.fast_wait {
            out.push_str("VECTREX_SET_ORIGIN:\n    JSR VECTREX_RESET0_FAST\n    RTS\n");
        } else {
            out.push_str("VECTREX_SET_ORIGIN:\n    JSR Reset0Ref\n    RTS\n");
        }
    }
    if w.contains("VECTREX_SET_INTENSITY") {
    out.push_str("VECTREX_SET_INTENSITY:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP       ; Set Direct Page to $D0 for BIOS\n    LDA VAR_ARG0+1\n    JSR __Intensity_a\n    RTS\n");
    }
    if w.contains("SETUP_DRAW_COMMON") {
        out.push_str(
            "; Common drawing setup - sets DP register and resets integrator origin\n; Eliminates repetitive LDA #$D0; TFR A,DP; JSR Reset0Ref sequences\nSETUP_DRAW_COMMON:\n    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)\n    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)\n    STA >$D00C     ; VIA_cntl\n    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n    RTS\n"
        );
    }
    if w.contains("VECTREX_WAIT_RECAL") || opts.fast_wait {
        if opts.fast_wait { out.push_str("VECTREX_WAIT_RECAL:\n    LDA #$D0\n    TFR A,DP\n    LDA FAST_WAIT_HIT\n    INCA\n    STA FAST_WAIT_HIT\n    RTS\n");
            out.push_str("VECTREX_RESET0_FAST:\n    LDA #$D0\n    TFR A,DP\n    CLR Vec_Dot_Dwell\n    CLR Vec_Loop_Count\n    RTS\n"); } else { out.push_str("VECTREX_WAIT_RECAL:\n    JSR Wait_Recal\n    RTS\n"); }
    }
    if w.contains("VECTREX_PLAY_MUSIC1") {
        // Simple wrapper to restart the default MUSIC1 tune each frame or once. BIOS expects U to point to music data table at (?), but calling MUSIC1 vector reinitializes tune.
        out.push_str("VECTREX_PLAY_MUSIC1:\n    JSR MUSIC1\n    RTS\n");
    }
    
    // BIOS music system handles all PSG operations internally - no custom helpers needed
    
    // DRAW_VECTOR_RUNTIME: Old helper - NO LONGER USED
    // Now using inline code with Draw_VLc BIOS function
    // (removed to avoid label conflicts with inline code)
    
    // PLAY_MUSIC_RUNTIME: Direct PSG music player (inspired by Christman2024/malbanGit)
    // Writes directly to PSG chip, bypassing BIOS
    // Force generation if music assets exist (for auto-inject UPDATE_MUSIC_PSG)
    let has_music_assets = opts.assets.iter().any(|a| {
        matches!(a.asset_type, crate::codegen::AssetType::Music)
    });
    if w.contains("PLAY_MUSIC_RUNTIME") || w.contains("STOP_MUSIC_RUNTIME") || has_music_assets {
        out.push_str(
            "; ============================================================================\n\
            ; PSG DIRECT MUSIC PLAYER (inspired by Christman2024/malbanGit)\n\
            ; ============================================================================\n\
            ; Writes directly to PSG chip using WRITE_PSG sequence\n\
            ;\n\
            ; Music data format (frame-based):\n\
            ;   FCB count           ; Number of register writes this frame\n\
            ;   FCB reg, val        ; PSG register/value pairs\n\
            ;   ...                 ; Repeat for each register\n\
            ;   FCB $FF             ; End marker\n\
            ;\n\
            ; PSG Registers:\n\
            ;   0-1: Channel A frequency (12-bit)\n\
            ;   2-3: Channel B frequency\n\
            ;   4-5: Channel C frequency\n\
            ;   6:   Noise period\n\
            ;   7:   Mixer control (enable/disable channels)\n\
            ;   8-10: Channel A/B/C volume\n\
            ;   11-12: Envelope period\n\
            ;   13:  Envelope shape\n\
            ; ============================================================================\n\
            \n\
            ; RAM variables (defined in RAM section above)\n\
            ; PSG_MUSIC_PTR    EQU RESULT+26  (2 bytes)\n\
            ; PSG_MUSIC_START  EQU RESULT+28  (2 bytes)\n\
            ; PSG_IS_PLAYING   EQU RESULT+30  (1 byte)\n\
            ; PSG_MUSIC_ACTIVE EQU RESULT+31  (1 byte) - Set=1 during UPDATE_MUSIC_PSG\n\
            \n\
            ; PLAY_MUSIC_RUNTIME - Start PSG music playback\n\
            ; Input: X = pointer to PSG music data\n\
            PLAY_MUSIC_RUNTIME:\n\
            STX >PSG_MUSIC_PTR     ; Store current music pointer (force extended)\n\
            STX >PSG_MUSIC_START   ; Store start pointer for loops (force extended)\n\
            LDA #$01\n\
            STA >PSG_IS_PLAYING ; Mark as playing (extended - var at 0xC8A0)\n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; UPDATE_MUSIC_PSG - Update PSG (call every frame)\n\
            ; ============================================================================\n\
            UPDATE_MUSIC_PSG:\n\
            ; CRITICAL: Set VIA to PSG mode BEFORE accessing PSG (don't assume state)\n\
            LDA #$00       ; VIA_cntl = $00 (PSG mode)\n\
            STA >$D00C     ; VIA_cntl\n\
            LDA #$01\n\
            STA >PSG_MUSIC_ACTIVE  ; Mark music system active (for PSG logging)\n\
            LDA >PSG_IS_PLAYING ; Check if playing (extended - var at 0xC8A0)\n\
            BEQ PSG_update_done    ; Not playing, exit\n\
            \n\
            LDX >PSG_MUSIC_PTR     ; Load pointer (force extended - LDX has no DP mode)\n\
            BEQ PSG_update_done    ; No music loaded\n\
            \n\
            ; Read frame count byte (number of register writes)\n\
            LDB ,X+\n\
            BEQ PSG_music_ended    ; Count=0 means end (no loop)\n\
            CMPB #$FF              ; Check for loop command\n\
            BEQ PSG_music_loop     ; $FF means loop (never valid as count)\n\
            \n\
            ; Process frame - push counter to stack\n\
            PSHS B                 ; Save count on stack\n\
            \n\
            ; Write register/value pairs to PSG\n\
PSG_write_loop:\n\
            LDA ,X+                ; Load register number\n\
            LDB ,X+                ; Load register value\n\
            PSHS X                 ; Save pointer (after reads)\n\
            \n\
            ; WRITE_PSG sequence\n\
            STA VIA_port_a         ; Store register number\n\
            LDA #$19               ; BDIR=1, BC1=1 (LATCH)\n\
            STA VIA_port_b\n\
            LDA #$01               ; BDIR=0, BC1=0 (INACTIVE)\n\
            STA VIA_port_b\n\
            LDA VIA_port_a         ; Read status\n\
            STB VIA_port_a         ; Store data\n\
            LDB #$11               ; BDIR=1, BC1=0 (WRITE)\n\
            STB VIA_port_b\n\
            LDB #$01               ; BDIR=0, BC1=0 (INACTIVE)\n\
            STB VIA_port_b\n\
            \n\
            PULS X                 ; Restore pointer\n\
            PULS B                 ; Get counter\n\
            DECB                   ; Decrement\n\
            BEQ PSG_frame_done     ; Done with this frame\n\
            PSHS B                 ; Save counter back\n\
            BRA PSG_write_loop\n\
            \n\
PSG_frame_done:\n\
            \n\
            ; Frame complete - update pointer and done\n\
            STX >PSG_MUSIC_PTR     ; Update pointer (force extended)\n\
            BRA PSG_update_done\n\
            \n\
PSG_music_ended:\n\
            CLR >PSG_IS_PLAYING ; Stop playback (extended - var at 0xC8A0)\n\
            ; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing\n\
            ; Music will fade naturally as frame data stops updating\n\
            BRA PSG_update_done\n\
            \n\
PSG_music_loop:\n\
            ; Loop command: $FF followed by 2-byte address (FDB)\n\
            ; X points past $FF, read the target address\n\
            LDD ,X                 ; Load 2-byte loop target address\n\
            STD >PSG_MUSIC_PTR     ; Update pointer to loop start\n\
            ; Exit - next frame will start from loop target\n\
            BRA PSG_update_done\n\
            \n\
PSG_update_done:\n\
            CLR >PSG_MUSIC_ACTIVE  ; Clear flag (music system done)\n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; STOP_MUSIC_RUNTIME - Stop music playback\n\
            ; ============================================================================\n\
            STOP_MUSIC_RUNTIME:\n\
            CLR >PSG_IS_PLAYING ; Clear playing flag (extended - var at 0xC8A0)\n\
            CLR >PSG_MUSIC_PTR     ; Clear pointer high byte (force extended)\n\
            CLR >PSG_MUSIC_PTR+1   ; Clear pointer low byte (force extended)\n\
            ; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing\n\
            RTS\n\
            \n"
        );
    }
    
    // PLAY_SFX_RUNTIME: Sound effects player for .vmus assets (one-shot, non-looping)
    // Only emit if PLAY_SFX() builtin is actually used in code
    if w.contains("PLAY_SFX_RUNTIME") {
        out.push_str(
            "; ============================================================================\n\
            ; PSG SOUND EFFECTS PLAYER RUNTIME\n\
            ; ============================================================================\n\
            ; SFX data structure (same as music .vmus format):\n\
            ;   +0: FDB tempo (BPM)\n\
            ;   +2: FDB ticks_per_beat\n\
            ;   +4: FDB total_ticks_hi, total_ticks_lo (32-bit)\n\
            ;   +8: FDB num_events\n\
            ;   +10: Event data (variable length)\n\
            ;\n\
            ; SFX player differences from music:\n\
            ;   - One-shot playback (no looping)\n\
            ;   - Higher priority than music (can interrupt)\n\
            ;   - Multiple SFX can play simultaneously (TODO: implement mixing)\n\
            ;\n\
            ; Current implementation: Simple placeholder\n\
            ; TODO: Implement full SFX queue with mixing\n\
            ; ============================================================================\n\
            \n\
            ; RAM variables for SFX (defined in RAM section)\n\
            ; SFX_PTR       FDB 0  ; Pointer to current SFX data\n\
            ; SFX_TICK      FDB 0, 0  ; Current playback tick (32-bit)\n\
            ; SFX_EVENT     FDB 0  ; Pointer to current event\n\
            ; SFX_ACTIVE    FCB 0  ; 0=stopped, 1=playing\n\
            \n\
            ; PLAY_SFX_RUNTIME - Initialize and start SFX playback\n\
            ; Input: X = pointer to SFX data structure\n\
            PLAY_SFX_RUNTIME:\n\
            ; Store SFX pointer\n\
            STX SFX_PTR\n\
            \n\
            ; Initialize playback state\n\
            CLRA\n\
            CLRB\n\
            STD SFX_TICK        ; Reset tick counter to 0\n\
            STD SFX_TICK+2\n\
            \n\
            ; Point to first event (skip 10-byte header)\n\
            TFR X,D\n\
            ADDD #10\n\
            STD SFX_EVENT\n\
            \n\
            ; Mark as active\n\
            LDA #1\n\
            STA SFX_ACTIVE\n\
            \n\
            RTS\n\
            \n\
            ; ============================================================================\n\
            ; SFX_UPDATE - Process SFX events (called from MUSIC_UPDATE)\n\
            ; TODO: Integrate into MUSIC_UPDATE or call separately\n\
            ; ============================================================================\n\
            SFX_UPDATE:\n\
            PSHS A,B,X,Y\n\
            \n\
            ; Check if SFX is active\n\
            TST SFX_ACTIVE\n\
            BEQ SFX_UPDATE_done             ; Not playing, skip\n\
            \n\
            ; Increment tick counter (same logic as music)\n\
            LDD SFX_TICK+2\n\
            ADDD #1\n\
            STD SFX_TICK+2\n\
            BCC SFX_UPDATE_no_carry\n\
            LDD SFX_TICK\n\
            ADDD #1\n\
            STD SFX_TICK\n\
            \n\
SFX_UPDATE_no_carry:\n\
            ; TODO: Process SFX events (similar to MUSIC_UPDATE)\n\
            ; For now, just placeholder\n\
            \n\
SFX_UPDATE_done:\n\
            PULS A,B,X,Y\n\
            RTS\n\
            \n"
        );
    }
    
    // Trig tables are emitted later in data section.
    
    // ===========================================================================
    // BIOS WRAPPERS - VIDE/gcc6809 compatible calling convention
    // ===========================================================================
    // These wrappers ensure DP=$D0 is set before each BIOS call, mimicking
    // the behavior of VIDE's auto-generated wrapper functions.
    // Using these wrappers instead of direct BIOS calls eliminates issues
    // with Direct Page register state across multiple calls.
    
    out.push_str("; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)\n");
    
    // __Intensity_a wrapper - VIDE compatible (JMP not JSR)
    out.push_str(
        "__Intensity_a:\n\
        TFR B,A         ; Move B to A (BIOS expects intensity in A)\n\
        JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Reset0Ref wrapper - VIDE compatible (JMP not JSR)
    out.push_str(
        "__Reset0Ref:\n\
        JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Moveto_d wrapper - VIDE compatible (JMP not JSR)
    // Caller pushes Y parameter on stack, X in B register
    out.push_str(
        "__Moveto_d:\n\
        LDA 2,S         ; Get Y from stack (after return address)\n\
        JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller\n"
    );
    
    // __Draw_Line_d wrapper - VIDE compatible (JMP not JSR)
    // Caller pushes dy parameter on stack, dx in B register
    out.push_str(
        "__Draw_Line_d:\n\
        LDA 2,S         ; Get dy from stack (after return address)\n\
        JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller\n"
    );

    // Draw_Sync_List - EXACT translation of Malban's draw_synced_list_c
    // Data format: intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
    out.push_str(
        "; ============================================================================\n\
        ; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c\n\
        ; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2\n\
        ; ============================================================================\n\
        Draw_Sync_List:\n\
        ; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)\n\
        LDA ,X+                 ; intensity\n\
        JSR $F2AB               ; BIOS Intensity_a (expects value in A)\n\
        LDB ,X+                 ; y_start\n\
        LDA ,X+                 ; x_start\n\
        STD TEMP_YX             ; Guardar en variable temporal (evita stack)\n\
        ; Reset completo\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move sequence\n\
        LDD TEMP_YX             ; Recuperar y,x\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A                  ; Save x\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore x\n\
        STA VIA_port_a          ; x to DAC\n\
        ; Timing setup\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move to complete\n\
        DSL_W1:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W1\n\
        ; Loop de dibujo\n\
        DSL_LOOP:\n\
        LDA ,X+                 ; Read flag\n\
        CMPA #2                 ; Check end marker\n\
        LBEQ DSL_DONE           ; Exit if end (long branch)\n\
        CMPA #1                 ; Check next path marker\n\
        LBEQ DSL_NEXT_PATH      ; Process next path (long branch)\n\
        ; Draw line\n\
        LDB ,X+                 ; dy\n\
        LDA ,X+                 ; dx\n\
        PSHS A                  ; Save dx\n\
        STB VIA_port_a          ; dy to DAC\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore dx\n\
        STA VIA_port_a          ; dx to DAC\n\
        CLR VIA_t1_cnt_hi\n\
        LDA #$FF\n\
        STA VIA_shift_reg\n\
        ; Wait for line draw\n\
        DSL_W2:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W2\n\
        CLR VIA_shift_reg\n\
        BRA DSL_LOOP\n\
        ; Next path: read new intensity and header, then continue drawing\n\
        DSL_NEXT_PATH:\n\
        ; Save current X position before reading anything\n\
        TFR X,D                 ; D = X (current position)\n\
        PSHS D                  ; Save X address\n\
        LDA ,X+                 ; Read intensity (X now points to y_start)\n\
        PSHS A                  ; Save intensity\n\
        LDB ,X+                 ; y_start\n\
        LDA ,X+                 ; x_start (X now points to next_y)\n\
        STD TEMP_YX             ; Save y,x\n\
        PULS A                  ; Get intensity back\n\
        PSHS A                  ; Save intensity again\n\
        LDA #$D0\n\
        TFR A,DP                ; Set DP=$D0 (BIOS requirement)\n\
        PULS A                  ; Restore intensity\n\
        JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)\n\
        ; Restore X to point to next_y,next_x (after the 3 bytes we read)\n\
        PULS D                  ; Get original X\n\
        ADDD #3                 ; Skip intensity, y_start, x_start\n\
        TFR D,X                 ; X now points to next_y\n\
        ; Reset to zero (same as Draw_Sync_List start)\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move to new start position\n\
        LDD TEMP_YX\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A\n\
        STA VIA_port_a          ; x to DAC\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move\n\
        DSL_W3:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSL_W3\n\
        CLR VIA_shift_reg       ; Clear before continuing\n\
        BRA DSL_LOOP            ; Continue drawing\n\
        DSL_DONE:\n\
        RTS\n\n\
        ; ============================================================================\n\
        ; Draw_Sync_List_At - Draw vector at offset position (DRAW_VEC_X, DRAW_VEC_Y)\n\
        ; Same as Draw_Sync_List but adds offset to y_start, x_start coordinates\n\
        ; Uses: DRAW_VEC_X, DRAW_VEC_Y (set by DRAW_VECTOR before calling this)\n\
        ; ============================================================================\n\
        Draw_Sync_List_At:\n\
        LDA ,X+                 ; intensity\n\
        PSHS A                  ; Save intensity\n\
        LDA #$D0\n\
        PULS A                  ; Restore intensity\n\
        JSR $F2AB               ; BIOS Intensity_a\n\
        LDB ,X+                 ; y_start from .vec\n\
        ADDB DRAW_VEC_Y         ; Add Y offset\n\
        LDA ,X+                 ; x_start from .vec\n\
        ADDA DRAW_VEC_X         ; Add X offset\n\
        STD TEMP_YX             ; Save adjusted position\n\
        ; Reset completo\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move sequence\n\
        LDD TEMP_YX             ; Recuperar y,x ajustado\n\
        STB VIA_port_a          ; y to DAC\n\
        PSHS A                  ; Save x\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore x\n\
        STA VIA_port_a          ; x to DAC\n\
        ; Timing setup\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X                ; Skip next_y, next_x\n\
        ; Wait for move to complete\n\
        DSLA_W1:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W1\n\
        ; Loop de dibujo (same as Draw_Sync_List)\n\
        DSLA_LOOP:\n\
        LDA ,X+                 ; Read flag\n\
        CMPA #2                 ; Check end marker\n\
        LBEQ DSLA_DONE\n\
        CMPA #1                 ; Check next path marker\n\
        LBEQ DSLA_NEXT_PATH\n\
        ; Draw line\n\
        LDB ,X+                 ; dy\n\
        LDA ,X+                 ; dx\n\
        PSHS A                  ; Save dx\n\
        STB VIA_port_a          ; dy to DAC\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A                  ; Restore dx\n\
        STA VIA_port_a          ; dx to DAC\n\
        CLR VIA_t1_cnt_hi\n\
        LDA #$FF\n\
        STA VIA_shift_reg\n\
        ; Wait for line draw\n\
        DSLA_W2:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W2\n\
        CLR VIA_shift_reg\n\
        BRA DSLA_LOOP\n\
        ; Next path: add offset to new coordinates too\n\
        DSLA_NEXT_PATH:\n\
        TFR X,D\n\
        PSHS D\n\
        LDA ,X+                 ; Read intensity\n\
        PSHS A\n\
        LDB ,X+                 ; y_start\n\
        ADDB DRAW_VEC_Y         ; Add Y offset to new path\n\
        LDA ,X+                 ; x_start\n\
        ADDA DRAW_VEC_X         ; Add X offset to new path\n\
        STD TEMP_YX\n\
        PULS A                  ; Get intensity back\n\
        JSR $F2AB\n\
        PULS D\n\
        ADDD #3\n\
        TFR D,X\n\
        ; Reset to zero\n\
        CLR VIA_shift_reg\n\
        LDA #$CC\n\
        STA VIA_cntl\n\
        CLR VIA_port_a\n\
        LDA #$82\n\
        STA VIA_port_b\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        NOP\n\
        LDA #$83\n\
        STA VIA_port_b\n\
        ; Move to new start position (already offset-adjusted)\n\
        LDD TEMP_YX\n\
        STB VIA_port_a\n\
        PSHS A\n\
        LDA #$CE\n\
        STA VIA_cntl\n\
        CLR VIA_port_b\n\
        LDA #1\n\
        STA VIA_port_b\n\
        PULS A\n\
        STA VIA_port_a\n\
        LDA #$7F\n\
        STA VIA_t1_cnt_lo\n\
        CLR VIA_t1_cnt_hi\n\
        LEAX 2,X\n\
        ; Wait for move\n\
        DSLA_W3:\n\
        LDA VIA_int_flags\n\
        ANDA #$40\n\
        BEQ DSLA_W3\n\
        CLR VIA_shift_reg\n\
        BRA DSLA_LOOP\n\
        DSLA_DONE:\n\
        RTS\n"
    );
    
    // Draw_Sync_List_At_Mirrored: Same as Draw_Sync_List_At but with center-based mirroring
    // This creates a horizontally mirrored sprite using the asset's calculated center as the mirror axis
    // Formula: x_mirrored = 2*center_x - x_original (reflects point across center_x)
    // dx values are also negated (NEGA) to flip line direction
    // (Always emitted as part of core drawing infrastructure)
    out.push_str(
        "Draw_Sync_List_At_Mirrored:\n\
        ; Horizontally mirrored drawing (uses center-relative coordinates)\n\
            ; With center-relative coords from vecres.rs, mirroring is simple:\n\
            ; 1. Negate X coordinate: x_mirrored = -x_start\n\
            ; 2. Negate dx deltas: dx_mirrored = -dx (via NEGA)\n\
            LDA ,X+                 ; intensity\n\
            PSHS A                  ; Save intensity\n\
            LDA #$D0\n\
            PULS A                  ; Restore intensity\n\
            JSR $F2AB               ; BIOS Intensity_a\n\
            LDB ,X+                 ; y_start from .vec (already relative to center)\n\
            ADDB DRAW_VEC_Y         ; Add Y offset\n\
            LDA ,X+                 ; x_start from .vec (already relative to center)\n\
            NEGA                    ; ← Negate for X-axis mirror\n\
            ADDA DRAW_VEC_X         ; Add X offset\n\
            STD TEMP_YX             ; Save adjusted (mirrored) position\n\
            ; Reset completo\n\
            CLR VIA_shift_reg\n\
            LDA #$CC\n\
            STA VIA_cntl\n\
            CLR VIA_port_a\n\
            LDA #$82\n\
            STA VIA_port_b\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            LDA #$83\n\
            STA VIA_port_b\n\
            ; Move sequence\n\
            LDD TEMP_YX             ; Recuperar y,x ajustado\n\
            STB VIA_port_a          ; y to DAC\n\
            PSHS A                  ; Save x\n\
            LDA #$CE\n\
            STA VIA_cntl\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A                  ; Restore x\n\
            STA VIA_port_a          ; x to DAC\n\
            ; Timing setup\n\
            LDA #$7F\n\
            STA VIA_t1_cnt_lo\n\
            CLR VIA_t1_cnt_hi\n\
            LEAX 2,X                ; Skip next_y, next_x\n\
            ; Wait for move to complete\n\
            DSLM_W1:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLM_W1\n\
            ; Loop de dibujo (MIRRORED: negate dx)\n\
            DSLM_LOOP:\n\
            LDA ,X+                 ; Read flag\n\
            CMPA #2                 ; Check end marker\n\
            LBEQ DSLM_DONE\n\
            CMPA #1                 ; Check next path marker\n\
            LBEQ DSLM_NEXT_PATH\n\
            ; Draw line with negated dx\n\
            LDB ,X+                 ; dy\n\
            LDA ,X+                 ; dx\n\
            NEGA                    ; ← NEGATE dx for mirror effect\n\
            PSHS A                  ; Save negated dx\n\
            STB VIA_port_a          ; dy to DAC\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A                  ; Restore negated dx\n\
            STA VIA_port_a          ; negated dx to DAC\n\
            CLR VIA_t1_cnt_hi\n\
            LDA #$FF\n\
            STA VIA_shift_reg\n\
            ; Wait for line draw\n\
            DSLM_W2:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLM_W2\n\
            CLR VIA_shift_reg\n\
            BRA DSLM_LOOP\n\
            ; Next path: negate X coordinate for new path too
            DSLM_NEXT_PATH:\n\
            TFR X,D\n\
            PSHS D\n\
            LDA ,X+                 ; Read intensity\n\
            PSHS A\n\
            LDB ,X+                 ; y_start (already relative to center)\n\
            ADDB DRAW_VEC_Y         ; Add Y offset to new path\n\
            LDA ,X+                 ; x_start (already relative to center)\n\
            NEGA                    ; ← Negate for X-axis mirror\n\
            ADDA DRAW_VEC_X         ; Add X offset\n\
            STD TEMP_YX\n\
            PULS A                  ; Get intensity back\n\
            JSR $F2AB\n\
            PULS D\n\
            ADDD #3\n\
            TFR D,X\n\
            ; Reset to zero\n\
            CLR VIA_shift_reg\n\
            LDA #$CC\n\
            STA VIA_cntl\n\
            CLR VIA_port_a\n\
            LDA #$82\n\
            STA VIA_port_b\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            LDA #$83\n\
            STA VIA_port_b\n\
            ; Move to new start position (already offset-adjusted)\n\
            LDD TEMP_YX\n\
            STB VIA_port_a\n\
            PSHS A\n\
            LDA #$CE\n\
            STA VIA_cntl\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A\n\
            STA VIA_port_a\n\
            LDA #$7F\n\
            STA VIA_t1_cnt_lo\n\
            CLR VIA_t1_cnt_hi\n\
            LEAX 2,X\n\
            ; Wait for move\n\
            DSLM_W3:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLM_W3\n\
            CLR VIA_shift_reg\n\
            BRA DSLM_LOOP\n\
            DSLM_DONE:\n\
            RTS\n"
    );
    
    // Draw_Sync_List_At_Mirrored_Y: Vertically mirrored drawing (Y-axis mirror)
    // Uses center-relative coordinates, mirrors across Y axis
    // 1. Negate Y coordinate: y_mirrored = -y_start
    // 2. Negate dy deltas: dy_mirrored = -dy (via NEGA)
    out.push_str(
        "Draw_Sync_List_At_Mirrored_Y:\n\
        ; Vertically mirrored drawing (uses center-relative coordinates)\n\
            ; With center-relative coords from vecres.rs, mirroring is simple:\n\
            ; 1. Negate Y coordinate: y_mirrored = -y_start\n\
            ; 2. Negate dy deltas: dy_mirrored = -dy (via NEGA)\n\
            LDA ,X+                 ; intensity\n\
            PSHS A                  ; Save intensity\n\
            LDA #$D0\n\
            PULS A                  ; Restore intensity\n\
            JSR $F2AB               ; BIOS Intensity_a\n\
            LDB ,X+                 ; y_start from .vec (already relative to center)\n\
            NEGB                    ; ← Negate for Y-axis mirror\n\
            ADDB DRAW_VEC_Y         ; Add Y offset\n\
            LDA ,X+                 ; x_start from .vec (already relative to center)\n\
            ADDA DRAW_VEC_X         ; Add X offset\n\
            STD TEMP_YX             ; Save adjusted (mirrored) position\n\
            ; Reset completo\n\
            CLR VIA_shift_reg\n\
            LDA #$CC\n\
            STA VIA_cntl\n\
            CLR VIA_port_a\n\
            LDA #$82\n\
            STA VIA_port_b\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            LDA #$83\n\
            STA VIA_port_b\n\
            ; Move sequence\n\
            LDD TEMP_YX             ; Recuperar y,x ajustado\n\
            STB VIA_port_a          ; y to DAC\n\
            PSHS A                  ; Save x\n\
            LDA #$CE\n\
            STA VIA_cntl\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A                  ; Restore x\n\
            STA VIA_port_a          ; x to DAC\n\
            ; Timing setup\n\
            LDA #$7F\n\
            STA VIA_t1_cnt_lo\n\
            CLR VIA_t1_cnt_hi\n\
            LEAX 2,X                ; Skip next_y, next_x\n\
            ; Wait for move to complete\n\
            DSLMY_W1:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLMY_W1\n\
            ; Loop de dibujo (MIRRORED_Y: negate dy)\n\
            DSLMY_LOOP:\n\
            LDA ,X+                 ; Read flag\n\
            CMPA #2                 ; Check end marker\n\
            LBEQ DSLMY_DONE\n\
            CMPA #1                 ; Check next path marker\n\
            LBEQ DSLMY_NEXT_PATH\n\
            ; Draw line with negated dy\n\
            LDB ,X+                 ; dy\n\
            NEGB                    ; ← NEGATE dy for Y-axis mirror effect\n\
            LDA ,X+                 ; dx\n\
            PSHS A                  ; Save dx\n\
            STB VIA_port_a          ; negated dy to DAC\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A                  ; Restore dx\n\
            STA VIA_port_a          ; dx to DAC\n\
            CLR VIA_t1_cnt_hi\n\
            LDA #$FF\n\
            STA VIA_shift_reg\n\
            ; Wait for line draw\n\
            DSLMY_W2:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLMY_W2\n\
            CLR VIA_shift_reg\n\
            BRA DSLMY_LOOP\n\
            ; Next path: negate Y coordinate for new path too\n\
            DSLMY_NEXT_PATH:\n\
            TFR X,D\n\
            PSHS D\n\
            LDA ,X+                 ; Read intensity\n\
            PSHS A\n\
            LDB ,X+                 ; y_start (already relative to center)\n\
            NEGB                    ; ← Negate for Y-axis mirror\n\
            ADDB DRAW_VEC_Y         ; Add Y offset to new path\n\
            LDA ,X+                 ; x_start (already relative to center)\n\
            ADDA DRAW_VEC_X         ; Add X offset\n\
            STD TEMP_YX\n\
            PULS A                  ; Get intensity back\n\
            JSR $F2AB\n\
            PULS D\n\
            ADDD #3\n\
            TFR D,X\n\
            ; Reset to zero\n\
            CLR VIA_shift_reg\n\
            LDA #$CC\n\
            STA VIA_cntl\n\
            CLR VIA_port_a\n\
            LDA #$82\n\
            STA VIA_port_b\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            NOP\n\
            LDA #$83\n\
            STA VIA_port_b\n\
            ; Move to new start position (already offset-adjusted)\n\
            LDD TEMP_YX\n\
            STB VIA_port_a\n\
            PSHS A\n\
            LDA #$CE\n\
            STA VIA_cntl\n\
            CLR VIA_port_b\n\
            LDA #1\n\
            STA VIA_port_b\n\
            PULS A\n\
            STA VIA_port_a\n\
            LDA #$7F\n\
            STA VIA_t1_cnt_lo\n\
            CLR VIA_t1_cnt_hi\n\
            LEAX 2,X\n\
            ; Wait for move\n\
            DSLMY_W3:\n\
            LDA VIA_int_flags\n\
            ANDA #$40\n\
            BEQ DSLMY_W3\n\
            CLR VIA_shift_reg\n\
            BRA DSLMY_LOOP\n\
            DSLMY_DONE:\n\
            RTS\n"
    );
    
    // ========== JOYSTICK SUPPORT ==========
    // VPy programs now use REAL BIOS routines just like commercial ROMs:
    // - Joy_Digital ($F1F8) - reads joystick axes, updates Vec_Joy_1_X/Y ($C81B/$C81C)
    // - Read_Btns ($F1BA) - reads button states, updates Vec_Btn_State ($C80F)
    //
    // Benefits:
    // 1. Perfect compatibility with real Vectrex hardware
    // 2. Minestorm and BIOS games work correctly with gamepad
    // 3. No custom memory-mapped registers needed
    // 4. Standard Vectrex programming practice
    //
    // The BIOS calls are inlined directly in emit_builtin_call() for J1_X(), J1_Y(), etc.
    // No helper routines needed - everything goes through official BIOS entry points.
}

// power_of_two_const: return shift count if expression is a numeric power-of-two (>1).

// format_expr_ref: helper for peephole comparisons.
// In the Vectrex context, all variables need DATA section definitions regardless of scope
