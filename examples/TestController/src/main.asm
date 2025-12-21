; --- Motorola 6809 backend (Vectrex) title='TestController' origin=$0000 ---
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
    FCC "TESTCONTROLLER"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary

    JMP START

VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to gap area (C000-C7FF)
    LDA VAR_ARG0+1   ; Load value to debug print
    STA $C000        ; Debug output value in unmapped gap
    LDA #$42         ; Debug marker
    STA $C001        ; Debug marker to indicate new output
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    LDD #0
    STD VAR_PLAYER_X
    LDD #0
    STD VAR_PLAYER_Y
    ; VPy_LINE:11
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 11
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
    LEAS -8,S ; allocate locals
    ; DEBUG: Processing 15 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    ; VPy_LINE:16
; NATIVE_CALL: J1_X at line 16
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
    STX 4 ,S
    ; DEBUG: Statement 1 - Discriminant(0)
    ; VPy_LINE:17
; NATIVE_CALL: J1_Y at line 17
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
    STX 6 ,S
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:23
; NATIVE_CALL: J1_BUTTON_1 at line 23
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .j1b1_not_pressed
    LDD #1
    BRA .j1b1_done
.j1b1_not_pressed:
    LDD #0
.j1b1_done:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:24
; NATIVE_CALL: J1_BUTTON_2 at line 24
; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$02
    BEQ .j1b2_not_pressed
    LDD #1
    BRA .j1b2_done
.j1b2_not_pressed:
    LDD #0
.j1b2_done:
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 4 - Discriminant(8)
    ; VPy_LINE:26
    LDD 4 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(joy_x) at line 26
    LDD RESULT
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_JOY_X
    STX $C002
    BRA DEBUG_SKIP_DATA_0
DEBUG_LABEL_JOY_X:
    FCC "joy_x"
    FCB $00
DEBUG_SKIP_DATA_0:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(8)
    ; VPy_LINE:27
    LDD 6 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(joy_y) at line 27
    LDD RESULT
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_JOY_Y
    STX $C002
    BRA DEBUG_SKIP_DATA_1
DEBUG_LABEL_JOY_Y:
    FCC "joy_y"
    FCB $00
DEBUG_SKIP_DATA_1:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(9)
    ; VPy_LINE:32
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
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
    LBEQ IF_NEXT_3
    ; VPy_LINE:33
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_2
IF_NEXT_3:
IF_END_2:
    ; DEBUG: Statement 7 - Discriminant(9)
    ; VPy_LINE:34
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
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
    ; VPy_LINE:35
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_6
IF_NEXT_7:
IF_END_6:
    ; DEBUG: Statement 8 - Discriminant(9)
    ; VPy_LINE:36
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_12
    LDD #0
    STD RESULT
    BRA CE_13
CT_12:
    LDD #1
    STD RESULT
CE_13:
    LDD RESULT
    LBEQ IF_NEXT_11
    ; VPy_LINE:37
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_10
IF_NEXT_11:
IF_END_10:
    ; DEBUG: Statement 9 - Discriminant(9)
    ; VPy_LINE:38
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_16
    LDD #0
    STD RESULT
    BRA CE_17
CT_16:
    LDD #1
    STD RESULT
CE_17:
    LDD RESULT
    LBEQ IF_NEXT_15
    ; VPy_LINE:39
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_14
IF_NEXT_15:
IF_END_14:
    ; DEBUG: Statement 10 - Discriminant(9)
    ; VPy_LINE:42
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65426
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_20
    LDD #0
    STD RESULT
    BRA CE_21
CT_20:
    LDD #1
    STD RESULT
CE_21:
    LDD RESULT
    LBEQ IF_NEXT_19
    ; VPy_LINE:43
    LDD #65426
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_18
IF_NEXT_19:
IF_END_18:
    ; DEBUG: Statement 11 - Discriminant(9)
    ; VPy_LINE:44
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_24
    LDD #0
    STD RESULT
    BRA CE_25
CT_24:
    LDD #1
    STD RESULT
CE_25:
    LDD RESULT
    LBEQ IF_NEXT_23
    ; VPy_LINE:45
    LDD #110
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_22
IF_NEXT_23:
IF_END_22:
    ; DEBUG: Statement 12 - Discriminant(9)
    ; VPy_LINE:46
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65446
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_28
    LDD #0
    STD RESULT
    BRA CE_29
CT_28:
    LDD #1
    STD RESULT
CE_29:
    LDD RESULT
    LBEQ IF_NEXT_27
    ; VPy_LINE:47
    LDD #65446
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_26
IF_NEXT_27:
IF_END_26:
    ; DEBUG: Statement 13 - Discriminant(9)
    ; VPy_LINE:48
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #90
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_32
    LDD #0
    STD RESULT
    BRA CE_33
CT_32:
    LDD #1
    STD RESULT
CE_33:
    LDD RESULT
    LBEQ IF_NEXT_31
    ; VPy_LINE:49
    LDD #90
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_30
IF_NEXT_31:
IF_END_30:
    ; DEBUG: Statement 14 - Discriminant(8)
    ; VPy_LINE:51
; DRAW_VECTOR("test", x, y) - 1 path(s) at position
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_TEST_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LEAS 8,S ; free locals
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_PLAYER_X EQU $CF10+0
VAR_PLAYER_Y EQU $CF10+2
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; Vector asset: test
; Generated from test.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3

_TEST_VECTORS:  ; Main entry
_TEST_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $14,$00,0,0        ; path0: header (y=20, x=0, next_y=0, next_x=0)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker

DRAW_VEC_X EQU RESULT+4
DRAW_VEC_Y EQU RESULT+5
