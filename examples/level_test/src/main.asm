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
; Total RAM used: 28 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0E   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$10   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$11   ; Temporary y storage (1 bytes)
LEVEL_PTR            EQU $C880+$12   ; Current level header pointer (2 bytes)
LEVEL_BG_PTR         EQU $C880+$14   ; Background layer objects pointer (2 bytes)
LEVEL_GAMEPLAY_PTR   EQU $C880+$16   ; Gameplay layer objects pointer (2 bytes)
LEVEL_FG_PTR         EQU $C880+$18   ; Foreground layer objects pointer (2 bytes)
NUM_STR              EQU $C880+$1A   ; String buffer for PRINT_NUMBER (2 bytes)

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

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG2   ; string pointer (ARG2 = third param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG0+1 ; X (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF      ; DP_to_C8 (restore before return - CRITICAL for TMPPTR access)
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
VECTREX_WAIT_RECAL:
    JSR Wait_Recal
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
BRA DSL_LOOP
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
BRA DSL_LOOP            ; Continue drawing
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
BRA DSLA_LOOP
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
BRA DSLA_LOOP
DSLA_DONE:
RTS
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
PSHS A                  ; Save intensity
LDA #$D0
PULS A                  ; Restore intensity
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
BRA DSWM_LOOP
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
BRA DSWM_LOOP
DSWM_DONE:
RTS
; ============================================================================
; DRAW_CIRCLE_RUNTIME - Draw circle with runtime parameters
; ============================================================================
; Follows Draw_Sync_List_At pattern: read params BEFORE DP change
; Inputs: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY (bytes in RAM)
; Uses 8 segments (octagon) with lookup table for efficiency
DRAW_CIRCLE_RUNTIME:
; Read ALL parameters into registers/stack BEFORE changing DP (critical!)
; (These are byte variables, use LDB not LDD)
LDB DRAW_CIRCLE_INTENSITY
PSHS B                 ; Save intensity on stack

LDB DRAW_CIRCLE_DIAM
SEX                    ; Sign-extend to 16-bit (diameter is unsigned 0..255)
LSRA                   ; Divide by 2 to get radius
RORB
STD DRAW_CIRCLE_TEMP   ; DRAW_CIRCLE_TEMP = radius (16-bit)

LDB DRAW_CIRCLE_XC     ; xc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+2 ; Save xc

LDB DRAW_CIRCLE_YC     ; yc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+4 ; Save yc

; NOW safe to setup BIOS (all params are in DRAW_CIRCLE_TEMP+stack)
LDA #$D0
TFR A,DP
JSR Reset0Ref

; Set intensity (from stack)
PULS A                 ; Get intensity from stack
CMPA #$5F
BEQ DCR_intensity_5F
JSR Intensity_a
BRA DCR_after_intensity
DCR_intensity_5F:
JSR Intensity_5F
DCR_after_intensity:

; Move to start position: (xc + radius, yc)
; radius = DRAW_CIRCLE_TEMP, xc = DRAW_CIRCLE_TEMP+2, yc = DRAW_CIRCLE_TEMP+4
LDD DRAW_CIRCLE_TEMP   ; D = radius
ADDD DRAW_CIRCLE_TEMP+2 ; D = xc + radius
TFR B,B                ; Keep X in B (low byte)
PSHS B                 ; Save X on stack
LDD DRAW_CIRCLE_TEMP+4 ; Load yc
TFR B,A                ; Y to A
PULS B                 ; X to B
JSR Moveto_d

; Loop through 8 segments using lookup table
LDX #DCR_DELTA_TABLE   ; Point to delta table
LDB #8                 ; 8 segments
PSHS B                 ; Save counter on stack

DCR_LOOP:
CLR Vec_Misc_Count     ; Relative drawing

; Load delta multipliers from table
LDA ,X+                ; dx multiplier (-1, 0, 1, or 2 for half)
LDB ,X+                ; dy multiplier
PSHS A,B               ; Save multipliers

; Calculate dy = (dy_mult * radius) / 2 if needed
LDD DRAW_CIRCLE_TEMP   ; Load radius
PULS A,B               ; Get multipliers (A=dx_mult, B=dy_mult)
PSHS A                 ; Save dx_mult

; Process dy_mult
TSTB
BEQ DCR_dy_zero        ; dy = 0
CMPB #2
BEQ DCR_dy_half        ; dy = r/2
CMPB #$FE              ; -2 (half negative)
BEQ DCR_dy_neg_half
CMPB #1
BEQ DCR_dy_pos         ; dy = r
; dy = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dy_done
DCR_dy_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dy_done
DCR_dy_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dy_done:
TFR B,A                ; Move dy result to A (we only need 8-bit for Vectrex coordinates)
PSHS A                 ; Save dy on stack

; Process dx_mult (same logic)
LDB 1,S                ; Get dx_mult from stack
TSTB
BEQ DCR_dx_zero
CMPB #2
BEQ DCR_dx_half
CMPB #$FE
BEQ DCR_dx_neg_half
CMPB #1
BEQ DCR_dx_pos
; dx = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dx_done
DCR_dx_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dx_done
DCR_dx_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dx_done:
TFR B,B                ; dx in B
PULS A                 ; dy in A
LEAS 1,S               ; Drop dx_mult

; Draw line with calculated deltas (preserve X - it points to table)
PSHS X                 ; Save table pointer
JSR Draw_Line_d
PULS X                 ; Restore table pointer

; Loop control
DEC ,S                 ; Decrement counter
BNE DCR_LOOP

LEAS 1,S               ; Clean counter from stack

; DP is ALREADY $D0 from BIOS, no need to restore (Draw_Sync_List_At doesn't restore either)
RTS

RTS

; Delta multiplier table: 8 segments (dx_mult, dy_mult)
; 0=zero, 1=r, -1=$FF=-r, 2=r/2, -2=$FE=-r/2
DCR_DELTA_TABLE:
FCB 2,2      ; Seg 1: dx=r/2, dy=r/2 (right-up)
FCB 0,1      ; Seg 2: dx=0, dy=r (up)
FCB $FE,2    ; Seg 3: dx=-r/2, dy=r/2 (left-up)
FCB $FF,0    ; Seg 4: dx=-r, dy=0 (left)
FCB $FE,$FE  ; Seg 5: dx=-r/2, dy=-r/2 (left-down)
FCB 0,$FF    ; Seg 6: dx=0, dy=-r (down)
FCB 2,$FE    ; Seg 7: dx=r/2, dy=-r/2 (right-down)
FCB 1,0      ; Seg 8: dx=r, dy=0 (right)

; LOAD_LEVEL_RUNTIME - Load level from ROM into RAM working variables
; Input: X = pointer to level ROM data (_LEVELNAME_LEVEL)
; Output: Level header copied to RAM, layer pointers extracted
; Modifies: D, X
LOAD_LEVEL_RUNTIME:
STX >LEVEL_PTR          ; Store level header pointer

; Extract layer pointers from header
; Header structure: 8 bytes bounds + 4 bytes metadata + 3 bytes counts = 15 bytes
; Then: 2-byte BG ptr, 2-byte gameplay ptr, 2-byte FG ptr
LEAX 15,X               ; Skip to layer pointers section

; Read background layer pointer
LDD ,X++                ; Load BG pointer (auto-increment by 2)
STD >LEVEL_BG_PTR       ; Store in RAM

; Read gameplay layer pointer
LDD ,X++                ; Load gameplay pointer
STD >LEVEL_GAMEPLAY_PTR ; Store in RAM

; Read foreground layer pointer
LDD ,X                  ; Load FG pointer (no increment needed)
STD >LEVEL_FG_PTR       ; Store in RAM

; Return level pointer in D for convenience
LDD >LEVEL_PTR
STD RESULT              ; Store result
RTS

; GET_OBJECT_COUNT_RUNTIME - Get number of objects in a layer
; Input: D = layer index (0=bg, 1=gameplay, 2=fg)
; Output: RESULT = object count (8-bit)
; Modifies: D, X
GET_OBJECT_COUNT_RUNTIME:
LDX >LEVEL_PTR          ; Load level header pointer
BEQ .GOC_ERROR          ; Error if level not loaded

; Object counts are at offset 12 (after 8 bytes bounds + 4 bytes metadata)
LEAX 12,X               ; X now points to bg_count

; Check layer index in D (low byte = B register)
CMPB #0                 ; Background?
BEQ .GOC_BG
CMPB #1                 ; Gameplay?
BEQ .GOC_GAMEPLAY
CMPB #2                 ; Foreground?
BEQ .GOC_FG
BRA .GOC_ERROR          ; Invalid layer

.GOC_BG:
LDB ,X                  ; Read bg_count (offset 0)
BRA .GOC_DONE

.GOC_GAMEPLAY:
LDB 1,X                 ; Read gameplay_count (offset 1)
BRA .GOC_DONE

.GOC_FG:
LDB 2,X                 ; Read fg_count (offset 2)
BRA .GOC_DONE

.GOC_ERROR:
LDB #0                  ; Return 0 on error

.GOC_DONE:
CLRA                    ; Zero high byte (count is 8-bit)
STD RESULT              ; Store result
RTS

; GET_OBJECT_PTR_RUNTIME - Get pointer to specific object in layer
; Input: RESULT+0 = layer index (0=bg, 1=gameplay, 2=fg)
;        RESULT+2 = object index (0-based)
; Output: RESULT = pointer to object data (22 bytes per object)
; Object structure: type(1), x(2), y(2), scale(2), rotation(2), intensity(1),
;                   velX(2), velY(2), physics_flags(1), collision_flags(1),
;                   width(1), height(1), spawn_delay(2), vector_ptr(2), properties_ptr(2)
; Total: 22 bytes per object
; Modifies: D, X, Y
GET_OBJECT_PTR_RUNTIME:
; Get layer pointer based on layer index
LDB RESULT+1            ; Load layer index (low byte)
CMPB #0                 ; Background?
BEQ .GOP_BG
CMPB #1                 ; Gameplay?
BEQ .GOP_GAMEPLAY
CMPB #2                 ; Foreground?
BEQ .GOP_FG
BRA .GOP_ERROR          ; Invalid layer

.GOP_BG:
LDX >LEVEL_BG_PTR       ; Load background layer base pointer
BRA .GOP_CALC

.GOP_GAMEPLAY:
LDX >LEVEL_GAMEPLAY_PTR ; Load gameplay layer base pointer
BRA .GOP_CALC

.GOP_FG:
LDX >LEVEL_FG_PTR       ; Load foreground layer base pointer

.GOP_CALC:
; X = layer base pointer
; Calculate offset: index * 22 (object size)
LDD RESULT+2            ; Load object index

; Multiply by 22: D = D * 22 = D * 16 + D * 4 + D * 2
; Save original index
PSHS D                  ; Save index

; D * 16
ASLA                    ; Shift left 4 times (x16)
ROLB
ASLA
ROLB
ASLA
ROLB
ASLA
ROLB
STD TMPPTR              ; Store D*16

; D * 4 (from original)
PULS D                  ; Restore original index
PSHS D                  ; Save again
ASLA                    ; Shift left 2 times (x4)
ROLB
ASLA
ROLB
ADDD TMPPTR             ; Add D*16 (now have D*20)
STD TMPPTR              ; Store D*20

; D * 2 (from original)
PULS D                  ; Restore original index
ASLA                    ; Shift left 1 time (x2)
ROLB
ADDD TMPPTR             ; Add D*20 (now have D*22)

; Add offset to base pointer
LEAX D,X                ; X = base + (index * 22)
TFR X,D                 ; Move result to D
STD RESULT              ; Store final pointer
RTS

.GOP_ERROR:
LDD #0                  ; Return null pointer on error
STD RESULT
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
    ; VPy_LINE:8
; LOAD_LEVEL("test_level") - load level data
    LDX #_TEST_LEVEL_LEVEL
    JSR LOAD_LEVEL_RUNTIME
    LDD RESULT  ; Returns level pointer
    LDX RESULT
    LDU #VAR_LEVEL_PTR
    STU TMPPTR
    STX ,U
    ; VPy_LINE:11
    ; StructInit(GET_LEVEL_BOUNDS)
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_BOUNDS
    STU TMPPTR
    STX ,U
    ; VPy_LINE:15
; NATIVE_CALL: GET_OBJECT_COUNT at line 15
; GET_OBJECT_COUNT(layer) - get object count
    LDD #0
    STD RESULT
    LDD RESULT
    JSR GET_OBJECT_COUNT_RUNTIME
    LDD RESULT  ; Returns object count
    LDX RESULT
    LDU #VAR_BG_COUNT
    STU TMPPTR
    STX ,U
    ; VPy_LINE:16
; NATIVE_CALL: GET_OBJECT_COUNT at line 16
; GET_OBJECT_COUNT(layer) - get object count
    LDD #1
    STD RESULT
    LDD RESULT
    JSR GET_OBJECT_COUNT_RUNTIME
    LDD RESULT  ; Returns object count
    LDX RESULT
    LDU #VAR_GAMEPLAY_COUNT
    STU TMPPTR
    STX ,U
    ; VPy_LINE:17
; NATIVE_CALL: GET_OBJECT_COUNT at line 17
; GET_OBJECT_COUNT(layer) - get object count
    LDD #2
    STD RESULT
    LDD RESULT
    JSR GET_OBJECT_COUNT_RUNTIME
    LDD RESULT  ; Returns object count
    LDX RESULT
    LDU #VAR_FG_COUNT
    STU TMPPTR
    STX ,U
    ; VPy_LINE:20
    LDD VAR_BG_COUNT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_GAMEPLAY_COUNT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_FG_COUNT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_TOTAL_OBJECTS
    STU TMPPTR
    STX ,U
    ; VPy_LINE:22
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 22
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
; VPy_LINE:6

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

    ; VPy_LINE:24
LOOP_BODY:
    LEAS -6,S ; allocate locals
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:25
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 25
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:28
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 28
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(8)
    ; VPy_LINE:29
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 29
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:32
; NATIVE_CALL: GET_OBJECT_COUNT at line 32
; GET_OBJECT_COUNT(layer) - get object count
    LDD #1
    STD RESULT
    LDD RESULT
    JSR GET_OBJECT_COUNT_RUNTIME
    LDD RESULT  ; Returns object count
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 4 - Discriminant(9)
    ; VPy_LINE:35
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:36
; NATIVE_CALL: GET_OBJECT_PTR at line 36
; GET_OBJECT_PTR(layer, index) - get object pointer
    LDD #1
    STD RESULT
    LDD RESULT
    STD RESULT+0  ; layer
    LDD #0
    STD RESULT
    LDD RESULT
    STD RESULT+2  ; index
    JSR GET_OBJECT_PTR_RUNTIME
    LDD RESULT  ; Returns object pointer
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:37
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 37
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 5 - Discriminant(9)
    ; VPy_LINE:40
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:41
; NATIVE_CALL: GET_OBJECT_PTR at line 41
; GET_OBJECT_PTR(layer, index) - get object pointer
    LDD #1
    STD RESULT
    LDD RESULT
    STD RESULT+0  ; layer
    LDD #1
    STD RESULT
    LDD RESULT
    STD RESULT+2  ; index
    JSR GET_OBJECT_PTR_RUNTIME
    LDD RESULT  ; Returns object pointer
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:42
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 42
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 6 - Discriminant(8)
    ; VPy_LINE:45
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$64
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(8)
    ; VPy_LINE:46
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$64
    LDB #$00
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(8)
    ; VPy_LINE:47
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$9C
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(8)
    ; VPy_LINE:48
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$9C
    LDB #$00
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    LEAS 6,S ; free locals
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; ========================================
; Level Asset: test_level (from /Users/daniel/projects/vectrex-pseudo-python/examples/level_test/assets/playground/test_level.vplay)
; ========================================
; ==== Level: TEST_LEVEL ====
; Author: VPy Team
; Difficulty: easy

_TEST_LEVEL_LEVEL:
    FCB 156  ; World bounds: xMin
    FCB 100  ; xMax
    FCB 156  ; yMin
    FCB 100  ; yMax
    FDB 120  ; Time limit (seconds)
    FDB 1000  ; Target score
    FCB 1  ; Background object count
    FCB 2  ; Gameplay object count
    FCB 0  ; Foreground object count
    FDB _TEST_LEVEL_BG_OBJECTS
    FDB _TEST_LEVEL_GAMEPLAY_OBJECTS
    FDB _TEST_LEVEL_FG_OBJECTS

_TEST_LEVEL_BG_OBJECTS:
; Object: bg_mountain (background)
    FCB 4  ; type
    FDB 0  ; x
    FDB -50  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 100  ; intensity
    FCB 0  ; velocity_x
    FCB 0  ; velocity_y
    FCB 0  ; physics_flags
    FCB 0  ; collision_flags
    FCB 10  ; collision_size
    FDB 0  ; spawn_delay
    FDB _MOUNTAIN_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_GAMEPLAY_OBJECTS:
; Object: enemy1 (enemy)
    FCB 1  ; type
    FDB 30  ; x
    FDB 0  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 127  ; intensity
    FCB 10  ; velocity_x
    FCB 0  ; velocity_y
    FCB 3  ; physics_flags
    FCB 5  ; collision_flags
    FCB 10  ; collision_size
    FDB 0  ; spawn_delay
    FDB _BUBBLE_LARGE_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: collectible1 (collectible)
    FCB 3  ; type
    FDB -30  ; x
    FDB 30  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 127  ; intensity
    FCB 0  ; velocity_x
    FCB 0  ; velocity_y
    FCB 0  ; physics_flags
    FCB 1  ; collision_flags
    FCB 15  ; collision_size
    FDB 0  ; spawn_delay
    FDB _COIN_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_FG_OBJECTS:


; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "BUILTINS WORKING"
    FCB $80
STR_1:
    FCC "COLLECTIBLE PTR OK"
    FCB $80
STR_2:
    FCC "FIRST ENEMY PTR OK"
    FCB $80
STR_3:
    FCC "LEVEL SYSTEM TEST"
    FCB $80
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+1
MIRROR_X EQU RESULT+2
MIRROR_Y EQU RESULT+3
DRAW_VEC_INTENSITY EQU RESULT+4
DRAW_CIRCLE_XC EQU RESULT+5
DRAW_CIRCLE_YC EQU RESULT+6
DRAW_CIRCLE_DIAM EQU RESULT+7
DRAW_CIRCLE_INTENSITY EQU RESULT+8
DRAW_CIRCLE_TEMP EQU RESULT+9
