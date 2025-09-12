        ORG $0000
        INCLUDE "../include/VECTREX.I"
; Header
        FCB $67,$20,$47,$43,$45,$20,$31,$39,$38,$33,$80
        FDB music1
        FCB $F8,$50,$20,$20
        FCC "SIMPLE LINES"
        FCB $80
        FCB 0

; Init
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

        ; Draw square 32x32 centered-ish using absolute moves + relative lines
        ; Move to (-16,-16)
        LDA #($F0)    ; -16
        LDB #($F0)    ; -16
        JSR Moveto_d
        ; Right 32
        LDA #0
        LDB #32
        JSR Draw_Line_d
        ; Down 32
        LDA #32
        LDB #0
        JSR Draw_Line_d
        ; Left 32
        LDA #0
        LDB #($E0)    ; -32
        JSR Draw_Line_d
        ; Up 32
        LDA #($E0)
        LDB #0
        JSR Draw_Line_d

        BRA MAIN_LOOP

        END
