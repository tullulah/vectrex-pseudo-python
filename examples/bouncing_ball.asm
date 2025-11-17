; --- Motorola 6809 backend (Vectrex) title='BOUNCING BALL DEMO' origin=$0000 ---
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
    FCC "BOUNCING BALL DEMO"
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
    STD VAR_BALL_X
    LDD #0
    STD VAR_BALL_Y
    LDD #3
    STD VAR_VEL_X
    LDD #2
    STD VAR_VEL_Y
    LDD #0
    STD VAR_FRAME_COUNT
    ; VPy_LINE:84
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:85
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:86
    LDD #3
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:87
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:88
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

I_BRIGHT EQU 127
I_MEDIUM EQU 80
I_DIM EQU 48
SCREEN_LEFT EQU 65436
SCREEN_RIGHT EQU 100
SCREEN_TOP EQU 80
SCREEN_BOTTOM EQU 65456
BALL_RADIUS EQU 8
DRAW_BALL: ; function
; --- function draw_ball ---
    LDD VAR_ARG0
    STD VAR_X
    LDD VAR_ARG1
    STD VAR_Y
    LDD VAR_ARG2
    STD VAR_INTENSITY
    ; VPy_LINE:33
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
; NATIVE_CALL: DRAW_CIRCLE at line 33
    JSR DRAW_CIRCLE
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:36
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:37
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

UPDATE_BALL_POSITION: ; function
; --- function update_ball_position ---
    ; VPy_LINE:41
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:42
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:45
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:46
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:47
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; VPy_LINE:49
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:50
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:51
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; VPy_LINE:54
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:55
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:56
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; VPy_LINE:58
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    LBEQ IF_NEXT_13
    ; VPy_LINE:59
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_BALL_RADIUS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:60
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_VEL_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    RTS

DRAW_BORDERS: ; function
; --- function draw_borders ---
    ; VPy_LINE:65
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_I_DIM
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:67
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_I_DIM
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:69
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_SCREEN_LEFT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_I_DIM
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:71
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_SCREEN_TOP
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_SCREEN_RIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_SCREEN_BOTTOM
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD VAR_I_DIM
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

DRAW_INFO: ; function
; --- function draw_info ---
    ; VPy_LINE:75
    LDD #65446
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 75
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:76
    LDD #65446
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65466
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 76
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:79
    LDD VAR_FRAME_COUNT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #63
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ANDA TMPRIGHT+1
    ANDB TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #32
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:80
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 80
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    RTS

LOOP_BODY:
    ; DEBUG: Processing 7 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    ; VPy_LINE:91
    LDD VAR_FRAME_COUNT
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
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:94
    JSR UPDATE_BALL_POSITION
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:97
    JSR DRAW_BORDERS
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:98
    JSR DRAW_INFO
    ; DEBUG: Statement 4 - Discriminant(1)
    ; VPy_LINE:101
    LDD VAR_I_BRIGHT
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(7)
    ; VPy_LINE:102
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_24
    LDD #0
    STD RESULT
    BRA CE_25
CT_24:
    LDD #1
    STD RESULT
CE_25:
    LDD RESULT
    BEQ AND_FALSE_26
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_28
    LDD #0
    STD RESULT
    BRA CE_29
CT_28:
    LDD #1
    STD RESULT
CE_29:
    LDD RESULT
    BEQ AND_FALSE_26
    LDD #1
    STD RESULT
    BRA AND_END_27
AND_FALSE_26:
    LDD #0
    STD RESULT
AND_END_27:
    LDD RESULT
    BNE OR_TRUE_22
    LDD VAR_VEL_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    BEQ AND_FALSE_32
    LDD VAR_VEL_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    BEQ AND_FALSE_32
    LDD #1
    STD RESULT
    BRA AND_END_33
AND_FALSE_32:
    LDD #0
    STD RESULT
AND_END_33:
    LDD RESULT
    BNE OR_TRUE_22
    LDD #0
    STD RESULT
    BRA OR_END_23
OR_TRUE_22:
    LDD #1
    STD RESULT
OR_END_23:
    LDD RESULT
    LBEQ IF_NEXT_21
    ; VPy_LINE:103
    LDD VAR_I_MEDIUM
    STD RESULT
    LDX RESULT
    LDU #VAR_BALL_INTENSITY
    STU TMPPTR
    STX ,U
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:106
    LDD VAR_BALL_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_BALL_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_BALL_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR DRAW_BALL
    RTS

VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
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
VAR_BALL_RADIUS EQU $CF00+0
VAR_I_BRIGHT EQU $CF00+2
VAR_I_DIM EQU $CF00+4
VAR_I_MEDIUM EQU $CF00+6
VAR_SCREEN_BOTTOM EQU $CF00+8
VAR_SCREEN_LEFT EQU $CF00+10
VAR_SCREEN_RIGHT EQU $CF00+12
VAR_SCREEN_TOP EQU $CF00+14
VAR_BALL_INTENSITY EQU $CF00+16
VAR_BALL_X EQU $CF00+18
VAR_BALL_Y EQU $CF00+20
VAR_FRAME_COUNT EQU $CF00+22
VAR_INTENSITY EQU $CF00+24
VAR_VEL_X EQU $CF00+26
VAR_VEL_Y EQU $CF00+28
VAR_X EQU $CF00+30
VAR_Y EQU $CF00+32
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "BOUNCING BALL"
    FCB $80
STR_1:
    FCC "DEMO"
    FCB $80
STR_2:
    FCC "FRAME"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+34
VLINE_DY EQU RESULT+35
VLINE_STEPS EQU RESULT+36
VLINE_LIST EQU RESULT+37
