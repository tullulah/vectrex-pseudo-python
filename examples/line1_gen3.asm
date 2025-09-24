; --- Motorola 6809 backend (Vectrex) title='SINGLE LINE' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "../include/VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1998"
    FCB $80
    FDB music1
    FCB $F8,$50,$20,-$45
    FCC "SINGLE LINE"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
main: JSR Wait_Recal
    JSR MAIN
    BRA main

X0 EQU 0
Y0 EQU 0
X1 EQU 50
Y1 EQU 100
INT EQU 95
MAIN: ; function
; --- function main ---
    LDA #$80
    STA VIA_t1_cnt_lo
    LDA #$00
    LDB #$00
    JSR Moveto_d
    JSR Intensity_5F
    CLR Vec_Misc_Count
    LDA #$64
    LDB #$32
    JSR Draw_Line_d
    CLRA
    CLRB
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
