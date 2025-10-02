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
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 35 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDA #-50
    LDB #-50
    JSR Moveto_d
    LDA #$28
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #100
    JSR Draw_Line_d
    ; DEBUG: Statement 1 - Discriminant(6)
    LDA #-50
    LDB #50
    JSR Moveto_d
    LDA #$28
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #100
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 2 - Discriminant(6)
    LDA #50
    LDB #50
    JSR Moveto_d
    LDA #$28
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-100
    JSR Draw_Line_d
    ; DEBUG: Statement 3 - Discriminant(6)
    LDA #50
    LDB #-50
    JSR Moveto_d
    LDA #$28
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-100
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 4 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(1)
    LDD #20
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    LDX #STR_11
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_8
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(0)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    LDU #VAR_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 9 - Discriminant(6)
    LDX #STR_10
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(6)
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(1)
    LDD #17
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(1)
    LDD VAR_Y
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
    ; DEBUG: Statement 13 - Discriminant(6)
    LDX #STR_17
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(6)
    LDX #STR_16
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_RESULT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
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
    ; DEBUG: Statement 16 - Discriminant(6)
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 17 - Discriminant(1)
    LDD #23
    STD RESULT
    ; DEBUG: Statement 18 - Discriminant(6)
    LDX #STR_19
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Z
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 19 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_9
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 20 - Discriminant(0)
    LDD VAR_Z
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    LDU #VAR_Z
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 21 - Discriminant(6)
    LDX #STR_18
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Z
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 22 - Discriminant(6)
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 23 - Discriminant(1)
    LDD #3
    STD RESULT
    ; DEBUG: Statement 24 - Discriminant(6)
    LDX #STR_15
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_W
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 25 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65516
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 26 - Discriminant(0)
    LDD VAR_W
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD RESULT
    LDX RESULT
    LDU #VAR_W
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 27 - Discriminant(6)
    LDX #STR_14
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_W
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 28 - Discriminant(6)
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65516
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 29 - Discriminant(1)
    LDD #19
    STD RESULT
    ; DEBUG: Statement 30 - Discriminant(6)
    LDX #STR_13
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_V
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 31 - Discriminant(6)
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 32 - Discriminant(0)
    LDD VAR_V
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #6
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
    LDU #VAR_V
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 33 - Discriminant(6)
    LDX #STR_12
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_V
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 34 - Discriminant(6)
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
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

VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
VECTREX_DEBUG_PRINT_LABELED:
    ; Debug print with label - emulator intercepts special addresses
    ; First write label marker (0xFE) to indicate labeled output
    LDA #$FE
    STA $DFFF        ; Label marker
    ; Write label string pointer to special address
    LDA VAR_ARG0     ; Label string pointer high byte
    STA $DFFE        ; Label pointer high
    LDA VAR_ARG0+1   ; Label string pointer low byte  
    STA $DFFD        ; Label pointer low
    ; Write value to debug output
    LDA VAR_ARG1+1   ; Load value to debug print
    STA $DFFF        ; Value to debug output
    RTS
VECTREX_SET_INTENSITY:
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
VECTREX_WAIT_RECAL:
    JSR WAIT_RECAL
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
VAR_RESULT EQU $C900+0
VAR_V EQU $C900+2
VAR_W EQU $C900+4
VAR_X EQU $C900+6
VAR_Y EQU $C900+8
VAR_Z EQU $C900+10
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "17//5 ->"
    FCB $80
STR_1:
    FCC "1?"
    FCB $80
STR_2:
    FCC "3?"
    FCB $80
STR_3:
    FCC "5?"
    FCB $80
STR_4:
    FCC "6?"
    FCB $80
STR_5:
    FCC "OPERATOR DIAGNOSIS"
    FCB $80
STR_6:
    FCC "V=19, V%=6 ->"
    FCB $80
STR_7:
    FCC "W=3, W*=2 ->"
    FCB $80
STR_8:
    FCC "X=20, X/=4 ->"
    FCB $80
STR_9:
    FCC "Z=23, Z//=7 ->"
    FCB $80
STR_10:
    FCC "AFTER_DIV_EQ"
    FCB $80
STR_11:
    FCC "BEFORE_DIV_EQ"
    FCB $80
STR_12:
    FCC "V_AFTER_MOD"
    FCB $80
STR_13:
    FCC "V_INITIAL"
    FCB $80
STR_14:
    FCC "W_AFTER_MULT"
    FCB $80
STR_15:
    FCC "W_INITIAL"
    FCB $80
STR_16:
    FCC "Y_FLOOR_DIV_5"
    FCB $80
STR_17:
    FCC "Y_INITIAL"
    FCB $80
STR_18:
    FCC "Z_AFTER_FLOOR_EQ"
    FCB $80
STR_19:
    FCC "Z_INITIAL"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
