; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
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
    FCC "UNTITLED"
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
    LDD #17
    STD RESULT
    LDD #5
    STD RESULT
    LDD VAR_A
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_B
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD VAR_A
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_B
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD #100
    STD RESULT
    LDD #7
    STD RESULT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_X
    STU TMPPTR
    STX ,U
    LDD VAR_RESULTADO_DIV_ENTERA
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD VAR_FINAL
    STD RESULT
    RTS

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 1 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(9)
    LDD #0
    STD RESULT
    RTS
    RTS

DIV16:
    LDD #0
    STD DIV_Q
    LDD DIV_A
    STD DIV_R
    LDD DIV_B
    BEQ DIV16_DONE
DIV16_LOOP:
    LDD DIV_R
    SUBD DIV_B
    BLO DIV16_DONE
    STD DIV_R
    LDD DIV_Q
    ADDD #1
    STD DIV_Q
    BRA DIV16_LOOP
DIV16_DONE:
    LDD DIV_Q
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
DIV_A   EQU RESULT+18
DIV_B   EQU RESULT+20
DIV_Q   EQU RESULT+22
DIV_R   EQU RESULT+24
; Call argument scratch space
