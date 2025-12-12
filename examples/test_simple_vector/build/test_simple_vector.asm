; --- Motorola 6809 backend (Vectrex) title='Simple Vector Test' origin=$0000 ---
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
    FCC "SIMPLE VECTOR TEST"
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
; DRAW_VECTOR("line") - render all paths using Draw_VLc
    LDX #_LINE_VECTORS ; Load pointer list
DRAW_VEC_LOOP_START:
    LDD ,X++            ; Load next path pointer
    BEQ DRAW_VEC_DONE   ; Exit if 0 (end of list)
    PSHS X              ; Save list pointer
    JSR Reset0Ref       ; Reset origin to center before EACH path
    TFR D,X             ; X = path data pointer
    LDA ,X+             ; A = Y0 (starting point)
    LDB ,X+             ; B = X0 (starting point)
    JSR Moveto_d        ; Move beam from origin to starting point
    ; X now points to count byte for Draw_VLc
    JSR Draw_VLc        ; Draw this path
    PULS X              ; Restore list pointer
    BRA DRAW_VEC_LOOP_START ; Next path
DRAW_VEC_DONE:
    LDD #0
    STD RESULT
    RTS

VECTREX_SET_INTENSITY:
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
VECTREX_WAIT_RECAL:
    JSR Wait_Recal
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "LINE"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; ASSET DATA SECTION
; Embedded .vec and .vmus resources
; ========================================

; Vector asset: line
; Generated from line.vec
; Total paths: 1, points: 2

_LINE_SEGMENT_VECTORS:
    FCB 0, 0          ; starting point (y0, x0)
    FCB 1              ; number of line segments
    FCB 10, 10          ; delta 0 (dy, dx)

_LINE_VECTORS:  ; Main path list for DRAW_VECTOR
    FDB _LINE_SEGMENT_VECTORS
    FDB 0               ; end of list

