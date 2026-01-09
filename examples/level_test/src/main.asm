; --- Motorola 6809 backend (Vectrex) title='Level System Test' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FDB music1
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "LEVEL SYSTEM TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 118 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$04   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$08   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$09   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$0A   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$0C   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$0D   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$0E   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$0F   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$10   ; Intensity override (0=use vector's, >0=override) (1 bytes)
LEVEL_PTR            EQU $C880+$11   ; Pointer to currently loaded level data (2 bytes)
LEVEL_BG_COUNT       EQU $C880+$13   ; SHOW_LEVEL: background object count (1 bytes)
LEVEL_GP_COUNT       EQU $C880+$14   ; SHOW_LEVEL: gameplay object count (1 bytes)
LEVEL_FG_COUNT       EQU $C880+$15   ; SHOW_LEVEL: foreground object count (1 bytes)
LEVEL_BG_PTR         EQU $C880+$16   ; SHOW_LEVEL: background objects pointer (RAM buffer) (2 bytes)
LEVEL_GP_PTR         EQU $C880+$18   ; SHOW_LEVEL: gameplay objects pointer (RAM buffer) (2 bytes)
LEVEL_FG_PTR         EQU $C880+$1A   ; SHOW_LEVEL: foreground objects pointer (RAM buffer) (2 bytes)
LEVEL_BG_ROM_PTR     EQU $C880+$1C   ; LOAD_LEVEL: background objects pointer (ROM) (2 bytes)
LEVEL_GP_ROM_PTR     EQU $C880+$1E   ; LOAD_LEVEL: gameplay objects pointer (ROM) (2 bytes)
LEVEL_FG_ROM_PTR     EQU $C880+$20   ; LOAD_LEVEL: foreground objects pointer (ROM) (2 bytes)
LEVEL_DYNAMIC_COUNT  EQU $C880+$22   ; Number of active dynamic objects (max 12) (1 bytes)
LEVEL_DYNAMIC_BUFFER EQU $C880+$23   ; Dynamic objects state (12 objects * 6 bytes) (72 bytes)
UGPC_OUTER_IDX       EQU $C880+$6B   ; Outer loop index for collision detection (1 bytes)
UGPC_OUTER_MAX       EQU $C880+$6C   ; Outer loop max value (count-1) (1 bytes)
UGPC_INNER_IDX       EQU $C880+$6D   ; Inner loop index for collision detection (1 bytes)
UGPC_DX              EQU $C880+$6E   ; Distance X temporary (16-bit) (2 bytes)
UGPC_DIST            EQU $C880+$70   ; Manhattan distance temporary (16-bit) (2 bytes)
VAR_ARG0             EQU $C880+$72   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$74   ; Function argument 1 (2 bytes)

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; === BUTTON SYSTEM - BIOS TRANSITIONS ===
; J1_BUTTON_1-4() - Read transition bits from $C811
; Read_Btns (auto-injected) calculates: ~(new) OR Vec_Prev_Btns
; Result: bit=1 ONLY on rising edge (0→1 transition)
; Returns: D = 1 (just pressed), 0 (not pressed or still held)

J1B1_BUILTIN:
    LDA $C811      ; Read transition bits (Vec_Button_1_1)
    ANDA #$01      ; Test bit 0 (Button 1)
    BEQ .J1B1_OFF
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    BEQ .J1B2_OFF
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    BEQ .J1B3_OFF
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    BEQ .J1B4_OFF
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
    RTS

VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller
__Reset0Ref:
JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller
__Moveto_d:
LDA 2,S         ; Get Y from stack (after return address)
JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller
__Draw_Line_d:
LDA 2,S         ; Get dy from stack (after return address)
JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller
; ============================================================================
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
STD TEMP_YX             ; Guardar en variable temporal (evita stack)
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX             ; Recuperar y,x
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSL_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W1
; Loop de dibujo
DSL_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSL_DONE           ; Exit if end (long branch)
CMPA #1                 ; Check next path marker
LBEQ DSL_NEXT_PATH      ; Process next path (long branch)
; Draw line
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSL_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W2
CLR VIA_shift_reg
LBRA DSL_LOOP            ; Long branch back to loop start
; Next path: read new intensity and header, then continue drawing
DSL_NEXT_PATH:
; Save current X position before reading anything
TFR X,D                 ; D = X (current position)
PSHS D                  ; Save X address
LDA ,X+                 ; Read intensity (X now points to y_start)
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start (X now points to next_y)
STD TEMP_YX             ; Save y,x
PULS A                  ; Get intensity back
PSHS A                  ; Save intensity again
LDA #$D0
TFR A,DP                ; Set DP=$D0 (BIOS requirement)
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)
; Restore X to point to next_y,next_x (after the 3 bytes we read)
PULS D                  ; Get original X
ADDD #3                 ; Skip intensity, y_start, x_start
TFR D,X                 ; X now points to next_y
; Reset to zero (same as Draw_Sync_List start)
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move
DSL_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W3
CLR VIA_shift_reg       ; Clear before continuing
LBRA DSL_LOOP            ; Continue drawing - LONG BRANCH
DSL_DONE:
RTS

; ============================================================================
; Draw_Sync_List_At - Draw vector at offset position (DRAW_VEC_X, DRAW_VEC_Y)
; Same as Draw_Sync_List but adds offset to y_start, x_start coordinates
; Uses: DRAW_VEC_X, DRAW_VEC_Y (set by DRAW_VECTOR before calling this)
; ============================================================================
Draw_Sync_List_At:
LDA ,X+                 ; intensity
PSHS A                  ; Save intensity
LDA #$D0
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX             ; Recuperar y,x ajustado
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSLA_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W1
; Loop de dibujo (same as Draw_Sync_List)
DSLA_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSLA_DONE
CMPA #1                 ; Check next path marker
LBEQ DSLA_NEXT_PATH
; Draw line
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSLA_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W2
CLR VIA_shift_reg
LBRA DSLA_LOOP           ; Long branch
; Next path: add offset to new coordinates too
DSLA_NEXT_PATH:
TFR X,D
PSHS D
LDA ,X+                 ; Read intensity
PSHS A
LDB ,X+                 ; y_start
ADDB DRAW_VEC_Y         ; Add Y offset to new path
LDA ,X+                 ; x_start
ADDA DRAW_VEC_X         ; Add X offset to new path
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position (already offset-adjusted)
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSLA_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W3
CLR VIA_shift_reg
LBRA DSLA_LOOP           ; Long branch
DSLA_DONE:
RTS
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
; NOTE: Caller must ensure DP=$D0 for VIA access
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec (already relative to center)
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_Y
NEGB                    ; ← Negate Y if flag set
DSWM_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec (already relative to center)
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_X
NEGA                    ; ← Negate X if flag set
DSWM_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSWM_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W1
; Loop de dibujo (conditional mirrors)
DSWM_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSWM_DONE
CMPA #1                 ; Check next path marker
LBEQ DSWM_NEXT_PATH
; Draw line with conditional negations
LDB ,X+                 ; dy
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_DY
NEGB                    ; ← Negate dy if flag set
DSWM_NO_NEGATE_DY:
LDA ,X+                 ; dx
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_DX
NEGA                    ; ← Negate dx if flag set
DSWM_NO_NEGATE_DX:
PSHS A                  ; Save final dx
STB VIA_port_a          ; dy (possibly negated) to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore final dx
STA VIA_port_a          ; dx (possibly negated) to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSWM_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W2
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
; Next path: repeat mirror logic for new path header
DSWM_NEXT_PATH:
TFR X,D
PSHS D
; Check intensity override (same logic as start)
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_NEXT_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_NEXT_SET_INTENSITY
DSWM_NEXT_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_NEXT_SET_INTENSITY:
PSHS A
LDB ,X+                 ; y_start
TST MIRROR_Y
BEQ DSWM_NEXT_NO_NEGATE_Y
NEGB
DSWM_NEXT_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start
TST MIRROR_X
BEQ DSWM_NEXT_NO_NEGATE_X
NEGA
DSWM_NEXT_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSWM_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W3
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
DSWM_DONE:
RTS
; === LOAD_LEVEL_RUNTIME ===
; Load level data from ROM and build dynamic objects index
; Input: X = pointer to level data in ROM
; Output: LEVEL_PTR = pointer to level header (persistent)
;         RESULT    = pointer to level header (return value)
;         LEVEL_DYNAMIC_COUNT = number of dynamic objects found
;         LEVEL_DYNAMIC_BUFFER = state for dynamic objects (6 bytes each)
; 
; OPTIMIZATION: Static objects (physicsEnabled=false) are NOT copied to RAM.
; They remain in ROM and are rendered directly. Only dynamic objects get RAM state.
; Dynamic state: rom_index(1), vel_x(2), vel_y(2), flags(1) = 6 bytes per object
LOAD_LEVEL_RUNTIME:
    PSHS D,X,Y,U     ; Preserve registers
    
    ; Store level pointer persistently
    STX LEVEL_PTR
    
    ; Skip world bounds (8 bytes) + time/score (4 bytes)
    LEAX 12,X        ; X now points to object counts
    
    ; Read object counts
    LDB ,X+          ; B = bgCount
    STB LEVEL_BG_COUNT
    LDB ,X+          ; B = gameplayCount
    STB LEVEL_GP_COUNT
    LDB ,X+          ; B = fgCount
    STB LEVEL_FG_COUNT
    
    ; Read layer pointers (ROM)
    LDD ,X++         ; D = bgObjectsPtr (ROM)
    STD LEVEL_BG_ROM_PTR
    LDD ,X++         ; D = gameplayObjectsPtr (ROM)
    STD LEVEL_GP_ROM_PTR
    LDD ,X++         ; D = fgObjectsPtr (ROM)
    STD LEVEL_FG_ROM_PTR
    
    ; === Clear dynamic buffer ===
    CLR LEVEL_DYNAMIC_COUNT
    LDA #$FF         ; Empty marker
    LDX #LEVEL_DYNAMIC_BUFFER
    LDB #72          ; 12 objects * 6 bytes
LLR_CLEAR_LOOP:
    STA ,X+
    DECB
    BNE LLR_CLEAR_LOOP
    
    ; === Scan ALL layers for dynamic objects ===
    ; U = dynamic buffer write pointer
    LDU #LEVEL_DYNAMIC_BUFFER
    
    ; Scan background layer
    LDB LEVEL_BG_COUNT
    BEQ LLR_SKIP_BG_SCAN
    LDX LEVEL_BG_ROM_PTR
    LDA #0           ; Start at index 0
    JSR LLR_SCAN_LAYER
LLR_SKIP_BG_SCAN:
    
    ; Scan gameplay layer
    LDB LEVEL_GP_COUNT
    BEQ LLR_SKIP_GP_SCAN
    LDX LEVEL_GP_ROM_PTR
    LDA LEVEL_BG_COUNT  ; Offset index by BG count
    JSR LLR_SCAN_LAYER
LLR_SKIP_GP_SCAN:
    
    ; Scan foreground layer
    LDB LEVEL_FG_COUNT
    BEQ LLR_SKIP_FG_SCAN
    LDX LEVEL_FG_ROM_PTR
    LDA LEVEL_BG_COUNT
    ADDA LEVEL_GP_COUNT  ; Offset by BG + GP count
    JSR LLR_SCAN_LAYER
LLR_SKIP_FG_SCAN:
    
    ; Return level pointer in RESULT
    LDX LEVEL_PTR
    STX RESULT
    
    PULS D,X,Y,U,PC  ; Restore and return
    
; === Subroutine: Scan Layer for Dynamic Objects ===
; Scan ROM objects and copy state for dynamic objects only
; Input: A = starting rom_index (for this layer)
;        B = object count in layer
;        X = ROM pointer to first object in layer
;        U = dynamic buffer write pointer
; Output: U = advanced past any added dynamic objects
;         LEVEL_DYNAMIC_COUNT = updated
; Object structure in ROM (24 bytes):
;   +0: type, +1: sprite_id, +2-3: x, +4-5: y
;   +6-7: width, +8-9: height
;   +10-11: velocity_x, +12-13: velocity_y
;   +14-15: flags (bit 0 = physicsEnabled/dynamic)
;   +16: intensity, +17: scale, +18: rotation, +19: collision_size
;   +20-21: spawn_delay, +22-23: vector_ptr
LLR_SCAN_LAYER:
    PSHS A           ; Save rom_index counter
LLR_SCAN_LOOP:
    TSTB
    BEQ LLR_SCAN_DONE
    
    ; Check if dynamic: Read flags at offset +14
    LDA 14,X         ; A = flags low byte
    ANDA #$01        ; Test bit 0 (physicsEnabled)
    BEQ LLR_SCAN_NEXT  ; Skip if static
    
    ; Dynamic object found - check if buffer full
    LDA LEVEL_DYNAMIC_COUNT
    CMPA #12
    BHS LLR_SCAN_OVERFLOW  ; Skip if >= 12 (buffer full)
    
    ; Copy to dynamic buffer (6 bytes):
    ; +0: rom_index
    PULS A           ; Get rom_index
    STA ,U+
    PSHS A           ; Save it back
    
    ; +1-2: velocity_x (from ROM offset +10-11)
    LDD 10,X
    STA ,U+          ; High byte
    STB ,U+          ; Low byte
    
    ; +3-4: velocity_y (from ROM offset +12-13)
    LDD 12,X
    STA ,U+          ; High byte
    STB ,U+          ; Low byte
    
    ; +5: active_flags (initialize to 0x01 = active)
    LDA #$01
    STA ,U+
    
    ; Increment dynamic count
    INC LEVEL_DYNAMIC_COUNT
    
LLR_SCAN_NEXT:
    ; Advance to next object (24 bytes)
    LEAX 24,X
    
    ; Increment rom_index
    PULS A
    INCA
    PSHS A
    
    DECB
    BRA LLR_SCAN_LOOP
    
LLR_SCAN_OVERFLOW:
    ; Buffer full - skip remaining objects
    ; TODO: Could emit warning/error in debug builds
LLR_SCAN_DONE:
    PULS A,PC        ; Restore and return

; === SHOW_LEVEL_RUNTIME ===
; Draw all level objects from loaded level
; Input: LEVEL_PTR = pointer to level data
; Level structure (from levelres.rs):
;   +0:  FDB xMin, xMax (world bounds)
;   +4:  FDB yMin, yMax
;   +8:  FDB timeLimit, targetScore
;   +12: FCB bgCount, gameplayCount, fgCount
;   +15: FDB bgObjectsPtr, gameplayObjectsPtr, fgObjectsPtr
; Object structure (20 bytes each):
;   +0:  FCB type
;   +1:  FDB x, y (position)
;   +5:  FDB scale (8.8 fixed point)
;   +7:  FCB rotation, intensity
;   +9:  FCB velocity_x, velocity_y
;   +11: FCB physics_flags, collision_flags, collision_size
;   +14: FDB spawn_delay
;   +16: FDB vector_ptr
;   +18: FDB properties_ptr
SHOW_LEVEL_RUNTIME:
    PSHS D,X,Y,U     ; Preserve registers
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access - ONCE at start)
    
    ; Get level pointer (persistent)
    LDX >LEVEL_PTR
    CMPX #0
    BEQ SLR_DONE     ; No level loaded
    
    ; Skip world bounds (8 bytes) + time/score (4 bytes)
    LEAX 12,X        ; X now points to object counts
    
    ; Read object counts (use LDB+STB to ensure 1-byte operations)
    LDB ,X+          ; B = bgCount
    STB >LEVEL_BG_COUNT
    LDB ,X+          ; B = gameplayCount
    STB >LEVEL_GP_COUNT
    LDB ,X+          ; B = fgCount
    STB >LEVEL_FG_COUNT
    
    ; Read layer pointers
    LDD ,X++         ; D = bgObjectsPtr
    STD >LEVEL_BG_PTR
    LDD ,X++         ; D = gameplayObjectsPtr
    STD >LEVEL_GP_PTR
    LDD ,X++         ; D = fgObjectsPtr
    STD >LEVEL_FG_PTR
    
    ; === Draw Background Layer ===
SLR_BG_COUNT:
    CLRB             ; Clear high byte to prevent corruption
    LDB >LEVEL_BG_COUNT
    CMPB #0
    BEQ SLR_GAMEPLAY
SLR_BG_PTR:
    LDX #LEVEL_BG_BUFFER ; Read from RAM buffer (not ROM)
    JSR SLR_DRAW_OBJECTS
    
    ; === Draw Gameplay Layer ===
SLR_GAMEPLAY:
SLR_GP_COUNT:
    CLRB             ; Clear high byte to prevent corruption
    LDB >LEVEL_GP_COUNT
    CMPB #0
    BEQ SLR_FOREGROUND
SLR_GP_PTR:
    LDX #LEVEL_GP_BUFFER ; Read from RAM buffer (not ROM)
    JSR SLR_DRAW_OBJECTS
    
    ; === Draw Foreground Layer ===
SLR_FOREGROUND:
SLR_FG_COUNT:
    CLRB             ; Clear high byte to prevent corruption
    LDB >LEVEL_FG_COUNT
    CMPB #0
    BEQ SLR_DONE
SLR_FG_PTR:
    LDX #LEVEL_FG_BUFFER ; Read from RAM buffer (not ROM)
    JSR SLR_DRAW_OBJECTS
    
SLR_DONE:
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access - ONCE at end)
    PULS D,X,Y,U,PC  ; Restore and return
    
; === Subroutine: Draw N Objects ===
; Input: B = count, X = objects ptr
; Each object is 20 bytes
SLR_DRAW_OBJECTS:
    ; NOTE: Use register-based loop (no stack juggling).
    ; Input: B = count, X = objects ptr. Clobbers B,X,Y,U.
SLR_OBJ_LOOP:
    TSTB             ; Test if count is zero
    BEQ SLR_OBJ_DONE ; Exit if zero (prevents off-by-one)
    
    PSHS B           ; CRITICAL: Save counter (B gets clobbered by LDD operations)
    
    ; X points to current object (20 bytes)
    ; Structure: FCB type (+0), FDB x (+1), FDB y (+3), FDB scale (+5),
    ;           FCB rotation (+7), FCB intensity (+8), ..., FDB vector_ptr (+16)
    
    ; Clear mirror flags (no mirroring support yet)
    CLR MIRROR_X
    CLR MIRROR_Y
    
    ; Read intensity (offset +8) and store as override
    LDA 8,X
    STA DRAW_VEC_INTENSITY
    
    ; Read y position (offset +3-4) - store LSB only
    LDD 3,X          ; WARNING: This modifies B!
    STB DRAW_VEC_Y
    
    ; Read x position (offset +1-2) - store LSB only
    LDD 1,X          ; WARNING: This modifies B!
    STB DRAW_VEC_X
    
    ; Read vector_ptr (offset +16-17)
    LDU 16,X
    PSHS X           ; Save object pointer on stack (Y may be corrupted by Draw_Sync_List)
    TFR U,X          ; X = vector data pointer (points to header)
    
    ; Read path_count from header (byte 0)
    LDB ,X+          ; B = path_count, X now points to pointer table
    PSHS B           ; Save path_count on stack
    
    ; Draw all paths using pointer table (DP already set to $D0 by SHOW_LEVEL_RUNTIME)
SLR_PATH_LOOP:
    PULS B           ; Get remaining count
    TSTB
    BEQ SLR_PATH_DONE
    DECB
    PSHS B           ; Save decremented count
    
    ; Read next path pointer from table (X points to current FDB entry)
    LDU ,X++         ; U = path pointer, X advances to next entry
    PSHS X           ; Save pointer table position
    TFR U,X          ; X = actual path data
    JSR Draw_Sync_List_At_With_Mirrors  ; Draw this path
    PULS X           ; Restore pointer table position
    BRA SLR_PATH_LOOP
    
SLR_PATH_DONE:
    ; NOTE: Counter already popped by PULS B before TSTB - no cleanup needed
    PULS X           ; Restore object pointer from stack
    
    ; Advance to next object (20 bytes)
    LEAX 20,X
    
    PULS B           ; Restore counter
    DECB             ; Decrement count AFTER drawing
    BRA SLR_OBJ_LOOP
    
SLR_OBJ_DONE:
    RTS

; === UPDATE_LEVEL_RUNTIME ===
; Update level state (physics, velocity, spawn delays)
; Applies physics to all objects in 3 layers (bg/gameplay/fg)
; CRITICAL: Works on RAM BUFFERS, not ROM!
;
UPDATE_LEVEL_RUNTIME:
    PSHS U,X,Y,D  ; Preserve all registers
    
    ; === Update Background Objects ===
    LDB LEVEL_BG_COUNT
    CMPB #0
    LBEQ ULR_GAMEPLAY  ; Long branch
    LDU #LEVEL_BG_BUFFER  ; U = RAM buffer
    BSR ULR_UPDATE_LAYER  ; Process objects
    
ULR_GAMEPLAY:
    ; === Update Gameplay Objects ===
    LDB LEVEL_GP_COUNT
    CMPB #0
    LBEQ ULR_FOREGROUND  ; Long branch
    LDU #LEVEL_GP_BUFFER  ; U = RAM buffer
    BSR ULR_UPDATE_LAYER  ; Process objects
    
    ; === Object-to-Object Collisions (GAMEPLAY only) ===
    JSR ULR_GAMEPLAY_COLLISIONS  ; Use JSR for long distance
    
ULR_FOREGROUND:
    ; === Update Foreground Objects ===
    LDB LEVEL_FG_COUNT
    CMPB #0
    LBEQ ULR_EXIT  ; Long branch
    LDU #LEVEL_FG_BUFFER  ; U = RAM buffer
    BSR ULR_UPDATE_LAYER  ; Process objects
    
ULR_EXIT:
    PULS D,Y,X,U  ; Restore registers
    RTS

; === ULR_UPDATE_LAYER - Process all objects in a layer ===
; Input: B = object count, U = buffer base address
; Uses: X for world bounds
ULR_UPDATE_LAYER:
    LDX LEVEL_PTR  ; Load level pointer for world bounds
    CMPX #0
    LBEQ ULR_LAYER_EXIT  ; No level loaded (long branch)
    
ULR_LOOP:
    ; U = pointer to object data (24 bytes per object)
    ; Object structure:
    ; +0: type (1 byte)
    ; +1: x (2 bytes signed)
    ; +3: y (2 bytes signed)
    ; +5: scale (2 bytes - not used by physics)
    ; +7: rotation (1 byte - not used by physics)
    ; +8: intensity (1 byte - not used by physics)
    ; +9: velocity_x (1 byte signed)
    ; +10: velocity_y (1 byte signed)
    ; +11: physics_flags (1 byte)
    ; +12: collision_flags (1 byte)
    ; +13-23: other fields (collision_size, spawn_delay, vector_ptr, properties_ptr)

    ; Check physics_flags (offset +11)
    PSHS B  ; Save loop counter
    LDB 11,U     ; Read flags
    CMPB #0
    LBEQ ULR_NEXT  ; Skip if no physics enabled (long branch)

    ; Check if dynamic physics enabled (bit 0)
    BITB #$01
    LBEQ ULR_NEXT  ; Skip if not dynamic (long branch)

    ; Check if gravity enabled (bit 1)
    BITB #$02
    LBEQ ULR_NO_GRAVITY  ; Long branch

    ; Apply gravity: velocity_y -= 1
    LDB 10,U      ; Read velocity_y
    DECB          ; Subtract gravity
    ; Clamp to -15..+15 (max velocity)
    CMPB #$F1     ; Compare with -15
    BGE ULR_VY_OK
    LDB #$F1      ; Clamp to -15
ULR_VY_OK:
    STB 10,U      ; Store updated velocity_y

ULR_NO_GRAVITY:
    ; Apply velocity to position
    ; x += velocity_x
    LDB 9,U       ; Load velocity_x (signed 8-bit)
    SEX           ; Sign-extend B to 16-bit in D
    ADDD 1,U      ; D = x + velocity_x (x at offset +1)
    STD 1,U       ; Store new x

    ; y += velocity_y
    LDB 10,U      ; Load velocity_y (signed 8-bit)
    SEX           ; Sign-extend B to 16-bit in D
    ADDD 3,U      ; D = y + velocity_y (y at offset +3)
    STD 3,U       ; Store new y

    ; === Check World Bounds (Wall Collisions) ===
    LDB 12,U      ; Load collision_flags
    BITB #$02     ; Check bounce_walls flag (bit 1)
    LBEQ ULR_NEXT  ; Skip bounce if not enabled (long branch)

    ; Load world bounds pointer from LEVEL_PTR
    LDX LEVEL_PTR
    ; LEVEL_PTR → +0: xMin, +2: xMax, +4: yMin, +6: yMax (direct values)

    ; === Check X Bounds (Left/Right walls) ===
    ; Check xMin: if (x - collision_size) < xMin then bounce
    LDB 13,U      ; collision_size (offset +13)
    SEX           ; Sign-extend to 16-bit in D
    PSHS D        ; Save collision_size on stack
    LDD 1,U       ; Load object x
    SUBD ,S++     ; D = x - collision_size (left edge), pop stack
    CMPD 0,X      ; Compare left edge with xMin
    LBGE ULR_X_MAX_CHECK  ; Skip if left_edge >= xMin (LONG)
    ; Hit xMin wall - only bounce if moving left (velocity_x < 0)
    LDB 9,U       ; velocity_x
    CMPB #0
    LBGE ULR_X_MAX_CHECK  ; Skip if moving right (LONG)
    ; Bounce: set position so left edge = xMin
    LDB 13,U      ; Reload collision_size
    SEX
    ADDD 0,X      ; D = xMin + collision_size (center position)
    STD 1,U       ; x = xMin + collision_size
    LDB 9,U       ; Reload velocity_x
    NEGB          ; velocity_x = -velocity_x
    STB 9,U

    ; Check xMax: if (x + collision_size) > xMax then bounce
ULR_X_MAX_CHECK:
    LDB 13,U      ; Reload collision_size
    SEX
    PSHS D        ; Save collision_size on stack
    LDD 1,U       ; Load object x
    ADDD ,S++     ; D = x + collision_size (right edge), pop stack
    CMPD 2,X      ; Compare right edge with xMax
    LBLE ULR_Y_BOUNDS  ; Skip if right_edge <= xMax (LONG)
    ; Hit xMax wall - only bounce if moving right (velocity_x > 0)
    LDB 9,U       ; velocity_x
    CMPB #0
    LBLE ULR_Y_BOUNDS  ; Skip if moving left (LONG)
    ; Bounce: set position so right edge = xMax
    LDB 13,U      ; Reload collision_size
    SEX
    TFR D,Y       ; Y = collision_size
    LDD 2,X       ; D = xMax
    PSHS Y        ; Push collision_size
    SUBD ,S++     ; D = xMax - collision_size (center position), pop
    STD 1,U       ; x = xMax - collision_size
    LDB 9,U       ; Reload velocity_x
    NEGB          ; velocity_x = -velocity_x
    STB 9,U

    ; === Check Y Bounds (Top/Bottom walls) ===
ULR_Y_BOUNDS:
    ; Check yMin: if (y - collision_size) < yMin then bounce
    LDB 13,U      ; Reload collision_size
    SEX
    PSHS D        ; Save collision_size on stack
    LDD 3,U       ; Load object y
    SUBD ,S++     ; D = y - collision_size (bottom edge), pop stack
    CMPD 4,X      ; Compare bottom edge with yMin
    LBGE ULR_Y_MAX_CHECK  ; Skip if bottom_edge >= yMin (LONG)
    ; Hit yMin wall - only bounce if moving down (velocity_y < 0)
    LDB 10,U      ; velocity_y
    CMPB #0
    LBGE ULR_Y_MAX_CHECK  ; Skip if moving up (LONG)
    ; Bounce: set position so bottom edge = yMin
    LDB 13,U      ; Reload collision_size
    SEX
    ADDD 4,X      ; D = yMin + collision_size (center position)
    STD 3,U       ; y = yMin + collision_size
    LDB 10,U      ; Reload velocity_y
    NEGB          ; velocity_y = -velocity_y
    STB 10,U

    ; Check yMax: if (y + collision_size) > yMax then bounce
ULR_Y_MAX_CHECK:
    LDB 13,U      ; Reload collision_size
    SEX
    PSHS D        ; Save collision_size on stack
    LDD 3,U       ; Load object y
    ADDD ,S++     ; D = y + collision_size (top edge), pop stack
    CMPD 6,X      ; Compare top edge with yMax
    LBLE ULR_NEXT  ; Skip if top_edge <= yMax (LONG)
    ; Hit yMax wall - only bounce if moving up (velocity_y > 0)
    LDB 10,U      ; velocity_y
    CMPB #0
    LBLE ULR_NEXT  ; Skip if moving down (LONG)
    ; Bounce: set position so top edge = yMax
    LDB 13,U      ; Reload collision_size
    SEX
    TFR D,Y       ; Y = collision_size
    LDD 6,X       ; D = yMax
    PSHS Y        ; Push collision_size
    SUBD ,S++     ; D = yMax - collision_size (center position), pop
    STD 3,U       ; y = yMax - collision_size
    LDB 10,U      ; Reload velocity_y
    NEGB          ; velocity_y = -velocity_y
    STB 10,U

ULR_NEXT:
    PULS B        ; Restore loop counter
    LEAU 20,U     ; Move to next object (20 bytes)
    DECB
    LBNE ULR_LOOP  ; Continue if more objects (long branch)

ULR_LAYER_EXIT:
    RTS

; === ULR_GAMEPLAY_COLLISIONS - Check collisions between gameplay objects ===
; Input: None (uses LEVEL_GP_BUFFER and LEVEL_GP_COUNT)
ULR_GAMEPLAY_COLLISIONS:
    ; Ultra-simple algorithm: NO stack juggling, use RAM variables
    LDA LEVEL_GP_COUNT
    CMPA #2
    BHS UGPC_START   ; Continue if >=2
    RTS              ; Early exit
UGPC_START:
    
    ; Store count-1 in temporary RAM (we'll iterate up to this)
    DECA
    STA UGPC_OUTER_MAX   ; Store at RESULT+20 (temp storage)
    CLR UGPC_OUTER_IDX   ; Start at 0
    
UGPC_OUTER_LOOP:
    ; Calculate U = LEVEL_GP_BUFFER + (UGPC_OUTER_IDX * 20)
    LDU #LEVEL_GP_BUFFER
    LDB UGPC_OUTER_IDX
    BEQ UGPC_SKIP_OUTER_MUL  ; If idx=0, U already correct
UGPC_OUTER_MUL:
    LEAU 20,U
    DECB
    BNE UGPC_OUTER_MUL
UGPC_SKIP_OUTER_MUL:
    
    ; Check if collidable
    LDB 12,U
    BITB #$01
    BEQ UGPC_NEXT_OUTER
    
    ; Inner loop: check against all objects AFTER current
    LDA UGPC_OUTER_IDX
    INCA             ; Start from next object
    STA UGPC_INNER_IDX
    
UGPC_INNER_LOOP:
    ; Check if inner reached count
    LDA UGPC_INNER_IDX
    CMPA LEVEL_GP_COUNT
    BHS UGPC_INNER_DONE  ; Done if idx >= count
    
    ; Calculate Y = LEVEL_GP_BUFFER + (UGPC_INNER_IDX * 20)
    LDY #LEVEL_GP_BUFFER
    LDB UGPC_INNER_IDX
    BEQ UGPC_SKIP_INNER_MUL
UGPC_INNER_MUL:
    LEAY 20,Y
    DECB
    BNE UGPC_INNER_MUL
UGPC_SKIP_INNER_MUL:
    
    ; Check if Y collidable
    LDB 12,Y
    BITB #$01
    BEQ UGPC_NEXT_INNER
    
    ; Manhattan distance |x1-x2| + |y1-y2|
    LDD 1,U          ; x1
    SUBD 1,Y         ; x1-x2
    BPL UGPC_DX_POS
    COMA
    COMB
    ADDD #1
UGPC_DX_POS:
    STD UGPC_DX      ; Store |dx| in temp
    
    LDD 3,U          ; y1
    SUBD 3,Y         ; y1-y2
    BPL UGPC_DY_POS
    COMA
    COMB
    ADDD #1
UGPC_DY_POS:
    ADDD UGPC_DX     ; distance = |dx| + |dy|
    STD UGPC_DIST
    
    ; Sum of radii
    LDB 13,U
    ADDB 13,Y
    SEX              ; D = sum_radius (normal, not doubled)
    ; Collision if distance < sum_radius (i.e., sum_radius > distance)
    CMPD UGPC_DIST   ; Compare sum_radius with distance
    BHI UGPC_COLLISION  ; Jump to collision if sum_radius > distance
    BRA UGPC_NEXT_INNER ; No collision, skip
    
UGPC_COLLISION:
    ; COLLISION! Swap velocities (elastic collision)
    ; Swap velocity_x
    LDA 9,U          ; A = vel_x of object 1
    LDB 9,Y          ; B = vel_x of object 2
    STB 9,U          ; Object 1 gets object 2's vel_x
    STA 9,Y          ; Object 2 gets object 1's vel_x
    ; Swap velocity_y
    LDA 10,U         ; A = vel_y of object 1
    LDB 10,Y         ; B = vel_y of object 2
    STB 10,U         ; Object 1 gets object 2's vel_y
    STA 10,Y         ; Object 2 gets object 1's vel_y
    
UGPC_NEXT_INNER:
    INC UGPC_INNER_IDX
    LBRA UGPC_INNER_LOOP
    
UGPC_INNER_DONE:
UGPC_NEXT_OUTER:
    INC UGPC_OUTER_IDX
    LDA UGPC_OUTER_IDX
    CMPA UGPC_OUTER_MAX
    BHI UGPC_EXIT    ; Exit if idx > max
    LBRA UGPC_OUTER_LOOP  ; Continue (LONG)
    
UGPC_EXIT:
    RTS
    
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:7
    ; VPy_LINE:8
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 8
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:10
; LOAD_LEVEL("test_level") - load level data
    LDX #_TEST_LEVEL_LEVEL
    JSR LOAD_LEVEL_RUNTIME
    LDD RESULT  ; Returns level pointer

MAIN:
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
    ; === Initialize Joystick (one-time setup) ===
    CLR $C823    ; CRITICAL: Clear analog mode flag (Joy_Analog does DEC on this)
    LDA #$01     ; CRITICAL: Resolution threshold (power of 2: $40=fast, $01=accurate)
    STA $C81A    ; Vec_Joy_Resltn (loop terminates when B=this value after LSRBs)
    LDA #$01
    STA $C81F    ; Vec_Joy_Mux_1_X (enable X axis reading)
    LDA #$03
    STA $C820    ; Vec_Joy_Mux_1_Y (enable Y axis reading)
    LDA #$00
    STA $C821    ; Vec_Joy_Mux_2_X (disable joystick 2 - CRITICAL!)
    STA $C822    ; Vec_Joy_Mux_2_Y (disable joystick 2 - saves cycles)
    ; Mux configured - J1_X()/J1_Y() can now be called

    ; JSR Wait_Recal is now called at start of LOOP_BODY (see auto-inject)
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

    ; VPy_LINE:12
LOOP_BODY:
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:13
; SHOW_LEVEL() - draw all level objects
    JSR SHOW_LEVEL_RUNTIME
    LDD #0
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:15
; NATIVE_CALL: UPDATE_LEVEL at line 15
    JSR UPDATE_LEVEL_RUNTIME
    CLRA
    CLRB
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; ASSET DATA SECTION
; Embedded 5 of 7 assets (unused assets excluded)
; ========================================

; Vector asset: coin
; Generated from coin.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-8, max=8, width=16
; Center: (0, 0)

_COIN_WIDTH EQU 16
_COIN_CENTER_X EQU 0
_COIN_CENTER_Y EQU 0

_COIN_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _COIN_PATH0        ; pointer to path 0

_COIN_PATH0:    ; Path 0
    FCB 51              ; path0: intensity
    FCB $08,$00,0,0        ; path0: header (y=8, x=0, relative to center)
    FCB $FF,$FE,$06          ; line 0: flag=-1, dy=-2, dx=6
    FCB $FF,$FA,$02          ; line 1: flag=-1, dy=-6, dx=2
    FCB $FF,$FA,$FE          ; line 2: flag=-1, dy=-6, dx=-2
    FCB $FF,$FE,$FA          ; line 3: flag=-1, dy=-2, dx=-6
    FCB $FF,$02,$FA          ; line 4: flag=-1, dy=2, dx=-6
    FCB $FF,$06,$FE          ; line 5: flag=-1, dy=6, dx=-2
    FCB $FF,$06,$02          ; line 6: flag=-1, dy=6, dx=2
    FCB $FF,$02,$06          ; closing line: flag=-1, dy=2, dx=6
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_huge
; Generated from bubble_huge.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-25, max=27, width=52
; Center: (1, 0)

_BUBBLE_HUGE_WIDTH EQU 52
_BUBBLE_HUGE_CENTER_X EQU 1
_BUBBLE_HUGE_CENTER_Y EQU 0

_BUBBLE_HUGE_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _BUBBLE_HUGE_PATH0        ; pointer to path 0

_BUBBLE_HUGE_PATH0:    ; Path 0
    FCB 49              ; path0: intensity
    FCB $00,$1A,0,0        ; path0: header (y=0, x=26, relative to center)
    FCB $FF,$12,$F8          ; line 0: flag=-1, dy=18, dx=-8
    FCB $FF,$08,$EE          ; line 1: flag=-1, dy=8, dx=-18
    FCB $FF,$F8,$EE          ; line 2: flag=-1, dy=-8, dx=-18
    FCB $FF,$EE,$F8          ; line 3: flag=-1, dy=-18, dx=-8
    FCB $FF,$EE,$08          ; line 4: flag=-1, dy=-18, dx=8
    FCB $FF,$F8,$12          ; line 5: flag=-1, dy=-8, dx=18
    FCB $FF,$08,$12          ; line 6: flag=-1, dy=8, dx=18
    FCB $FF,$12,$08          ; closing line: flag=-1, dy=18, dx=8
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_large
; Generated from bubble_large.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-15, max=15, width=30
; Center: (0, 0)

_BUBBLE_LARGE_WIDTH EQU 30
_BUBBLE_LARGE_CENTER_X EQU 0
_BUBBLE_LARGE_CENTER_Y EQU 0

_BUBBLE_LARGE_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _BUBBLE_LARGE_PATH0        ; pointer to path 0

_BUBBLE_LARGE_PATH0:    ; Path 0
    FCB 49              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$FB,$0A          ; line 0: flag=-1, dy=-5, dx=10
    FCB $FF,$F6,$05          ; line 1: flag=-1, dy=-10, dx=5
    FCB $FF,$F6,$FB          ; line 2: flag=-1, dy=-10, dx=-5
    FCB $FF,$FB,$F6          ; line 3: flag=-1, dy=-5, dx=-10
    FCB $FF,$05,$F6          ; line 4: flag=-1, dy=5, dx=-10
    FCB $FF,$0A,$FB          ; line 5: flag=-1, dy=10, dx=-5
    FCB $FF,$0A,$05          ; line 6: flag=-1, dy=10, dx=5
    FCB $FF,$05,$0A          ; closing line: flag=-1, dy=5, dx=10
    FCB 2                ; End marker (path complete)

; Vector asset: fuji_bg
; Generated from fuji_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 64
; X bounds: min=-124, max=125, width=249
; Center: (0, 12)

_FUJI_BG_WIDTH EQU 249
_FUJI_BG_CENTER_X EQU 0
_FUJI_BG_CENTER_Y EQU 12

_FUJI_BG_VECTORS:  ; Main entry (header + 5 path(s))
    FCB 5               ; path_count (runtime metadata)
    FDB _FUJI_BG_PATH0        ; pointer to path 0
    FDB _FUJI_BG_PATH1        ; pointer to path 1
    FDB _FUJI_BG_PATH2        ; pointer to path 2
    FDB _FUJI_BG_PATH3        ; pointer to path 3
    FDB _FUJI_BG_PATH4        ; pointer to path 4

_FUJI_BG_PATH0:    ; Path 0
    FCB 80              ; path0: intensity
    FCB $DC,$84,0,0        ; path0: header (y=-36, x=-124, relative to center)
    FCB $FF,$0A,$1E          ; line 0: flag=-1, dy=10, dx=30
    FCB $FF,$0E,$1E          ; line 1: flag=-1, dy=14, dx=30
    FCB $FF,$0F,$15          ; line 2: flag=-1, dy=15, dx=21
    FCB $FF,$11,$17          ; line 3: flag=-1, dy=17, dx=23
    FCB $FF,$0E,$0E          ; line 4: flag=-1, dy=14, dx=14
    FCB $FF,$FE,$03          ; line 5: flag=-1, dy=-2, dx=3
    FCB $FF,$03,$04          ; line 6: flag=-1, dy=3, dx=4
    FCB $FF,$FE,$04          ; line 7: flag=-1, dy=-2, dx=4
    FCB $FF,$01,$07          ; line 8: flag=-1, dy=1, dx=7
    FCB $FF,$02,$04          ; line 9: flag=-1, dy=2, dx=4
    FCB $FF,$FD,$06          ; line 10: flag=-1, dy=-3, dx=6
    FCB $FF,$03,$03          ; line 11: flag=-1, dy=3, dx=3
    FCB $FF,$EB,$11          ; line 12: flag=-1, dy=-21, dx=17
    FCB $FF,$F4,$11          ; line 13: flag=-1, dy=-12, dx=17
    FCB $FF,$F0,$16          ; line 14: flag=-1, dy=-16, dx=22
    FCB $FF,$F6,$14          ; line 15: flag=-1, dy=-10, dx=20
    FCB $FF,$F6,$18          ; line 16: flag=-1, dy=-10, dx=24
    FCB $FF,$00,$00          ; line 17: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH1:    ; Path 1
    FCB 95              ; path1: intensity
    FCB $0E,$F1,0,0        ; path1: header (y=14, x=-15, relative to center)
    FCB $FF,$06,$03          ; line 0: flag=-1, dy=6, dx=3
    FCB $FF,$04,$03          ; line 1: flag=-1, dy=4, dx=3
    FCB $FF,$FD,$04          ; line 2: flag=-1, dy=-3, dx=4
    FCB $FF,$FC,$FC          ; line 3: flag=-1, dy=-4, dx=-4
    FCB $FF,$FD,$FA          ; line 4: flag=-1, dy=-3, dx=-6
    FCB $FF,$00,$00          ; line 5: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH2:    ; Path 2
    FCB 95              ; path2: intensity
    FCB $13,$07,0,0        ; path2: header (y=19, x=7, relative to center)
    FCB $FF,$F9,$FD          ; line 0: flag=-1, dy=-7, dx=-3
    FCB $FF,$FA,$02          ; line 1: flag=-1, dy=-6, dx=2
    FCB $FF,$F9,$FD          ; line 2: flag=-1, dy=-7, dx=-3
    FCB $FF,$FD,$04          ; line 3: flag=-1, dy=-3, dx=4
    FCB $FF,$08,$03          ; line 4: flag=-1, dy=8, dx=3
    FCB $FF,$07,$FE          ; line 5: flag=-1, dy=7, dx=-2
    FCB $FF,$06,$01          ; line 6: flag=-1, dy=6, dx=1
    FCB $FF,$02,$FE          ; line 7: flag=-1, dy=2, dx=-2
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH3:    ; Path 3
    FCB 95              ; path3: intensity
    FCB $15,$18,0,0        ; path3: header (y=21, x=24, relative to center)
    FCB $FF,$F7,$05          ; line 0: flag=-1, dy=-9, dx=5
    FCB $FF,$F7,$0C          ; line 1: flag=-1, dy=-9, dx=12
    FCB $FF,$0B,$FA          ; line 2: flag=-1, dy=11, dx=-6
    FCB $FF,$07,$F5          ; line 3: flag=-1, dy=7, dx=-11
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH4:    ; Path 4
    FCB 100              ; path4: intensity
    FCB $F9,$C7,0,0        ; path4: header (y=-7, x=-57, relative to center)
    FCB $FF,$09,$1A          ; line 0: flag=-1, dy=9, dx=26
    FCB $FF,$EF,$F2          ; line 1: flag=-1, dy=-17, dx=-14
    FCB $FF,$1B,$22          ; line 2: flag=-1, dy=27, dx=34
    FCB $FF,$F2,$FB          ; line 3: flag=-1, dy=-14, dx=-5
    FCB $FF,$00,$03          ; line 4: flag=-1, dy=0, dx=3
    FCB $FF,$F7,$FB          ; line 5: flag=-1, dy=-9, dx=-5
    FCB $FF,$FA,$01          ; line 6: flag=-1, dy=-6, dx=1
    FCB $FF,$0E,$0E          ; line 7: flag=-1, dy=14, dx=14
    FCB $FF,$F1,$00          ; line 8: flag=-1, dy=-15, dx=0
    FCB $FF,$0A,$05          ; line 9: flag=-1, dy=10, dx=5
    FCB $FF,$EA,$06          ; line 10: flag=-1, dy=-22, dx=6
    FCB $FF,$1C,$05          ; line 11: flag=-1, dy=28, dx=5
    FCB $FF,$EF,$06          ; line 12: flag=-1, dy=-17, dx=6
    FCB $FF,$03,$01          ; line 13: flag=-1, dy=3, dx=1
    FCB $FF,$FD,$04          ; line 14: flag=-1, dy=-3, dx=4
    FCB $FF,$0B,$03          ; line 15: flag=-1, dy=11, dx=3
    FCB $FF,$F5,$05          ; line 16: flag=-1, dy=-11, dx=5
    FCB $FF,$10,$FF          ; line 17: flag=-1, dy=16, dx=-1
    FCB $FF,$EE,$13          ; line 18: flag=-1, dy=-18, dx=19
    FCB $FF,$12,$F7          ; line 19: flag=-1, dy=18, dx=-9
    FCB $FF,$F9,$0E          ; line 20: flag=-1, dy=-7, dx=14
    FCB $FF,$04,$02          ; line 21: flag=-1, dy=4, dx=2
    FCB $FF,$FC,$14          ; line 22: flag=-1, dy=-4, dx=20
    FCB 2                ; End marker (path complete)

; Level Asset: test_level (from /Users/daniel/projects/vectrex-pseudo-python/examples/level_test/assets/playground/test_level.vplay)
; ==== Level: TEST_LEVEL ====
; Author: 
; Difficulty: medium

_TEST_LEVEL_LEVEL:
    FDB -96  ; World bounds: xMin (16-bit signed)
    FDB 95  ; xMax (16-bit signed)
    FDB -128  ; yMin (16-bit signed)
    FDB 127  ; yMax (16-bit signed)
    FDB 0  ; Time limit (seconds)
    FDB 0  ; Target score
    FCB 1  ; Background object count
    FCB 3  ; Gameplay object count
    FCB 0  ; Foreground object count
    FDB _TEST_LEVEL_BG_OBJECTS
    FDB _TEST_LEVEL_GAMEPLAY_OBJECTS
    FDB _TEST_LEVEL_FG_OBJECTS

_TEST_LEVEL_BG_OBJECTS:
; Object: obj_1767949012188 (enemy)
    FCB 1  ; type
    FDB 0  ; x
    FDB 0  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 0  ; velocity_x
    FCB 0  ; velocity_y
    FCB 0  ; physics_flags
    FCB 0  ; collision_flags
    FCB 10  ; collision_size
    FDB 0  ; spawn_delay
    FDB _FUJI_BG_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_GAMEPLAY_OBJECTS:
; Object: obj_1767862794353 (enemy)
    FCB 1  ; type
    FDB 45  ; x
    FDB 71  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 255  ; velocity_x
    FCB 255  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 27  ; collision_size
    FDB 0  ; spawn_delay
    FDB _BUBBLE_HUGE_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: obj_1767883264744 (enemy)
    FCB 1  ; type
    FDB -52  ; x
    FDB 35  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 2  ; velocity_x
    FCB 1  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 8  ; collision_size
    FDB 0  ; spawn_delay
    FDB _COIN_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: obj_1767873800421 (enemy)
    FCB 1  ; type
    FDB -58  ; x
    FDB 68  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 0  ; velocity_x
    FCB 255  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 15  ; collision_size
    FDB 0  ; spawn_delay
    FDB _BUBBLE_LARGE_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_FG_OBJECTS:


