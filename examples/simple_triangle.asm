; --- Motorola 6809 backend (Vectrex) title='TRIANGLE TEST' origin=$0000 ---
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
    FCC "TRIANGLE TEST"
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
    LDA #$FF
    JSR Intensity_a
    JSR Reset0Ref
    LDA #$E2
    LDB #$00
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$C0
    LDB #$19
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    LDA #$FF
    JSR Intensity_a
    JSR Reset0Ref
    LDA #$0F
    LDB #$19
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$3F
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    LDA #$FF
    JSR Intensity_a
    JSR Reset0Ref
    LDA #$0F
    LDB #$E7
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$3F
    LDB #$C0
    JSR Draw_Line_d
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
