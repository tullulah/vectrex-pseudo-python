; --- Motorola 6809 backend (Vectrex) title='FRACTAL_WITH_FUNCTION' origin=$0000 ---
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
    FCC "FRACTAL WITH FUNCTION"
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

DRAW_TREE: ; function
; --- function draw_tree ---
    LEAS -72,S ; allocate locals
    LDD #0
    STD RESULT
    LDX RESULT
    STX 40 ,S
    LDD #65486
    STD RESULT
    LDX RESULT
    STX 56 ,S
    LDD #0
    STD RESULT
    LDX RESULT
    STX 42 ,S
    LDD 56 ,S
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
    STX 58 ,S
    LDD 40 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 56 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 42 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 58 ,S
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
    STX 24 ,S
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
    STX 0 ,S
    CLRA
    CLRB
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LDD 0 ,S
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
    STX 12 ,S
    LDD 0 ,S
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
    STX 28 ,S
    LDD 42 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 12 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 24 ,S
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
    STX 44 ,S
    LDD 58 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 28 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 24 ,S
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
    STX 60 ,S
    LDD 42 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 58 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 44 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 60 ,S
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
    LDD 2 ,S
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
    STX 14 ,S
    LDD 2 ,S
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
    STX 30 ,S
    LDD 42 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 14 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 24 ,S
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
    STX 46 ,S
    LDD 58 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 30 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 24 ,S
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
    STX 62 ,S
    LDD 42 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 58 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 46 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 62 ,S
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
    LDD 24 ,S
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
    STX 26 ,S
    LDD 0 ,S
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
    STX 4 ,S
    LDD 0 ,S
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
    STX 6 ,S
    LDD 4 ,S
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
    STX 16 ,S
    LDD 4 ,S
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
    STX 32 ,S
    LDD 44 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 16 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 48 ,S
    LDD 60 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 32 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 64 ,S
    LDD 44 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 60 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 48 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 64 ,S
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
    LDD 6 ,S
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
    STX 18 ,S
    LDD 6 ,S
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
    STX 34 ,S
    LDD 44 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 18 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 50 ,S
    LDD 60 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 34 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 66 ,S
    LDD 44 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 60 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 50 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 66 ,S
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
    LDD 2 ,S
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
    STX 8 ,S
    LDD 2 ,S
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
    STX 10 ,S
    LDD 8 ,S
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
    STX 20 ,S
    LDD 8 ,S
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
    STX 36 ,S
    LDD 46 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 20 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 52 ,S
    LDD 62 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 36 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 68 ,S
    LDD 46 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 62 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 52 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 68 ,S
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
    LDD 10 ,S
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
    STX 22 ,S
    LDD 10 ,S
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
    STX 38 ,S
    LDD 46 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 22 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 54 ,S
    LDD 62 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 38 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 26 ,S
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
    STX 70 ,S
    LDD 46 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD 62 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD 54 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD 70 ,S
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
    LEAS 72,S ; free locals
    RTS

LOOP_BODY:
    ; DEBUG: Processing 9 statements in loop() body
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
    ; DEBUG: Statement 8 - Discriminant(6)
    JSR DRAW_TREE
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
VAR_ANGLE_DELTA EQU $C900+0
VAR_ANGLE_OFFSET EQU $C900+2
VAR_ANIMATION_FRAME EQU $C900+4
VAR_BASE_LENGTH EQU $C900+6
VAR_INTENSITY_ EQU $C900+8
VAR_SCALE_FACTOR EQU $C900+10
VAR_TREE_DEPTH EQU $C900+12
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+14
VLINE_DY EQU RESULT+15
VLINE_STEPS EQU RESULT+16
VLINE_LIST EQU RESULT+17
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
