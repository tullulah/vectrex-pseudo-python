; --- Motorola 6809 backend (Vectrex) title='TEST_DEBUG_PRINT' origin=$0000 ---
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
    FCC "TEST DEBUG PRINT"
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
    ; DEBUG: Processing 9 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
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
    ; DEBUG: Statement 2 - Discriminant(6)
    LDD #42
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(6)
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(6)
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(1)
    LDD #75
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(0)
    LDD VAR_X
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
    LDU #VAR_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 8 - Discriminant(6)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    RTS

VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to end of RAM (safe area)
    LDA VAR_ARG0+1   ; Load value to debug print
    STA $CF00        ; Debug output value at end of RAM
    LDA #$42         ; Debug marker
    STA $CF01        ; Debug marker to indicate new output
    RTS
VECTREX_MOVE_TO:
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Moveto_d
    ; store new current position
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
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
VAR_X EQU $CF00+0
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "DEBUG TEST"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VCUR_X EQU RESULT+2
VCUR_Y EQU RESULT+3
