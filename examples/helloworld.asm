; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
        ORG $0000
; Basic Vectrex header (placeholder)
    FCB $67,$20,$56,$45,$43,$54,$52,$45,$58,$20,$47,$41,$4D,$45,$20
    FCB $00,$00,$00,$00

JSR INIT_ENGINE
MAIN: ; function
; --- function main ---
main:
    JSR SET_ORIGIN
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR MOVE_TO
    CLRA
    CLRB
    STD RESULT
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #15
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LDD #25
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #15
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #15
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    RTS

JSR MAIN
END_LOOP: BRA END_LOOP

; Runtime helpers
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

; --- Vectrex built-in wrappers ---
PRINT_TEXT:
    ; args: x=VAR_ARG0 y=VAR_ARG1 ptr=VAR_ARG2
    RTS
MOVE_TO:
    ; args: x=VAR_ARG0 y=VAR_ARG1
    RTS
DRAW_TO:
    ; args: x=VAR_ARG0 y=VAR_ARG1 intensity=VAR_ARG2
    RTS
DRAW_LINE:
    ; args: x1=VAR_ARG0 y1=VAR_ARG1 x2=VAR_ARG2 y2=VAR_ARG3 inten=VAR_ARG4
    RTS
SET_ORIGIN:
    ; no args
    RTS
; Variables
; String literals (null-terminated)
STR_0: FCC "HELLO VECTREX"
    FCB 0
; Call argument scratch space
VAR_ARG0: FDB 0
VAR_ARG1: FDB 0
VAR_ARG2: FDB 0
VAR_ARG3: FDB 0
