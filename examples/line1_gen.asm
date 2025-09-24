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
    FCB $F8,$50,$20,$AA
    FCC "SINGLE LINE"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
main: JSR Wait_Recal
    JSR Intensity_5F
    JSR MAIN
    BRA main

X0 EQU 0
Y0 EQU 0
X1 EQU 50
Y1 EQU 100
INT EQU 95
MAIN: ; function
; --- function main ---
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR VECTREX_DRAW_LINE
    CLRA
    CLRB
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
; Draw single line using vector list. Args: (x0,y0,x1,y1,intensity) low bytes.
; Assumes WAIT_RECAL already left DP at $D0. Only switches to $C8 for Draw_VL.
VECTREX_DRAW_LINE:
    ; Set intensity
    LDA VAR_ARG4+1
    JSR Intensity_a
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR Moveto_d
    ; Compute deltas (end - start) using low bytes
    LDA VAR_ARG2+1
    SUBA VAR_ARG0+1
    STA VLINE_DX
    LDA VAR_ARG3+1
    SUBA VAR_ARG1+1
    STA VLINE_DY
    ; Clamp to +/-63
    LDA VLINE_DX
    CMPA #63
    BLE VLX_OK_HI
    LDA #63
VLX_OK_HI: CMPA #-64
    BGE VLX_OK_LO
    LDA #-64
VLX_OK_LO: STA VLINE_DX
    LDA VLINE_DY
    CMPA #63
    BLE VLY_OK_HI
    LDA #63
VLY_OK_HI: CMPA #-64
    BGE VLY_OK_LO
    LDA #-64
VLY_OK_LO: STA VLINE_DY
    ; Build 2-byte vector list (Y|endbit, X)
    LDA VLINE_DY
    ORA #$80
    STA VLINE_LIST
    LDA VLINE_DX
    STA VLINE_LIST+1
    ; Switch to vector DP and draw, no restore (next WAIT_RECAL resets)
    JSR DP_to_C8
    LDU #VLINE_LIST
    JSR Draw_VL
    RTS
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
VLINE_DX EQU RESULT+36
VLINE_DY EQU RESULT+37
VLINE_STEPS EQU RESULT+38
VLINE_LIST EQU RESULT+39
