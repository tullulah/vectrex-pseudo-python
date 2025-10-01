; --- Motorola 6809 backend (Vectrex) title='TEST_BOTH_DRAW' origin=$0000 ---
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
    FCC "TEST BOTH DRAW"
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

    ; Call MAIN once for initialization
    JSR MAIN

MAIN_LOOP:
    JSR Wait_Recal
    LDA #$D0
    TFR A,DP
    JSR Intensity_5F
    JSR Reset0Ref
    JSR LOOP    ; Call user loop function every frame
    BRA MAIN_LOOP

MAIN: ; function
; --- function main ---
    RTS

LOOP: ; function
; --- function loop ---
    LDA #$D0
    TFR A,DP
    LDA #$1E
    LDB #$CE
    JSR Moveto_d ; move to (206, 30)
    LDA #$FF
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$64
    JSR Draw_Line_d ; dy=0, dx=-65436
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$CE
    LDB #$E2
    JSR Moveto_d ; move to (226, 206)
    CLR Vec_Misc_Count
    LDA #$64
    LDB #$00
    JSR Draw_Line_d ; dy=-65436, dx=0
    CLRA
    CLRB
    STD RESULT
    RTS

    INCLUDE "runtime/vectorlist_runtime.asm"
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
