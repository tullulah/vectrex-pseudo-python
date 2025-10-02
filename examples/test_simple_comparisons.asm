; --- Motorola 6809 backend (Vectrex) title='TEST_SIMPLE_COMPARISONS' origin=$0000 ---
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
    FCC "TEST SIMPLE COMPARISONS"
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
    ; DEBUG: Processing 3 statements in loop() body
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
    LDD #30
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
    LDA #-50
    LDB #-50
    JSR Moveto_d
    LDA #$50
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #100
    JSR Draw_Line_d
    LBRA IF_END_0
IF_NEXT_1:
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #60
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
    LDA #50
    LDB #-50
    JSR Moveto_d
    LDA #$50
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #100
    JSR Draw_Line_d
    LBRA IF_END_0
IF_NEXT_4:
    LDA #-50
    LDB #-50
    JSR Moveto_d
    LDA #$50
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #100
    LDB #0
    JSR Draw_Line_d
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(7)
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #90
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
    LBRA IF_END_7
IF_NEXT_8:
IF_END_7:
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
VAR_COUNTER EQU $C900+0
; Call argument scratch space
