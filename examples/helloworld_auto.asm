; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
WAIT_RECAL    EQU $F192
INTENSITY_5F EQU $F2A5
INTENSITY_A  EQU $F2A7 ; set intensity from A
PRINT_STR_D  EQU $F37A
MOVETO_D     EQU $F312 ; move to absolute coordinate in D (A=Y,B=X)
RESET0REF    EQU $F354 ; reset zero reference
DP_TO_C8     EQU $F1A2 ; BIOS set direct page to $C8
MUSIC1       EQU $FD0D

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80
    FDB MUSIC1
    FCB $F8
    FCB $50
    FCB $20
    FCB $AA ; legacy reserved pattern
    FCC "UNTITLED"
    FCB $80
    FCB 0
    RMB $0030-* ; pad header to $30

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Init then implicit frame loop (auto_loop enabled)
INIT_START: JSR DP_TO_C8
    JSR INTENSITY_5F
    JSR RESET0REF
    BRA ENTRY_LOOP
ENTRY_LOOP: JSR WAIT_RECAL
    JSR RESET0REF
    JSR MAIN
    BRA ENTRY_LOOP

MAIN: ; function
; --- function main ---
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
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
VECTREX_PRINT_TEXT:
    ; Position then print string
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR MOVETO_D
    LDU VAR_ARG2
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR PRINT_STR_D
    RTS
VECTREX_MOVE_TO:
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR MOVETO_D
    ; store new current position
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
; TODO: implement DRAW_TO using vector list (Draw_VL) or incremental delta steps.
VECTREX_DRAW_TO:
    ; update current position (no actual drawing yet)
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
; Simple incremental line (prototype, low precision).
VECTREX_DRAW_LINE:
    ; intensity in arg4 low byte
    LDA VAR_ARG4+1
    JSR INTENSITY_A
    ; load start (x0,y0) into A/B and move
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR MOVETO_D
    ; dx = x1 - x0 (low bytes) -> store in TMPLEFT low
    LDA VAR_ARG2+1
    SUBA VAR_ARG0+1
    STA VLINE_DX
    ; dy = y1 - y0
    LDA VAR_ARG3+1
    SUBA VAR_ARG1+1
    STA VLINE_DY
    ; steps = 16 (fixed small line segmentation)
    LDA #16
    STA VLINE_STEPS
VLINE_LOOP:
    LDA VLINE_STEPS
    BEQ VLINE_DONE
    ; x += dx/16 (arithmetic shift) -> naive: use dx >> 4 accumulate in VCUR_X
    LDA VLINE_DX
    ; sign extend not handled (prototype)
    LSRA
    LSRA
    LSRA
    LSRA
    ADDA VCUR_X
    STA VCUR_X
    LDA VLINE_DY
    LSRA
    LSRA
    LSRA
    LSRA
    ADDA VCUR_Y
    STA VCUR_Y
    ; Move to new point
    LDA VCUR_Y
    LDB VCUR_X
    JSR MOVETO_D
    DEC VLINE_STEPS
    BRA VLINE_LOOP
VLINE_DONE:
    RTS
VECTREX_SET_ORIGIN:
    JSR RESET0REF
    RTS
VECTREX_SET_INTENSITY:
    LDA VAR_ARG0+1
    JSR INTENSITY_A
    RTS
VECTREX_WAIT_RECAL:
    JSR WAIT_RECAL
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables
RESULT:   FDB 0
TMPLEFT:  FDB 0
TMPRIGHT: FDB 0
TMPPTR:   FDB 0
MUL_A:    FDB 0
MUL_B:    FDB 0
MUL_RES:  FDB 0
MUL_TMP:  FDB 0
MUL_CNT:  FDB 0
DIV_A:    FDB 0
DIV_B:    FDB 0
DIV_Q:   FDB 0
DIV_R:   FDB 0
; Call argument scratch space
VAR_ARG0: FDB 0
VAR_ARG1: FDB 0
VAR_ARG2: FDB 0
VAR_ARG3: FDB 0
VAR_ARG4: FDB 0
; Current beam position (low byte storage)
VCUR_X: FCB 0
VCUR_Y: FCB 0
; Line drawing temps
VLINE_DX: FCB 0
VLINE_DY: FCB 0
VLINE_STEPS: FCB 0
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
