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
    FCB $F8,$50,$20,-$45
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

MAIN_LOOP:
    JSR Wait_Recal
    LDA #$D0
    TFR A,DP
    JSR Intensity_5F
    JSR Reset0Ref
    JSR MAIN    ; Call main function every frame (Vectrex style)
    JSR LOOP    ; Call optional loop function every frame
    BRA MAIN_LOOP

MAIN: ; function
; --- function main ---
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    RTS

LOOP: ; function
; --- function loop ---
    RTS

VECTREX_MOVE_TO:
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Moveto_d
    ; store new current position
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VCUR_X EQU RESULT+30
VCUR_Y EQU RESULT+31
