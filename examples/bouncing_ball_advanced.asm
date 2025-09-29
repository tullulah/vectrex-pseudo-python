; --- Motorola 6809 backend (Vectrex) title='BOUNCING BALL ADV' origin=$0000 ---
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
    FCC "BOUNCING BALL ADV"
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
    LDD #65446
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
    LDD #65446
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
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65466
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
    LDD #65466
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
    LDA #$D0
    TFR A,DP
    LDA #$BA
    LDB #$A6
    JSR Moveto_d ; move to (166, 186)
    LDA #$30
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$B4
    JSR Draw_Line_d ; dy=0, dx=-65356
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$46
    LDB #$A6
    JSR Moveto_d ; move to (166, 70)
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$B4
    JSR Draw_Line_d ; dy=0, dx=-65356
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$BA
    LDB #$A6
    JSR Moveto_d ; move to (166, 186)
    CLR Vec_Misc_Count
    LDA #$8C
    LDB #$00
    JSR Draw_Line_d ; dy=-65396, dx=0
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$BA
    LDB #$5A
    JSR Moveto_d ; move to (90, 186)
    CLR Vec_Misc_Count
    LDA #$8C
    LDB #$00
    JSR Draw_Line_d ; dy=-65396, dx=0
    CLRA
    CLRB
    STD RESULT
    LDD #65456
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$3E
    LDB #$5A
    JSR Moveto_d ; move to (90, 62)
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$08
    LDB #$06
    JSR Draw_Line_d ; dy=8, dx=6
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$46
    LDB #$60
    JSR Moveto_d ; move to (96, 70)
    CLR Vec_Misc_Count
    LDA #$08
    LDB #$FA
    JSR Draw_Line_d ; dy=8, dx=-6
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$4E
    LDB #$5A
    JSR Moveto_d ; move to (90, 78)
    CLR Vec_Misc_Count
    LDA #$F8
    LDB #$FA
    JSR Draw_Line_d ; dy=-8, dx=-6
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$46
    LDB #$54
    JSR Moveto_d ; move to (84, 70)
    CLR Vec_Misc_Count
    LDA #$F8
    LDB #$06
    JSR Draw_Line_d ; dy=-8, dx=6
    CLRA
    CLRB
    STD RESULT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BGT CT_10
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BGT CT_14
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    LBEQ IF_NEXT_13
    LDA #$D0
    TFR A,DP
    LDA #$3C
    LDB #$50
    JSR Moveto_d ; move to (80, 60)
    LDA #$20
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$05
    LDB #$05
    JSR Draw_Line_d ; dy=5, dx=5
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BLT CT_18
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BLT CT_22
    BRA CE_23
CT_22:
    LDD #1
    STD RESULT
CE_23:
    LDD RESULT
    LBEQ IF_NEXT_21
    LDA #$D0
    TFR A,DP
    LDA #$50
    LDB #$64
    JSR Moveto_d ; move to (100, 80)
    CLR Vec_Misc_Count
    LDA #$FB
    LDB #$FB
    JSR Draw_Line_d ; dy=-5, dx=-5
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    RTS

    INCLUDE "runtime/vectorlist_runtime.asm"
VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
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
VAR_FRAME_COUNT EQU RESULT+30
VAR_VEL_X EQU RESULT+32
VAR_VEL_Y EQU RESULT+34
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "BOUNCING BALL"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+36
VAR_ARG1 EQU RESULT+38
VAR_ARG2 EQU RESULT+40
VAR_ARG3 EQU RESULT+42
VAR_ARG4 EQU RESULT+44
