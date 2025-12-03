; --- Motorola 6809 backend (Vectrex) title='TEST_SIMPLE_DRAW' origin=$0000 ---
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
    FCC "TEST SIMPLE DRAW"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

main:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; Initialize global variables
    LDD #127
    STD VAR_BRIGHTNESS
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA main

LOOP_BODY:
    ; DEBUG: Processing 6 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:13
    LDA #0
    LDB #-50
    JSR Moveto_d
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #100
    JSR Draw_Line_d
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:16
    LDA #-50
    LDB #0
    JSR Moveto_d
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #100
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:19
    LDA #-30
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #60
    JSR Draw_Line_d
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:20
    LDA #-30
    LDB #30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #60
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:21
    LDA #30
    LDB #30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-60
    JSR Draw_Line_d
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:22
    LDA #30
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-60
    LDB #0
    JSR Draw_Line_d
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
RESULT    EQU $C880
VAR_BRIGHTNESS EQU $CF00+0
