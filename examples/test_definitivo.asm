; --- Motorola 6809 backend (Vectrex) title='TEST_DEFINITIVO' origin=$0000 ---
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
    FCC "TEST DEFINITIVO"
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

    ; *** DEBUG *** main() function code inline (initialization)
    LDD #0
    STD VAR_COUNTER
    LDD #1
    STD VAR_DIRECTION
    LDD #0
    STD VAR_BALL_X
    LDD #0
    STD VAR_BALL_Y
    LDD #3
    STD VAR_BALL_VX
    LDD #2
    STD VAR_BALL_VY
    LDD #80
    STD VAR_INTENSITY_LEVEL
    LDD #0
    STD VAR_TEST_MODE
    LDD #0
    STD RESULT

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 15 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTER
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 1 - Discriminant(7)
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    LDA #60
    LDB #-80
    JSR Moveto_d
    LDA #$28
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-96
    JSR Draw_Line_d
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_MODE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_5
    LDD #0
    STD RESULT
    BRA CE_6
CT_5:
    LDD #1
    STD RESULT
CE_6:
    LDD RESULT
    LBEQ IF_NEXT_4
    LDA #60
    LDB #-80
    JSR Moveto_d
    LDA #$46
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-120
    LDB #0
    JSR Draw_Line_d
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_MODE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_4:
    LDA #-60
    LDB #-80
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #120
    LDB #-96
    JSR Draw_Line_d
    LDD #3
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_MODE
    STU TMPPTR
    STX ,U
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(7)
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #120
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_9
    LDD #0
    STD RESULT
    BRA CE_10
CT_9:
    LDD #1
    STD RESULT
CE_10:
    LDD RESULT
    LBEQ IF_NEXT_8
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTER
    STU TMPPTR
    STX ,U
    LDD VAR_DIRECTION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDX RESULT
    LDU #VAR_DIRECTION
    STU TMPPTR
    STX ,U
    LBRA IF_END_7
IF_NEXT_8:
IF_END_7:
    ; DEBUG: Statement 3 - Discriminant(0)
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_VX
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
    ; DEBUG: Statement 4 - Discriminant(0)
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_VY
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
    ; DEBUG: Statement 5 - Discriminant(7)
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_13
    LDD #0
    STD RESULT
    BRA CE_14
CT_13:
    LDD #1
    STD RESULT
CE_14:
    LDD RESULT
    LBEQ IF_NEXT_12
    LDD #65533
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_VX
    STU TMPPTR
    STX ,U
    LDD #95
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_LEVEL
    STU TMPPTR
    STX ,U
    LBRA IF_END_11
IF_NEXT_12:
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
    BLT CT_15
    LDD #0
    STD RESULT
    BRA CE_16
CT_15:
    LDD #1
    STD RESULT
CE_16:
    LDD RESULT
    LBEQ IF_END_11
    LDD #3
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_VX
    STU TMPPTR
    STX ,U
    LDD #95
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_LEVEL
    STU TMPPTR
    STX ,U
    LBRA IF_END_11
IF_END_11:
    ; DEBUG: Statement 6 - Discriminant(7)
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #80
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_19
    LDD #0
    STD RESULT
    BRA CE_20
CT_19:
    LDD #1
    STD RESULT
CE_20:
    LDD RESULT
    LBEQ IF_NEXT_18
    LDD #65534
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_VY
    STU TMPPTR
    STX ,U
    LDD #95
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_LEVEL
    STU TMPPTR
    STX ,U
    LBRA IF_END_17
IF_NEXT_18:
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
    BLT CT_21
    LDD #0
    STD RESULT
    BRA CE_22
CT_21:
    LDD #1
    STD RESULT
CE_22:
    LDD RESULT
    LBEQ IF_END_17
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_VY
    STU TMPPTR
    STX ,U
    LDD #95
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_LEVEL
    STU TMPPTR
    STX ,U
    LBRA IF_END_17
IF_END_17:
    ; DEBUG: Statement 7 - Discriminant(7)
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_25
    LDD #0
    STD RESULT
    BRA CE_26
CT_25:
    LDD #1
    STD RESULT
CE_26:
    LDD RESULT
    LBEQ IF_NEXT_24
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_LEVEL
    STU TMPPTR
    STX ,U
    LBRA IF_END_23
IF_NEXT_24:
IF_END_23:
    ; DEBUG: Statement 8 - Discriminant(7)
    LDD VAR_TEST_MODE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_29
    LDD #0
    STD RESULT
    BRA CE_30
CT_29:
    LDD #1
    STD RESULT
CE_30:
    LDD RESULT
    LBEQ IF_NEXT_28
    LDD #65516
    STD VAR_ARG0
    LDD #0
    STD VAR_ARG1
    LDD #20
    STD VAR_ARG2
    LDD #0
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #0
    STD VAR_ARG0
    LDD #65516
    STD VAR_ARG1
    LDD #0
    STD VAR_ARG2
    LDD #20
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_27
IF_NEXT_28:
    LDD VAR_TEST_MODE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_32
    LDD #0
    STD RESULT
    BRA CE_33
CT_32:
    LDD #1
    STD RESULT
CE_33:
    LDD RESULT
    LBEQ IF_NEXT_31
    LDD #65521
    STD VAR_ARG0
    LDD #65521
    STD VAR_ARG1
    LDD #15
    STD VAR_ARG2
    LDD #65521
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #15
    STD VAR_ARG0
    LDD #65521
    STD VAR_ARG1
    LDD #15
    STD VAR_ARG2
    LDD #15
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #15
    STD VAR_ARG0
    LDD #15
    STD VAR_ARG1
    LDD #65521
    STD VAR_ARG2
    LDD #15
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #65521
    STD VAR_ARG0
    LDD #15
    STD VAR_ARG1
    LDD #65521
    STD VAR_ARG2
    LDD #65521
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_27
IF_NEXT_31:
    LDD VAR_TEST_MODE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_END_27
    LDD #0
    STD VAR_ARG0
    LDD #65511
    STD VAR_ARG1
    LDD #65524
    STD VAR_ARG2
    LDD #8
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #65524
    STD VAR_ARG0
    LDD #8
    STD VAR_ARG1
    LDD #12
    STD VAR_ARG2
    LDD #8
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD #12
    STD VAR_ARG0
    LDD #8
    STD VAR_ARG1
    LDD #0
    STD VAR_ARG2
    LDD #65511
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_27
IF_END_27:
    ; DEBUG: Statement 9 - Discriminant(7)
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    ; quotient in RESULT, need remainder: A - Q*B
    LDD DIV_A
    STD TMPLEFT
    LDD RESULT
    STD MUL_A
    LDD DIV_B
    STD MUL_B
    JSR MUL16
    ; product in RESULT, subtract from original A (TMPLEFT)
    LDD TMPLEFT
    SUBD RESULT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_38
    LDD #0
    STD RESULT
    BRA CE_39
CT_38:
    LDD #1
    STD RESULT
CE_39:
    LDD RESULT
    LBEQ IF_NEXT_37
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_LEVEL
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_36
IF_NEXT_37:
IF_END_36:
    ; DEBUG: Statement 10 - Discriminant(7)
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_42
    LDD #0
    STD RESULT
    BRA CE_43
CT_42:
    LDD #1
    STD RESULT
CE_43:
    LDD RESULT
    LBEQ IF_NEXT_41
    LDD #0
    STD VAR_ARG0
    LDD #0
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #60
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_40
IF_NEXT_41:
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_44
    LDD #0
    STD RESULT
    BRA CE_45
CT_44:
    LDD #1
    STD RESULT
CE_45:
    LDD RESULT
    LBEQ IF_END_40
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #60
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_40
IF_END_40:
    ; DEBUG: Statement 11 - Discriminant(6)
    LDA #-80
    LDB #-100
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-56
    JSR Draw_Line_d
    ; DEBUG: Statement 12 - Discriminant(6)
    LDA #-80
    LDB #100
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-96
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 13 - Discriminant(6)
    LDA #80
    LDB #100
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #56
    JSR Draw_Line_d
    ; DEBUG: Statement 14 - Discriminant(6)
    LDA #80
    LDB #-100
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #96
    LDB #0
    JSR Draw_Line_d
    RTS

MUL16:
    LDD MUL_A
    STD MUL_RES
    LDD #0
    STD MUL_TMP
    LDD MUL_B
    STD MUL_CNT
MUL16_LOOP:
    LDD MUL_CNT
    BEQ MUL16_DONE
    LDD MUL_CNT
    ANDA #1
    BEQ MUL16_SKIP
    LDD MUL_RES
    ADDD MUL_TMP
    STD MUL_TMP
MUL16_SKIP:
    LDD MUL_RES
    ASLB
    ROLA
    STD MUL_RES
    LDD MUL_CNT
    LSRA
    RORB
    STD MUL_CNT
    BRA MUL16_LOOP
MUL16_DONE:
    LDD MUL_TMP
    STD RESULT
    RTS

DIV16:
    LDD #0
    STD DIV_Q
    LDD DIV_A
    STD DIV_R
    LDD DIV_B
    BEQ DIV16_DONE
DIV16_LOOP:
    LDD DIV_R
    SUBD DIV_B
    BLO DIV16_DONE
    STD DIV_R
    LDD DIV_Q
    ADDD #1
    STD DIV_Q
    BRA DIV16_LOOP
DIV16_DONE:
    LDD DIV_Q
    STD RESULT
    RTS

; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly
; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.
; ALWAYS sets intensity and handles Reset0Ref properly.
DRAW_LINE_WRAPPER:
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA VAR_ARG4+1
    JSR Intensity_a
    ; CRITICAL: Reset integrator origin before each line
    JSR Reset0Ref
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DX_CLAMP_HI_OK
    LDD #127
DLW_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DX_CLAMP_LO_OK
    LDD #-128
DLW_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DY_CLAMP_HI_OK
    LDD #127
DLW_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DY_CLAMP_LO_OK
    LDD #-128
DLW_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; Further clamp to Vectrex hardware limits (-64 to +63)
    LDA VLINE_DX
    CMPA #63
    BLE DLW_DX_HI_OK
    LDA #63
DLW_DX_HI_OK: CMPA #-64
    BGE DLW_DX_LO_OK
    LDA #-64
DLW_DX_LO_OK: STA VLINE_DX
    ; Clamp dy to Vectrex limits
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

; DRAW_LINE_FAST - optimized version that skips redundant setup
; Use this for multiple consecutive draws with same intensity
; Args: (x0,y0,x1,y1) only - intensity must be set beforehand
DRAW_LINE_FAST:
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DX_CLAMP_HI_OK
    LDD #127
DLF_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DX_CLAMP_LO_OK
    LDD #-128
DLF_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DY_CLAMP_HI_OK
    LDD #127
DLF_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DY_CLAMP_LO_OK
    LDD #-128
DLF_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; Further clamp to Vectrex hardware limits (-64 to +63)
    LDA VLINE_DX
    CMPA #63
    BLE DLF_DX_HI_OK
    LDA #63
DLF_DX_HI_OK: CMPA #-64
    BGE DLF_DX_LO_OK
    LDA #-64
DLF_DX_LO_OK: STA VLINE_DX
    ; Clamp dy to Vectrex limits
    LDA VLINE_DY
    CMPA #63
    BLE DLF_DY_HI_OK
    LDA #63
DLF_DY_HI_OK: CMPA #-64
    BGE DLF_DY_LO_OK
    LDA #-64
DLF_DY_LO_OK: STA VLINE_DY
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
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
MUL_A    EQU RESULT+8
MUL_B    EQU RESULT+10
MUL_RES  EQU RESULT+12
MUL_TMP  EQU RESULT+14
MUL_CNT  EQU RESULT+16
DIV_A   EQU RESULT+18
DIV_B   EQU RESULT+20
DIV_Q   EQU RESULT+22
DIV_R   EQU RESULT+24
VAR_BALL_VX EQU $C900+0
VAR_BALL_VY EQU $C900+2
VAR_BALL_X EQU $C900+4
VAR_BALL_Y EQU $C900+6
VAR_COUNTER EQU $C900+8
VAR_DIRECTION EQU $C900+10
VAR_INTENSITY_LEVEL EQU $C900+12
VAR_TEST_MODE EQU $C900+14
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+16
VLINE_DY EQU RESULT+17
VLINE_STEPS EQU RESULT+18
VLINE_LIST EQU RESULT+19
