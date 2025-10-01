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
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "UNTITLED"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

main:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** DEBUG *** loop() function code inline (executed every frame)
    LDA #0
    LDB #0
    JSR Moveto_d
    JSR Intensity_5F
    CLR Vec_Misc_Count
    LDA #50
    LDB #50
    JSR Draw_Line_d
    JMP main

;***************************************************************************
; DATA SECTION
;***************************************************************************
