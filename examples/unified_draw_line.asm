; --- Motorola 6809 backend (Vectrex) title='UNIFIED DRAW_LINE TEST' origin=$0000 ---
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
    FDB $0000
    FCB $F8,$50,$20,-$45
    FCC "UNIFIED DRAW LINE TEST"
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
    JSR DRAW_LINE_WRAPPER
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
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    CLRA
    CLRB
    STD RESULT
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    CLRA
    CLRB
    STD RESULT
    RTS

    INCLUDE "runtime/vectorlist_runtime.asm"
; DRAW_LINE unified wrapper - always reliable, no inline optimization
; Args: (x0,y0,x1,y1,intensity) low bytes.
; ALWAYS sets intensity and handles Reset0Ref properly.
DRAW_LINE_WRAPPER:
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA VAR_ARG4+1
    JSR Intensity_a
    ; Move to start (y in A, x in B)
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR Moveto_d
    ; Compute deltas (end-start)
    LDA VAR_ARG2+1
    SUBA VAR_ARG0+1
    STA VLINE_DX
    LDA VAR_ARG3+1
    SUBA VAR_ARG1+1
    STA VLINE_DY
    ; Clamp dx to +/-63
    LDA VLINE_DX
    CMPA #63
    BLE DLW_DX_HI_OK
    LDA #63
DLW_DX_HI_OK: CMPA #-64
    BGE DLW_DX_LO_OK
    LDA #-64
DLW_DX_LO_OK: STA VLINE_DX
    ; Clamp dy to +/-63
    LDA VLINE_DY
    CMPA #63
    BLE DLW_DY_HI_OK
    LDA #63
DLW_DY_HI_OK: CMPA #-64
    BGE DLW_DY_LO_OK
    LDA #-64
DLW_DY_LO_OK: STA VLINE_DY
    ; Clear Vec_Misc_Count for proper timing
    CLR Vec_Misc_Count
    ; Draw line (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
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
