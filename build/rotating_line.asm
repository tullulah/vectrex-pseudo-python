; --- Motorola 6809 backend (Vectrex) title='ROTATING LINE' origin=$0000 ---
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
    FDB $0000
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "ROTATING LINE"
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
    STD VAR_ANGLE
    ; VPy_LINE:10
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE
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
    ; DEBUG: Processing 5 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(1)
    ; VPy_LINE:19
    LDD VAR_ANGLE
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
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(1)
    ; VPy_LINE:20
    LDD VAR_ANGLE
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
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:24
    LDD #0
    STD VAR_ARG0
    LDD #0
    STD VAR_ARG1
    LDD VAR_X_END
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y_END
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:27
    LDD VAR_ANGLE
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
    LDU #VAR_ANGLE
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 4 - Discriminant(7)
    ; VPy_LINE:28
    LDD VAR_ANGLE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:29
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_ANGLE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
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
DIV_A   EQU RESULT+18
DIV_B   EQU RESULT+20
DIV_Q   EQU RESULT+22
DIV_R   EQU RESULT+24
VAR_ANGLE EQU $CF00+0
VAR_X_END EQU $CF00+2
VAR_Y_END EQU $CF00+4
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+6
VLINE_DY EQU RESULT+7
VLINE_STEPS EQU RESULT+8
VLINE_LIST EQU RESULT+9
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
