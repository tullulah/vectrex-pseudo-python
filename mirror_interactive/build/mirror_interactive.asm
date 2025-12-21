; --- Motorola 6809 backend (Vectrex) title='Mirror Interactive - Joystick Control' origin=$0000 ---
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
    FCC "MIRROR INTERACTIVE   JOY"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary

    JMP START

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
LDA ,X+                 ; intensity
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
LDA ,X+                 ; Read intensity
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
    LEAS -10,S ; allocate locals
    ; DEBUG: Processing 10 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    ; VPy_LINE:9
; NATIVE_CALL: J1_X at line 9
; J1_X() - Read Joystick 1 X axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF00 (128=center)
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
; Convert unsigned to digital: <108=left(-1), 108-148=center(0), >148=right(+1)
    CMPB #108    ; Check if < 108 (left)
    BLO J1X_LEFT ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (right)
    BHI J1X_RIGHT ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1X_END
J1X_LEFT:
    LDD #$FFFF   ; Left (-1)
    BRA J1X_END
J1X_RIGHT:
    LDD #1       ; Right (+1)
J1X_END:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 1 - Discriminant(0)
    ; VPy_LINE:10
; NATIVE_CALL: J1_Y at line 10
; J1_Y() - Read Joystick 1 Y axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF01 (128=center)
    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)
; Convert unsigned to digital: <108=down(-1), 108-148=center(0), >148=up(+1)
    CMPB #108    ; Check if < 108 (down)
    BLO J1Y_DOWN ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (up)
    BHI J1Y_UP   ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1Y_END
J1Y_DOWN:
    LDD #$FFFF   ; Down (-1)
    BRA J1Y_END
J1Y_UP:
    LDD #1       ; Up (+1)
J1Y_END:
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:13
    LDD #64
    STD RESULT
    LDX RESULT
    STX 6 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:14
    LDD #64
    STD RESULT
    LDX RESULT
    STX 8 ,S
    ; DEBUG: Statement 4 - Discriminant(9)
    ; VPy_LINE:17
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:18
    LDD #30
    STD RESULT
    LDX RESULT
    STX 6 ,S
    LBRA IF_END_0
IF_NEXT_1:
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
    BEQ CT_4
    LDD #0
    STD RESULT
    BRA CE_5
CT_4:
    LDD #1
    STD RESULT
CE_5:
    LDD RESULT
    LBEQ IF_END_0
    ; VPy_LINE:20
    LDD #98
    STD RESULT
    LDX RESULT
    STX 6 ,S
    LBRA IF_END_0
IF_END_0:
    ; DEBUG: Statement 5 - Discriminant(9)
    ; VPy_LINE:23
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_8
    LDD #0
    STD RESULT
    BRA CE_9
CT_8:
    LDD #1
    STD RESULT
CE_9:
    LDD RESULT
    LBEQ IF_NEXT_7
    ; VPy_LINE:24
    LDD #30
    STD RESULT
    LDX RESULT
    STX 8 ,S
    LBRA IF_END_6
IF_NEXT_7:
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_END_6
    ; VPy_LINE:26
    LDD #98
    STD RESULT
    LDX RESULT
    STX 8 ,S
    LBRA IF_END_6
IF_END_6:
    ; DEBUG: Statement 6 - Discriminant(0)
    ; VPy_LINE:29
    LDD #0
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; DEBUG: Statement 7 - Discriminant(9)
    ; VPy_LINE:32
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
    BGT CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    LBEQ IF_NEXT_13
    ; VPy_LINE:33
    LDD #1
    STD RESULT
    LDX RESULT
    STX 4 ,S
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; DEBUG: Statement 8 - Discriminant(9)
    ; VPy_LINE:36
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:37
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 4 ,S
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; DEBUG: Statement 9 - Discriminant(8)
    ; VPy_LINE:40
; DRAW_VECTOR_EX("spaceship", x, y, mirror) - 2 path(s), width=10, center_x=0
    LDD 6 ,S
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD 8 ,S
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 4 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_20
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_20:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_21
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_21:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_22
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_22:
    LDX #_SPACESHIP_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X and MIRROR_Y flags
    LDX #_SPACESHIP_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X and MIRROR_Y flags
    LDD #0
    STD RESULT
    LEAS 10,S ; free locals
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
TEMP_X    EQU RESULT+28   ; Temporary x storage (1 byte)
TEMP_Y    EQU RESULT+29   ; Temporary y storage (1 byte)
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

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; Vector asset: spaceship
; Generated from spaceship.vec (Malban Draw_Sync_List format)
; Total paths: 2, points: 10
; X bounds: min=-5, max=5, width=10
; Center: (0, 1)

_SPACESHIP_WIDTH EQU 10
_SPACESHIP_CENTER_X EQU 0
_SPACESHIP_CENTER_Y EQU 1

_SPACESHIP_VECTORS:  ; Main entry
_SPACESHIP_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $07,$00,0,0        ; path0: header (y=7, x=0, relative to center)
    FCB $FF,$F4,$FB          ; line 0: flag=-1, dy=-12, dx=-5
    FCB $FF,$02,$02          ; line 1: flag=-1, dy=2, dx=2
    FCB $FF,$FC,$03          ; line 2: flag=-1, dy=-4, dx=3
    FCB $FF,$04,$03          ; line 3: flag=-1, dy=4, dx=3
    FCB $FF,$FE,$02          ; line 4: flag=-1, dy=-2, dx=2
    FCB $FF,$0C,$FB          ; closing line: flag=-1, dy=12, dx=-5
    FCB 2                ; End marker

_SPACESHIP_PATH1:
    FCB 100              ; path1: intensity
    FCB $01,$FF,0,0        ; path1: header (y=1, x=-1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker

DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+1
MIRROR_X EQU RESULT+2
MIRROR_Y EQU RESULT+3
