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
; DRAW_VECTOR("line") - Malban Draw_Sync_List format
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDA #$D0
    TFR A,DP            ; Set DP for hardware
    LDX #_LINE_VECTORS  ; X = sync list pointer
DSL_LOOP_1:
    LDA ,X+             ; A = intensity/marker
    CMPA #2
    BEQ DSL_DONE_1            ; End if marker=2
    CMPA #1
    BEQ DSL_LOOP_1            ; Next segment if marker=1
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x position
    JSR Moveto_d_7F     ; Move to position
DSL_LOOP_1_INNER:
    LDA ,X+             ; A = draw marker
    BPL DSL_CHECK       ; Branch if >= 0 (move or break)
    LDD ,X++            ; D = dy,dx
    JSR Draw_Line_d     ; Draw with intensity
    BRA DSL_LOOP_1_INNER            ; Continue inner loop
DSL_CHECK:
    BNE DSL_NEXT_SEG    ; If A!=0, next segment
    LDD ,X++            ; D = dy,dx
    JSR Moveto_d_7F     ; Move beam
    BRA DSL_LOOP_1_INNER            ; Continue inner loop
DSL_NEXT_SEG:
    BRA DSL_LOOP_1            ; Back to outer loop
DSL_DONE_1:
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
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; Vector asset: line
; Generated from line.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 2

_LINE_VECTORS:
    FCB 127              ; seg0: intensity
    FCB $00,$00          ; seg0: position (y=0, x=0)
    FCB -1              ; draw line 0
    FCB $0A,$0A          ; delta (dy=10, dx=10)
    FCB 2               ; end of list

