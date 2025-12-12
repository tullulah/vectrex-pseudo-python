; --- Motorola 6809 backend (Vectrex) title='Test: Draw Line' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "include/VECTREX.I"

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
    FCC "TEST  DRAW LINE"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

START:
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:4
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 4
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
    ; DEBUG: Processing 1 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:7
    ; Call BIOS through wrappers (ensures DP state)
    LDB #127
    JSR __Intensity_a
    JSR __Reset0Ref
    LDA #0
    PSHS A
    LDB #0
    JSR __Moveto_d
    LEAS 1,S
    LDA #50
    PSHS A
    LDB #50
    JSR __Draw_Line_d
    LEAS 1,S
    RTS

VECTREX_SET_INTENSITY:
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
LDA #$D0
TFR A,DP        ; Set Direct Page before BIOS call
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JSR Intensity_a
RTS
__Reset0Ref:
LDA #$D0
TFR A,DP        ; Set Direct Page before BIOS call
JSR Reset0Ref
RTS
__Moveto_d:
LDA #$D0
TFR A,DP        ; Set Direct Page before BIOS call
LDA 3,S         ; Get A from stack (caller pushed it)
JSR Moveto_d    ; B is already in register
RTS
__Draw_Line_d:
LDA #$D0
TFR A,DP        ; Set Direct Page before BIOS call
LDA 3,S         ; Get A from stack (caller pushed it)
JSR Draw_Line_d ; B is already in register
RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
MUSIC_PTR     EQU RESULT+26
MUSIC_TICK    EQU RESULT+28   ; 32-bit tick counter
MUSIC_EVENT   EQU RESULT+32   ; Current event pointer
MUSIC_ACTIVE  EQU RESULT+34   ; Playback state (1 byte)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; NO ASSETS EMBEDDED
; All 5 discovered assets are unused in code
; ========================================


; ========================================
; VECTOR LIST DATA (Malban format)
; ========================================
_SQUARE:
    FCB 0, 0, 0          ; Header (y=0, x=0, next_y=0)
    FCB $FF, $D8, $D8    ; Line 1: flag=-1, dy=-40, dx=-40
    FCB $FF, 0, 80       ; Line 2: flag=-1, dy=0, dx=80
    FCB $FF, 80, 0       ; Line 3: flag=-1, dy=80, dx=0
    FCB $FF, 0, $B0      ; Line 4: flag=-1, dy=0, dx=-80
    FCB 2                ; End marker

