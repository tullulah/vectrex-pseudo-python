; --- Motorola 6809 backend (Vectrex) title='SIMPLE LINE' origin=$0000 ---
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
    FCC "SIMPLE LINE"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
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
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:7
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 7
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:8
    ; VIDE-compatible code generation
    LDB #127
    JSR __Intensity_a
    CLR ,-S       ; Push Y=0 using CLR (VIDE style)
    CLRB          ; X=0 using CLRB (VIDE style)
    JSR __Moveto_d
    LEAS 1,S
    LDB #50
    STB ,-S       ; Push dy using STB (VIDE style)
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

