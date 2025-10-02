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
    LDD #0
    STD VAR_X
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_X
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
    ; DEBUG: Processing 16 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDA #-100
    LDB #-100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-56
    JSR Draw_Line_d
    ; DEBUG: Statement 1 - Discriminant(6)
    LDA #-100
    LDB #100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-56
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 2 - Discriminant(6)
    LDA #100
    LDB #100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #56
    JSR Draw_Line_d
    ; DEBUG: Statement 3 - Discriminant(6)
    LDA #100
    LDB #-100
    JSR Moveto_d
    LDA #$32
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #56
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 4 - Discriminant(6)
    LDA #0
    LDB #-80
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #-96
    JSR Draw_Line_d
    ; DEBUG: Statement 5 - Discriminant(6)
    LDA #-80
    LDB #0
    JSR Moveto_d
    LDA #$1E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #-96
    LDB #0
    JSR Draw_Line_d
    ; DEBUG: Statement 6 - Discriminant(0)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    ; DEBUG: Statement 7 - Discriminant(7)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #160
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
    LDD #65376
    STD RESULT
    LDX RESULT
    LDU #VAR_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 8 - Discriminant(1)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #0
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65516
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #20
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65521
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #65521
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65521
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #15
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #15
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #15
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(6)
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #15
    STD VAR_ARG1
    LDD VAR_POS
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #65521
    STD VAR_ARG3
    LDD #80
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(7)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #80
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    LDA #90
    LDB #-90
    JSR Moveto_d
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #5
    JSR Draw_Line_d
    LDA #85
    LDB #-90
    JSR Moveto_d
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #0
    LDB #5
    JSR Draw_Line_d
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
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
VAR_POS EQU $C900+0
VAR_X EQU $C900+2
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+4
VLINE_DY EQU RESULT+5
VLINE_STEPS EQU RESULT+6
VLINE_LIST EQU RESULT+7
