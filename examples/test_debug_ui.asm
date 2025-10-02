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
    ; DEBUG: Processing 13 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDD #42
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(1)
    LDD #50
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(1)
    LDD #75
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    LDX #STR_2
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
    ; DEBUG: Statement 6 - Discriminant(6)
    LDX #STR_4
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
    ; DEBUG: Statement 7 - Discriminant(1)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(6)
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_SUM
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(0)
    LDD VAR_X
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
    LDX RESULT
    LDU #VAR_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 10 - Discriminant(0)
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD RESULT
    LDX RESULT
    LDU #VAR_Y
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 11 - Discriminant(6)
    LDX #STR_1
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
    ; DEBUG: Statement 12 - Discriminant(6)
    LDX #STR_3
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

VECTREX_DEBUG_PRINT:
    ; Debug print to console - emulator intercepts special address
    LDA VAR_ARG0+1   ; Load value to debug print
    STA $DFFF        ; Special debug output address - emulator intercepts
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
    JSR Wait_Recal
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
VAR_SUM EQU $C900+0
VAR_X EQU $C900+2
VAR_Y EQU $C900+4
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "SUM"
    FCB $80
STR_1:
    FCC "X_AFTER_PLUS"
    FCB $80
STR_2:
    FCC "X_VALUE"
    FCB $80
STR_3:
    FCC "Y_AFTER_MULT"
    FCB $80
STR_4:
    FCC "Y_VALUE"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
