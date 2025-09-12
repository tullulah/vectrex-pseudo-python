; --- Motorola 6809 backend (Vectrex) title='VECTORLIST' origin=$0000 ---
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
    FCC "VECTORLIST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    INCLUDE "../runtime/vectorlist_runtime.asm"
INIT_ONCE:
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #Vec_Default_Stk
main:
    JSR Wait_Recal
    JSR Intensity_5F ; set default intensity
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref ; center beam
    JSR MAIN
    BRA main

VL_SIMPLE_SHAPE:
    FCB 13
    FCB $00,$00,CMD_INT
    FCB $7F ; intensity
    FCB $00,$00,CMD_ZERO
    FCB $EC,$EC,CMD_START
    FCB $00,$28,CMD_LINE
    FCB $28,$00,CMD_LINE
    FCB $00,$D8,CMD_LINE
    FCB $D8,$00,CMD_LINE
    FCB $F6,$F6,CMD_START
    FCB $00,$00,CMD_START
    FCB $00,$14,CMD_LINE
    FCB $0F,$F6,CMD_LINE
    FCB $F1,$F6,CMD_LINE
    FCB $00,$00,CMD_END
MAIN: ; function
; --- function main ---
    JSR VL_SIMPLE_SHAPE
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
VAR_SIMPLE_SHAPE EQU RESULT+26
; Call argument scratch space
VAR_ARG0 EQU RESULT+28
