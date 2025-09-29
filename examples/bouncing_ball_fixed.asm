; --- Motorola 6809 backend (Vectrex) title='BOUNCING BALL' origin=$0000 ---
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
    FCB $F8,$50,$20,-$45
    FCC "BOUNCING BALL"
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

MAIN: ; function
; --- function main ---
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65436
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BLE CT_2
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #65436
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65456
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BLE CT_6
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    LDD #65456
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LDA #$D0
    TFR A,DP
    LDA #$4B
    LDB #$5F
    JSR Moveto_d ; move to (95, 75)
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0A
    JSR Draw_Line_d ; dy=0, dx=10
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$4B
    LDB #$69
    JSR Moveto_d ; move to (105, 75)
    CLR Vec_Misc_Count
    LDA #$0A
    LDB #$00
    JSR Draw_Line_d ; dy=10, dx=0
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$55
    LDB #$69
    JSR Moveto_d ; move to (105, 85)
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$F6
    JSR Draw_Line_d ; dy=0, dx=-10
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$55
    LDB #$5F
    JSR Moveto_d ; move to (95, 85)
    CLR Vec_Misc_Count
    LDA #$F6
    LDB #$00
    JSR Draw_Line_d ; dy=-10, dx=0
    CLRA
    CLRB
    STD RESULT
    RTS

    INCLUDE "runtime/vectorlist_runtime.asm"
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
VAR_BALL_X EQU RESULT+26
VAR_BALL_Y EQU RESULT+28
VAR_VEL_X EQU RESULT+30
VAR_VEL_Y EQU RESULT+32
; Call argument scratch space
VAR_ARG0 EQU RESULT+34
VAR_ARG1 EQU RESULT+36
VAR_ARG2 EQU RESULT+38
VAR_ARG3 EQU RESULT+40
VAR_ARG4 EQU RESULT+42
