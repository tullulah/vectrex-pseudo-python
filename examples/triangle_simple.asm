; --- Motorola 6809 backend (Vectrex) title='TRI DEMO' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "../include/VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FDB $0000
    FCB $F8,$50,$20,-$45
    FCC "TRI DEMO"
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
    JSR MAIN
    BRA MAIN_LOOP

X0 EQU 0
Y0 EQU 0
X1 EQU 40
Y1 EQU 0
X2 EQU 20
Y2 EQU 40
INT EQU 95
MAIN: ; function
; --- function main ---
    LDA #$D0
    TFR A,DP
    LDA #$00
    LDB #$00
    JSR Moveto_d ; move to (0, 0)
    JSR Intensity_5F
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$28
    JSR Draw_Line_d ; dy=0, dx=40
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$00
    LDB #$28
    JSR Moveto_d ; move to (40, 0)
    CLR Vec_Misc_Count
    LDA #$28
    LDB #$EC
    JSR Draw_Line_d ; dy=40, dx=-20
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$28
    LDB #$14
    JSR Moveto_d ; move to (20, 40)
    CLR Vec_Misc_Count
    LDA #$D8
    LDB #$EC
    JSR Draw_Line_d ; dy=-40, dx=-20
    CLRA
    CLRB
    STD RESULT
    RTS

    INCLUDE "../runtime/vectorlist_runtime.asm"
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
