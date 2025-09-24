; --- Motorola 6809 backend (Vectrex) title='SHAPES DEMO' origin=$0000 ---
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
    FDB music1
    FCB $F8,$50,$20,-$45
    FCC "SHAPES DEMO"
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

VL_SHAPES:
    FCB 65
    FCB $00,$00,CMD_START ; START to (+0, +0)
    FCB $00,$00,CMD_INT ; INTENSITY (next byte) at current (+0, +0)
    FCB $3F ; intensity value
    FCB $F0,$F0,CMD_START ; START to (-16, -16)
    FCB $00,$20,CMD_LINE ; LINE dy=+0 dx=+32 -> (+16, -16)
    FCB $20,$00,CMD_LINE ; LINE dy=+32 dx=+0 -> (+16, +16)
    FCB $00,$E0,CMD_LINE ; LINE dy=+0 dx=-32 -> (-16, +16)
    FCB $E0,$00,CMD_LINE ; LINE dy=-32 dx=+0 -> (-16, -16)
    FCB $00,$00,CMD_ZERO ; ZERO (Reset0Ref) => origin (0,0)
    FCB $F0,$00,CMD_START ; START to (+0, -16)
    FCB $10,$10,CMD_LINE ; LINE dy=+16 dx=+16 -> (+16, +0)
    FCB $10,$F0,CMD_LINE ; LINE dy=+16 dx=-16 -> (+0, +16)
    FCB $F0,$F0,CMD_LINE ; LINE dy=-16 dx=-16 -> (-16, +0)
    FCB $F0,$10,CMD_LINE ; LINE dy=-16 dx=+16 -> (+0, -16)
    FCB $00,$00,CMD_ZERO ; ZERO (Reset0Ref) => origin (0,0)
    FCB $00,$0C,CMD_START ; START to (+12, +0)
    FCB $03,$00,CMD_LINE ; LINE dy=+3 dx=+0 -> (+12, +3)
    FCB $03,$FE,CMD_LINE ; LINE dy=+3 dx=-2 -> (+10, +6)
    FCB $02,$FE,CMD_LINE ; LINE dy=+2 dx=-2 -> (+8, +8)
    FCB $02,$FE,CMD_LINE ; LINE dy=+2 dx=-2 -> (+6, +10)
    FCB $02,$FD,CMD_LINE ; LINE dy=+2 dx=-3 -> (+3, +12)
    FCB $00,$FD,CMD_LINE ; LINE dy=+0 dx=-3 -> (+0, +12)
    FCB $00,$FD,CMD_LINE ; LINE dy=+0 dx=-3 -> (-3, +12)
    FCB $FE,$FD,CMD_LINE ; LINE dy=-2 dx=-3 -> (-6, +10)
    FCB $FE,$FE,CMD_LINE ; LINE dy=-2 dx=-2 -> (-8, +8)
    FCB $FE,$FE,CMD_LINE ; LINE dy=-2 dx=-2 -> (-10, +6)
    FCB $FD,$FE,CMD_LINE ; LINE dy=-3 dx=-2 -> (-12, +3)
    FCB $FD,$00,CMD_LINE ; LINE dy=-3 dx=+0 -> (-12, +0)
    FCB $FD,$00,CMD_LINE ; LINE dy=-3 dx=+0 -> (-12, -3)
    FCB $FD,$02,CMD_LINE ; LINE dy=-3 dx=+2 -> (-10, -6)
    FCB $FE,$02,CMD_LINE ; LINE dy=-2 dx=+2 -> (-8, -8)
    FCB $FE,$02,CMD_LINE ; LINE dy=-2 dx=+2 -> (-6, -10)
    FCB $FE,$03,CMD_LINE ; LINE dy=-2 dx=+3 -> (-3, -12)
    FCB $00,$03,CMD_LINE ; LINE dy=+0 dx=+3 -> (+0, -12)
    FCB $00,$03,CMD_LINE ; LINE dy=+0 dx=+3 -> (+3, -12)
    FCB $02,$03,CMD_LINE ; LINE dy=+2 dx=+3 -> (+6, -10)
    FCB $02,$02,CMD_LINE ; LINE dy=+2 dx=+2 -> (+8, -8)
    FCB $02,$02,CMD_LINE ; LINE dy=+2 dx=+2 -> (+10, -6)
    FCB $03,$02,CMD_LINE ; LINE dy=+3 dx=+2 -> (+12, -3)
    FCB $03,$00,CMD_LINE ; LINE dy=+3 dx=+0 -> (+12, +0)
    FCB $00,$00,CMD_ZERO ; ZERO (Reset0Ref) => origin (0,0)
    FCB $00,$10,CMD_START ; START to (+16, +0)
    FCB $02,$00,CMD_LINE ; LINE dy=+2 dx=+0 -> (+16, +2)
    FCB $02,$FF,CMD_LINE ; LINE dy=+2 dx=-1 -> (+15, +4)
    FCB $02,$00,CMD_LINE ; LINE dy=+2 dx=+0 -> (+15, +6)
    FCB $02,$FF,CMD_LINE ; LINE dy=+2 dx=-1 -> (+14, +8)
    FCB $02,$FE,CMD_LINE ; LINE dy=+2 dx=-2 -> (+12, +10)
    FCB $02,$FF,CMD_LINE ; LINE dy=+2 dx=-1 -> (+11, +12)
    FCB $01,$FE,CMD_LINE ; LINE dy=+1 dx=-2 -> (+9, +13)
    FCB $01,$FE,CMD_LINE ; LINE dy=+1 dx=-2 -> (+7, +14)
    FCB $01,$FE,CMD_LINE ; LINE dy=+1 dx=-2 -> (+5, +15)
    FCB $01,$FE,CMD_LINE ; LINE dy=+1 dx=-2 -> (+3, +16)
    FCB $00,$FE,CMD_LINE ; LINE dy=+0 dx=-2 -> (+1, +16)
    FCB $00,$FE,CMD_LINE ; LINE dy=+0 dx=-2 -> (-1, +16)
    FCB $00,$FE,CMD_LINE ; LINE dy=+0 dx=-2 -> (-3, +16)
    FCB $FF,$FE,CMD_LINE ; LINE dy=-1 dx=-2 -> (-5, +15)
    FCB $FF,$FE,CMD_LINE ; LINE dy=-1 dx=-2 -> (-7, +14)
    FCB $FF,$FE,CMD_LINE ; LINE dy=-1 dx=-2 -> (-9, +13)
    FCB $FF,$FE,CMD_LINE ; LINE dy=-1 dx=-2 -> (-11, +12)
    FCB $FE,$FF,CMD_LINE ; LINE dy=-2 dx=-1 -> (-12, +10)
    FCB $FE,$FE,CMD_LINE ; LINE dy=-2 dx=-2 -> (-14, +8)
    FCB $FE,$FF,CMD_LINE ; LINE dy=-2 dx=-1 -> (-15, +6)
    FCB $FE,$00,CMD_LINE ; LINE dy=-2 dx=+0 -> (-15, +4)
    FCB $FE,$FF,CMD_LINE ; LINE dy=-2 dx=-1 -> (-16, +2)
    FCB $FE,$00,CMD_LINE ; LINE dy=-2 dx=+0 -> (-16, +0)
    FCB $00,$00,CMD_END ; END
MAIN: ; function
; --- function main ---
    LDX #VL_SHAPES
    JSR Run_VectorList
    RTS

    INCLUDE "../runtime/vectorlist_runtime.asm"
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "SHAPES"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
