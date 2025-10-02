; --- Motorola 6809 backend (Vectrex) title='TEST_OPERADORES_NUEVOS' origin=$0000 ---
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
    FCC "TEST OPERADORES NUEVOS"
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
    STD VAR_TEST_FRAME
    LDD #0
    STD VAR_TEST_PHASE
    LDD #0
    STD VAR_OBJECT_X
    LDD #0
    STD VAR_OBJECT_Y
    LDD #20
    STD VAR_OBJECT_SIZE
    LDD #100
    STD VAR_TEST_VALUE
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_FRAME
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_PHASE
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_Y
    STU TMPPTR
    STX ,U
    LDD #20
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_SIZE
    STU TMPPTR
    STX ,U
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 30 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    LDD VAR_TEST_FRAME
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
    LDU #VAR_TEST_FRAME
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 1 - Discriminant(7)
    LDD VAR_TEST_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #120
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_FRAME
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_PHASE
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
    LDU #VAR_TEST_PHASE
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_PHASE
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_Y
    STU TMPPTR
    STX ,U
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(1)
    LDD VAR_OBJECT_X
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(1)
    LDD VAR_OBJECT_Y
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(1)
    LDD VAR_OBJECT_SIZE
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(6)
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(6)
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BASE_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(6)
    LDA #0
    LDB #-127
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-2
    JSR Draw_Line_d
    ; DEBUG: Statement 10 - Discriminant(6)
    LDA #-127
    LDB #0
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-2
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 11 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    LDA #60
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDA #60
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #40
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; DEBUG: Statement 12 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    LBEQ IF_NEXT_13
    LDA #60
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDA #70
    LDB #-10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #60
    LDB #10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_OBJECT_X
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
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; DEBUG: Statement 13 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    LDA #60
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDA #60
    LDB #10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_OBJECT_Y
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
    LDU #VAR_OBJECT_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; DEBUG: Statement 14 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_22
    LDD #0
    STD RESULT
    BRA CE_23
CT_22:
    LDD #1
    STD RESULT
CE_23:
    LDD RESULT
    LBEQ IF_NEXT_21
    LDA #70
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #10
    JSR Draw_Line_d
    LDA #70
    LDB #-10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #-10
    JSR Draw_Line_d
    LDA #60
    LDB #10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_OBJECT_SIZE
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_SIZE
    STU TMPPTR
    STX ,U
    LDD VAR_OBJECT_SIZE
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_SIZE
    STU TMPPTR
    STX ,U
    LDD VAR_OBJECT_SIZE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    LBEQ IF_NEXT_25
    LDD #20
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_SIZE
    STU TMPPTR
    STX ,U
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; DEBUG: Statement 15 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    LBEQ IF_NEXT_29
    LDA #70
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #10
    JSR Draw_Line_d
    LDA #60
    LDB #-10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #-10
    JSR Draw_Line_d
    LDA #60
    LDB #10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_NEXT_33
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LBRA IF_END_32
IF_NEXT_33:
IF_END_32:
    LBRA IF_END_28
IF_NEXT_29:
IF_END_28:
    ; DEBUG: Statement 16 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
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
    LDA #70
    LDB #-25
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-25
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #10
    JSR Draw_Line_d
    LDA #70
    LDB #-15
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #0
    JSR Draw_Line_d
    LDA #60
    LDB #-15
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #5
    JSR Draw_Line_d
    LDA #60
    LDB #10
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
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
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #25
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_OBJECT_Y
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
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
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LBRA IF_END_36
IF_NEXT_37:
IF_END_36:
    ; DEBUG: Statement 17 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #6
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_42
    LDD #0
    STD RESULT
    BRA CE_43
CT_42:
    LDD #1
    STD RESULT
CE_43:
    LDD RESULT
    LBEQ IF_NEXT_41
    LDA #70
    LDB #-25
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-25
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #10
    JSR Draw_Line_d
    LDA #60
    LDB #-15
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #-10
    JSR Draw_Line_d
    LDA #70
    LDB #-5
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-5
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #10
    JSR Draw_Line_d
    LDA #60
    LDB #5
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #-10
    JSR Draw_Line_d
    LDD VAR_TEST_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD VAR_TEMP_CALC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LDD VAR_TEMP_CALC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_OBJECT_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_40
IF_NEXT_41:
IF_END_40:
    ; DEBUG: Statement 18 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_46
    LDD #0
    STD RESULT
    BRA CE_47
CT_46:
    LDD #1
    STD RESULT
CE_47:
    LDD RESULT
    LBEQ IF_NEXT_45
    LDA #70
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-30
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #10
    JSR Draw_Line_d
    LDA #60
    LDB #-20
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #-10
    JSR Draw_Line_d
    LDA #70
    LDB #-15
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-20
    LDB #0
    JSR Draw_Line_d
    LDA #70
    LDB #-15
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #10
    JSR Draw_Line_d
    LDA #60
    LDB #-5
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-10
    LDB #-10
    JSR Draw_Line_d
    LDA #60
    LDB #5
    JSR Moveto_d
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #20
    JSR Draw_Line_d
    LDD VAR_TEST_VALUE
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
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_OBJECT_X
    STU TMPPTR
    STX ,U
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_50
    LDD #0
    STD RESULT
    BRA CE_51
CT_50:
    LDD #1
    STD RESULT
CE_51:
    LDD RESULT
    LBEQ IF_NEXT_49
    LDD #120
    STD RESULT
    LDX RESULT
    LDU #VAR_TEST_VALUE
    STU TMPPTR
    STX ,U
    LBRA IF_END_48
IF_NEXT_49:
IF_END_48:
    LBRA IF_END_44
IF_NEXT_45:
IF_END_44:
    ; DEBUG: Statement 19 - Discriminant(1)
    LDD #100
    STD RESULT
    ; DEBUG: Statement 20 - Discriminant(1)
    LDD #90
    STD RESULT
    ; DEBUG: Statement 21 - Discriminant(1)
    LDD #0
    STD RESULT
    ; DEBUG: Statement 22 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_54
    LDD #0
    STD RESULT
    BRA CE_55
CT_54:
    LDD #1
    STD RESULT
CE_55:
    LDD RESULT
    LBEQ IF_NEXT_53
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_52
IF_NEXT_53:
IF_END_52:
    ; DEBUG: Statement 23 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_58
    LDD #0
    STD RESULT
    BRA CE_59
CT_58:
    LDD #1
    STD RESULT
CE_59:
    LDD RESULT
    LBEQ IF_NEXT_57
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_56
IF_NEXT_57:
IF_END_56:
    ; DEBUG: Statement 24 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_62
    LDD #0
    STD RESULT
    BRA CE_63
CT_62:
    LDD #1
    STD RESULT
CE_63:
    LDD RESULT
    LBEQ IF_NEXT_61
    LDD VAR_INDICATOR_X
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
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_60
IF_NEXT_61:
IF_END_60:
    ; DEBUG: Statement 25 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_66
    LDD #0
    STD RESULT
    BRA CE_67
CT_66:
    LDD #1
    STD RESULT
CE_67:
    LDD RESULT
    LBEQ IF_NEXT_65
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #17
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_64
IF_NEXT_65:
IF_END_64:
    ; DEBUG: Statement 26 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_70
    LDD #0
    STD RESULT
    BRA CE_71
CT_70:
    LDD #1
    STD RESULT
CE_71:
    LDD RESULT
    LBEQ IF_NEXT_69
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_68
IF_NEXT_69:
IF_END_68:
    ; DEBUG: Statement 27 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_74
    LDD #0
    STD RESULT
    BRA CE_75
CT_74:
    LDD #1
    STD RESULT
CE_75:
    LDD RESULT
    LBEQ IF_NEXT_73
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_72
IF_NEXT_73:
IF_END_72:
    ; DEBUG: Statement 28 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #6
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_78
    LDD #0
    STD RESULT
    BRA CE_79
CT_78:
    LDD #1
    STD RESULT
CE_79:
    LDD RESULT
    LBEQ IF_NEXT_77
    LDD VAR_INDICATOR_X
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
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_76
IF_NEXT_77:
IF_END_76:
    ; DEBUG: Statement 29 - Discriminant(7)
    LDD VAR_TEST_PHASE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_82
    LDD #0
    STD RESULT
    BRA CE_83
CT_82:
    LDD #1
    STD RESULT
CE_83:
    LDD RESULT
    LBEQ IF_NEXT_81
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_INDICATOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #17
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INDICATOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #100
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA IF_END_80
IF_NEXT_81:
IF_END_80:
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
VAR_BASE_X EQU $C900+0
VAR_BASE_Y EQU $C900+2
VAR_INDICATOR_X EQU $C900+4
VAR_INDICATOR_Y EQU $C900+6
VAR_OBJECT_SIZE EQU $C900+8
VAR_OBJECT_X EQU $C900+10
VAR_OBJECT_Y EQU $C900+12
VAR_SIZE EQU $C900+14
VAR_TEMP_CALC EQU $C900+16
VAR_TEST_FRAME EQU $C900+18
VAR_TEST_PHASE EQU $C900+20
VAR_TEST_VALUE EQU $C900+22
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+24
VLINE_DY EQU RESULT+25
VLINE_STEPS EQU RESULT+26
VLINE_LIST EQU RESULT+27
