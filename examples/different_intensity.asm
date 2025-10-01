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
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR VECTREX_DRAW_LINE
    CLRA
    CLRB
    STD RESULT
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #200
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR VECTREX_DRAW_LINE
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
