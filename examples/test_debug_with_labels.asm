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
    ; DEBUG: Processing 10 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(1)
    LDD #42
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(1)
    LDD #100
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(6)
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
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
    ; DEBUG: Statement 5 - Discriminant(6)
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR STR
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
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
    ; DEBUG: Statement 7 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
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
    ; DEBUG: Statement 8 - Discriminant(6)
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR STR
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(6)
    LDD VAR_Y
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
    ; Debug print to console - emulator intercepts special address
    LDA VAR_ARG0+1   ; Load value to debug print
    STA $DFFF        ; Special debug output address - emulator intercepts
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
VAR_X EQU $C900+0
VAR_Y EQU $C900+2
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "X="
    FCB $80
STR_1:
    FCC "Y="
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
