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
    JMP START

START:
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    LDD #3
    STD VAR_TREE_DEPTH
    LDD #40
    STD VAR_BASE_LENGTH
    LDD #30
    STD VAR_ANGLE_DELTA
    LDD #0
    STD VAR_ANIMATION_FRAME
    LDD #120
    STD VAR_INTENSITY_
    LDD #100
    STD VAR_SCALE_FACTOR
    LDD #0
    STD VAR_ANGLE_OFFSET
    LDD #0
    STD VAR_X1
    LDD #0
    STD VAR_Y1
    LDD #0
    STD VAR_X2
    LDD #0
    STD VAR_Y2
    LDD #0
    STD VAR_LENGTH1
    LDD #0
    STD VAR_LENGTH2
    LDD #0
    STD VAR_ANGLE1_LEFT
    LDD #0
    STD VAR_ANGLE1_RIGHT
    LDD #0
    STD VAR_ANGLE2_LL
    LDD #0
    STD VAR_ANGLE2_LR
    LDD #0
    STD VAR_ANGLE2_RL
    LDD #0
    STD VAR_ANGLE2_RR
    LDD #0
    STD VAR_COS1_LEFT
    LDD #0
    STD VAR_SIN1_LEFT
    LDD #0
    STD VAR_COS1_RIGHT
    LDD #0
    STD VAR_SIN1_RIGHT
    LDD #0
    STD VAR_COS2_LL
    LDD #0
    STD VAR_SIN2_LL
    LDD #0
    STD VAR_COS2_LR
    LDD #0
    STD VAR_SIN2_LR
    LDD #0
    STD VAR_COS2_RL
    LDD #0
    STD VAR_SIN2_RL
    LDD #0
    STD VAR_COS2_RR
    LDD #0
    STD VAR_SIN2_RR
    LDD #0
    STD VAR_X2_LEFT
    LDD #0
    STD VAR_Y2_LEFT
    LDD #0
    STD VAR_X2_RIGHT
    LDD #0
    STD VAR_Y2_RIGHT
    LDD #0
    STD VAR_X3_LL
    LDD #0
    STD VAR_Y3_LL
    LDD #0
    STD VAR_X3_LR
    LDD #0
    STD VAR_Y3_LR
    LDD #0
    STD VAR_X3_RL
    LDD #0
    STD VAR_Y3_RL
    LDD #0
    STD VAR_X3_RR
    LDD #0
    STD VAR_Y3_RR
    LDD #3
    STD RESULT
    LDX RESULT
    LDU #VAR_TREE_DEPTH
    STU TMPPTR
    STX ,U
    LDD #40
    STD RESULT
    LDX RESULT
    LDU #VAR_BASE_LENGTH
    STU TMPPTR
    STX ,U
    LDD #30
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE_DELTA
    STU TMPPTR
    STX ,U
    LDD #120
    STD RESULT
    LDX RESULT
    LDU #VAR_INTENSITY_
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
    ; DEBUG: Processing 51 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    LDD VAR_ANIMATION_FRAME
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
    LDU #VAR_ANIMATION_FRAME
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 1 - Discriminant(7)
    LDD VAR_ANIMATION_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #360
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
    LDU #VAR_ANIMATION_FRAME
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(0)
    LDD VAR_ANIMATION_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDX RESULT
    LDU #VAR_ANGLE_OFFSET
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 3 - Discriminant(0)
    LDD #90
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANIMATION_FRAME
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
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_SCALE_FACTOR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 4 - Discriminant(6)
    LDA #-100
    LDB #-100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-56
    JSR Draw_Line_d
    ; DEBUG: Statement 5 - Discriminant(6)
    LDA #-100
    LDB #100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-56
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 6 - Discriminant(6)
    LDA #100
    LDB #100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #56
    JSR Draw_Line_d
    ; DEBUG: Statement 7 - Discriminant(6)
    LDA #100
    LDB #-100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #56
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 8 - Discriminant(0)
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_X1
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 9 - Discriminant(0)
    LDD #65486
    STD RESULT
    LDX RESULT
    LDU #VAR_Y1
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 10 - Discriminant(0)
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_X2
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 11 - Discriminant(0)
    LDD VAR_Y1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BASE_LENGTH
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y2
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 12 - Discriminant(6)
    LDD VAR_X1
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X2
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y2
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(0)
    LDD VAR_BASE_LENGTH
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #75
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_LENGTH1
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 14 - Discriminant(0)
    LDD #270
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANGLE_DELTA
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE1_LEFT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 15 - Discriminant(0)
    CLRA
    CLRB
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE1_RIGHT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 16 - Discriminant(0)
    LDD VAR_ANGLE1_LEFT
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS1_LEFT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 17 - Discriminant(0)
    LDD VAR_ANGLE1_LEFT
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN1_LEFT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 18 - Discriminant(0)
    LDD VAR_X2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS1_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X2_LEFT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 19 - Discriminant(0)
    LDD VAR_Y2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN1_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y2_LEFT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 20 - Discriminant(6)
    LDD VAR_X2
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 21 - Discriminant(0)
    LDD VAR_ANGLE1_RIGHT
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS1_RIGHT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 22 - Discriminant(0)
    LDD VAR_ANGLE1_RIGHT
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN1_RIGHT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 23 - Discriminant(0)
    LDD VAR_X2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS1_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X2_RIGHT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 24 - Discriminant(0)
    LDD VAR_Y2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN1_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y2_RIGHT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 25 - Discriminant(6)
    LDD VAR_X2
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 26 - Discriminant(0)
    LDD VAR_LENGTH1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_LENGTH2
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 27 - Discriminant(0)
    LDD VAR_ANGLE1_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANGLE_DELTA
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE2_LL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 28 - Discriminant(0)
    LDD VAR_ANGLE1_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANGLE_DELTA
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE2_LR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 29 - Discriminant(0)
    LDD VAR_ANGLE2_LL
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS2_LL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 30 - Discriminant(0)
    LDD VAR_ANGLE2_LL
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN2_LL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 31 - Discriminant(0)
    LDD VAR_X2_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS2_LL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X3_LL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 32 - Discriminant(0)
    LDD VAR_Y2_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN2_LL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y3_LL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 33 - Discriminant(6)
    LDD VAR_X2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X3_LL
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y3_LL
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 34 - Discriminant(0)
    LDD VAR_ANGLE2_LR
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS2_LR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 35 - Discriminant(0)
    LDD VAR_ANGLE2_LR
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN2_LR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 36 - Discriminant(0)
    LDD VAR_X2_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS2_LR
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X3_LR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 37 - Discriminant(0)
    LDD VAR_Y2_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN2_LR
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y3_LR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 38 - Discriminant(6)
    LDD VAR_X2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X3_LR
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y3_LR
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 39 - Discriminant(0)
    LDD VAR_ANGLE1_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANGLE_DELTA
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE2_RL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 40 - Discriminant(0)
    LDD VAR_ANGLE1_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ANGLE_DELTA
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE2_RR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 41 - Discriminant(0)
    LDD VAR_ANGLE2_RL
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS2_RL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 42 - Discriminant(0)
    LDD VAR_ANGLE2_RL
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN2_RL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 43 - Discriminant(0)
    LDD VAR_X2_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS2_RL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X3_RL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 44 - Discriminant(0)
    LDD VAR_Y2_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN2_RL
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y3_RL
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 45 - Discriminant(6)
    LDD VAR_X2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X3_RL
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y3_RL
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 46 - Discriminant(0)
    LDD VAR_ANGLE2_RR
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    LDX #COS_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_COS2_RR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 47 - Discriminant(0)
    LDD VAR_ANGLE2_RR
    STD RESULT
    LDD RESULT
    ANDB #$7F
    CLRA
    ASLB
    ROLA
    LDX #SIN_TABLE
    ABX
    LDD ,X
    STD RESULT
    LDX RESULT
    LDU #VAR_SIN2_RR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 48 - Discriminant(0)
    LDD VAR_X2_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_COS2_RR
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_X3_RR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 49 - Discriminant(0)
    LDD VAR_Y2_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SIN2_RR
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_LENGTH2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCALE_FACTOR
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_Y3_RR
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 50 - Discriminant(6)
    LDD VAR_X2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y2_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X3_RR
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y3_RR
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY_
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
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
VAR_ANGLE1_LEFT EQU $C900+0
VAR_ANGLE1_RIGHT EQU $C900+2
VAR_ANGLE2_LL EQU $C900+4
VAR_ANGLE2_LR EQU $C900+6
VAR_ANGLE2_RL EQU $C900+8
VAR_ANGLE2_RR EQU $C900+10
VAR_ANGLE_DELTA EQU $C900+12
VAR_ANGLE_OFFSET EQU $C900+14
VAR_ANIMATION_FRAME EQU $C900+16
VAR_BASE_LENGTH EQU $C900+18
VAR_COS1_LEFT EQU $C900+20
VAR_COS1_RIGHT EQU $C900+22
VAR_COS2_LL EQU $C900+24
VAR_COS2_LR EQU $C900+26
VAR_COS2_RL EQU $C900+28
VAR_COS2_RR EQU $C900+30
VAR_INTENSITY_ EQU $C900+32
VAR_LENGTH1 EQU $C900+34
VAR_LENGTH2 EQU $C900+36
VAR_SCALE_FACTOR EQU $C900+38
VAR_SIN1_LEFT EQU $C900+40
VAR_SIN1_RIGHT EQU $C900+42
VAR_SIN2_LL EQU $C900+44
VAR_SIN2_LR EQU $C900+46
VAR_SIN2_RL EQU $C900+48
VAR_SIN2_RR EQU $C900+50
VAR_TREE_DEPTH EQU $C900+52
VAR_X1 EQU $C900+54
VAR_X2 EQU $C900+56
VAR_X2_LEFT EQU $C900+58
VAR_X2_RIGHT EQU $C900+60
VAR_X3_LL EQU $C900+62
VAR_X3_LR EQU $C900+64
VAR_X3_RL EQU $C900+66
VAR_X3_RR EQU $C900+68
VAR_Y1 EQU $C900+70
VAR_Y2 EQU $C900+72
VAR_Y2_LEFT EQU $C900+74
VAR_Y2_RIGHT EQU $C900+76
VAR_Y3_LL EQU $C900+78
VAR_Y3_LR EQU $C900+80
VAR_Y3_RL EQU $C900+82
VAR_Y3_RR EQU $C900+84
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+86
VLINE_DY EQU RESULT+87
VLINE_STEPS EQU RESULT+88
VLINE_LIST EQU RESULT+89
; Trig tables (shared)
SIN_TABLE:
    FDB 0
    FDB 6
    FDB 12
    FDB 19
    FDB 25
    FDB 31
    FDB 37
    FDB 43
    FDB 49
    FDB 54
    FDB 60
    FDB 65
    FDB 71
    FDB 76
    FDB 81
    FDB 85
    FDB 90
    FDB 94
    FDB 98
    FDB 102
    FDB 106
    FDB 109
    FDB 112
    FDB 115
    FDB 117
    FDB 120
    FDB 122
    FDB 123
    FDB 125
    FDB 126
    FDB 126
    FDB 127
    FDB 127
    FDB 127
    FDB 126
    FDB 126
    FDB 125
    FDB 123
    FDB 122
    FDB 120
    FDB 117
    FDB 115
    FDB 112
    FDB 109
    FDB 106
    FDB 102
    FDB 98
    FDB 94
    FDB 90
    FDB 85
    FDB 81
    FDB 76
    FDB 71
    FDB 65
    FDB 60
    FDB 54
    FDB 49
    FDB 43
    FDB 37
    FDB 31
    FDB 25
    FDB 19
    FDB 12
    FDB 6
    FDB 0
    FDB -6
    FDB -12
    FDB -19
    FDB -25
    FDB -31
    FDB -37
    FDB -43
    FDB -49
    FDB -54
    FDB -60
    FDB -65
    FDB -71
    FDB -76
    FDB -81
    FDB -85
    FDB -90
    FDB -94
    FDB -98
    FDB -102
    FDB -106
    FDB -109
    FDB -112
    FDB -115
    FDB -117
    FDB -120
    FDB -122
    FDB -123
    FDB -125
    FDB -126
    FDB -126
    FDB -127
    FDB -127
    FDB -127
    FDB -126
    FDB -126
    FDB -125
    FDB -123
    FDB -122
    FDB -120
    FDB -117
    FDB -115
    FDB -112
    FDB -109
    FDB -106
    FDB -102
    FDB -98
    FDB -94
    FDB -90
    FDB -85
    FDB -81
    FDB -76
    FDB -71
    FDB -65
    FDB -60
    FDB -54
    FDB -49
    FDB -43
    FDB -37
    FDB -31
    FDB -25
    FDB -19
    FDB -12
    FDB -6
COS_TABLE:
    FDB 127
    FDB 127
    FDB 126
    FDB 126
    FDB 125
    FDB 123
    FDB 122
    FDB 120
    FDB 117
    FDB 115
    FDB 112
    FDB 109
    FDB 106
    FDB 102
    FDB 98
    FDB 94
    FDB 90
    FDB 85
    FDB 81
    FDB 76
    FDB 71
    FDB 65
    FDB 60
    FDB 54
    FDB 49
    FDB 43
    FDB 37
    FDB 31
    FDB 25
    FDB 19
    FDB 12
    FDB 6
    FDB 0
    FDB -6
    FDB -12
    FDB -19
    FDB -25
    FDB -31
    FDB -37
    FDB -43
    FDB -49
    FDB -54
    FDB -60
    FDB -65
    FDB -71
    FDB -76
    FDB -81
    FDB -85
    FDB -90
    FDB -94
    FDB -98
    FDB -102
    FDB -106
    FDB -109
    FDB -112
    FDB -115
    FDB -117
    FDB -120
    FDB -122
    FDB -123
    FDB -125
    FDB -126
    FDB -126
    FDB -127
    FDB -127
    FDB -127
    FDB -126
    FDB -126
    FDB -125
    FDB -123
    FDB -122
    FDB -120
    FDB -117
    FDB -115
    FDB -112
    FDB -109
    FDB -106
    FDB -102
    FDB -98
    FDB -94
    FDB -90
    FDB -85
    FDB -81
    FDB -76
    FDB -71
    FDB -65
    FDB -60
    FDB -54
    FDB -49
    FDB -43
    FDB -37
    FDB -31
    FDB -25
    FDB -19
    FDB -12
    FDB -6
    FDB 0
    FDB 6
    FDB 12
    FDB 19
    FDB 25
    FDB 31
    FDB 37
    FDB 43
    FDB 49
    FDB 54
    FDB 60
    FDB 65
    FDB 71
    FDB 76
    FDB 81
    FDB 85
    FDB 90
    FDB 94
    FDB 98
    FDB 102
    FDB 106
    FDB 109
    FDB 112
    FDB 115
    FDB 117
    FDB 120
    FDB 122
    FDB 123
    FDB 125
    FDB 126
    FDB 126
    FDB 127
TAN_TABLE:
    FDB 0
    FDB 1
    FDB 2
    FDB 3
    FDB 4
    FDB 5
    FDB 6
    FDB 7
    FDB 8
    FDB 9
    FDB 11
    FDB 12
    FDB 13
    FDB 15
    FDB 16
    FDB 18
    FDB 20
    FDB 22
    FDB 24
    FDB 27
    FDB 30
    FDB 33
    FDB 37
    FDB 42
    FDB 48
    FDB 56
    FDB 66
    FDB 80
    FDB 101
    FDB 120
    FDB 120
    FDB 120
    FDB -120
    FDB -120
    FDB -120
    FDB -120
    FDB -101
    FDB -80
    FDB -66
    FDB -56
    FDB -48
    FDB -42
    FDB -37
    FDB -33
    FDB -30
    FDB -27
    FDB -24
    FDB -22
    FDB -20
    FDB -18
    FDB -16
    FDB -15
    FDB -13
    FDB -12
    FDB -11
    FDB -9
    FDB -8
    FDB -7
    FDB -6
    FDB -5
    FDB -4
    FDB -3
    FDB -2
    FDB -1
    FDB 0
    FDB 1
    FDB 2
    FDB 3
    FDB 4
    FDB 5
    FDB 6
    FDB 7
    FDB 8
    FDB 9
    FDB 11
    FDB 12
    FDB 13
    FDB 15
    FDB 16
    FDB 18
    FDB 20
    FDB 22
    FDB 24
    FDB 27
    FDB 30
    FDB 33
    FDB 37
    FDB 42
    FDB 48
    FDB 56
    FDB 66
    FDB 80
    FDB 101
    FDB 120
    FDB 120
    FDB 120
    FDB -120
    FDB -120
    FDB -120
    FDB -120
    FDB -101
    FDB -80
    FDB -66
    FDB -56
    FDB -48
    FDB -42
    FDB -37
    FDB -33
    FDB -30
    FDB -27
    FDB -24
    FDB -22
    FDB -20
    FDB -18
    FDB -16
    FDB -15
    FDB -13
    FDB -12
    FDB -11
    FDB -9
    FDB -8
    FDB -7
    FDB -6
    FDB -5
    FDB -4
    FDB -3
    FDB -2
    FDB -1
