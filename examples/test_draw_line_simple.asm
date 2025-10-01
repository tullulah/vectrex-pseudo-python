; --- Motorola 6809 backend (Vectrex) title='DRAW LINE TEST' origin=$0000 ---
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
    FCC "DRAW LINE TEST"
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
    RTS

LOOP: ; function
; --- function loop ---
    LDA #$D0
    TFR A,DP
    LDA #$FF
    JSR Intensity_a
    JSR Reset0Ref
    LDA #$00
    LDB #$00
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$32
    LDB #$32
    JSR Draw_Line_d
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
