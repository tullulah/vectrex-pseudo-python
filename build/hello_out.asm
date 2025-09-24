; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "../include/VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1998"
    FCB $80
    FDB music1 ; default BIOS tune
    FCB $F8 ; height
    FCB $50 ; width
    FCB $20 ; rel y
    FCB $AA ; rel x (-$56)
    FCC "UNTITLED"
    FCB $80 ; title terminator
    FCB 0 ; reserved
    RMB $0030-* ; pad header to $30
    ORG $0030

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Simple implicit frame loop
ENTRY_LOOP: JSR Wait_Recal
    JSR Intensity_5F
    JSR MAIN
    BRA ENTRY_LOOP

MAIN: ; function
; --- function main ---
    LDD #65456
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #16
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
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
    IF * < $2000
PADSIZE SET $2000-*
    FILL $FF,PADSIZE
    ENDC
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; String literals (high-bit terminated for Vectrex PRINT_STR_D)
STR_0:
    FCB $48
    FCB $45
    FCB $4C
    FCB $4C
    FCB $4F
    FCB $20
    FCB $57
    FCB $4F
    FCB $52
    FCB $4C
    FCB $C4
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
