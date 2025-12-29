; --- Motorola 6809 backend (Vectrex) title='CIRCLE TEST' origin=$0000 ---
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
    FCC "CIRCLE TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 8 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TEMP_YX              EQU $C880+$02   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$04   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$05   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$06   ; String buffer for PRINT_NUMBER (2 bytes)

    JMP START

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF00 (128=center)
; Returns: D = -1 (left), 0 (center), +1 (right)
J1X_BUILTIN:
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
    CMPB #108    ; Check if < 108 (left)
    BLO .J1X_LEFT
    CMPB #148    ; Check if > 148 (right)
    BHI .J1X_RIGHT
    LDD #0       ; Center
    RTS
.J1X_LEFT:
    LDD #$FFFF   ; Left (-1)
    RTS
.J1X_RIGHT:
    LDD #1       ; Right (+1)
    RTS

; J1_Y() - Read Joystick 1 Y axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF01 (128=center)
; Returns: D = -1 (down), 0 (center), +1 (up)
J1Y_BUILTIN:
    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)
    CMPB #108    ; Check if < 108 (down)
    BLO .J1Y_DOWN
    CMPB #148    ; Check if > 148 (up)
    BHI .J1Y_UP
    LDD #0       ; Center
    RTS
.J1Y_DOWN:
    LDD #$FFFF   ; Down (-1)
    RTS
.J1Y_UP:
    LDD #1       ; Up (+1)
    RTS

; === BUTTON BUILTIN SUBROUTINES ===
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
; Returns: D = 0 (released), 1 (pressed)
J1B1_BUILTIN:
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .J1B1_OFF
    LDD #1
    RTS
.J1B1_OFF:
    LDD #0
    RTS

; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)
J1B2_BUILTIN:
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$02
    BEQ .J1B2_OFF
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

; J1_BUTTON_3() - Read Joystick 1 button 3 (BIOS)
J1B3_BUILTIN:
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$04
    BEQ .J1B3_OFF
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

; J1_BUTTON_4() - Read Joystick 1 button 4 (BIOS)
J1B4_BUILTIN:
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$08
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:5
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 5
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 4 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:8
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 8
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:11
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$7F
    JSR Intensity_a
    LDA #$32
    LDB #$14
    JSR Moveto_d
    LDA #$FF
    STA VIA_shift_reg  ; Integrator ON
    LDA #$CE
    STA VIA_cntl       ; Shift register mode
    LDB #$03
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$03
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FD
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FD
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FD
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FD
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FD
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FD
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FD
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FD
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$03
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$03
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$03
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$03
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$03
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$03
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    CLR VIA_shift_reg  ; Integrator OFF
    LDD #0
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(8)
    ; VPy_LINE:12
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$64
    JSR Intensity_a
    LDA #$E2
    LDB #$DD
    JSR Moveto_d
    LDA #$FF
    STA VIA_shift_reg  ; Integrator ON
    LDA #$CE
    STA VIA_cntl       ; Shift register mode
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    CLR VIA_shift_reg  ; Integrator OFF
    LDD #0
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(8)
    ; VPy_LINE:13
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$50
    JSR Intensity_a
    LDA #$E2
    LDB #$3C
    JSR Moveto_d
    LDA #$FF
    STA VIA_shift_reg  ; Integrator ON
    LDA #$CE
    STA VIA_cntl       ; Shift register mode
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FE
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$FF
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FE
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$FF
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$02
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$00
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$01
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$02
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    LDB #$01
    STB VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDB #$00
    STB VIA_port_a
    NOP
    NOP
    NOP
    CLR VIA_shift_reg  ; Integrator OFF
    LDD #0
    STD RESULT
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
VAR_ARG4 EQU $C8BA
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+1
MIRROR_X EQU RESULT+2
MIRROR_Y EQU RESULT+3
DRAW_VEC_INTENSITY EQU RESULT+4
