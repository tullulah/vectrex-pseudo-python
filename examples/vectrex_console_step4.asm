; --- Motorola 6809 backend (Vectrex) title='VECTREX STEP4' origin=$0000 ---
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
    FCC "VECTREX STEP4"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
INIT_ONCE:
    LDA #$80
    STA VIA_t1_cnt_lo
main: JSR Wait_Recal
    JSR Intensity_5F ; set default intensity
    JSR Reset0Ref ; center beam
    JSR MAIN
    BRA main

I_STRONG EQU 95
I_MED EQU 64
MAIN: ; function
; --- function main ---
    JSR Intensity_5F
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$4B
    LDB #$AD
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$53
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$53
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$4B
    LDB #$53
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$F4
    LDB #$0C
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$3F
    LDB #$5F
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$C1
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$C1
    LDB #$00
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$C1
    LDB #$5F
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$F4
    LDB #$F4
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$B5
    LDB #$53
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$AD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$AD
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$B5
    LDB #$AD
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$0C
    LDB #$F4
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$C1
    LDB #$A1
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$3F
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$3F
    LDB #$00
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$3F
    LDB #$A1
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$0C
    LDB #$0C
    JSR Draw_Line_d
    CLRA
    CLRB
    STD RESULT
    JSR Intensity_5F
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$3C
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$46
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$46
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$3C
    LDB #$46
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$CE
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$CE
    LDB #$00
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$D8
    LDB #$46
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$BA
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$BA
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$D8
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$32
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$32
    LDB #$00
    JSR Draw_Line_d
    CLRA
    CLRB
    STD RESULT
    LDA #$40
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$34
    LDB #$C2
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$3E
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$3E
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$34
    LDB #$3E
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$D6
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$D6
    LDB #$00
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$E0
    LDB #$3E
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$C2
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$C2
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$E0
    LDB #$C2
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$2A
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$2A
    LDB #$00
    JSR Draw_Line_d
    CLRA
    CLRB
    STD RESULT
    JSR Intensity_5F
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$D8
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$46
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$46
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$D8
    LDB #$46
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$E4
    LDB #$00
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$BC
    LDB #$46
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$BA
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$BA
    JSR Draw_Line_d
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$BC
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$1C
    LDB #$00
    JSR Draw_Line_d
    CLRA
    CLRB
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
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
